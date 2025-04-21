use ortalib::{Card, Edition, Joker, JokerCard, Rank, Suit};
use std::collections::HashMap;

use crate::scoring::joker::on_held_joker as OnHeld;
use crate::scoring::joker::on_independent_joker as Independent;
use crate::scoring::joker::on_scored_joker as OnScored;

// File contains custom data types to help implement joker effects

// define joker card activation types
#[derive(PartialEq)]
pub enum JokerActivation {
    Independent,
    OnScore,
    OnHeld,
    Other,
    Copy,
}

// general joker card wrapper.
// used for triggering independent joker effects and..
// edition bonuses for all jokers
pub struct JokerWrapper {
    pub joker_card: JokerCard,
    pub joker_activation: JokerActivation,
}

// joker card wrapper for joker cards of "on held" activation
// used for triggering "on held" joker effects
pub struct JokerOnHeldWrapper {
    pub joker_card: JokerCard,
}

// joker card wrapper for joker cards of "on scored" activation
// used for triggering "on scored" joker effects
pub struct JokerOnScoredWrapper {
    pub joker_card: JokerCard,
}

// container for passive joker effect flags
// used to determine whether a passive joker effect is active
#[derive(Default)]
pub struct JokerEffectFlags {
    pub four_fingers: bool,
    pub shortcut: bool,
    pub pareidolia: bool,
    pub splash: bool,
    pub smeared_joker: bool,
}

// define effect function for general joker wrapper
// returns updated chips and mult after applying joker effect
impl JokerWrapper {
    pub fn apply_effect(
        &self,
        chips: f64,
        mult: f64,
        rank_count: &HashMap<Rank, usize>,
        suit_count: &HashMap<Suit, usize>,
        wild_count: usize,
        cards: &[Card],
        cards_in_hand: &[Card],
        cards_scored: &[Card],
        joker_cards: &[JokerCard],
        joker_effects: &JokerEffectFlags,
    ) -> (f64, f64) {
        let joker_card = &self.joker_card;
        let mut updated_chips = chips;
        let mut updated_mult = mult;

        // apply foil and holographic edition bonuses
        if let Some(edition) = joker_card.edition {
            match edition {
                Edition::Foil => {
                    updated_chips += 50.0;
                }
                Edition::Holographic => {
                    updated_mult += 10.0;
                }
                _ => {}
            }
        }

        // only apply joker effect if joker activation is Independent
        if self.joker_activation == JokerActivation::Independent {
            // match joker to specific joker effect function
            match joker_card.joker {
                Joker::Joker => Independent::joker_effect(&mut updated_mult),
                Joker::JollyJoker => {
                    Independent::jolly_joker_effect(&mut updated_mult, rank_count, cards)
                }
                Joker::ZanyJoker => {
                    Independent::zany_joker_effect(&mut updated_mult, rank_count, cards)
                }
                Joker::MadJoker => {
                    Independent::mad_joker_effect(&mut updated_mult, rank_count, cards)
                }
                Joker::CrazyJoker => Independent::crazy_joker_effect(
                    &mut updated_mult,
                    rank_count,
                    cards,
                    joker_effects,
                ),
                Joker::DrollJoker => Independent::droll_joker_effect(
                    &mut updated_mult,
                    suit_count,
                    wild_count,
                    cards,
                    joker_effects,
                ),
                Joker::SlyJoker => {
                    Independent::sly_joker_effect(&mut updated_chips, rank_count, cards)
                }
                Joker::WilyJoker => {
                    Independent::wily_joker_effect(&mut updated_chips, rank_count, cards)
                }
                Joker::CleverJoker => {
                    Independent::clever_joker_effect(&mut updated_chips, rank_count, cards)
                }
                Joker::DeviousJoker => Independent::devious_joker_effect(
                    &mut updated_chips,
                    rank_count,
                    cards,
                    joker_effects,
                ),
                Joker::CraftyJoker => Independent::crafty_joker_effect(
                    &mut updated_chips,
                    suit_count,
                    wild_count,
                    cards,
                    joker_effects,
                ),
                Joker::AbstractJoker => {
                    Independent::abstract_joker_effect(&mut updated_mult, joker_cards)
                }
                Joker::Blackboard => {
                    Independent::blackboard_effect(&mut updated_mult, cards_in_hand)
                }
                Joker::FlowerPot => {
                    Independent::flower_pot_effect(&mut updated_mult, cards_scored, joker_effects)
                }
                _ => {}
            }
        }

        // apply polychrome edition bonus
        if let Some(edition) = joker_card.edition {
            match edition {
                Edition::Polychrome => {
                    updated_mult *= 1.5;
                }
                _ => {}
            }
        }

        (updated_chips, updated_mult)
    }
}

// define effect function for "on held" joker wrapper
// returns updated chips and mult after applying joker effect
impl JokerOnHeldWrapper {
    pub fn apply_effect(&self, card: &Card, cards: &[Card], chips: f64, mult: f64) -> (f64, f64) {
        let joker_card = &self.joker_card;
        let updated_chips = chips;
        let mut updated_mult = mult;

        // match joker to specific joker effect function
        match joker_card.joker {
            Joker::RaisedFist => OnHeld::raised_fist_effect(&mut updated_mult, card, cards),
            Joker::Baron => OnHeld::baron_effect(&mut updated_mult, card),
            _ => {}
        }

        (updated_chips, updated_mult)
    }
}

// define effect function for "on scored" joker wrapper
// returns updated chips and mult after applying joker effect
impl JokerOnScoredWrapper {
    pub fn apply_effect(
        &self,
        card: &Card,
        cards: &[Card],
        chips: f64,
        mult: f64,
        joker_effects: &JokerEffectFlags,
    ) -> (f64, f64) {
        let joker_card = &self.joker_card;
        let mut updated_chips = chips;
        let mut updated_mult = mult;

        // match joker to specific joker effect function
        match joker_card.joker {
            Joker::GreedyJoker => {
                OnScored::greedy_joker_effect(&mut updated_mult, card, joker_effects)
            }
            Joker::LustyJoker => {
                OnScored::lusty_joker_effect(&mut updated_mult, card, joker_effects)
            }
            Joker::WrathfulJoker => {
                OnScored::wrathful_joker_effect(&mut updated_mult, card, joker_effects)
            }
            Joker::GluttonousJoker => {
                OnScored::gluttonous_joker_effect(&mut updated_mult, card, joker_effects)
            }
            Joker::Fibonacci => OnScored::fibonacci_effect(&mut updated_mult, card),
            Joker::ScaryFace => {
                OnScored::scary_face_effect(&mut updated_chips, card, joker_effects)
            }
            Joker::EvenSteven => OnScored::even_steven_effect(&mut updated_mult, card),
            Joker::OddTodd => OnScored::odd_todd_effect(&mut updated_chips, card),
            Joker::Photograph => {
                OnScored::photograph_effect(&mut updated_mult, card, cards, joker_effects)
            }
            Joker::SmileyFace => {
                OnScored::smiley_face_effect(&mut updated_mult, card, joker_effects)
            }
            _ => {}
        }

        (updated_chips, updated_mult)
    }
}
