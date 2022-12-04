use regex::Regex;

#[derive(PartialEq)]
pub enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq,Debug)]
pub enum RoundResult {
    Win,
    Draw,
    Loss,
}

pub fn play_round(them: &Action, me: &Action) -> RoundResult {
    match (them, me) {
        (Action::Rock, Action::Paper) |
            (Action::Paper, Action::Scissors) |
            (Action::Scissors, Action::Rock) => RoundResult::Win,
        (act1, act2) if act1 == act2 => RoundResult::Draw,
        _ => RoundResult::Loss
    }
}

pub fn score_guide_round(input_str: &str) -> usize {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let caps = re.captures(input_str).unwrap();

    let them = match caps.get(1).unwrap().as_str() {
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissors,
        _ => panic!("Invalid input")
    };
    let required_result = match caps.get(2).unwrap().as_str() {
        "X" => RoundResult::Loss,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Invalid input")
    };
    let me = if required_result == play_round(&them, &Action::Rock) {
        Action::Rock
    } else if required_result == play_round(&them, &Action::Paper) {
        Action::Paper
    } else {
        Action::Scissors
    };

    let round_score = match play_round(&them, &me) {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    };
    round_score + match me {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors_round() {
        assert_eq!(play_round(&Action::Rock, &Action::Rock), RoundResult::Draw);
        assert_eq!(play_round(&Action::Rock, &Action::Paper), RoundResult::Win);
        assert_eq!(play_round(&Action::Rock, &Action::Scissors), RoundResult::Loss);
        assert_eq!(play_round(&Action::Paper, &Action::Rock), RoundResult::Loss);
        assert_eq!(play_round(&Action::Paper, &Action::Paper), RoundResult::Draw);
        assert_eq!(play_round(&Action::Paper, &Action::Scissors), RoundResult::Win);
        assert_eq!(play_round(&Action::Scissors, &Action::Rock), RoundResult::Win);
        assert_eq!(play_round(&Action::Scissors, &Action::Paper), RoundResult::Loss);
        assert_eq!(play_round(&Action::Scissors, &Action::Scissors), RoundResult::Draw);
    }

    #[test]
    fn test_example_2022_day2() {
        let input_lines = "A Y
B X
C Z"
            .lines()
            .map(|s| s.to_string());
        assert_eq!(
            input_lines
                .fold(0, |cum, s| cum + score_guide_round(&s)),
            15
        );
    }

}
