use std::{
    collections::{HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let number_of_cases: u8 = lines.next().unwrap().unwrap().parse().unwrap();


    for _ in 0..number_of_cases{  
        let number_of_islands: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut islands: Vec<(f64, f64)> = Vec::with_capacity(number_of_islands);
        for _ in 0..number_of_islands{
            let island: String = lines.next().unwrap().unwrap();
            let island: Vec<f64> = island
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
            let coord = (island[0] as f64, island[1] as f64);
            islands.push(coord);
        }
        println!("{:?}", prims_algorithm(&islands));
    }
}

fn calculate_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn prims_algorithm(islands: &Vec<(f64, f64)>) -> f64 {
    let n = islands.len();
    let mut visited: HashSet<usize> = HashSet::new();   
    let mut unvisited: Vec<usize> = (1..n).collect();
    let mut total_distance = 0.0;

    visited.insert(0);

    while !unvisited.is_empty() {
        let mut min_distance = f64::MAX;
        let mut next_index: usize = 0;

       for i in visited.iter() {
            for &j in &unvisited {
                let distance = calculate_distance(islands[*i], islands[j]);
                if distance < min_distance {
                    min_distance = distance;
                    next_index = Some(j).unwrap();
                }
            }
        }

        total_distance += min_distance;
        visited.insert(next_index);
        unvisited.retain(|&k| k != next_index);
      
    }
    total_distance
}

