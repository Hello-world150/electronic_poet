use rand::{rng, Rng};
use std::fs::read_to_string;

fn get_rand_v<'a>(v: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    v[rng.random_range(0..v.len())]
}

fn get_rand_t<'a>(t: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    t[rng.random_range(0..t.len())]
}

fn get_rand_i<'a>(i: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    i[rng.random_range(0..i.len())]
}

fn get_rand_a<'a>(a: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    a[rng.random_range(0..a.len())]
}

fn get_rand_n<'a>(n: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    n[rng.random_range(0..n.len())]
}

fn get_rand_sentence<'a>(sentence: &'a [&'a str]) -> &'a str {
    let mut rng = rng();
    sentence[rng.random_range(0..sentence.len())]
}

fn main() {
    // read meta data from file into `String`
    let v = read_to_string("v.txt").expect("Failed read v file.");
    let t = read_to_string("t.txt").expect("Failed read t file.");
    let i = read_to_string("i.txt").expect("Failed read i file.");
    let a = read_to_string("a.txt").expect("Failed read a file.");
    let n = read_to_string("n.txt").expect("Failed read n file.");
    let sentence = read_to_string("sentence.txt").expect("Failed read sentence file.");

    // spilt `String` with `\n` and convert this into `Vec<&str>`
    let v: Vec<&str> = v.split("\n").collect();
    let t: Vec<&str> = t.split("\n").collect();
    let i: Vec<&str> = i.split("\n").collect();
    let a: Vec<&str> = a.split("\n").collect();
    let n: Vec<&str> = n.split("\n").collect();
    let sentence: Vec<&str> = sentence.split("\n").collect();

    for _times in 1..1000000000 {
        let sentence = get_rand_sentence(&sentence);

        let mut result = String::new();
        for current_char in sentence.chars() {
            let v = get_rand_v(&v);
            let t = get_rand_t(&t);
            let i = get_rand_i(&i);
            let a = get_rand_a(&a);
            let n = get_rand_n(&n);

            match current_char {
                'v' => result.push_str(v),
                't' => result.push_str(t),
                'i' => result.push_str(i),
                'a' => result.push_str(a),
                'n' => result.push_str(n),
                _ => result.push_str(&current_char.to_string()[..]),
            }
        }
        println!("{result}");
    }
}
