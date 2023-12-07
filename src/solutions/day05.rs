use std::fs::read_to_string;
use std::cmp::min;
use std::cmp::max;

fn map_seed(seed: u32, map: String) -> u32 {
    let mut new_seed = seed;
    let mut found_map = false;
    let (_, maplines) = map.split_once("\n").unwrap();
    
    for map in maplines.split("\n") {
        let tmp: Vec<_> = map.split(" ").collect();
        let d_start: u32 = tmp[0].parse().unwrap();
        let s_start: u32 = tmp[1].parse().unwrap();
        let length: u32 = tmp[2].parse().unwrap();

        if seed >= s_start && seed <= s_start + length && !found_map {
            new_seed = seed - s_start + d_start;
            found_map = true;
        }
    }

    return new_seed;
}

pub fn day5_part1() {
    let seeds_and_maps: Vec<_> = read_to_string("input/05.txt")
        .unwrap()
        .split("\n\n")
        .map(str::to_owned)
        .collect();

    // there are 7 maps in total
    let seeds: Vec<_> = seeds_and_maps[0].split(' ')
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
            
    let loc: u32 = seeds.iter()
        .map(|seed| {
            let mut s = *seed;
            for map in seeds_and_maps.iter().skip(1) {
                s = map_seed(s, map.to_string());
            }

            s
        })
        .min()
        .unwrap();

    println!("{}", loc);

    

}

struct MyRange {
    start: u32,
    end: u32,
}

impl MyRange {
    fn intersect(b: &MyRange) -> Vec<MyRange> {
        let ranges = Vec::new();
        if b.start > a.end || a.start > b.end {
            return a;
        } else {
            let overlap = MyRange {
                start: max(a.start, b.start),
                end: min(a.end, b.end),
            };
            if a.start <= b.start {
                let a_before = MyRange {
                    start: a.start,
                    end: min(b.start, a.end),
                };
            }
            if a.end >= b.end {
                let a_after = MyRange {
                    start: max(b.end, a.start),
                    end: a.end,
                };
            }

            
        }
    }
}

fn map_seed_range(seed: MyRange, map: String) -> Vec<MyRange> {
    let mut new_seed = seed;
    let mut found_map = false;
    let (_, maplines) = map.split_once("\n").unwrap();
    
    for map in maplines.split("\n") {
        let tmp: Vec<_> = map.split(" ").collect();
        let d_start: u32 = tmp[0].parse().unwrap();
        let s_start: u32 = tmp[1].parse().unwrap();
        let length: u32 = tmp[2].parse().unwrap();

        if seed >= s_start && seed <= s_start + length && !found_map {
            new_seed = seed - s_start + d_start;
            found_map = true;
        }
    }

    return new_seed;
}

pub fn day5_part2() {
    let seeds_and_maps: Vec<_> = read_to_string("input/05.txt")
        .unwrap()
        .split("\n\n")
        .map(str::to_owned)
        .collect();

    // there are 7 maps in total
    let seed_numbers: Vec<_> = seeds_and_maps[0].split(' ')
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    
    let seed_ranges: Vec<MyRange> = Vec::new();
    for i in 0..seed_numbers.len()-1 {
        seed_ranges.push(MyRange{start: seed_numbers[i], end: seed_numbers[i] + seed_numbers[i+1]});
    }
            
    let loc: u32 = seeds_ranges.iter()
        .map(|seed_range| {
            let mut ranges = Vec![*seed_range];
            for map in seeds_and_maps.iter().skip(1) {
                (overlap, s) = map_seed_range(s, map.to_string());
            }

            s
        })
        .min()
        .unwrap();

    println!("{}", loc);
}