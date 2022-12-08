use std::{collections::VecDeque};

fn is_visible(grid: &VecDeque<VecDeque<u32>>, x: usize, y: usize) -> bool {
    let content = grid[x][y];
    // return true for the ones in the corners
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return true;
    }
    // return false for smallest ones
    if content == 0 {
        return false;
    }
    // check the 4 directions
    // up
    for i in 0..x+1 {
        
        if i == x {
            return true;
        }
        let val = grid[i][y];
        if val >= content {
            break;
        }
    }
    // down
    for i in x+1..grid.len()+1 {
        if i == grid.len() {
            return true;
        }
        if grid[i][y] >= content {
            break;
        }
    }
    // left
    for i in 0..y+1 {
        if i == y {
            return true;
        }
        if grid[x][i] >= content {
            break;
        }
    }
    // right
    for i in y+1..grid[x].len()+1 {
        if i == grid[x].len() {
            return true;
        }
        if grid[x][i] >= content {
            break;
        }
    }
    return false;
}

pub fn run(input: &str) -> String {
    let lines = input.lines();
    let grid = 
        lines
            .map(|f| f.chars()
            .map(|c| c.to_digit(10).unwrap())
                .collect::<VecDeque<_>>())
        .collect::<VecDeque<_>>();
    let mut visible = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let x_visible = is_visible(&grid, i, j);
            if x_visible {
                visible += 1;
            }
            print!("{}", if x_visible { "X" } else { " " })
        }
        println!("");
    }
    return visible.to_string();
}

