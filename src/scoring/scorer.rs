use ortalib::{Card, Chips, Edition, Enhancement, Joker, Mult, PokerHand, Rank, Round, Suit};
use std::collections::HashMap;

use crate::scoring::card::card_util::get_base_score;
use crate::scoring::card::card_util::get_scoring_cards;
use crate::scoring::card::hands;
use crate::scoring::joker::joker_util::apply_blueprint_jokers;
use crate::scoring::joker::joker_util::get_joker_activation;
use crate::scoring::joker::joker_util::set_joker_effects;
use crate::scoring::joker::joker_wrappers::JokerActivation;
use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use crate::scoring::joker::joker_wrappers::JokerOnHeldWrapper;
use crate::scoring::joker::joker_wrappers::JokerOnScoredWrapper;
use crate::scoring::joker::joker_wrappers::JokerWrapper;

// Takes a round and calculates score in Chips and Mult
// Manages broad score calculation logic
pub fn calculate_score(round: Round) -> (Chips, Mult) {
    // extract cards frrom round
    let cards_played = round.cards_played;
    let cards_held_in_hand = round.cards_held_in_hand;
    let jokers = round.jokers;

    // set passive effect jokers flags
    let joker_effect_flags = set_joker_effects(&jokers);

    // modify jokers, accounting for blue_print jokers
    let joker_cards = apply_blueprint_jokers(&jokers);

    // wrap jokers so that joker effect functions can be implemented for them
    // <JokerWrapper> implements independent joker effects and edition bonuses for all jokers
    let wrapped_jokers: Vec<JokerWrapper> = joker_cards
        .clone()
        .into_iter()
        .map(|joker_card| JokerWrapper {
            joker_activation: get_joker_activation(&joker_card.joker),
            joker_card,
        })
        .collect();

    // <JokerWrapper> implements on held joker effects
    let on_held_jokers: Vec<JokerOnHeldWrapper> = joker_cards
        .clone()
        .into_iter()
        .filter(|joker_card| get_joker_activation(&joker_card.joker) == JokerActivation::OnHeld)
        .map(|joker_card| JokerOnHeldWrapper { joker_card })
        .collect();

    // <JokerWrapper> implements on scored joker effects
    let on_scored_jokers: Vec<JokerOnScoredWrapper> = joker_cards
        .clone()
        .into_iter()
        .filter(|joker_card| get_joker_activation(&joker_card.joker) == JokerActivation::OnScore)
        .map(|joker_card| JokerOnScoredWrapper { joker_card })
        .collect();

    // count number of cards for each suit and rank. Also count wild cards.
    // for purpose of finding best hand among played cards
    let mut rank_count = HashMap::new();
    let mut suit_count = HashMap::new();
    let mut wild_count = 0;

    for card in &cards_played {
        // count rank appearances
        *rank_count.entry(card.rank).or_insert(0) += 1;

        // if card is wild card then skip suit count entry
        if let Some(enhancement) = &card.enhancement {
            if *enhancement == Enhancement::Wild {
                wild_count += 1;
                continue;
            }
        }

        // count suit appearances
        *suit_count.entry(card.suit).or_insert(0) += 1;
    }

    // Get best hand
    let (best_hand, best_hand_cards) = find_best_hand(
        &cards_played,
        &rank_count,
        &suit_count,
        wild_count,
        &joker_effect_flags,
    );

    // Get base chips and mult according to best hand
    let (base_chips, base_mult) = get_base_score(best_hand);

    // Get scoring cards. Can be different depending on whether Splash joker is active
    let scoring_cards = get_scoring_cards(&cards_played, &best_hand_cards, &joker_effect_flags);

    // Get number of triggers for scored and in hand cards
    let scored_trigger = 1 + joker_cards
        .iter()
        .filter(|joker_card| matches!(joker_card.joker, Joker::SockAndBuskin))
        .count();

    let held_trigger = 1 + joker_cards
        .iter()
        .filter(|joker_card| matches!(joker_card.joker, Joker::Mime))
        .count();

    // Score scored cards
    let (played_chips, played_mult) = scoring_cards.iter().fold(
        (base_chips, base_mult),
        |(current_chips, current_mult), card| {
            let mut new_chips = current_chips;
            let mut new_mult = current_mult;

            // by Sock and Buskin retriggers depend on if card is a face
            // an active pareidolia makes all cards considered a face rank
            let repeat_count = if card.rank.is_face() || joker_effect_flags.pareidolia {
                scored_trigger
            } else {
                1
            };

            // apply card effects multiple times based on repeat_count
            for _ in 0..repeat_count {
                let (updated_chips, updated_mult) = apply_card(
                    card,
                    &scoring_cards,
                    &on_scored_jokers,
                    new_chips,
                    new_mult,
                    &joker_effect_flags,
                );
                new_chips = updated_chips;
                new_mult = updated_mult;
            }

            (new_chips, new_mult)
        },
    );

    // score held cards
    let (held_chips, held_mult) = cards_held_in_hand.iter().fold(
        (played_chips, played_mult),
        |(current_chips, current_mult), card| {
            let mut new_chips = current_chips;
            let mut new_mult = current_mult;

            for _ in 0..held_trigger {
                let (updated_chips, updated_mult) = apply_in_hand_card(
                    card,
                    &cards_held_in_hand,
                    &on_held_jokers,
                    new_chips,
                    new_mult,
                );
                new_chips = updated_chips;
                new_mult = updated_mult;
            }
            (new_chips, new_mult)
        },
    );

    // apply the effects of the independent jokers and edition bonuses on all jokers
    let (final_chips, final_mult) = wrapped_jokers.iter().fold(
        (held_chips, held_mult),
        |(current_chips, current_mult), joker| {
            joker.apply_effect(
                current_chips,
                current_mult,
                &rank_count,
                &suit_count,
                wild_count,
                &cards_played,
                &cards_held_in_hand,
                &best_hand_cards,
                &joker_cards,
                &joker_effect_flags,
            )
        },
    );

    // return final chips and mult
    (final_chips, final_mult)
}

// takes cards and various stats and returns best hand
pub fn find_best_hand(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    joker_effects: &JokerEffectFlags,
) -> (PokerHand, Vec<Card>) {
    // check for best hand starting from highest scoring

    // check flush five
    if let Some(flush_five_cards) =
        hands::get_flush_five_cards(cards, rank_count, suit_count, wild_count, joker_effects)
    {
        return (PokerHand::FlushFive, flush_five_cards);
    }
    // check flush house
    else if let Some(flush_house_cards) =
        hands::get_flush_house_cards(cards, rank_count, suit_count, wild_count, joker_effects)
    {
        return (PokerHand::FlushHouse, flush_house_cards);
    }
    // check five of a kind
    else if let Some(five_of_a_kind_cards) =
        hands::get_five_of_a_kind_cards(cards, rank_count, suit_count)
    {
        return (PokerHand::FiveOfAKind, five_of_a_kind_cards);
    }
    // check straight flush
    else if let Some(straight_flush_cards) =
        hands::get_straight_flush_cards(cards, rank_count, suit_count, wild_count, joker_effects)
    {
        return (PokerHand::StraightFlush, straight_flush_cards);
    // check four of a kind
    } else if let Some(four_cards) = hands::get_four_of_a_kind_cards(cards, rank_count) {
        return (PokerHand::FourOfAKind, four_cards);
    // check full house
    } else if let Some(full_house_cards) = hands::get_full_house_cards(cards, rank_count) {
        return (PokerHand::FullHouse, full_house_cards);
    // check flush
    } else if let Some(flush_cards) =
        hands::get_flush_cards(cards, suit_count, wild_count, joker_effects)
    {
        return (PokerHand::Flush, flush_cards);
    // check straight
    } else if let Some(straight_cards) = hands::get_straight_cards(cards, rank_count, joker_effects)
    {
        return (PokerHand::Straight, straight_cards);
    // check three of a kind
    } else if let Some(three_cards) = hands::get_three_of_a_kind_cards(cards, rank_count) {
        return (PokerHand::ThreeOfAKind, three_cards);
    // check two pairs
    } else if let Some(two_pair_cards) = hands::get_two_pair_cards(cards, rank_count) {
        return (PokerHand::TwoPair, two_pair_cards);
    // check pair
    } else if let Some(pair_cards) = hands::get_pair_cards(cards, rank_count) {
        return (PokerHand::Pair, pair_cards);
    // check high card
    } else if let Some(high_cards) = hands::get_high_card(cards) {
        return (PokerHand::HighCard, high_cards);
    }

    (PokerHand::HighCard, vec![])
}

// function applies base value, enhancements, editions and jokers for scored card
fn apply_card(
    card: &Card,
    cards: &[Card],
    on_scored_jokers: &[JokerOnScoredWrapper],
    chips: f64,
    mult: f64,
    joker_effects: &JokerEffectFlags,
) -> (f64, f64) {
    let mut updated_chips = chips;
    let mut updated_mult = mult;

    // apply card's rank value to chips
    updated_chips += card.rank.rank_value();

    // apply enhancement
    if let Some(enhancement) = &card.enhancement {
        match enhancement {
            Enhancement::Bonus => {
                updated_chips += 30.0;
            }
            Enhancement::Mult => {
                updated_mult += 4.0;
            }
            Enhancement::Glass => {
                updated_mult *= 2.0;
            }
            _ => {}
        }
    }

    // apply edition
    if let Some(edition) = &card.edition {
        match edition {
            Edition::Foil => {
                updated_chips += 50.0;
            }
            Edition::Holographic => {
                updated_mult += 10.0;
            }
            Edition::Polychrome => {
                updated_mult *= 1.5;
            }
        }
    }

    // apply "on scored" jokers
    for joker in on_scored_jokers {
        let (new_chips, new_mult) =
            joker.apply_effect(card, cards, updated_chips, updated_mult, joker_effects);
        updated_chips = new_chips;
        updated_mult = new_mult;
    }

    // Return the updated chips and multiplier
    (updated_chips, updated_mult)
}

// function applies enhancements and jokers for in hand card
fn apply_in_hand_card(
    card: &Card,
    cards: &[Card],
    on_held_jokers: &[JokerOnHeldWrapper],
    chips: f64,
    mult: f64,
) -> (f64, f64) {
    let mut updated_mult = mult;
    let mut updated_chips = chips;

    // apply enhancement
    if let Some(enhancement) = &card.enhancement {
        match enhancement {
            Enhancement::Steel => {
                updated_mult *= 1.5;
            }
            _ => {}
        }
    }

    // apply "on held" jokers
    for joker in on_held_jokers {
        let (new_chips, new_mult) = joker.apply_effect(card, cards, updated_chips, updated_mult);
        updated_chips = new_chips;
        updated_mult = new_mult;
    }

    // return the updated chips and multiplier
    (updated_chips, updated_mult)
}
