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


}

fn open_input_file() -> String {
    let input_filename = "input.txt";
    fs_err::read_to_string(input_filename).unwrap()
}

