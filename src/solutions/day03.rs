use std::fs::read_to_string;
use std::cmp::min;

pub fn day3_part1() {
    let lines: Vec<String> = read_to_string("input/03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    let line_length = lines[0].len();
    for i in 0..=lines.len()-1 {
        let mut num_string = "".to_string();
        let mut num: i32 = 0;
        for (j, c) in lines[i].chars().enumerate() {
            
            if c.is_digit(10) {
                num_string.push(c);
            }

            if (c.is_digit(10) && (j == line_length-1))
            || (!c.is_digit(10) && (num_string.len() > 0)) {
                // Line ends number or number ends with symbol

                // check number connected to a symbol (not  '.')
                let start = if j - num_string.len() == 0 {0} else {j - num_string.len() - 1};
                for x in start..=j {
                    let above = if i == 0 {0} else {i-1};
                    let char_above = lines[above].chars().collect::<Vec<_>>()[x]; //.nth next time
                    let char_on = lines[i].chars().collect::<Vec<_>>()[x];
                    let char_below = lines[min(lines.len()-1, i+1)].chars().collect::<Vec<_>>()[x];

                    if (!char_above.is_digit(10) && (char_above != '.')) 
                    || (!char_on.is_digit(10) && (char_on != '.'))
                    || (!char_below.is_digit(10) && (char_below != '.')) {
                        num = num_string.parse().unwrap();
                        // println!("{}", num);
                    }
                }
                num_string = "".to_string();
            }

            sum += num;
            num = 0;
        }
    }

    println!("{}", sum);
}


pub fn day3_part2() {
    let lines: Vec<String> = read_to_string("input/03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    let line_length = lines[0].len();
    for i in 0..=lines.len()-1 {
        for (j, c) in lines[i].chars().enumerate() {
            if c == '*' {
                // println!();
                // check for numbers around it and add to surrounding numbers vec
                let mut surrounding_numbers = Vec::new();

                // get all numbers on the row above, on and below and check if they touch this range
                let above = if i == 0 {0} else {i-1};
                let below = min(lines.len()-1, i+1);
                for line_offset in above..=below {
                    let mut num_string = "".to_string();

                    for (k, ch) in lines[line_offset].chars().enumerate() {
                        if ch.is_digit(10) {
                            num_string.push(ch);
                        }

                        // println!("before check");
                        if (ch.is_digit(10) && (k == line_length-1))
                        || (!ch.is_digit(10) && (num_string.len() > 0)) {
                            // end of number
                            
                            // connected to * iff [k-len(num),k] and [j-1,j+1] overlap
                            let ast_interval_start = if j == 0 {0} else {j-1};
                            let num_interval_start = k - num_string.len();
                            let ast_interval_end = min(j+1, line_length-1);
                            let num_interval_end = k-1;

                            // println!("number {}\t numrange: {}-{}; astrange: {}-{}", num_string, num_interval_start, num_interval_end, ast_interval_start, ast_interval_end);

                            if (num_interval_start <= ast_interval_end)
                            && (num_interval_end >= ast_interval_start) {
                                let num: i32 = num_string.parse().unwrap();
                                surrounding_numbers.push(num);
                            }
                            
                            num_string = "".to_string();
                        }
                    }
                }

                // check if only two are connected
                if surrounding_numbers.len() == 2 {
                    // println!("{:?}", surrounding_numbers);
                    sum += surrounding_numbers[0] * surrounding_numbers[1];
                }
            }
        }
    }

    println!("{}", sum);
}