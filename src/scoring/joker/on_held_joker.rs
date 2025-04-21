use crate::scoring::card::card_util::rank_to_order;
use ortalib::{Card, Rank};

// File contains joker effect functions for specifc "on held" joker cards

pub fn raised_fist_effect(mult: &mut f64, card: &Card, cards: &[Card]) {
    if cards.is_empty() {
        return;
    }

    // find lowest rank
    let min_rank = cards
        .iter()
        .map(|c| rank_to_order(&c.rank, false))
        .min()
        .unwrap();

    // get the right-most card with the lowest rank
    let rightmost_lowest_card = cards
        .iter()
        .rev()
        .find(|c| rank_to_order(&c.rank, false) == min_rank);

    // update mult if the given card is the right-most lowest-ranked card
    if let Some(lowest_card) = rightmost_lowest_card {
        if lowest_card == card {
            *mult += 2.0 * card.rank.rank_value();
        }
    }
}

pub fn baron_effect(mult: &mut f64, card: &Card) {
    if card.rank == Rank::King {
        *mult *= 1.5;
    }
}
