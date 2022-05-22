// import { Regex } from "regex";
use regex::Regex;

fn main() {
    // const re = /^\d{4}-\d{2}-\d{2}$/;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // console.assert(re.test("2014-01-01"))
    assert!(re.is_match("2014-01-01"));
}