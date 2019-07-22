#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Beginning {
    Consonant,
    VowelOrMuteH,
}