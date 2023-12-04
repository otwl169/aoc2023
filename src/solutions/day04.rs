use std::fs::read_to_string;
use std::collections::HashSet;

pub fn day4_part1() {
    let lines: i32 = read_to_string("input/04.txt")
        .unwrap()
        .lines()
        .fold(0, |mut sum: i32, line| {
            line.to_string();
            let (_card, numbers) = line.split_once(":").unwrap();
            let (winning_numbers_str, owned_numbers_str) = numbers.split_once("|").unwrap();
            let mut winning_numbers_set: HashSet<_>  = winning_numbers_str.trim().split(" ").collect();
            let owned_numbers_set: HashSet<_> = owned_numbers_str.trim().split(" ").collect();
            winning_numbers_set.remove(&""); // remove empty string
    
            let wins: u32 = winning_numbers_set.intersection(&owned_numbers_set).count().try_into().unwrap();
            if wins != 0 {
                sum += 2_i32.pow(wins-1);
            }
           
            sum
        });
    
    println!("{}", lines);
}

pub fn day4_part2() {
    let wins_vec: Vec<u32> = read_to_string("input/04.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.to_string();
            let (_card, numbers) = line.split_once(":").unwrap();
            let (winning_numbers_str, owned_numbers_str) = numbers.split_once("|").unwrap();
            let mut winning_numbers_set: HashSet<_>  = winning_numbers_str.trim().split(" ").collect();
            let owned_numbers_set: HashSet<_> = owned_numbers_str.trim().split(" ").collect();
            winning_numbers_set.remove(&""); // remove empty string
    
            let wins: u32 = winning_numbers_set.intersection(&owned_numbers_set).count().try_into().unwrap();
            wins
        })
        .collect();
    
    let mut cards_vec = vec![1; wins_vec.len()];
    for (i, &win) in wins_vec.iter().enumerate() {
        for j in 1..=win {
            if (i + j as usize) < cards_vec.len() {
                cards_vec[i+j as usize] += 1 * cards_vec[i];
            }
        }
    }

    let sum: u32 = cards_vec.iter().sum();
    println!("{}", sum);
}