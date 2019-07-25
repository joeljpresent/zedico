use crate::common::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdjectivePosition {
    /// Antéposé (with masculine singular form before vowel or mute H)
    BeforeNoun(String),
    /// Postposé
    AfterNoun,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Adjective {
    masc_sing: String,
    fem_sing: String,
    masc_pl: String,
    fem_pl: String,
    beginning: Beginning,
    position: AdjectivePosition,
}

impl Adjective {
    pub fn new(masc_sing: String,
        fem_sing: String,
        masc_pl: String,
        fem_pl: String,
        beginning: Beginning,
        position: AdjectivePosition) -> Self {
    
        Adjective {
            masc_sing, fem_sing, masc_pl, fem_pl,
            beginning, position 
        }
    }

    pub fn masc_sing(&self) -> &str {
        &self.masc_sing
    }

    pub fn fem_sing(&self) -> &str {
        &self.fem_sing
    }

    pub fn masc_pl(&self) -> &str {
        &self.masc_pl
    }

    pub fn fem_pl(&self) -> &str {
        &self.fem_pl
    }

    pub fn beginning(&self) -> Beginning {
        self.beginning
    }

    pub fn position(&self) -> &AdjectivePosition {
        &self.position
    }
}