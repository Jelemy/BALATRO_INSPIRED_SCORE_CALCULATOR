use crate::scoring::joker::joker_wrappers::JokerActivation;
use crate::scoring::joker::joker_wrappers::JokerEffectFlags;
use ortalib::{Joker, JokerCard};

// file contains joker related helper functions

// given a joker, return its activation, independent, on score, on held or other
pub fn get_joker_activation(joker: &Joker) -> JokerActivation {
    match joker {
        // Independent
        Joker::Joker => JokerActivation::Independent,
        Joker::JollyJoker => JokerActivation::Independent,
        Joker::ZanyJoker => JokerActivation::Independent,
        Joker::MadJoker => JokerActivation::Independent,
        Joker::CrazyJoker => JokerActivation::Independent,
        Joker::DrollJoker => JokerActivation::Independent,
        Joker::SlyJoker => JokerActivation::Independent,
        Joker::WilyJoker => JokerActivation::Independent,
        Joker::CleverJoker => JokerActivation::Independent,
        Joker::DeviousJoker => JokerActivation::Independent,
        Joker::CraftyJoker => JokerActivation::Independent,
        Joker::AbstractJoker => JokerActivation::Independent,
        Joker::Blackboard => JokerActivation::Independent,
        Joker::FlowerPot => JokerActivation::Independent,

        // On Score
        Joker::GreedyJoker => JokerActivation::OnScore,
        Joker::LustyJoker => JokerActivation::OnScore,
        Joker::WrathfulJoker => JokerActivation::OnScore,
        Joker::GluttonousJoker => JokerActivation::OnScore,
        Joker::Fibonacci => JokerActivation::OnScore,
        Joker::ScaryFace => JokerActivation::OnScore,
        Joker::EvenSteven => JokerActivation::OnScore,
        Joker::OddTodd => JokerActivation::OnScore,
        Joker::Photograph => JokerActivation::OnScore,
        Joker::SmileyFace => JokerActivation::OnScore,
        Joker::SockAndBuskin => JokerActivation::OnScore,

        // On Held
        Joker::RaisedFist => JokerActivation::OnHeld,
        Joker::Baron => JokerActivation::OnHeld,
        Joker::Mime => JokerActivation::OnHeld,

        // Other
        Joker::FourFingers => JokerActivation::Other,
        Joker::Shortcut => JokerActivation::Other,
        Joker::Pareidolia => JokerActivation::Other,
        Joker::Splash => JokerActivation::Other,
        Joker::SmearedJoker => JokerActivation::Other,

        // Copy
        Joker::Blueprint => JokerActivation::Copy,
    }
}

// Goes through list of jokers and sets flags for passive joker effects.
// returns struct with flags
pub fn set_joker_effects(jokers: &[JokerCard]) -> JokerEffectFlags {
    let mut flags = JokerEffectFlags::default();

    for joker in jokers {
        match joker.joker {
            Joker::FourFingers => flags.four_fingers = true,
            Joker::Shortcut => flags.shortcut = true,
            Joker::Pareidolia => flags.pareidolia = true,
            Joker::Splash => flags.splash = true,
            Joker::SmearedJoker => flags.smeared_joker = true,
            _ => {}
        }
    }

    flags
}

// takes a list of joker cards and returns a modified version influenced by..
// blue print jokers.
pub fn apply_blueprint_jokers(joker_cards: &[JokerCard]) -> Vec<JokerCard> {
    let mut resolved_jokers = vec![];
    // Loop through joker cards
    let mut i = 0;
    while i < joker_cards.len() {
        let current_card = &joker_cards[i];

        // ff current card is a Blueprint
        if matches!(current_card.joker, Joker::Blueprint) {
            // add the Blueprint joker to the result
            resolved_jokers.push(current_card.clone());

            // ook to the right for the next valid Joker
            let mut target_index = i + 1;

            // keep moving right until valid copiable joker found
            while target_index < joker_cards.len() {
                match joker_cards[target_index].joker {
                    // if another Blueprint is found, continue searching further
                    Joker::Blueprint => {
                        target_index += 1;
                    }

                    // if it's an "Other" activation Joker, stop searching
                    Joker::FourFingers
                    | Joker::Shortcut
                    | Joker::Pareidolia
                    | Joker::Splash
                    | Joker::SmearedJoker => {
                        break;
                    }

                    // if valid Joker found, insert a copy of it right after the Blueprint
                    // only joker enum is copied. not joker card edition
                    _ => {
                        resolved_jokers.push(JokerCard::new(joker_cards[target_index].joker, None));
                        break;
                    }
                }
            }
        } else {
            // if it's not a Blueprint, just add it as is
            resolved_jokers.push(current_card.clone());
        }

        // move to the next card
        i += 1;
    }

    resolved_jokers
}
