mod common;
use common::*;

mod noun;
use noun::*;

fn main() {
    let saucisse = Noun::new("saucisse".into(), "saucisses".into(), Gender::Feminine, Beginning::Consonant);
    let joyau = Noun::new("joyau".into(), "joyaux".into(), Gender::Masculine, Beginning::Consonant);
    let souris = Noun::new("souris".into(), "souris".into(), Gender::Feminine, Beginning::Consonant);
    let oeil = Noun::new("œil".into(),"yeux".into(), Gender::Masculine, Beginning::VowelOrMuteH);
    println!("{} - {} ({:?} {:?})", saucisse.singular(), saucisse.plural(), saucisse.gender(), saucisse.beginning());
    println!("{} - {}", joyau.singular(), joyau.plural());
    println!("{} - {}", souris.singular(), souris.plural());
    println!("{} - {}", oeil.singular(), oeil.plural());
}
