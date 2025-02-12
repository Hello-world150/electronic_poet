use exitcode::{DATAERR, IOERR};
use rand::{rng, Rng};
use std::{fs::read_to_string, process::exit};
use structopt::StructOpt;

/// 电子诗人
#[derive(StructOpt)]
struct Args {
    /// 每段行数
    line: usize,
    /// 总段数
    paragraph: usize,
}

/// read file convert to Vec<String>
fn file_to_vec(path: &str) -> Vec<String> {
    match read_to_string(path) {
        Ok(s) => {
            s.lines().map(|s| s.to_string()).collect() // return `Vec<String>` type
        }
        Err(_) => {
            eprintln!("Failed to read {path}");
            exit(IOERR);
        }
    }
}

/// get random element from Vec<String>
fn get_rand_element<R: Rng>(vec: &[String], rng: &mut R) -> String {
    vec[rng.random_range(0..vec.len())].clone()
}

fn main() {
    let mut rng = rng();
    // convert args from string to struct
    let args = match Args::from_args_safe() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            exit(DATAERR);
        }
    };

    // read meta data from files
    let v = file_to_vec("v.txt");
    let t = file_to_vec("t.txt");
    let i = file_to_vec("i.txt");
    let a = file_to_vec("a.txt");
    let n = file_to_vec("n.txt");
    let sentence = file_to_vec("sentence.txt");

    for _para_number in 1..=args.paragraph {
        for _line_number in 1..=args.line {
            let sentence = get_rand_element(&sentence, &mut rng);

            let mut line_result = String::with_capacity(sentence.len() * 2);
            // match every char in random sentence and replace special sign into random string, push random string to a new String and print it
            for current_char in sentence.chars() {
                match current_char {
                    'v' => line_result.push_str(&get_rand_element(&v, &mut rng)),
                    't' => line_result.push_str(&get_rand_element(&t, &mut rng)),
                    'i' => line_result.push_str(&get_rand_element(&i, &mut rng)),
                    'a' => line_result.push_str(&get_rand_element(&a, &mut rng)),
                    'n' => line_result.push_str(&get_rand_element(&n, &mut rng)),
                    _ => line_result.push_str(&current_char.to_string()[..]), // as `&str` type
                }
            }

            println!("{line_result}");
        }

        println!();
    }
}
