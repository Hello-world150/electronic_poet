use exitcode::{DATAERR, IOERR};
use rand::{rng, Rng};
use std::{fs::read_to_string, process::exit};
use structopt::StructOpt;

/// 电子诗人
#[derive(StructOpt)]
struct Args {
    /// 每段行数
    line: u128,
    /// 总段数
    paragraph: u128,
}
fn file_to_vec(path: &str) -> Vec<String> {
    match read_to_string(path) {
        Ok(s) => {
            s.split("\n").map(|s| s.to_string()).collect() // return `Vec<String>` type
        }
        Err(error) => {
            eprintln!("{error}: {path}");
            exit(IOERR);
        }
    }
}

fn get_rand_element(vec: &[String]) -> String {
    let mut rng = rng();
    vec[rng.random_range(0..vec.len())].clone()
}

fn main() {
    let args = match Args::from_args_safe() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            exit(DATAERR);
        }
    };

    let v = file_to_vec("v.txt");
    let t = file_to_vec("t.txt");
    let i = file_to_vec("i.txt");
    let a = file_to_vec("a.txt");
    let n = file_to_vec("n.txt");
    let sentence = file_to_vec("sentence.txt");

    for _para_number in 1..=args.paragraph {
        for _line_number in 1..=args.line {
            let sentence = get_rand_element(&sentence);

            let mut line_result = String::new();
            for current_char in sentence.chars() {
                match current_char {
                    'v' => line_result.push_str(&get_rand_element(&v)),
                    't' => line_result.push_str(&get_rand_element(&t)),
                    'i' => line_result.push_str(&get_rand_element(&i)),
                    'a' => line_result.push_str(&get_rand_element(&a)),
                    'n' => line_result.push_str(&get_rand_element(&n)),
                    _ => line_result.push_str(&current_char.to_string()[..]), // as `&str` type
                }
            }

            println!("{line_result}");
        }

        println!();
    }
}
