use std::{collections::{VecDeque}};

pub fn print_grid(grid: Vec<Vec<char>>) {
    for row in grid.iter() {
        for col in row.iter().enumerate() {
            if col.0 > 2450 && col.0 < 2600 {
                print!("{}",col.1);
            }
        }
        println!("");
    }
}



pub fn run(input: &str) -> String {
    let mut rock_lines = vec![];
    for line in input.lines() {
        let paths = line.split(" -> ").collect::<Vec<&str>>();
        for i in 1..paths.len() {
            let start = 
            paths[i-1].split_once(",")
                .map(|x| (x.0.trim().parse::<usize>().unwrap() + 2000,x.1.trim().parse::<usize>().unwrap()))
                .unwrap();
            let end =
            paths[i].split_once(",")
                .map(|x| (x.0.trim().parse::<usize>().unwrap() + 2000,x.1.trim().parse::<usize>().unwrap()))
                .unwrap();
            rock_lines.push((start,end));
        }
    }
    for rock_line in rock_lines.iter() {
        // println!("{:?}",rock_line);
    }
    // get max x and y
    let mut max_x = 0;
    let mut max_y = 0;
    for rock_line in rock_lines.iter() {
        if rock_line.0.0 > max_x {
            max_x = rock_line.0.0;
        }
        if rock_line.0.1 > max_y {
            max_y = rock_line.0.1;
        }
        if rock_line.1.0 > max_x {
            max_x = rock_line.1.0;
        }
        if rock_line.1.1 > max_y {
            max_y = rock_line.1.1;
        }
    }
    let mut grid = vec![vec!['.';max_x+1000];max_y+3];
    for rock_line in rock_lines.iter() {
        // draw straigt horizontal or vertical line between the two points
        // they can be in any order
        if rock_line.0.0 == rock_line.1.0 {
            // vertical line
            let start = rock_line.0.1.min(rock_line.1.1);
            let end = rock_line.0.1.max(rock_line.1.1);
            for y in start..=end {
                grid[y][rock_line.0.0] = '#';
            }
        } else {
            // horizontal line
            let start = rock_line.0.0.min(rock_line.1.0);
            let end = rock_line.0.0.max(rock_line.1.0);
            for x in start..=end {
                grid[rock_line.0.1][x] = '#';
            }
        }
    }
    let end = grid.len()-1;
    // add floor
    for x in 0..grid[0].len() {
        grid[end][x] = '#';
    }
    let mut sand_count = 0;
    let mut interactions = 0;
    grid[0][2500] = 'o';
    let mut sand = (1500,0);
    print_grid(grid.clone());
    loop {
        interactions += 1;
        // if the sand is at the bottom, we are done
        if sand.1 == grid.len()-1 {
            break;
        }
        // if the sand is not at the bottom, check if there is block below it
        if grid[sand.1+1][sand.0] == '.' {
            // if there is sand below it, move it down
            grid[sand.1+1][sand.0] = 'o';
            grid[sand.1][sand.0] = '.';
            sand = (sand.0,sand.1+1);
        } else {
            // if there is no sand below it, check if there is block to the left or right
            if grid[sand.1+1][sand.0-1] == '.' {
                // if there is sand to the left, move it down and left
                grid[sand.1+1][sand.0-1] = 'o';
                grid[sand.1][sand.0] = '.';
                sand = (sand.0-1,sand.1+1);
            } else if grid[sand.1+1][sand.0+1] == '.' {
                // if there is sand to the right, move it down and right
                grid[sand.1+1][sand.0+1] = 'o';
                grid[sand.1][sand.0] = '.';
                sand = (sand.0+1,sand.1+1);
            } else {
                if sand == (2500,0) {
                    break;
                }
                grid[sand.1][sand.0] = 'x';
                grid[0][2500] = 'o';
                sand = (2500,0);
                sand_count += 1;
            }
        }
        if interactions % 1000000 == 0 {
            println!("interactions: {}",interactions);
            print_grid(grid.clone());
        }
        
    }
    print_grid(grid.clone());
    println!("----------------");
    sand_count.to_string()
}
