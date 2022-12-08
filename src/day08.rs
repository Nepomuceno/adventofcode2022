use std::{collections::VecDeque};

fn scenic_score(grid: &VecDeque<VecDeque<u32>>, x: usize, y: usize) -> u32 {
    let content = grid[x][y];
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return 0;
    }
    // check the 4 directions
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    
    // up
    for i in (0..x).rev() {
        up += 1;
        if grid[i][y] >= content {
            break;
        }
    }
    // down
    for i in x+1..grid.len() {
        down += 1;
        if grid[i][y] >= content {
            break;
        }
    }
    // left
    for i in (0..y).rev() {
        left += 1;
        if grid[x][i] >= content {
            break;
        }
    }
    // right
    for i in y+1..grid[x].len() {
        right += 1;
        if grid[x][i] >= content {
            break;
        }
    }
    return up*down*left*right;
}

pub fn run(input: &str) -> String {
    let lines = input.lines();
    let grid = 
        lines
            .map(|f| f.chars()
            .map(|c| c.to_digit(10).unwrap())
                .collect::<VecDeque<_>>())
        .collect::<VecDeque<_>>();
    let mut max_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let score = scenic_score(&grid, i, j);
            if score > max_score {
                max_score = score;
            }
            print!("{} ", score)
        }
        println!("");
    }
    return max_score.to_string();
}

