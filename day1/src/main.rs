use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;
use std::{path::Path, fs::File};
use std::fs::read_to_string;

use itertools::{multipeek, MultiPeek};


fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let lines: Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();
    let total = lines.into_iter().fold(0_u32, |acc, val| {
        parse_line_with_word_digits(val) + acc
    });
    println!("{}", total)
}


fn parse_line(line: String) -> u32 {
        let mut first = None;
        let mut last = None;

        for c in line.chars() {
            let num: Option<u32> = c.to_digit(10);
            match num {
                Some(v) => match first {
                    Some(_) => last = Some(v),
                    None => first = Some(v),
                },
                None => continue,
            }
        }
        match last {
            Some(v) => {
                let mut str = first.unwrap().to_string();
                str.push_str(v.to_string().as_str());
                str.parse::<u32>().unwrap()
            },
            None => {
                let mut str = first.unwrap().to_string();
                str.push_str(str.to_string().as_str());
                str.parse::<u32>().unwrap()
            },
        }
}


fn parse_line_with_word_digits(line: String) -> u32 {
    let mut first = None;
    let mut last = None;

    let mut line_iter  = line.chars().peekable();

    while line_iter.peek().is_some() {
        let c = line_iter.next().unwrap();
        let num: Option<u32> = c.to_digit(10);
        match num {
            Some(v) => match first {
                Some(_) => last = Some(v),
                None => first = Some(v),
            },
            // try to parse a digit from letters
            // if we find the first letter of a digit, try to iterate the rest
            None => {
                match try_digits(c, line_iter.clone()) {
                    Some(v) => {
                        match first {
                        Some(_) => last = Some(v),
                        None => first = Some(v),
                    }},
                    None => continue,
                }
            }
        }
    }
    match last {
        Some(v) => {
            let mut str = first.unwrap().to_string();
            str.push_str(v.to_string().as_str());
            str.parse::<u32>().unwrap()
        },
        None => {
            let mut str = first.unwrap().to_string();
            str.push_str(str.to_string().as_str());
            str.parse::<u32>().unwrap()
        },
    }
}


fn try_digits(current: char, chars: Peekable<Chars>) -> Option<u32> {
    match current {
        'o' => check_digit(chars, Digit::One),
        't' => check_digit(chars.clone(), Digit::Two).or_else(|| {check_digit(chars, Digit::Three)}),
        'f' => check_digit(chars.clone(), Digit::Four).or_else(|| {check_digit(chars, Digit::Five)}),
        's' => check_digit(chars.clone(), Digit::Six).or_else(|| {check_digit(chars, Digit::Seven)}),
        'e' => check_digit(chars, Digit::Eight),
        'n' => check_digit(chars, Digit::Nine),
        _ => None
    }
} 

fn check_digit(iter: Peekable<Chars>, d: Digit) -> Option<u32> {
    let mut i = multipeek(iter);
    let digit = d.to_string();
    let mut digit_iter = digit.chars();

    // we should skip the first letter of the digit iter, since we have already seen it
    digit_iter.next();

    for c in digit_iter {
        let next = i.next().unwrap_or('z'); 

        if c != next {
            return None;
        }
    }
    Some(d as u32)
    
}

pub enum Digit {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::One => write!(f, "one"),
            Digit::Two => write!(f, "two"),
            Digit::Three => write!(f, "three"),
            Digit::Four => write!(f, "four"),
            Digit::Five => write!(f, "five"),
            Digit::Six => write!(f, "six"),
            Digit::Seven => write!(f, "seven"),
            Digit::Eight => write!(f, "eight"),
            Digit::Nine => write!(f, "nine"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_with_first_last_word_digits() {
        let line = "two1nine";
        let expected = 29_u32;

        assert_eq!(parse_line_with_word_digits(line.into()), expected)
    }

    #[test]
    fn parse_line_with_first_word_digits() {
        let line = "two1nine3";
        let expected = 23_u32;

        assert_eq!(parse_line_with_word_digits(line.into()), expected)
    }

    #[test]
    fn parse_line_with_last_word_digits() {
        let line = "1two3ninea";
        let expected = 19_u32;

        assert_eq!(parse_line_with_word_digits(line.into()), expected)
    }

    #[test]
    fn try_digits_works() {
        let line = "five";
        let mut iter = line.chars();
        let c = iter.next().unwrap();

        assert_eq!(try_digits(c, iter.peekable()).unwrap(), 5)
    }

    #[test]
    fn parse_line_2_digits() {
        let line = "pqr3stu8vwx";
        let expected = 38_u32;

        assert_eq!(parse_line(line.into()), expected)
    }

    #[test]
    fn parse_line_1_digit() {
        let line = "treb7uchet";
        let expected = 77_u32;

        assert_eq!(parse_line(line.into()), expected)
    }
}