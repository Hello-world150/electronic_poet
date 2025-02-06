use exitcode::IOERR;
use rand::{rng, Rng};
use std::{fs::read_to_string, process::exit};

fn file_to_vec(path: &str) -> Vec<String> {
    match read_to_string(path) {
        Ok(s) => {
            s.split("\n").map(|s| s.to_string()).collect() // return `Vec<String>` type
        }
        Err(_) => {
            eprintln!("打开 {path} 失败");
            exit(IOERR);
        }
    }
}

fn get_rand_element(vec: &[String]) -> String {
    let mut rng = rng();
    vec[rng.random_range(0..vec.len())].clone()
}

fn main() {
    let v = file_to_vec("v.txt");
    let t = file_to_vec("t.txt");
    let i = file_to_vec("i.txt");
    let a = file_to_vec("a.txt");
    let n = file_to_vec("n.txt");
    let sentence = file_to_vec("sentence.txt");

    for _times in 1..50 {
        let sentence = get_rand_element(&sentence);

        let mut result = String::new();
        for current_char in sentence.chars() {
            match current_char {
                'v' => result.push_str(&get_rand_element(&v)),
                't' => result.push_str(&get_rand_element(&t)),
                'i' => result.push_str(&get_rand_element(&i)),
                'a' => result.push_str(&get_rand_element(&a)),
                'n' => result.push_str(&get_rand_element(&n)),
                _ => result.push_str(&current_char.to_string()[..]), // as `&str` type
            }
        }
        println!("{result}");
    }
}
