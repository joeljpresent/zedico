#![allow(unused)]

use std::borrow::Cow;

#[derive(Debug)]
enum Gender {
    Masculine,
    Feminine
}

#[derive(Debug)]
enum PluralRule {
    Invariable,
    WithS,
    WithX,
    Irregular(String)
}

#[derive(Debug)]
struct Noun {
    base_word: String,
    gender: Gender,
    plural_rule: PluralRule
}

impl Noun {
    fn new(base_word: String, gender: Gender, plural_rule: PluralRule) -> Noun {
        Noun {
            base_word: base_word,
            gender: gender,
            plural_rule: plural_rule
        }
    }
    
    fn get_singular(&self) -> Cow<str> {
        Cow::Borrowed(&self.base_word)
    }
    
    fn get_plural(&self) -> Cow<str> {
        match &self.plural_rule {
            PluralRule::Invariable => Cow::Borrowed(&self.base_word),
            PluralRule::WithS => Cow::Owned(format!("{}s", self.base_word)),
            PluralRule::WithX => Cow::Owned(format!("{}x", self.base_word)),
            PluralRule::Irregular(s) => Cow::Borrowed(s)
        }
    }
}

fn main() {
    let saucisse = Noun::new("saucisse".into(), Gender::Feminine, PluralRule::WithS);
    let joyau = Noun::new("joyau".into(), Gender::Masculine, PluralRule::WithX);
    let souris = Noun::new("souris".into(), Gender::Feminine, PluralRule::Invariable);
    let oeil = Noun::new("œil".into(), Gender::Masculine, PluralRule::Irregular("yeux".into()));
    println!("{} - {}", saucisse.get_singular(), saucisse.get_plural());
    println!("{} - {}", joyau.get_singular(), joyau.get_plural());
    println!("{} - {}", souris.get_singular(), souris.get_plural());
    println!("{} - {}", oeil.get_singular(), oeil.get_plural());
}
