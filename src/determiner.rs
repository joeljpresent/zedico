use crate::common::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeterminerKind {
    DefiniteArticle,
    IndefeniteArticle,
    PartitiveArticle,
    // PossessiveDeterminer,
    Demonstrative,
    Interrogative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Determiner {
    kind: DeterminerKind,
    gender: Gender,
    number: Number,
    beginning: Beginning,
}

impl Determiner {
    pub fn new(kind: DeterminerKind, gender: Gender, number: Number,
               beginning: Beginning) -> Self {
        Determiner {kind, gender, number, beginning}
    }

    pub fn word(&self) -> &str {
        match self.kind {
            DeterminerKind::DefiniteArticle => {
                if self.number == Number::Plural {
                    "les"
                } else if self.beginning == Beginning::VowelOrMuteH {
                    "l'"
                } else if self.gender == Gender::Masculine {
                    "le"
                } else {
                    "la"
                }
            }

            DeterminerKind::Demonstrative => {
                if self.number == Number::Plural {
                    "ces"
                } else if self.gender == Gender::Feminine {
                    "cette"
                } else if self.beginning == Beginning::VowelOrMuteH {
                    "cet"
                } else {
                    "ce"
                }
            }

            DeterminerKind::IndefeniteArticle => {
                if self.number == Number::Plural {
                    "des"
                } else if self.gender == Gender::Masculine {
                    "un"
                } else {
                    "une"
                }
            }

            DeterminerKind::Interrogative => {
                if self.number == Number::Singular {
                    if self.gender == Gender::Masculine {
                        "quel"
                    } else {
                        "quelle"
                    }
                } else if self.gender == Gender::Masculine {
                        "quels"
                } else {
                    "quelles"
                }
            }

            DeterminerKind::PartitiveArticle => {
                if self.number == Number::Plural {
                    "des"
                } else if self.beginning == Beginning::VowelOrMuteH {
                    "de l'"
                } else if self.gender == Gender::Masculine {
                    "du"
                } else {
                    "de la"
                }
            }
        }
    }

    pub fn kind(&self) -> DeterminerKind {
        self.kind()
    }

    pub fn gender_of_noun(&self) -> Gender {
        self.gender
    }

    pub fn number_of_noun(&self) -> Number {
        self.number
    }

    pub fn beginning_of_following_word(&self) -> Beginning {
        self.beginning
    }

    pub fn beginning_of_determiner(&self) -> Beginning {
        // If this determiner is "un" or "une"
        if self.kind == DeterminerKind::IndefeniteArticle && self.number == Number::Singular {
            Beginning::VowelOrMuteH
        } else {
            Beginning::Consonant
        }
    }
}