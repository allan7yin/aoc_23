use lazy_regex::regex;
use std::fs::File;
use std::io::{self, BufRead};

pub fn compute_calibration_sum(path: &str, match_words: bool) -> u32 {
    let file = File::open(path).expect("Error in opening the file");
    let mut calibration_sum = 0;

    // Process each line to find calibration values
    for line in io::BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let found_digits = find_digits(&line, match_words);
        if let (Some(&first), Some(&last)) = (found_digits.first(), found_digits.last()) {
            println!("{}, {}", first, last);
            calibration_sum += first * 10 + last;
        }
    }

    calibration_sum
}

fn find_digits(line: &str, match_words: bool) -> Vec<u32> {
    let re = if match_words {
        regex!(r"^(one|two|three|four|five|six|seven|eight|nine|\d)")
    } else {
        regex!(r"^(\d)")
    };
    let mut digits = Vec::new();
    for i in 0..line.len() {
        if let Some(m) = re.find(&line[i..]) {
            let digit = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse::<u32>().unwrap(),
            };
            digits.push(digit);
        }
    }
    digits
}
