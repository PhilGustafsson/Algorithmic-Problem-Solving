use std::{io::{self, BufRead}, collections::{HashMap, VecDeque}};


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let first_line: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let cols: u8 = first_line[0] as u8; 
    let rows: u8 = first_line[1] as u8;

    let mut grid_mappings :HashMap<(u8, u8), char> = HashMap::new();
    let mut player_pos:(u8, u8) = (0,0);

    for (i, line) in lines.enumerate() {
        if i >= rows as usize {
            break;
        }

        let grid_line = line.unwrap();
        
        for (j, ch) in grid_line.chars().enumerate() {

            if ch == 'P' {
                player_pos = (j as u8, i as u8);
            }
            grid_mappings.insert((j as u8,i as u8), ch);
        }
    }

    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
    let mut visited: HashMap<(u8, u8), bool> = HashMap::new();
    let mut count: u32 = 0;

    queue.push_back(player_pos);
    visited.insert(player_pos, true);

    //BFS
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let neighbours = get_neighbours(&cols, &rows, &current);

        if !has_valid_neighbors(&neighbours, &grid_mappings) {
            continue;
        }

        for neighbour in neighbours {
            if !visited.contains_key(&neighbour) && grid_mappings.get(&neighbour).unwrap() != &'#' {
                if grid_mappings.get(&neighbour).unwrap() == &'G' {
                    count += 1;
                }
                visited.insert(neighbour, true);
                queue.push_back(neighbour);
            }
        }
    }

    println!("{}", count);

}

fn get_neighbours(col_size: &u8, row_size: &u8, pos: &(u8, u8)) -> Vec<(u8, u8)> {
    let mut neighbours = Vec::new();
    let (x, y) = pos;
    let x = *x as u8;
    let y = *y as u8;

    if x > 1 {
        neighbours.push((x - 1, y));
    }

    if x + 1 < *col_size {
        neighbours.push((x + 1, y));
    }

    if y > 1 {
        neighbours.push((x, y - 1));
    }

    if y + 1 < *row_size {
        neighbours.push((x, y + 1));
    }

    return neighbours
}

// Check so that no neighbour is a trap
fn has_valid_neighbors(neighbours: &Vec<(u8, u8)>, grid_mappings: &HashMap<(u8, u8), char>) -> bool {
    for neighbour in neighbours {
        if grid_mappings.get(&neighbour).unwrap() == &'T' {
            return false;
        }
    }
    return true;
}

