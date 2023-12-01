use std::fs::read_to_string;

fn main() {
    day1_part2();
}

fn day1_part1() {
    println!("Day 1 Solution");
    let lines: Vec<String> = read_to_string("input/day1_test.txt")
        .unwrap() // panic on file read errors
        .lines() // split string into iterator of string slices
        .map(String::from) // make slices into strings
        .collect(); // make into a vector

    let mut sum = 0;
    for line in lines {
        let digits: Vec<char> = line.chars()
                        .filter(|x| x.is_digit(10))
                        .collect();

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let mut calibration_string = first.to_string();
        calibration_string.push(*last); 

        let calibration_value = calibration_string.parse::<i32>().unwrap();
        sum += calibration_value;
    }

    println!("{}", sum)

}

fn day1_part2() {
    println!("Day 1 Solution");
    let lines: Vec<String> = read_to_string("input/day1.txt")
        .unwrap() // panic on file read errors
        .lines() // split string into iterator of string slices
        .map(String::from) // make slices into strings
        .collect(); // make into a vector
    
    let mut sum = 0;
    
    for mut line in lines {
        line.push_str("    ");
        let chars: Vec<char> = line.chars().collect();
        let mut numbers = Vec::new();

        for window in chars.windows(5) {
            match window {
                ['o', 'n', 'e', _, _]     => numbers.push("1"),
                ['t', 'w', 'o', _, _]     => numbers.push("2"),
                ['t', 'h', 'r', 'e', 'e'] => numbers.push("3"),
                ['f', 'o', 'u', 'r', _]   => numbers.push("4"),
                ['f', 'i', 'v', 'e', _]   => numbers.push("5"),
                ['s', 'i', 'x', _, _]     => numbers.push("6"),
                ['s', 'e', 'v', 'e', 'n'] => numbers.push("7"),
                ['e', 'i', 'g', 'h', 't'] => numbers.push("8"),
                ['n', 'i', 'n', 'e', _]   => numbers.push("9"),
                ['1', _, _, _, _]         => numbers.push("1"),
                ['2', _, _, _, _]         => numbers.push("2"),
                ['3', _, _, _, _]         => numbers.push("3"),
                ['4', _, _, _, _]         => numbers.push("4"),
                ['5', _, _, _, _]         => numbers.push("5"),
                ['6', _, _, _, _]         => numbers.push("6"),
                ['7', _, _, _, _]         => numbers.push("7"),
                ['8', _, _, _, _]         => numbers.push("8"),
                ['9', _, _, _, _]         => numbers.push("9"),
                _ => (),
            }

        }

        let mut calibration_string: String = numbers.first().unwrap().to_owned().to_string();
        calibration_string.push_str(numbers.last().unwrap());

        let calibration_value = calibration_string.parse::<i32>().unwrap();
        sum += calibration_value;
    }

    println!("{}", sum)
}