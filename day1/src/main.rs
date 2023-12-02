fn main() {
    println!("Advent of Code 2022 - Day 1");

    let input = open_input_file();
    let calibration_document_string_part1:Vec<&str> = input.split("\n").collect(); // Split on newline

    // Part 1

    let mut calibration_accumulator = 0;

    for line in calibration_document_string_part1 {
        let numbers:Vec<&str> = line.matches(char::is_numeric).collect();
        let calibration:isize = format!("{}{}",numbers.first().unwrap(), numbers.last().unwrap()).parse().unwrap();
        //println!("Calibration value: {}",calibration);
        calibration_accumulator += calibration;

    }

    println!("Answer for Part One: {}",calibration_accumulator);

    
    // Part 2

    let input = open_input_file();
    let calibration_document_string_part2:Vec<&str> = input.split("\n").collect(); // Split on newline


    let mut calibration_accumulator_part2 = 0;

    for line in calibration_document_string_part2 {
        let line_after_replacement = replace_written_digits(line);
        let numbers:Vec<&str> = line_after_replacement.matches(char::is_numeric).collect();
        let calibration:isize = format!("{}{}",numbers.first().unwrap(), numbers.last().unwrap()).parse().unwrap();
        //println!("Calibration value: {}",calibration);
        calibration_accumulator_part2 += calibration;

    }


    println!("Answer for Part Two: {}",calibration_accumulator_part2);



}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}


fn replace_written_digits(line: &str) -> String {

    let mut line_after_replacement: String = line.into();


    // Manually checking for the corner cases
    line_after_replacement = line_after_replacement.replace("oneight", "18");
    line_after_replacement = line_after_replacement.replace("twone", "21");
    line_after_replacement = line_after_replacement.replace("threeight", "38");
    line_after_replacement = line_after_replacement.replace("fiveight", "58");
    line_after_replacement = line_after_replacement.replace("sevenine", "79");
    line_after_replacement = line_after_replacement.replace("eightwo", "82");
    line_after_replacement = line_after_replacement.replace("eighthree", "83");
    line_after_replacement = line_after_replacement.replace("nineight", "98");
    // Normal values
    line_after_replacement = line_after_replacement.replace("one", "1");
    line_after_replacement = line_after_replacement.replace("two", "2");
    line_after_replacement = line_after_replacement.replace("three", "3");
    line_after_replacement = line_after_replacement.replace("four", "4");
    line_after_replacement = line_after_replacement.replace("five", "5");
    line_after_replacement = line_after_replacement.replace("six", "6");
    line_after_replacement = line_after_replacement.replace("seven", "7");
    line_after_replacement = line_after_replacement.replace("eight", "8");
    line_after_replacement = line_after_replacement.replace("nine", "9");
 


    line_after_replacement
}