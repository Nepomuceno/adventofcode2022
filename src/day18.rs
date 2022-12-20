use std::{collections::HashSet, vec};


const PRINT_ENABLED: bool = true;

fn is_encapsulated(mut position: (i32, i32, i32), capsule: HashSet<(i32, i32, i32)>) -> bool {
    let mut check_free = capsule.clone();
    let free_faces = free_faces(&position, check_free);
    if free_faces.0.len() == 5 {
        return false;
    }
    let mut result = true;
    position = (position.0, position.1 + 1, position.2);
    if !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    position = (position.0, position.1 - 1, position.2);
    if result && !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    position = (position.0 + 1, position.1, position.2);
    if result && !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    position = (position.0 - 1, position.1, position.2);
    if result && !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    position = (position.0, position.1, position.2 + 1);
    if result && !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    position = (position.0, position.1, position.2 - 1);
    if result && !capsule.contains(&position) {
        let mut new_capsule = capsule.clone();
        new_capsule.insert(position);
        result = is_encapsulated(position, new_capsule)
    }
    result
}

fn free_faces(position: &(i32, i32, i32), cube_positions: HashSet<(i32, i32, i32)>) -> (HashSet<(i32,i32,i32)>,HashSet<(i32,i32,i32)>) {
    let mut free_faces = HashSet::new();
    let mut touching_faces = HashSet::new();
    if !cube_positions.contains(&(position.0, position.1 + 1, position.2)) {
        free_faces.insert((position.0, position.1 + 1, position.2));
    } else {
        touching_faces.insert((position.0, position.1 + 1, position.2));
    }
    if !cube_positions.contains(&(position.0, position.1 - 1, position.2)) {
        free_faces.insert((position.0, position.1 - 1, position.2));
    } else {
        touching_faces.insert((position.0, position.1 - 1, position.2));
    }
    if !cube_positions.contains(&(position.0 + 1, position.1, position.2)) {
        free_faces.insert((position.0 + 1, position.1, position.2));
    } else {
        touching_faces.insert((position.0 + 1, position.1, position.2));
    }
    if !cube_positions.contains(&(position.0 - 1, position.1, position.2)) {
        free_faces.insert((position.0 - 1, position.1, position.2));
    } else {
        touching_faces.insert((position.0 - 1, position.1, position.2));
    }
    if !cube_positions.contains(&(position.0, position.1, position.2 + 1)) {
        free_faces.insert((position.0, position.1, position.2 + 1));
    } else {
        touching_faces.insert((position.0, position.1, position.2 + 1));
    }
    if !cube_positions.contains(&(position.0, position.1, position.2 - 1)) {
        free_faces.insert((position.0, position.1, position.2 - 1));
    } else {
        touching_faces.insert((position.0, position.1, position.2 - 1));
    }
    (free_faces, touching_faces)
}

pub fn run(input: &str) -> String {
    let cube_positions = input.lines().map(|x| -> (i32, i32, i32) {
        let content = x.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        (content[0], content[1], content[2])
    })
    .collect::<HashSet<(i32,i32,i32)>>();
    let mut exposed_faces: HashSet<(i32,i32,i32)> = HashSet::new();
    let mut total_faces_exposed = 0;
    for position in cube_positions.iter() {
        let result = free_faces(position, cube_positions.clone());
        exposed_faces.extend(result.0.iter());
        total_faces_exposed += result.0.len();
    }
    
    if PRINT_ENABLED {
        println!("Positions exposed: {:?}", exposed_faces.len());
        println!("Total faces exposed: {:?}", total_faces_exposed);
        println!("Total faces: {:?}", cube_positions.len() * 6);
    }

    let mut x_encapsulated_faces = HashSet::new();
    for position in exposed_faces.iter() {
        if is_encapsulated(*position, cube_positions.clone()) {
            x_encapsulated_faces.insert(*position);
        }
    }
    println!("------ x ------");
    if PRINT_ENABLED {
        println!("Encapsulated faces: {:?}", x_encapsulated_faces.len());
        println!("Total faces: {:?}", exposed_faces.len() * 6);
    }
    let mut y_faces_covered: HashSet<(i32,i32,i32)> = HashSet::new();
    let mut y_exposed_faces: HashSet<(i32,i32,i32)> = HashSet::new();
    let non_encapsulated = exposed_faces.difference(&x_encapsulated_faces).collect::<HashSet<_>>();
    let mut y_total_faces_exposed = 0;
    let mut y_total_faces_covered = 0;
    for position in non_encapsulated.iter() {
        let result = free_faces(position, cube_positions.clone());
        y_exposed_faces.extend(result.0.iter());
        y_faces_covered.extend(result.1.iter());
        y_total_faces_exposed += result.0.len();
        y_total_faces_covered += result.1.len();
    }

    println!("------ y ------");
    if PRINT_ENABLED {
        println!("Positions covered: {:?}", y_faces_covered.len());
        println!("Positions exposed: {:?}", y_exposed_faces.len());
        println!("Total faces covered: {:?}", y_total_faces_covered);
        println!("Total faces exposed: {:?}", y_total_faces_exposed);
        println!("Total faces: {:?}", cube_positions.len() * 6);
    }

    (y_total_faces_covered as i32).to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_day18() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!("58", run(input));
    }
    #[test]
    fn test_simple() {
        let input = "0,0,0
0,0,1
0,1,0
0,1,1
1,0,0
1,0,1
1,1,0
1,1,1";
        assert_eq!("24", run(input));
    }

    #[test]
    fn test_square_with_hole() {
        let mut input = "".to_string();
        let mut content = HashSet::new();
        let mut empty = HashSet::new();
        for i in 1..=2 {
            for j in 1..=2 {
                for k in 1..=2 {
                    empty.insert((i,j,k));
                }
            }
        }
        for i in 0..=3 {
            for j in 0..=3 {
                for k in 0..=3 {
                    content.insert((i,j,k));
                }
            }
        }
        content = content.difference(&empty).map(|x| *x).collect();
        for position in content.iter() {
            let temp = input.to_string();
            input = format!("{}\n{},{},{}", temp, position.0, position.1, position.2);
        }
        assert_eq!("96", run(&input.trim()));

    }

    #[test]
    fn test_square_with_hole_punched() {
        let mut input = "".to_string();
        let mut content = HashSet::new();
        let mut empty = HashSet::new();
        for i in 1..=2 {
            for j in 1..=2 {
                for k in 1..=2 {
                    empty.insert((i,j,k));
                }
            }
        }
        for i in 0..=3 {
            for j in 0..=3 {
                for k in 0..=3 {
                    content.insert((i,j,k));
                }
            }
        }
        content = content.difference(&empty).map(|x| *x).collect();
        let _removed = content.remove(&(3,2,0));
        let _removed = content.remove(&(3,2,1));
        for position in content.iter() {
            let temp = input.to_string();
            input = format!("{}\n{},{},{}", temp, position.0, position.1, position.2);
        }
        assert_eq!("122", run(&input.trim()));
    }
    #[test]
    fn test_complex() {
        let file = fs::read_to_string(format!("data/day18.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("2534", run(&file));
    }
}