use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use ortalib::{Card, Chips, Enhancement, Mult, PokerHand, Rank, Suit, SuitColor};
use std::collections::HashMap;

// file contains helper functions for cards and hands related use

// takes a rank and returns its order value
// used for determining straights
pub fn rank_to_order(rank: &Rank, is_low_ace: bool) -> usize {
    if is_low_ace {
        match rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    } else {
        match rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

// Gets base score of pokerhand
pub fn get_base_score(hand: PokerHand) -> (Chips, Mult) {
    match hand {
        PokerHand::HighCard => (5.0, 1.0),
        PokerHand::Pair => (10.0, 2.0),
        PokerHand::TwoPair => (20.0, 2.0),
        PokerHand::ThreeOfAKind => (30.0, 3.0),
        PokerHand::Straight => (30.0, 4.0),
        PokerHand::Flush => (35.0, 4.0),
        PokerHand::FullHouse => (40.0, 4.0),
        PokerHand::FourOfAKind => (60.0, 7.0),
        PokerHand::StraightFlush => (100.0, 8.0),
        PokerHand::FiveOfAKind => (120.0, 12.0),
        PokerHand::FlushHouse => (140.0, 14.0),
        PokerHand::FlushFive => (160.0, 16.0),
    }
}

// helper function for finding straight takes sorted list of rank order values..
// (see rank_to_order) and returns vec of cards if there is a consecutive sequence..
//  of specified length
pub fn find_consecutive_sequence(
    sorted_ranks: &[usize],
    length: usize,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    // try to find consecutive sequence with Ace as high (14)
    if let Some(straight) =
        find_sequence_with_ace(sorted_ranks, length, cards, false, joker_effects)
    {
        return Some(straight);
    }

    // if no sequence is found and there is an Ace (14), try again with Ace as low (1)
    if sorted_ranks.contains(&14) {
        let mut adjusted_ranks = sorted_ranks.to_vec();
        // remove Ace (14)
        adjusted_ranks.retain(|&rank| rank != 14);
        // prepend Ace as low (1)
        adjusted_ranks.insert(0, 1);

        // try find a consecutive sequence with Ace as low (1)
        if let Some(straight) =
            find_sequence_with_ace(&adjusted_ranks, length, cards, true, joker_effects)
        {
            return Some(straight);
        }
    }

    None
}

// helper helper function to find consecutive sequences using Ace as either high (14) or low (1)
fn find_sequence_with_ace(
    sorted_ranks: &[usize],
    length: usize,
    cards: &[Card],
    is_low_ace: bool,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    // check all windows of specified length for consecutive sequence
    for window in sorted_ranks.windows(length) {
        // if shortcut joker active then allow at most 1 rank gap
        // otherwise accept consecutive numbers only
        let is_valid_straight = if joker_effects.shortcut {
            window.windows(2).all(|w| (w[1] - w[0]) <= 2)
        } else {
            window.windows(2).all(|w| w[1] == w[0] + 1)
        };

        // If window is valid straight then create and return list of cards..
        // by going through list of cards and checking if its rank order value..
        // is in current window
        if is_valid_straight {
            let mut straight_cards = vec![];

            for card in cards {
                let rank_value = rank_to_order(&card.rank, is_low_ace);

                if window.contains(&rank_value) {
                    straight_cards.push(card.clone());

                    if straight_cards.len() == length {
                        return Some(straight_cards);
                    }
                }
            }
        }
    }
    None
}

// helper function for finding flush in cards.
pub fn find_flush_cards(
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    cards: &[Card],
    required_cards: usize,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    // if smeared joker is in effect, then use color count hashmap to check for flush
    // otherwise use original suits count hashmap
    if joker_effects.smeared_joker {
        // create hashmap to group suits by color
        let mut color_count: HashMap<SuitColor, usize> = HashMap::new();

        for (suit, &count) in suit_count.iter() {
            let color = suit.color();
            *color_count.entry(color).or_insert(0) += count;
        }

        // check for flush using the color_count map
        if let Some((&color, &_)) = color_count
            .iter()
            .find(|&(_, &count)| count + wild_count >= required_cards)
        {
            // filter cards that belong to the color and wild cards
            let flush_cards: Vec<Card> = cards
                .iter()
                .filter(|card| {
                    card.suit.color() == color || card.enhancement == Some(Enhancement::Wild)
                })
                .cloned()
                .take(required_cards)
                .collect();

            return Some(flush_cards);
        }
    } else {
        if let Some((&suit, &_)) = suit_count
            .iter()
            .find(|&(_, &count)| count + wild_count >= required_cards)
        {
            return Some(
                cards
                    .iter()
                    .filter(|card| card.suit == suit || card.enhancement == Some(Enhancement::Wild))
                    .cloned()
                    .take(required_cards)
                    .collect(),
            );
        }
    }

    // handle case where all cards are wild and there are no suits
    if suit_count.is_empty() && wild_count >= required_cards {
        return Some(
            cards
                .iter()
                .filter(|card| card.enhancement == Some(Enhancement::Wild))
                .cloned()
                .take(required_cards)
                .collect(),
        );
    }

    None
}

// returns scoring cards accounting for if splash is in effect
pub fn get_scoring_cards<'a>(
    cards_played: &'a [Card],
    best_hand_cards: &'a [Card],
    joker_effect_flags: &JokerEffectFlags,
) -> &'a [Card] {
    if joker_effect_flags.splash {
        cards_played
    } else {
        best_hand_cards
    }
}
