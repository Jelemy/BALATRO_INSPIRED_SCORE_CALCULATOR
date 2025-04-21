use crate::scoring::card::card_util::find_consecutive_sequence;
use crate::scoring::card::card_util::find_flush_cards;
use crate::scoring::card::card_util::rank_to_order;
use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use ortalib::{Card, Rank, Suit};
use std::collections::HashMap;

// File contains functions that takes a list/vec of cards and returns..
// a specific pokerhand.

pub fn get_flush_five_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    if rank_count.len() == 1 {
        if let Some(_) = get_flush_cards(cards, suit_count, wild_count, joker_effects) {
            return Some(cards.to_vec());
        }
    }
    None
}

pub fn get_flush_house_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    if let Some(_) = get_full_house_cards(cards, rank_count) {
        if let Some(_) = get_flush_cards(cards, suit_count, wild_count, joker_effects) {
            return Some(cards.to_vec());
        }
    }
    None
}

pub fn get_five_of_a_kind_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    suit_count: &HashMap<Suit, usize>,
) -> Option<Vec<Card>> {
    if rank_count.values().any(|&count| count == 5) && suit_count.len() > 1 {
        return Some(cards.to_vec());
    }
    None
}

pub fn get_straight_flush_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    // get flush cards
    if let Some(flush_cards) = get_flush_cards(cards, suit_count, wild_count, joker_effects) {
        // get straight cards
        if let Some(straight_cards) = get_straight_cards(cards, rank_count, joker_effects) {
            // if four_fingers is inactive then return cards as is
            if !joker_effects.four_fingers {
                return Some(cards.to_vec());
            }
            // if four_fingers is active and we have 4 cards from both flush and straight
            if joker_effects.four_fingers {
                if flush_cards.len() == 4 && straight_cards.len() == 4 {
                    // check if they have the same cards
                    let common_cards: Vec<_> = flush_cards
                        .iter()
                        .filter(|&&card| straight_cards.contains(&card))
                        .cloned()
                        .collect();

                    // if they are the same, return the 4 cards
                    if common_cards.len() == 4 {
                        return Some(common_cards);
                    // Otherwise return all cards
                    } else {
                        return Some(cards.to_vec());
                    }
                } else {
                    return Some(cards.to_vec());
                }
            }
        }
    }

    None
}

pub fn get_four_of_a_kind_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
) -> Option<Vec<Card>> {
    if let Some(four_kind_rank) = rank_count
        .iter()
        .find(|&(_, &count)| count >= 4)
        .map(|(rank, _)| rank)
    {
        let four_of_a_kind_cards: Vec<Card> = cards
            .iter()
            .filter(|&card| &card.rank == four_kind_rank)
            .take(4)
            .cloned()
            .collect();

        return Some(four_of_a_kind_cards);
    }
    None
}

pub fn get_full_house_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
) -> Option<Vec<Card>> {
    if rank_count.len() != 2 {
        return None;
    }

    let mut has_triplet = false;
    let mut has_pair = false;

    for (_, &count) in rank_count.iter() {
        if count == 3 {
            has_triplet = true;
        } else if count == 2 {
            has_pair = true;
        }
    }

    if has_triplet && has_pair {
        return Some(cards.to_vec());
    }
    None
}

pub fn get_flush_cards(
    cards: &[Card],
    suit_count: &HashMap<Suit, usize>,
    wild_count: usize,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    // Try to find a flush with 5 cards first
    if let Some(flush) = find_flush_cards(suit_count, wild_count, cards, 5, joker_effects) {
        return Some(flush);
    }

    // If no 5-card flush found, check for 4-card flush if four_fingers flag is set
    if joker_effects.four_fingers {
        if let Some(flush) = find_flush_cards(suit_count, wild_count, cards, 4, joker_effects) {
            return Some(flush);
        }
    }

    None
}

pub fn get_straight_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
    joker_effects: &JokerEffectFlags,
) -> Option<Vec<Card>> {
    let mut rank_values: Vec<usize> = rank_count
        .keys()
        .map(|rank| rank_to_order(rank, false))
        .collect();
    rank_values.sort();

    if let Some(straight) = find_consecutive_sequence(&rank_values, 5, cards, joker_effects) {
        return Some(straight);
    }

    // if cant find 5 straight check if four fingers active and try find 4 straight instead
    if joker_effects.four_fingers {
        if let Some(straight) = find_consecutive_sequence(&rank_values, 4, cards, joker_effects) {
            return Some(straight);
        }
    }

    None
}

pub fn get_three_of_a_kind_cards(
    cards: &[Card],
    rank_count: &HashMap<Rank, usize>,
) -> Option<Vec<Card>> {
    if let Some(three_kind_rank) = rank_count
        .iter()
        .find(|&(_, &count)| count >= 3)
        .map(|(rank, _)| rank)
    {
        let three_of_a_kind_cards: Vec<Card> = cards
            .iter()
            .filter(|&card| &card.rank == three_kind_rank)
            .take(3)
            .cloned()
            .collect();

        return Some(three_of_a_kind_cards);
    }
    None
}

pub fn get_two_pair_cards(cards: &[Card], rank_count: &HashMap<Rank, usize>) -> Option<Vec<Card>> {
    let pairs: Vec<Rank> = rank_count
        .iter()
        .filter(|&(_, &count)| count == 2)
        .map(|(rank, _)| *rank)
        .collect();

    // contunue if 2 pairs found
    if pairs.len() == 2 {
        let mut two_pair_cards = Vec::new();

        // add the two cards for each pair
        for pair_rank in pairs {
            let pair_cards: Vec<Card> = cards
                .iter()
                .filter(|&card| &card.rank == &pair_rank)
                .take(2)
                .cloned()
                .collect();

            two_pair_cards.extend(pair_cards);
        }

        return Some(two_pair_cards);
    }

    None
}

pub fn get_pair_cards(cards: &[Card], rank_count: &HashMap<Rank, usize>) -> Option<Vec<Card>> {
    let pair_rank = rank_count
        .iter()
        .find(|&(_, &count)| count >= 2)
        .map(|(rank, _)| *rank);

    if let Some(pair_rank) = pair_rank {
        let pair_cards: Vec<Card> = cards
            .iter()
            .filter(|&card| &card.rank == &pair_rank)
            .take(2)
            .cloned()
            .collect();

        return Some(pair_cards);
    }

    None
}

pub fn get_high_card(cards: &[Card]) -> Option<Vec<Card>> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by_key(|card| rank_to_order(&card.rank, false));
    sorted_cards.reverse();

    if sorted_cards.is_empty() {
        None
    } else {
        Some(vec![sorted_cards[0].clone()])
    }
}
