use std::{collections::{VecDeque}};


pub fn solve(grid: VecDeque<VecDeque<char>>,start: (usize,usize),previous_starts: &Vec<(usize,usize)>) -> usize {
    let mut positions_to_check = vec![vec![start]];
    let mut checked_positions:Vec<(usize,usize)> = vec![];
    let mut number_of_visits:usize = 0;
    let mut found = false;
    'total: loop {
        if positions_to_check.len() == 0 {
            break;
        }
        let positions = positions_to_check.pop().unwrap();
        let mut next_check = vec![];
        
        for position in &positions {
            let x = position.0;
            let y = position.1;
            let mut value = grid[x][y];
            if checked_positions.contains(&(x, y)) || previous_starts.contains(&(x, y)){
                continue;
            }
            checked_positions.push((x, y));
            if value == 'S' {
                value = '{';
            }
            if value == 'E' {
                found = true;
                break 'total;
            }
            let mut elevation_value = 0;
            
            if y > 0 {    
                if grid[x][y-1] == 'E' { elevation_value = 'z' as usize; } else { elevation_value = grid[x][y-1] as usize; }
                if value as usize + 1 >=  elevation_value {
                    next_check.push((x, y - 1));
                }
            }
            if y < grid[x].len() - 1 {
                if grid[x][y+1] == 'E' { elevation_value = 'z' as usize; } else { elevation_value = grid[x][y+1] as usize; }
                if value as usize + 1 >=  elevation_value {
                    next_check.push((x, y + 1));
                }
            }
            if x > 0 {
                if grid[x-1][y] == 'E' { elevation_value = 'z' as usize; } else { elevation_value = grid[x-1][y] as usize; }
                if value as usize + 1 >=  elevation_value {
                    next_check.push((x - 1, y));
                }
            }
            if x < grid.len() - 1 {
                if grid[x+1][y] == 'E' { elevation_value = 'z' as usize; } else { elevation_value = grid[x+1][y] as usize; }
                if value as usize + 1 >=  elevation_value {
                    next_check.push((x + 1, y));
                }
            }
        }
        if next_check.len() > 0 {
            positions_to_check.push(next_check);
        }
        number_of_visits += 1;
    }
    if found {
        number_of_visits
    } else {
        10000000
    }
}


pub fn run(input: &str) -> String {
    let replaced = input.replace("S", "a");
    let mut grid: VecDeque<VecDeque<char>>  = VecDeque::from(replaced.lines()
    .map(|x| VecDeque::from(x.chars().collect::<VecDeque<char>>()))
    .collect::<VecDeque<VecDeque<char>>>());
    let mut starts = vec![];
    let mut starts_visited = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value == 'a' {
                starts.push((x, y));
            }
        }
    }
    let mut minimum_path_length = 10000000;
    for start in starts {
        let path_length = solve(grid.clone(), start, &starts_visited);
        starts_visited.push(start);
        if path_length < minimum_path_length {
            minimum_path_length = path_length;
        }
    }
    minimum_path_length.to_string()
}

