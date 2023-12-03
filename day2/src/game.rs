use std::{num::ParseIntError, str::FromStr};

use nom::{
    bytes::complete::take,
    character::complete::{alpha1, char, digit1, multispace0},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};
use thiserror::Error;

#[derive(PartialEq, Eq, Debug)]
pub struct Game {
    pub id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self, LocalError> {
        let cube_parser = delimited(
            multispace0,
            separated_pair(digit1, char(' '), alpha1::<_, nom::error::Error<&str>>),
            multispace0,
        );
        let subset_parser = separated_list0(char(','), cube_parser);

        let mut game_parser = delimited(
            take::<_, _, nom::error::Error<&str>>(5_u8),
            digit1,
            char(':'),
        );
        let mut subsets_parser = separated_list0(char(';'), subset_parser);
        let (rest, game_id) = game_parser(input).unwrap();
        let (_, subsets) = subsets_parser(rest).unwrap();
        let subsets: Result<Vec<Subset>, LocalError> = subsets
            .into_iter()
            .map(|subset| Subset::from_strs(subset))
            .collect();
        Ok(Self {
            id: u32::from_str(game_id)?,
            subsets: subsets?,
        })
    }

    pub fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        let valid = self
            .subsets
            .iter()
            .all(|subset| subset.is_valid(max_red, max_green, max_blue));
        valid
    }

    pub fn min_bag_size(&self) -> (u32, u32, u32) {
        let mins: Vec<(u32, u32, u32)> = self.subsets.iter().map(|s| s.min_bag_size()).collect();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        mins.iter().for_each(|(r, g, b)| {
            if r > &max_red {
                max_red = *r;
            };
            if g > &max_green {
                max_green = *g;
            };
            if b > &max_blue {
                max_blue = *b;
            };
        });

        (max_red, max_green, max_blue)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Subset {
    subset: Vec<Cube>,
}

impl Subset {
    fn new(subset: Vec<Cube>) -> Self {
        Subset { subset }
    }

    fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        let mut total_red = 0_u32;
        let mut total_green = 0_u32;
        let mut total_blue = 0_u32;

        self.subset.iter().for_each(|cube| match cube {
            Cube::Red(num) => total_red += num,
            Cube::Green(num) => total_green += num,
            Cube::Blue(num) => total_blue += num,
        });

        let red_valid = total_red <= max_red;
        let green_valid = total_green <= max_green;

        let blue_valid = total_blue <= max_blue;

        red_valid && green_valid && blue_valid
    }

    fn min_bag_size(&self) -> (u32, u32, u32) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        self.subset.iter().for_each(|c| {
            match c {
                Cube::Red(num) => if num > &max_red {max_red = *num},
                Cube::Green(num) => if num > &max_green {max_green = *num},
                Cube::Blue(num) => if num > &max_blue {max_blue = *num},
            }
        });

        (max_red, max_green, max_blue)
    }

    fn parse<'a>(input: &'a str) -> Result<Self, LocalError<'a>> {
        let cube_parser = delimited(
            multispace0,
            separated_pair(digit1, char(' '), alpha1::<_, nom::error::Error<&str>>),
            multispace0,
        );
        let mut subset_parser = separated_list0(char(','), cube_parser);
        let raw_cubes =
            subset_parser(input).map_err(|e| LocalError::NomError(e.map(|e| e.to_string())))?;
        let cubes: Result<Vec<Cube>, LocalError> = raw_cubes
            .1
            .iter()
            .map(|(num, val)| Cube::from_strs(num, val))
            .collect();
        Ok(Self { subset: cubes? })
    }

    fn from_strs<'a>(input: Vec<(&'a str, &'a str)>) -> Result<Self, LocalError<'a>> {
        let cubes: Result<Vec<Cube>, LocalError> = input
            .iter()
            .map(|(num, colour)| Cube::from_strs(num, colour))
            .collect();
        Ok(Self { subset: cubes? })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    fn parse(input: &str) -> Result<Self, LocalError> {
        let mut cube_parser = delimited(
            multispace0,
            separated_pair(digit1, char(' '), alpha1::<_, nom::error::Error<&str>>),
            multispace0,
        );
        // let mut cube_parser = separated_pair(digit1, char(' '), alpha1::<_, nom::error::Error<&str>>);
        let (_, (num, colour)) =
            cube_parser(input).map_err(|e| LocalError::NomError(e.map(|e| e.to_string())))?;
        Self::from_strs(num, colour)
    }

    fn from_strs<'a>(num: &'a str, colour: &'a str) -> Result<Self, LocalError<'a>> {
        match colour {
            "red" => Ok(Self::red(num)?),
            "green" => Ok(Self::green(num)?),
            "blue" => Ok(Self::blue(num)?),
            _ => Err(LocalError::ColourNotRecognized { value: colour }),
        }
    }

    fn green(number: &str) -> Result<Self, ParseIntError> {
        Ok(Self::Green(u32::from_str(number)?))
    }

    fn blue(number: &str) -> Result<Self, ParseIntError> {
        Ok(Self::Blue(u32::from_str(number)?))
    }

    fn red(number: &str) -> Result<Self, ParseIntError> {
        Ok(Self::Red(u32::from_str(number)?))
    }
}

#[derive(Error, Debug)]
pub enum LocalError<'a> {
    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    NomError(#[from] nom::Err<String>),
    #[error("colour not recognized: {value}")]
    ColourNotRecognized { value: &'a str },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let subset = "Game 1: 7 green, 14 red, 5 blue; 8 red, 4 green; 6 green, 18 red, 9 blue";
        // let expected = Subset::new(vec![Cube::Blue(3), Cube::Red(4)]);
        let actual = Game::parse(subset).unwrap();
        println!("{:?}", actual)
    }

    #[test]
    fn parse_subset() {
        let subset = "3 blue, 4 red";
        let expected = Subset::new(vec![Cube::Blue(3), Cube::Red(4)]);
        let actual = Subset::parse(subset).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_cube_works() {
        let input = "7 green";
        let expected = Cube::Green(7);

        assert_eq!(Cube::parse(input).unwrap(), expected);
    }

    #[test]
    fn parse_cube_trailing_leading_whitspace_works() {
        let input = " 7 red ";
        let expected = Cube::Red(7);

        assert_eq!(Cube::parse(input).unwrap(), expected);
    }
}
