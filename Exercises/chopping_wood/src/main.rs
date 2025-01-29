use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = reader.lines();
    let output_size: u128 = input.next().unwrap().unwrap().parse().unwrap();
    let mut last_element: i128 = 0;

    let mut leaves: BinaryHeap<i128> = BinaryHeap::new();
    let mut non_leaves: HashMap<i128, u128> = HashMap::new();
    let mut input_list: Vec<i128> = Vec::new();

    for line in input {
        let num: i128 = line.unwrap().parse().unwrap();
        input_list.push(num);

        if num > last_element {
            last_element = num;
        }

        if non_leaves.contains_key(&num) {
            *non_leaves.get_mut(&num).unwrap() += 1;
        } else {
            non_leaves.insert(num, 1);
        }
    }

    if output_size != (last_element - 1).try_into().unwrap() {
        println!("Error");
    } else {
        for i in 1..=last_element {
            if !non_leaves.contains_key(&i) {
                leaves.push(-1 * i);
            }
        }

        for num in input_list {
            println!("{}", -1 * leaves.pop().unwrap());

            let new_value = non_leaves.get(&num).unwrap() - 1;
            non_leaves.insert(num, new_value);

            if new_value == 0 {
                leaves.push(-1 * num);
            }
        }
    }
}
