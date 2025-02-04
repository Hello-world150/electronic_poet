use rand::{rng, Rng};
use std::fs::read_to_string;

fn get_rand_verb<'a>(verb: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    verb[rng.random_range(0..verb.len())]
}

fn get_rand_adj<'a>(adj: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    adj[rng.random_range(0..adj.len())]
}

fn get_rand_noun<'a>(noun: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    noun[rng.random_range(0..noun.len())]
}

fn get_rand_sentence<'a>(sentence: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    sentence[rng.random_range(0..sentence.len())]
}

fn main() {
    // read meta data from file into `String`
    let verb = read_to_string("verb.txt").expect("Failed read verb file.");
    let adj = read_to_string("adjecive.txt").expect("Failed read adjecive file.");
    let noun = read_to_string("noun.txt").expect("Failed read noun file.");
    let sentence = read_to_string("sentence.txt").expect("Failed read sentence file.");

    // spilt `String` with `\n` and convert this into `Vec<&str>`
    let verb: Vec<&str> = verb.split("\n").collect();
    let adj: Vec<&str> = adj.split("\n").collect();
    let noun: Vec<&str> = noun.split("\n").collect();
    let sentence: Vec<&str> = sentence.split("\n").collect();

    for _times in 1..100 {
        let rand_sentence = get_rand_sentence(&sentence);
        let mut current_sentence = String::new();
        for current_char in rand_sentence.chars() {
            match current_char {
                'v' => current_sentence.push_str(get_rand_verb(&verb)),
                'a' => current_sentence.push_str(get_rand_adj(&adj)),
                'n' => current_sentence.push_str(get_rand_noun(&noun)),
                _ => current_sentence.push(current_char),
            }
        }
        println!("{current_sentence}");
    }
}
