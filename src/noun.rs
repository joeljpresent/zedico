use crate::common::*;

/// A common noun, such as "chien" or "maison".
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Noun {
    singular: Option<String>,
    plural: Option<String>,
    gender: Gender,
    beginning: Beginning,
}

impl Noun {
    /// Create a common noun.
    pub fn new(singular: Option<String>, plural: Option<String>, gender: Gender, beginning: Beginning) -> Self {
        Noun {
            singular,
            plural,
            gender,
            beginning,
        }
    }

    pub fn new_noun(singular: &str, plural: &str, gender: Gender, beginning: Beginning) -> Self {
        Noun {
            singular: Some(singular.into()),
            plural: Some(plural.into()),
            gender,
            beginning,
        }
    }

    pub fn new_masculine_noun(singular: &str, plural: &str, beginning: Beginning) -> Self {
        Noun {
            singular: Some(singular.into()),
            plural: Some(plural.into()),
            gender: Gender::Masculine,
            beginning,
        }
    }

    pub fn new_feminine_noun(singular: &str, plural: &str, beginning: Beginning) -> Self {
        Noun {
            singular: Some(singular.into()),
            plural: Some(plural.into()),
            gender: Gender::Feminine,
            beginning,
        }
    }

    pub fn singular(&self) -> Option<&str> {
        if let Some(word) = &self.singular {
            Some(&word)
        } else {
            None
        }
    }

    pub fn plural(&self) -> Option<&str> {
        if let Some(word) = &self.plural {
            Some(&word)
        } else {
            None
        }
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    /// Get the beginning type (consonant, or vowel/mute H).
    pub fn beginning(&self) -> Beginning {
        self.beginning
    }
}