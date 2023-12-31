use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"];
    let mut scores: HashMap<i32, HashMap<&str, i32>> = HashMap::new();

    for game in input.iter() {
        let mut game_iter = game.split(": ");
        let game_number = game_iter.next().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        let game_scores = game_iter.next().unwrap().split("; ");

        let mut color_scores: HashMap<&str, i32> = HashMap::new();
        for score in game_scores {
            let mut score_iter = score.split(", ");
            while let Some(color_score) = score_iter.next() {
                let mut color_score_iter = color_score.split_whitespace();
                let score = color_score_iter.next().unwrap().parse::<i32>().unwrap();
                let color = color_score_iter.next().unwrap();
                *color_scores.entry(color).or_insert(0) += score;
            }
            
        }
        scores.insert(game_number, color_scores);
    }
    println!("{:?}", scores);
}