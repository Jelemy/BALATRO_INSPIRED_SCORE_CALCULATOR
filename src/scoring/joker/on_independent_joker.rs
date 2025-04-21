use crate::scoring::card::hands;
use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use ortalib::{Card, Enhancement, JokerCard, Rank, Suit, SuitColor};
use std::collections::HashMap;
use std::collections::HashSet;

// File contains joker effect functions for specifc "independent" joker cards

pub fn joker_effect(mult: &mut f64) {
    *mult += 4.0;
}

pub fn jolly_joker_effect(mult: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_pair_cards(cards, rank_count).is_some() {
        *mult += 8.0;
    }
}

pub fn zany_joker_effect(mult: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_three_of_a_kind_cards(cards, rank_count).is_some() {
        *mult += 12.0;
    }
}

pub fn mad_joker_effect(mult: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_two_pair_cards(cards, rank_count).is_some() {
        *mult += 10.0;
    }
}

pub fn crazy_joker_effect(
    mult: &mut f64,
    rank_count: &HashMap<Rank, usize>,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) {
    if hands::get_straight_cards(cards, rank_count, joker_effects).is_some() {
        *mult += 12.0;
    }
}

pub fn droll_joker_effect(
    mult: &mut f64,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) {
    if hands::get_flush_cards(cards, suit_count, wild_count, joker_effects).is_some() {
        *mult += 10.0;
    }
}

pub fn sly_joker_effect(chips: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_pair_cards(cards, rank_count).is_some() {
        *chips += 50.0;
    }
}

pub fn wily_joker_effect(chips: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_three_of_a_kind_cards(cards, rank_count).is_some() {
        *chips += 100.0;
    }
}

pub fn clever_joker_effect(chips: &mut f64, rank_count: &HashMap<Rank, usize>, cards: &[Card]) {
    if hands::get_two_pair_cards(cards, rank_count).is_some() {
        *chips += 80.0;
    }
}

pub fn devious_joker_effect(
    chips: &mut f64,
    rank_count: &HashMap<Rank, usize>,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) {
    if hands::get_straight_cards(cards, rank_count, joker_effects).is_some() {
        *chips += 100.0;
    }
}

pub fn crafty_joker_effect(
    chips: &mut f64,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) {
    if hands::get_flush_cards(cards, suit_count, wild_count, joker_effects).is_some() {
        *chips += 80.0;
    }
}

pub fn abstract_joker_effect(mult: &mut f64, jokers: &[JokerCard]) {
    *mult += 3.0 * jokers.len() as f64;
}

pub fn blackboard_effect(mult: &mut f64, cards: &[Card]) {
    if cards.is_empty() {
        return;
    }

    if cards.iter().all(|card| {
        matches!(card.suit, Suit::Spades | Suit::Clubs)
            || card.enhancement == Some(Enhancement::Wild)
    }) {
        *mult *= 3.0;
    }
}

pub fn flower_pot_effect(mult: &mut f64, cards: &[Card], joker_effect: &JokerEffectFlags) {
    if cards.len() < 4 {
        return;
    }

    // number of valid cards that can contribute to flower pot (4 unique suits)
    let mut valid_cards = 0;
    // number of wild counts
    let mut wild_count = 0;

    if joker_effect.smeared_joker {
        let mut red_count = 0;
        let mut black_count = 0;
        for card in cards {
            if let Some(Enhancement::Wild) = card.enhancement {
                wild_count += 1;
            } else {
                match card.suit.color() {
                    // Ensure color counts does not exceed 2
                    // e.g 3 reds and 1 black can't make a flowerpot
                    SuitColor::Red => {
                        if red_count < 2 {
                            red_count += 1;
                        }
                    }
                    SuitColor::Black => {
                        if black_count < 2 {
                            black_count += 1;
                        }
                    }
                }
            }
        }

        valid_cards += red_count + black_count;
    } else {
        let mut suit_set = HashSet::new();

        for card in cards {
            if let Some(Enhancement::Wild) = card.enhancement {
                wild_count += 1;
            } else {
                suit_set.insert(card.suit);
            }
        }

        valid_cards += suit_set.len();
    }

    // if wilds can fill in for missing suits, or if number of valid cards are enough..
    // apply the effect
    if wild_count + valid_cards >= 4 {
        *mult *= 3.0;
    }
}
