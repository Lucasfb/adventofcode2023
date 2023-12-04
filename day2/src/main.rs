use nom::{IResult, Parser,character::complete::i32,bytes::complete::tag,sequence::{separated_pair, delimited,tuple,terminated}};
use std::error::Error;
use std::fmt;

#[derive(Debug,PartialEq,Clone)]
struct GameSet {
    Red: i32,
    Green: i32,
    Blue: i32,
}

#[derive(Debug,PartialEq)]
struct Game {
    game_ID: i32,
    sets: [GameSet;3],
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Game {}", self.game_ID);
        writeln!(f,"Set 1: Red: {}, Green: {}, Blue: {}", self.sets[0].Red,self.sets[0].Green,self.sets[0].Blue);
        writeln!(f,"Set 2: Red: {}, Green: {}, Blue: {}", self.sets[1].Red,self.sets[1].Green,self.sets[1].Blue);
        writeln!(f,"Set 3: Red: {}, Green: {}, Blue: {}", self.sets[2].Red,self.sets[2].Green,self.sets[2].Blue)
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

fn get_game_set(input: &str) -> IResult<&str, GameSet> {
    // Parse from X green, Y blue, Z red to a struct
    // Order of colors doesn't matter

    

    let empty_game_set: GameSet = GameSet{Red:0,Green:0,Blue:0}; // Empty
    Ok((input,empty_game_set)) // Do nothing
}


fn get_game(input: &str) -> IResult<&str,Game>{
    let empty_game_set: GameSet = GameSet{Red:0,Green:0,Blue:0}; // Empty

    let (remaining_all_sets,game_id) = get_game_id(input)?;

    let (remaining_sets,set_1) = terminated(
        get_game_set,
        tag("; ")
    )(remaining_all_sets)?;

    //let (remaining,(set_2,set_3)) = separated_pair(
    //    get_game_set,
    //    tag("; "),
    //    get_game_set,
    //)(remaining_sets)?;

    Ok((input,Game{game_ID:game_id,sets:[empty_game_set.clone(),empty_game_set.clone(),empty_game_set.clone()]}))
}




fn main()  {
    println!("Advent of Code 2022 - Day 2");

    let input = open_input_file();
    let game_list_input:Vec<&str> = input.split("\n").collect(); // Split on newline

    // Parse the input and create a list of all games

    for game in game_list_input {
        let (_,parsed_game) = get_game(game).unwrap();
        println!("Game: {}",parsed_game);
    }

    // Part 1


}

fn open_input_file() -> String {
    let input_filename = "sample_input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}
