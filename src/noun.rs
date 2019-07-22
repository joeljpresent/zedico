use crate::common::*;

/// A common noun, such as "chien" or "maison".
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Noun {
    word: String,
    number: Number,
    gender: Gender,
    beginning: Beginning
}

impl Noun {
    /// Create a common noun.
    pub fn new(word: String, number: Number, gender: Gender, beginning: Beginning) -> Noun {
        Noun {
            word,
            number,
            gender,
            beginning,
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }
    
    /// Get the plural form.
    pub fn number(&self) -> Number {
        self.number
    }

    /// Get the grammatical gender.
    pub fn gender(&self) -> Gender {
        self.gender
    }

    /// Get the beginning type (consonant, or vowel/mute H).
    pub fn beginning(&self) -> Beginning {
        self.beginning
    }
}