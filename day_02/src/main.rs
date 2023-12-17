use std::fs;
use std::num;
use std::cmp;
use thiserror::Error;

/*
 * Errors
 */

#[derive(Error, Debug)]
pub enum ParseLineError {
    #[error("invalid game id `{0}`")]
    ParseGameId(String, #[source] num::ParseIntError),

    #[error("invalid color count `{0}")]
    ParseColorCount(String, #[source] num::ParseIntError),

    #[error("invalid color `{0}`")]
    InvalidColor(String),
}

type Result<T> = std::result::Result<T, ParseLineError>;

/*
 * Structs
 */

struct Reach {
    red: u32,
    green: u32,
    blue: u32
}

impl Reach {
    fn set_color_count(&mut self, count: u32, color_name: &str) -> Result<()> {
        match color_name {
            "red" => Ok(self.red = count),
            "green" => Ok(self.green = count),
            "blue" => Ok(self.blue = count),
            _ => Err(ParseLineError::InvalidColor(color_name.to_string())),
        }
    }

    fn power(&self) -> u32 {self.red * self.green * self.blue}
}

struct Game {
    id: u32,
    reaches: Vec<Reach>
}

impl Game {
    fn get_min_rule(&self) -> Result<Reach> {
        self.reaches.iter().try_fold(
            Reach {red: 0, green: 0, blue: 0},
            |acc, reach| {
                Ok(Reach {
                    red: cmp::max(acc.red, reach.red),
                    green: cmp::max(acc.green, reach.green),
                    blue: cmp::max(acc.blue, reach.blue),
                })
            }
        )
    }
}

/*
 * Functions
 */

fn parse_line(line: &str) -> Result<Game> {
    let line_split: Vec<&str> = line.split(": ").collect();

    // parse game id

    let game_id_string = &line_split[0][5..];
    let game_id = game_id_string.parse::<u32>()
        .map_err(|err| ParseLineError::ParseGameId(game_id_string.to_string(), err))?;

    // parse reaches
    let reaches = line_split[1].split("; ")
        .map(|reach_string: &str| -> Result<Reach> {
            let mut reach = Reach {red: 0, green: 0, blue: 0};

            reach_string.split(", ")
                .try_for_each(|color_string: &str| -> Result<()> {
                    let color_string_split: Vec<&str> = color_string.split(" ").collect();
                    color_string_split[0].parse::<u32>()
                        .map_err(|err| ParseLineError::ParseColorCount(color_string_split[0].to_string(), err))
                        .map(|count| {
                            reach.set_color_count(count, color_string_split[1])
                        })?
                })?;

            return Ok(reach);
        })
        .collect::<Result<Vec<Reach>>>()?;

    return Ok(Game {
        id: game_id,
        reaches: reaches
    });
}

fn check_rule(rule: &Reach, reach: &Reach) -> bool {
    return rule.red >= reach.red
        && rule.green >= reach.green
        && rule.blue >= reach.blue;
}

/*
 * Main
 */

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file 'input.txt'!");
    let rule = Reach {red: 12, green: 13, blue: 14};

    match input.lines().try_fold([0, 0], |acc, line| -> Result<[u32; 2]> {
        let mut results = acc;

        let game = parse_line(line)?;
        if game.reaches.iter().all(|reach| check_rule(&rule, reach)) {
            results[0] += game.id;
        }

        let min_rule = game.get_min_rule()?;
        results[1] += min_rule.power();

        Ok(results)
    }) {
        Ok(result) => println!("Result 1: {}, Result 2: {}", result[0], result[1]),
        Err(err) => eprintln!("Error: {}", err),
    }
}
