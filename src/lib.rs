mod old {
    pub mod bingo;
    pub mod bitaccumulator;
    pub mod calories;
    pub mod command;
    pub mod coordinates;
    pub mod crabs;
    pub mod lanternfish;
    pub mod position;
    pub mod rockpaperscissors;
    pub mod segment_display;
    pub mod valuemap;
}

use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use crate::old::bingo::BingoState;
use crate::old::bitaccumulator::DiagsReport;
use crate::old::calories::CalorieCounter;
use crate::old::command::Command;
use crate::old::coordinates::{GridCounter, LineSegment};
use crate::old::lanternfish::LanternShoal;
use crate::old::position::Position;
use crate::old::rockpaperscissors::score_guide_round;
use crate::old::segment_display::{SegmentDisplay, SegmentMapping};
use crate::old::crabs;

pub fn run_solution(
    year: usize,
    day: usize,
    puzzle: usize,
    input_strings: impl Iterator<Item = String>,
) -> Option<usize> {
    match (year, day, puzzle) {
        (2021, 1, 1) => Some(
            parse_input(input_strings)
                .tuple_windows()
                .filter(|tuple: &(usize, usize)| tuple.1 > tuple.0)
                .count(),
        ),
        (2021, 1, 2) => Some(
            parse_input(input_strings)
                .tuple_windows()
                .map(|tuple: (usize, usize, usize)| tuple.0 + tuple.1 + tuple.2)
                .tuple_windows()
                .filter(|tuple: &(usize, usize)| tuple.1 > tuple.0)
                .count(),
        ),
        (2021, 2, 1) => {
            println!("Solution no longer available");
            None
        }
        (2021, 2, 2) => Some(
            nonempty_input_lines(input_strings)
                .map(|command_str| Command::from_str(&command_str))
                .fold(Position::at_zero(), |pos, com| pos.exec_command(com))
                .multiply_x_by_depth(),
        ),
        (2021, 3, 1) => {
            let dr = DiagsReport::from_iter(12, binary_from_input(input_strings));
            Some(dr.gamma_rate() * dr.epsilon_rate())
        }
        (2021, 3, 2) => {
            let dr = DiagsReport::from_iter(12, binary_from_input(input_strings));
            Some(dr.oxygen_rate() * dr.co2_scrub_rate())
        }
        (2021, 4, 1) => Some(
            BingoState::from_strs(5, nonempty_input_lines(input_strings))
                .run_until(|b| b.any_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 4, 2) => Some(
            BingoState::from_strs(5, nonempty_input_lines(input_strings))
                .run_until(|b| b.all_complete())
                .multiply_complete_sum_unmarked_by_last_number()
                .unwrap(),
        ),
        (2021, 5, 1) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .filter(|ls| ls.is_horiz() || ls.is_vert())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 5, 2) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|seg_str| LineSegment::from_str(&seg_str).ok())
                .flat_map(|ls| ls.coords())
                .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
                .into_values()
                .filter(|&v| v > 1)
                .count(),
        ),
        (2021, 6, 1) => Some(
            LanternShoal::from_str(&single_line_from_input(input_strings))
                .proceed_n_days(80)
                .count(),
        ),
        (2021, 6, 2) => Some(
            LanternShoal::from_str(&single_line_from_input(input_strings))
                .proceed_n_days(256)
                .count(),
        ),
        (2021, 7, 1) => {
            println!("Solution no longer available");
            None
        }
        (2021, 7, 2) => {
            let inputs: Vec<usize> = single_line_from_input(input_strings)
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let min_cost = (0..).fold_while(usize::MAX, |old_cost, new_position| {
                crabs::fold_step(&inputs, old_cost, new_position)
            });
            Some(min_cost.into_inner())
        }
        (2021, 8, 1) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|display| {
                    display
                        .split(" | ")
                        .map(|segments| segments.split(' ').map(|s| s.to_string()).collect_vec())
                        .collect_tuple::<(_, _)>()
                })
                .fold(0, |outer_count, (input_segs, output_segs)| {
                    let mapping =
                        SegmentMapping::find_valid(&input_segs).expect("No valid mapping");
                    outer_count
                        + output_segs.iter().fold(0, |count, seg| {
                            match SegmentDisplay::from_str_with_mapping(seg, &mapping).to_int() {
                                Some(1) | Some(4) | Some(7) | Some(8) => count + 1,
                                _ => count,
                            }
                        })
                }),
        ),
        (2021, 8, 2) => Some(
            nonempty_input_lines(input_strings)
                .filter_map(|display| {
                    display
                        .split(" | ")
                        .map(|segments| segments.split(' ').map(|s| s.to_string()).collect_vec())
                        .collect_tuple::<(_, _)>()
                })
                .fold(0, |outer_sum, (input_segs, output_segs)| {
                    let mapping =
                        SegmentMapping::find_valid(&input_segs).expect("No valid mapping");
                    outer_sum
                        + output_segs
                            .iter()
                            .fold((0, 1000), |(sum, pos), seg| {
                                (
                                    sum + SegmentDisplay::from_str_with_mapping(seg, &mapping)
                                        .to_int()
                                        .unwrap()
                                        * pos,
                                    pos / 10,
                                )
                            })
                            .0
                }),
        ),
        (2022, 1, 1) => Some(
            input_strings
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .max,
        ),
        (2022, 1, 2) => Some(
            input_strings
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .top_three_sum(),
        ),
        (2022, 2, 1) => {
            println!("Solution no longer available");
            None
        }
        (2022, 2, 2) => Some(input_strings.map(|s| score_guide_round(&s)).sum()),
        (2023, 1, 1) => Some(
            input_strings
                .map(|s| fix_calibration_line(&s).expect(&s))
                .sum(),
        ),
        (2023, 1, 2) => Some(
            input_strings
                .map(|s| fix_calibration_line_with_string_digits(&s).expect(&s))
                .sum(),
        ),
        (2023, 7, 1) => {
            let mut hands: Vec<_> = input_strings.filter_map(|s| CamelPokerHand::from_str(&s)).collect();
            hands.sort();
            Some(hands.iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum())
        }
        (2023, 7, 2) => None,
        _ => {
            println!("Puzzle solution not yet available");
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CamelPokerHand {
    hand: Hand,
    bid: usize,
}

impl CamelPokerHand {
    fn from_str(s: &str) -> Option<CamelPokerHand> {
        if let Ok(regex) = Regex::new(r"^([0-9AKQJT]+) (\d+)$") {
            if let Some(captures) = regex.captures(s) {
                let try_hand = if let Some(hand_match) = captures.get(1) {
                    Hand::from_str(hand_match.as_str())
                } else {
                    None
                };
                let try_bid = if let Some(bid_match) = captures.get(2) {
                    bid_match.as_str().parse::<usize>().ok()
                } else {
                    None
                };
                if let (Some(hand), Some(bid)) = (try_hand, try_bid) {
                    Some(CamelPokerHand {hand, bid})
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl Hand {
    fn from_str(s: &str) -> Option<Hand> {
        let cards_in_hand: Vec<Card> = s.chars().flat_map(|c| Card::from_char(&c)).collect();

        let (_, mut all_counts) = cards_in_hand.iter().fold(
            (Vec::<Card>::new(), Vec::<u8>::new()),
            |(mut cards, mut counts), &next_card| {
                if let Some(card_position) = cards.iter().position(|&c| c == next_card) {
                    counts[card_position] += 1
                } else {
                    cards.push(next_card.clone());
                    counts.push(1)
                }
                (cards, counts)
            }
        );

        all_counts.sort();

        match all_counts[..] {
            [5] => Some(Hand::FiveOfAKind(cards_in_hand)),
            [1, 4] => Some(Hand::FourOfAKind(cards_in_hand)),
            [2, 3] => Some(Hand::FullHouse(cards_in_hand)),
            [1, 1, 3] => Some(Hand::ThreeOfAKind(cards_in_hand)),
            [1, 2, 2] => Some(Hand::TwoPair(cards_in_hand)),
            [1, 1, 1, 2] => Some(Hand::OnePair(cards_in_hand)),
            [1, 1, 1, 1, 1] => Some(Hand::HighCard(cards_in_hand)),
            _ => None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Card {
    Joker,
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: &char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Number(10)),
            '9' => Some(Card::Number(9)),
            '8' => Some(Card::Number(8)),
            '7' => Some(Card::Number(7)),
            '6' => Some(Card::Number(6)),
            '5' => Some(Card::Number(5)),
            '4' => Some(Card::Number(4)),
            '3' => Some(Card::Number(3)),
            '2' => Some(Card::Number(2)),
            _ => None,
        }
    }
}

fn fix_calibration_line(s: &str) -> Result<usize> {
    let mut iterator = s.chars().filter_map(|c| c.to_digit(10)).peekable();
    let first_digit = iterator.peek().ok_or(anyhow!("No first digit"))?.clone();
    let last_digit = iterator.last().ok_or(anyhow!("No last digit"))?;
    Ok((first_digit * 10 + last_digit).try_into()?)
}

fn fix_calibration_line_with_string_digits(s: &str) -> Result<usize> {
    let first_digit_regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]")?;
    let last_digit_regex =
        Regex::new("(?:.*)(one|two|three|four|five|six|seven|eight|nine|[0-9])")?;
    let first_digit = to_digit_incl_text(
        first_digit_regex
            .find(s)
            .ok_or(anyhow!("No first digit"))?
            .as_str(),
    )?;
    let last_digit = to_digit_incl_text(
        last_digit_regex
            .captures(s)
            .ok_or(anyhow!("No last digit"))?
            .get(1)
            .ok_or(anyhow!("No last digit"))?
            .as_str(),
    )?;
    Ok((first_digit * 10 + last_digit).try_into()?)
}

fn to_digit_incl_text(s: &str) -> Result<usize> {
    match s {
        "0" => Ok(0),
        "one" | "1" => Ok(1),
        "two" | "2" => Ok(2),
        "three" | "3" => Ok(3),
        "four" | "4" => Ok(4),
        "five" | "5" => Ok(5),
        "six" | "6" => Ok(6),
        "seven" | "7" => Ok(7),
        "eight" | "8" => Ok(8),
        "nine" | "9" => Ok(9),
        _ => Err(anyhow!("Invalid digit")),
    }
}

fn parse_input<T: std::str::FromStr, IS>(input_strings: IS) -> impl Iterator<Item = T>
where
    IS: Iterator<Item = String>,
{
    input_strings.filter_map(|s| s.parse::<T>().ok())
}

fn binary_from_input(input_strings: impl Iterator<Item = String>) -> impl Iterator<Item = usize> {
    input_strings.filter_map(|s| usize::from_str_radix(&s, 2).ok())
}

fn nonempty_input_lines(
    input_strings: impl Iterator<Item = String>,
) -> impl Iterator<Item = String> {
    input_strings.filter(|s| !s.is_empty())
}

pub fn file_lines_as_strings(input_path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(input_path).unwrap();
    BufReader::new(file).lines().filter_map(|l| l.ok())
}

fn single_line_from_input(mut input_strings: impl Iterator<Item = String>) -> String {
    input_strings.next().unwrap()
}

pub fn cargo_input_file_path(year: usize, day: usize) -> PathBuf {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("inputs"));
    let filename = format!("input-{:04}-day{:02}", year, day);
    Path::new(&base_dir).join(Path::new(&filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::from_char(&'A'), Some(Card::Ace));
        assert_eq!(Card::from_char(&'9'), Some(Card::Number(9)));
        assert_eq!(Card::from_char(&'x'), None);
        assert!(Card::Ace > Card::Number(7));
        assert!(Card::Number(7) > Card::Number(2));
    }

    fn run_solution_for_test(year: usize, day: usize, puzzle: usize) -> usize {
        let input_lines = file_lines_as_strings(&cargo_input_file_path(year, day));
        run_solution(year, day, puzzle, input_lines).unwrap()
    }

    fn run_solution_for_example(year: usize, day: usize, puzzle: usize) -> usize {
        let input_lines = example_input(year, day, puzzle)
            .lines()
            .map(|s| s.to_string());
        run_solution(year, day, puzzle, input_lines).unwrap()
    }

    fn example_input(year: usize, day: usize, puzzle: usize) -> &'static str {
        match (year, day, puzzle) {
            (2022, 1, _) => {
                "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            }
            (2022, 2, _) => {
                "\
A Y
B X
C Z"
            }
            (2023, 1, 1) => {
                "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            }
            (2023, 1, 2) => {
                "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            }
            (2023, 7, _) => {
                "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            }
            // Add next example above this line.
            _ => "",
        }
    }

    #[test_case(2023, 1, 1, 142)]
    #[test_case(2023, 1, 2, 281)]
    #[test_case(2023, 7, 1, 6440)]
    #[test_case(2023, 7, 2, 0)]
    fn check_examples(year: usize, day: usize, puzzle: usize, result: usize) {
        assert_eq!(run_solution_for_example(year, day, puzzle), result)
    }

    #[test_case(2022, 1, 1, 24000)]
    // Test case fails: #[test_case(2022, 1, 2, 45000)]
    #[test_case(2022, 2, 2, 12)]
    fn check_examples_old(year: usize, day: usize, puzzle: usize, result: usize) {
        assert_eq!(run_solution_for_example(year, day, puzzle), result)
    }

    #[test_case(2021, 1, 1, 1466)]
    #[test_case(2021, 1, 2, 1491)]
    #[test_case(2021, 2, 2, 1947878632)]
    #[test_case(2021, 3, 1, 4006064)]
    #[test_case(2021, 3, 2, 5941884)]
    #[test_case(2021, 4, 1, 2496)]
    #[test_case(2021, 4, 2, 25925)]
    #[test_case(2021, 5, 1, 5084)]
    #[test_case(2021, 5, 2, 17882)]
    #[test_case(2021, 6, 1, 352195)]
    #[test_case(2021, 6, 2, 1600306001288)]
    #[test_case(2021, 7, 2, 92881128)]
    #[test_case(2021, 8, 1, 397)]
    #[test_case(2021, 8, 2, 1027422)]
    #[test_case(2022, 1, 1, 71506)]
    #[test_case(2022, 1, 2, 209603)]
    #[ignore]
    fn check_solutions_old(year: usize, day: usize, puzzle: usize, result: usize) {
        assert_eq!(run_solution_for_test(year, day, puzzle), result)
    }

    #[test_case(2023, 1, 1, 57346)]
    #[test_case(2023, 1, 2, 57345)]
    #[test_case(2023, 7, 1, 0)]
    #[test_case(2023, 7, 2, 0)]
    fn check_solutions(year: usize, day: usize, puzzle: usize, result: usize) {
        assert_eq!(run_solution_for_test(year, day, puzzle), result)
    }
}
