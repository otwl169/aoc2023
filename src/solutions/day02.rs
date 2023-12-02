use std::fs::read_to_string;
use std::cmp::max;

pub fn day2_part1() {
    let lines: Vec<String> = read_to_string("input/02.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect(); // parts[0] = "Game x", parts[1] = 3 Blue, ...
        
        
        let mut blues = 0;
        let mut reds = 0;
        let mut greens = 0;
        let rounds: Vec<&str> = parts[1].split(";").collect(); // rounds[0] = 3 Blue, 4 Red
        for round in rounds {
            let reveals: Vec<&str> = round.split(",").collect();

            for reveal in reveals {
                let count_color: Vec<&str> = reveal.split(' ').collect();
                let count = count_color[1].parse::<i32>().unwrap(); // count_color[0] = ""
                let color = count_color[2];

                match color {
                    "blue" => blues = max(count, blues),
                    "red" => reds = max(count, reds),
                    "green" => greens = max(count, greens),
                    _ => (),
                }
            }
        }
        
        if reds <= 12 && greens <= 13 && blues <= 14 {
            let game_id = parts[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            sum += game_id;
        }
    }

    println!("{}", sum);
}


pub fn day2_part2() {
    let lines: Vec<String> = read_to_string("input/02.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect(); // parts[0] = "Game x", parts[1] = 3 Blue, ...
        
        
        let mut blues = 0;
        let mut reds = 0;
        let mut greens = 0;
        let rounds: Vec<&str> = parts[1].split(";").collect(); // rounds[0] = 3 Blue, 4 Red
        for round in rounds {
            let reveals: Vec<&str> = round.split(",").collect();

            for reveal in reveals {
                let count_color: Vec<&str> = reveal.split(' ').collect();
                let count = count_color[1].parse::<i32>().unwrap(); // count_color[0] = ""
                let color = count_color[2];

                match color {
                    "blue" => blues = max(count, blues),
                    "red" => reds = max(count, reds),
                    "green" => greens = max(count, greens),
                    _ => (),
                }
            }
        }
        
        sum += reds * blues * greens;
    }

    println!("{}", sum);
}