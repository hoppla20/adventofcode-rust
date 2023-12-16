use std::fs;

const NUMBER_STRINGS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn digit_at_position(line: &str, pos: usize) -> Option<u32> {
    let char = line.chars().nth(pos).expect("Index out of bounds.");

    if char.is_digit(10) {
        return char.to_digit(10);
    }

    for num in NUMBER_STRINGS {
        if pos + num.0.len() > line.len() {
            continue;
        }

        let substring = &line[pos..pos + num.0.len()];
        if substring == num.0.to_string() {
            return Some(num.1);
        }
    }

    return None;
}

fn main() {
    let string = fs::read_to_string("inputs.txt").expect("Could not read file 'inputs.txt'!");

    let mut result: u32 = 0;
    for line in string.lines() {
        let mut calibration_value: u32 = 0;

        for i in 0..line.len() {
            match digit_at_position(line, i) {
                Some(n) => {
                    calibration_value += 10 * n;
                    break;
                },
                None => ()
            }
        }

        for i in (0..line.len()).rev() {
            match digit_at_position(line, i) {
                Some(n) => {
                    calibration_value += n;
                    break;
                },
                None => ()
            }
        }

        result += calibration_value
    }

    println!("{:?}", result)
}
