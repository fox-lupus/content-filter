use reqwest;

fn main() {

	let body = reqwest::blocking::get("https://www.rust-lang.org").expect("Response").text();

	println!("body = {body:?}");
}
