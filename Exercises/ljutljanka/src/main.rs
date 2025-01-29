use std::{io::{self, BufRead}, collections::{BinaryHeap, HashMap}};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let mut amount_of_candy: u128 = first_line_split.next().unwrap().parse().unwrap();
    let amount_of_children: u128 = first_line_split.next().unwrap().parse().unwrap();

    let mut candy_wants = BinaryHeap::new();
    let mut desire_tracker = HashMap::new();
    for line in lines {
        let desire: u128 = line.unwrap().parse().unwrap();
        let entry = desire_tracker.entry(desire).or_insert(0);
        if *entry == 0 {
            candy_wants.push(desire);
        }
        *entry += 1;
    }

    let mut current_desire = candy_wants.pop().unwrap();
    while amount_of_candy > 0 {
        println!("Current desire: {}", current_desire);
        let mut amount_of_kids_with_desire = desire_tracker.get_mut(&current_desire).unwrap();
        let kids_with_desire = *amount_of_kids_with_desire;
        let mut is_last_desire = false;
        let next_desire = match candy_wants.pop() {
            Some(desire) => desire,
            None => {
                is_last_desire = true;
                0// Keep the current desire as there's no next desire
            },
        };

        let candies_to_hand_out = *amount_of_kids_with_desire * (current_desire - next_desire);
   
        if candies_to_hand_out <= amount_of_candy && !is_last_desire { 
            amount_of_candy -= candies_to_hand_out;

            *desire_tracker.get_mut(&next_desire).unwrap() += kids_with_desire;
            *desire_tracker.get_mut(&current_desire).unwrap() = 0;
            current_desire = next_desire;

        } else {
           
            let remaining_candies =  amount_of_candy % kids_with_desire;
            let candies_per_kid = amount_of_candy / kids_with_desire;
          
            let new_desire_1 = current_desire - candies_per_kid;
            let new_desire_2 = current_desire - candies_per_kid - 1;
      
            if candies_per_kid != 0 {
                *desire_tracker.entry(new_desire_1).or_insert(0) += kids_with_desire - remaining_candies;
                *desire_tracker.entry(current_desire).or_insert(0) -= kids_with_desire - remaining_candies;
                
            }
            if remaining_candies != 0 {
                *desire_tracker.entry(new_desire_2).or_insert(0) += remaining_candies;
                *desire_tracker.entry(current_desire).or_insert(0) -= remaining_candies;
               
            }
           
            amount_of_candy = 0;
        }
    }

    let sum_of_squares: u128 = desire_tracker.iter().map(|(&k, &v)| k * k * v as u128).sum();

    println!("{}", sum_of_squares);
}