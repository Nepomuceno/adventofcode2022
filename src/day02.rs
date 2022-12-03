pub fn run(input: &str) -> String {
    let mut score = 0;
    for line in input.lines() {
        let round: i32;
        match line {
            "A X" => round = 3,
            "A Y" => round = 4,
            "A Z" => round = 8,
            "B X" => round = 1,
            "B Y" => round = 5,
            "B Z" => round = 9,
            "C X" => round = 2,
            "C Y" => round = 6,
            "C Z" => round = 7,
            _ => panic!("Invalid score")
        }
        score += round;
        println!("round: {line} score: {round} / {score} ")
    }
    score.to_string()
}