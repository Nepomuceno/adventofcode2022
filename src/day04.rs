use std::collections::VecDeque;

pub fn run(input: &str) -> String {
    let mut overlap = 0;
    for line in input.lines() {
        let pair = line.split(',')
        .map(|x| x.split('-')
                        .map(|f| f.parse::<u32>().unwrap()).collect::<VecDeque<_>>())
        .collect::<VecDeque<_>>();
        if pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]
        {
            overlap += 1;
        } else if pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1] {
            overlap += 1;
        }
        println!("{overlap}")
    }
    overlap.to_string()
}