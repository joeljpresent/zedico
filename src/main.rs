#![allow(unused)]

mod adjective;
use adjective::*;

mod common;
use common::*;

mod noun;
use noun::*;

mod determiner;
use determiner::*;

fn main() {
    let saucisse = Noun::new_feminine_noun("saucisse", "saucisses", Beginning::Consonant);
    let les = Determiner::new(DeterminerKind::DefiniteArticle, Gender::Masculine, Number::Plural, Beginning::Consonant);
    let oeil = Noun::new_masculine_noun("œil", "yeux", Beginning::VowelOrMuteH);
    let beau = Adjective::new("beau".into(), "belle".into(), "beaux".into(), "belles".into(), Beginning::Consonant, AdjectivePosition::BeforeNoun("bel".into()));
    println!("{} {} {}", les.word(), beau.fem_pl(), saucisse.plural().unwrap());
}
