extern crate advtools;
extern crate itertools;

use itertools::Itertools;

fn main() {
    let phrases = advtools::iter_input::<Vec<String>>().collect_vec();
    let count1 = phrases.iter().filter(|ph| {
        ph.len() == ph.iter().unique().count()
    }).count();
    println!("Valid passphrases: {}", count1);
    let count2 = phrases.iter().filter(|ph| {
        ph.len() == ph.iter().unique_by(|w| w.chars().sorted()).count()
    }).count();
    println!("Valid without anagrams: {}", count2);
}