use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use ortalib::{Card, Enhancement, Rank, Suit, SuitColor};

// File contains joker effect functions for specifc "on_scored" joker cards

pub fn greedy_joker_effect(mult: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if joker_effects.smeared_joker {
        // smear effect means red suits are considered the same
        if card.suit.color() == SuitColor::Red || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    } else {
        // trigger only for diamonds normally
        if card.suit == Suit::Diamonds || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    }
}

pub fn lusty_joker_effect(mult: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if joker_effects.smeared_joker {
        // smear effect means red suits are considered the same
        if card.suit.color() == SuitColor::Red || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    } else {
        // trigger only for hearts normally
        if card.suit == Suit::Hearts || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    }
}

pub fn wrathful_joker_effect(mult: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if joker_effects.smeared_joker {
        // smear effect means black suits are considered the same
        if card.suit.color() == SuitColor::Black || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    } else {
        // trigger only for spades normally
        if card.suit == Suit::Spades || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    }
}

pub fn gluttonous_joker_effect(mult: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if joker_effects.smeared_joker {
        // smear effect means black suits are considered the same
        if card.suit.color() == SuitColor::Black || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    } else {
        // trigger only for clubs normally
        if card.suit == Suit::Clubs || card.enhancement == Some(Enhancement::Wild) {
            *mult += 3.0;
        }
    }
}

pub fn fibonacci_effect(mult: &mut f64, card: &Card) {
    if matches!(card.rank, Rank::Ace | Rank::Two | Rank::Five | Rank::Eight) {
        *mult += 8.0;
    }
}

pub fn scary_face_effect(chips: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if card.rank.is_face() || joker_effects.pareidolia {
        *chips += 30.0;
    }
}

pub fn even_steven_effect(mult: &mut f64, card: &Card) {
    if matches!(
        card.rank,
        Rank::Ten | Rank::Eight | Rank::Six | Rank::Four | Rank::Two
    ) {
        *mult += 4.0;
    }
}

pub fn odd_todd_effect(chips: &mut f64, card: &Card) {
    if matches!(
        card.rank,
        Rank::Ace | Rank::Nine | Rank::Seven | Rank::Five | Rank::Three
    ) {
        *chips += 31.0;
    }
}

pub fn photograph_effect(
    mult: &mut f64,
    card: &Card,
    cards: &[Card],
    joker_effects: &JokerEffectFlags,
) {
    if joker_effects.pareidolia {
        // with Pareidolia, every card is considered a face card
        if let Some(first_card) = cards.first() {
            if first_card as *const _ == card as *const _ {
                *mult *= 2.0;
            }
        }
    } else {
        // normally find the first actual face card
        if let Some(first_face_card) = cards.iter().find(|c| c.rank.is_face()) {
            if first_face_card as *const _ == card as *const _ {
                *mult *= 2.0;
            }
        }
    }
}

pub fn smiley_face_effect(mult: &mut f64, card: &Card, joker_effects: &JokerEffectFlags) {
    if card.rank.is_face() || joker_effects.pareidolia {
        *mult += 5.0;
    }
}
