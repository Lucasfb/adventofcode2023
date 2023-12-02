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
        // To keep the same procedure, I replace the spelled numbers with digits in the string
        // A better way would be to change the procedure, so replacing is not needed
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

    // // Weird method, manually considering corner cases
    // // Order of replacements affect the final result
    // // Found idea here: https://github.com/aiell0/advent_of_code_2023/blob/main/day1/main.go
    // line_after_replacement = line_after_replacement.replace("oneight", "18");
    // line_after_replacement = line_after_replacement.replace("twone", "21");
    // line_after_replacement = line_after_replacement.replace("threeight", "38");
    // line_after_replacement = line_after_replacement.replace("fiveight", "58");
    // line_after_replacement = line_after_replacement.replace("sevenine", "79");
    // line_after_replacement = line_after_replacement.replace("eightwo", "82");
    // line_after_replacement = line_after_replacement.replace("eighthree", "83");
    // line_after_replacement = line_after_replacement.replace("nineight", "98");
    // // Normal values
    // line_after_replacement = line_after_replacement.replace("one", "1");
    // line_after_replacement = line_after_replacement.replace("two", "2");
    // line_after_replacement = line_after_replacement.replace("three", "3");
    // line_after_replacement = line_after_replacement.replace("four", "4");
    // line_after_replacement = line_after_replacement.replace("five", "5");
    // line_after_replacement = line_after_replacement.replace("six", "6");
    // line_after_replacement = line_after_replacement.replace("seven", "7");
    // line_after_replacement = line_after_replacement.replace("eight", "8");
    // line_after_replacement = line_after_replacement.replace("nine", "9");
 
    // Neater, but still weird, method
    // Preserve the first and last letter, so that overlapping strings are not ignored
    // However, only considers the case where only one letter overlaps
    // Found idea here: https://github.com/arpadav/aoc/blob/main/aoc2023/day01p2/src/main.rs
    line_after_replacement = line_after_replacement.replace("one", "o1e");
    line_after_replacement = line_after_replacement.replace("two", "t2o");
    line_after_replacement = line_after_replacement.replace("three", "t3e");
    line_after_replacement = line_after_replacement.replace("four", "f4r");
    line_after_replacement = line_after_replacement.replace("five", "f5e");
    line_after_replacement = line_after_replacement.replace("six", "s6x");
    line_after_replacement = line_after_replacement.replace("seven", "s7n");
    line_after_replacement = line_after_replacement.replace("eight", "e8t");
    line_after_replacement = line_after_replacement.replace("nine", "n9e");


    line_after_replacement
}