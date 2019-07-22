use crate::common::*;

#[derive(Clone, Debug)]
pub struct Noun {
    singular: String,
    plural: String,
    gender: Gender,
    beginning: Beginning
}

impl Noun {
    /// Create a common noun.
    pub fn new(singular: String, plural: String, gender: Gender, beginning: Beginning) -> Noun {
        Noun {
            singular,
            plural,
            gender,
            beginning,
        }
    }

    /// Get the singular form.
    pub fn singular(&self) -> &str {
        &self.singular
    }
    
    /// Get the plural form.
    pub fn plural(&self) -> &str {
        &self.plural
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