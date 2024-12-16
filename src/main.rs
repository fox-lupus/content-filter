//htps l
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

//find and returns all unique words contained with in a string
fn get_uniq(check:String) -> HashSet<String> {
    let mut uniq_word = HashSet::new();
    for part in check.split_whitespace() {

        uniq_word.insert(part.to_lowercase());
    }
    return uniq_word
}
//checks a url if it contains
fn check_url(url:String, banned:Vec<String>) -> Result<bool, reqwest::Error> {
    //get html of url
    let body = reqwest::blocking::get(url)?.text().expect("");
    //find every unique word
    let words = get_uniq(body.clone());

    let matcher = SkimMatcherV2::default();
    let mut found = false;

    for word in words {
        //stop so don't have to find dupe
        if found {
            break;
        }
        for ban in &banned {
            if let Some(score) = matcher.fuzzy_indices(&word, &ban) {
                let (score, _) = score; //trupe deconstuction
                if score >= 190 {
                    found = true;
                    //println!("{word} {ban}");
                    //println!("{:?}", index);
                }
            }
        }
    }

    return Ok(found);
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
    let ban_loc = match args.get(1){
        Some(loc) => loc,
        _ => panic!("requires command line arg for location of banned wordlist \nexample: en.txt url"),
    };
    let ban_list = read_ban(ban_loc.to_string());
    //println!("{:?}", banList);
    let url = match args.get(2){
        Some(url) => url,
        _ => panic!("url of website to be checked is required \nexample: file https://google.com"),
    };
    println!("Contain profanity? {:?}", check_url(url.to_string(), ban_list));
        
    //println!("body = {body:?}");
}
