use std::collections::HashSet;

const PRINT_ENABLED: bool = false;

pub fn run(input: &str) -> String {
    let cube_positions = input.lines().map(|x| -> (i32, i32, i32) {
        let content = x.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        (content[0], content[1], content[2])
    })
    .collect::<HashSet<(i32,i32,i32)>>();
    let mut faces_covered = vec![];
    for position in cube_positions.iter() {
        let mut faces = 6;
        if cube_positions.contains(&(position.0, position.1 + 1, position.2)) {
            faces = faces - 1;
        }
        if cube_positions.contains(&(position.0, position.1 - 1, position.2)) {
            faces = faces - 1;
        }
        if cube_positions.contains(&(position.0 + 1, position.1, position.2)) {
            faces = faces - 1;
        }
        if cube_positions.contains(&(position.0 - 1, position.1, position.2)) {
            faces = faces - 1;
        }
        if cube_positions.contains(&(position.0, position.1, position.2 + 1)) {
            faces = faces - 1;
        }
        if cube_positions.contains(&(position.0, position.1, position.2 - 1)) {
            faces = faces - 1;
        }
        faces_covered.push(faces);
    }
    faces_covered.iter().sum::<i32>().to_string()
}
