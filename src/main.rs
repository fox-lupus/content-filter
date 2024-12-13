//https lib
use reqwest;
//args
use std::env;
//fuzzy search
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::BufRead;

use std::io;

fn get_uniq(Check:String) -> HashSet<String> {
    let mut uniqWord = HashSet::new();
    for part in Check.split_whitespace() {

        uniqWord.insert(part.to_lowercase());
    }
    return uniqWord
}

fn check_url(url:String, banned:Vec<String>) {
    let body = reqwest::blocking::get(url).expect("Response").text();
    let words = get_uniq(body.expect("Response"));

    let matcher = SkimMatcherV2::default();

    for word in words {
        for ban in &banned {
            if let Some(score) = matcher.fuzzy_indices(&word, &ban) {
                let (score, index) = score;
                if score >= 190 {
                    println!("{word} {ban}");
                    println!("{:?}", index);
                }
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_ban(filename:String) -> Vec<String> { 
    let mut words = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(s) = line {
                words.push(s)
            }       
        }
    }
    else {
        eprintln!("file IO");
    }
    return words;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let banLoc = args.get(1); //temp
    let banList = read_ban("en.txt".to_string());
    //println!("{:?}", banList);
    check_url("http://en.wikipedia.org/wiki/transgender".to_string(), banList);
        
    //println!("body = {body:?}");
}
