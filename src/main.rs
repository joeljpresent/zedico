mod common;
use common::*;

mod noun;
use noun::*;

mod determiner;
use determiner::*;

fn main() {
    let saucisses = Noun::new("saucisses".into(), Number::Plural, Gender::Feminine, Beginning::Consonant);
    let des = Determiner::new(DeterminerKind::IndefeniteArticle, Gender::Masculine, Number::Plural, Beginning::Consonant);
    let joyau = Noun::new("joyau".into(), Number::Singular, Gender::Masculine, Beginning::Consonant);
    let souris = Noun::new("souris".into(), Number::Plural, Gender::Feminine, Beginning::Consonant);
    let oeil = Noun::new("œil".into(), Number::Singular, Gender::Masculine, Beginning::VowelOrMuteH);
    println!("{:?}\n{:?}\n{:?}\n{:?}", saucisses, joyau, souris, oeil);
    println!("{} {}", des.word(), saucisses.word());
}
