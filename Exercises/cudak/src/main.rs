use std::io::{self};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("Failed to parse integer"))
        .collect();

    let start: u64 = input[0];
    let end: u64 = input[1];
    let target: u64 = input[2];

    let mut min_value = find_smallest(start, end, target);

    let count = count_up_to(end, target) - count_up_to(start - 1, target);
    println!("{} {}", count, min_value);
}

fn dynamic_counter(
    pos: usize,
    initial_sum: u64,
    restricted: bool,
    digits: Vec<u8>,
    target_sum: u64,
    cache: &mut HashMap<(usize, u64, bool), u64>,
) -> u64 {
    
    if pos == digits.len() {
        return if initial_sum == target_sum { 1 } else { 0 };
    }

    let key = (pos, initial_sum, restricted);
    if let Some(&res) = cache.get(&key) {
        return res;
    }

    let max_digit = if restricted { digits[pos] } else { 9 };
    let mut count = 0;

    for d in 0..=max_digit {
        let new_sum = initial_sum + d as u64;
        if new_sum > target_sum {
            continue;
        }
      
        let updated_restricted = restricted && d == max_digit;
        let digits_copy = digits.clone();
        count += dynamic_counter(pos + 1, new_sum, updated_restricted, digits_copy, target_sum, cache);
    }

    cache.insert(key, count);
    return count
}

fn count_up_to(x: u64, target: u64) -> u64 {
    if x == 0 {
        return 0;
    }

    let digits: Vec<u8> = x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    
    let mut cache = HashMap::new();
    dynamic_counter(0, 0, true, digits, target, &mut cache)
}

fn find_smallest(start: u64, end: u64, target: u64) -> u64 {
    let mut low: u64 = start;
    let mut high:u64 = end;
    let mut min_value:u64 = end;
    
    while low <= high{
        let mut mid: u64 = low + (high - low) / 2;
        if count_up_to(mid, target) - count_up_to(low - 1, target) > 0{
            min_value = mid;
            high = mid - 1;
        }
        else{
            low = mid + 1;
        }
    }
    
    return min_value;
}