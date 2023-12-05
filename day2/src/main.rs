use nom::{IResult, Parser,character::complete::{i32, alpha0, alpha1,alphanumeric0,space1},bytes::complete::tag,sequence::{separated_pair, delimited,terminated, preceded},combinator::{rest,opt}};
use nom::multi::separated_list0;

use std::{fmt, cmp::max};

#[derive(Debug,PartialEq,Clone,Copy)]
enum Colors {
    Red(i32),
    Green(i32),
    Blue(i32),
}

 #[derive(Debug,PartialEq,Clone,Copy)]
 struct ColorResult {
     color: Colors,
 }

#[derive(Debug,PartialEq,Clone,Copy)]
struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug,PartialEq,Clone)]
struct Game {
    game_ID: i32,
    sets: Vec<GameSet>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Game {}", self.game_ID);
        Ok(for set in &self.sets{
            let set_count = 1;
            writeln!(f,"Set {}: Red: {}, Green: {}, Blue: {}",set_count,set.red,set.green,set.blue);

        })
    }
}

fn get_game_id(input: &str) -> IResult<&str, i32> {
    let(remaining,id) = delimited(
        tag("Game "),
        i32,
        tag(": ")
    )(input)?;
    Ok((remaining,id))
}

fn get_color_number_name(input: &str) -> IResult<&str, ColorResult> {
    let result: ColorResult = ColorResult{color: Colors::Red(0)};

    let (remaining,(color_number,color_name)) = separated_pair(i32, space1, alpha1)(input)?;


    let result = {
        match color_name {
            "red" => ColorResult{color: Colors::Red(color_number)},
            "green" => ColorResult{color: Colors::Green(color_number)},
            "blue" => ColorResult{color: Colors::Blue(color_number)},
            _ => panic!("Not a color"),
        }
    };

    Ok((remaining,result)) // Empty
}

fn get_game_set(input: &str) -> IResult<&str, GameSet> {
    // Parse from X green, Y blue, Z red to a struct
    // Order of colors doesn't matter

    let mut new_game: GameSet = GameSet{red:0,green:0,blue:0}; // Empty

    let (remaining,first_color) = get_color_number_name(input)?;

    match first_color.color {
        Colors::Red(x) => { new_game.red = x},
        Colors::Green(x) => { new_game.green = x},
        Colors::Blue(x) => { new_game.blue = x},
    }

    let (remaining,optional_second_color) = opt(preceded(tag(", "), get_color_number_name))(remaining)?;

    

    // Using nested matches here is not good. There probably is another way
    match optional_second_color {
        Some(x) => {
            match x.color {
                Colors::Red(x) => { new_game.red = x},
                Colors::Green(x) => { new_game.green = x},
                Colors::Blue(x) => { new_game.blue = x},
            }
        }
        None => (),
    }

    
    let (remaining,optional_third_color) = opt(preceded(tag(", "), get_color_number_name))(remaining)?;

    match optional_third_color {
        Some(x) => {
            match x.color {
                Colors::Red(x) => { new_game.red = x},
                Colors::Green(x) => { new_game.green = x},
                Colors::Blue(x) => { new_game.blue = x},
            }
        }
        None => (),
    }

    //let empty_game_set: GameSet = GameSet{Red:0,Green:0,Blue:0}; // Empty
    Ok((remaining,new_game)) // Do nothing
}


fn get_game(input: &str) -> IResult<&str,Game>{
    let empty_game_set: GameSet = GameSet{red:0,green:0,blue:0}; // Empty

    

    let (remaining_all_sets,game_id) = get_game_id(input)?;

    let mut new_game = Game{game_ID:game_id,sets:Vec::new()};


    // There is always at least 1 set
    // let (remaining_sets,set_1) = get_game_set(remaining_all_sets)?;

    // new_game.sets.push(set_1);
        

    let (remaining,mut list_of_sets) = separated_list0(tag("; "), get_game_set)(remaining_all_sets)?;

    new_game.sets.append(&mut list_of_sets);

    // while remaining_sets != "" {
    //     let (remaining_sets, new_set) = preceded(tag("; "),get_game_set)(remaining_sets)?;
    //     new_game.sets.push(new_set);
    // }

    Ok((remaining,new_game))
}

fn main()  {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();
    let game_list_input:Vec<&str> = input.split("\n").collect(); // Split on newline

    // Parse the input and create a list of all games
    let mut game_list: Vec<Game> = Vec::new();
    for game in game_list_input {
        let (_,parsed_game) = get_game(game).unwrap();
        game_list.push(parsed_game);
        //println!("{}",parsed_game);
    }

    // Part 1
    let mut possible_games_sum_id = 0;

    for game in game_list.clone() {

        let mut possible_game = true;

        for set in game.sets {
            if (set.red > 12 || set.green > 13 || set.blue > 14) {
                possible_game = false;
            }
        }

        if possible_game {
            possible_games_sum_id += game.game_ID;
        }


    }

    println!("Answer for Part One: {}",possible_games_sum_id);



    // Part 2
    let mut sum_of_power_of_min_set = 0;

    for game in game_list {

        let mut min_possible_red = 0;
        let mut min_possible_green = 0;
        let mut min_possible_blue = 0;

        for set in game.sets {
            min_possible_red = max(min_possible_red,set.red);
            min_possible_green = max(min_possible_green,set.green);
            min_possible_blue = max(min_possible_blue,set.blue); 
        }

        sum_of_power_of_min_set += min_possible_red*min_possible_blue*min_possible_green;

    }

    println!("Answer for Part Two: {}",sum_of_power_of_min_set);

}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}
