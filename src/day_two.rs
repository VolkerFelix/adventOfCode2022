use std::fs;
use std::collections::HashMap;

#[derive(Default, Hash, Eq, PartialEq, Copy, Clone)]
enum ERPS {
    #[default]
    Rock,
    Paper,
    Scissors
}

#[derive(Hash, Eq, PartialEq, Default)]
enum EResults {
    #[default]
    Win,
    Loss,
    Draw
}

#[derive(Default, Copy, Clone)]
struct Oponent {
    m_hand: ERPS
} 

#[derive(Default, Copy, Clone)]
struct Me {
    m_hand: ERPS,
}

#[derive(Default)]
struct DesiredOutcome {
    m_des_outcome: EResults,
}

struct Round {
    m_id: u32,
    m_oponent: Oponent,
    m_me: Me,
    m_score: u32
}

pub fn dayTwo() {

    let round_result: HashMap<(ERPS, EResults), ERPS> = HashMap::from([
        ((ERPS::Rock, EResults::Draw), ERPS::Rock),
        ((ERPS::Paper, EResults::Draw), ERPS::Paper),
        ((ERPS::Scissors, EResults::Draw), ERPS::Scissors),
        ((ERPS::Rock, EResults::Loss), ERPS::Scissors),
        ((ERPS::Paper, EResults::Loss), ERPS::Rock),
        ((ERPS::Scissors, EResults::Loss), ERPS::Paper),
        ((ERPS::Paper, EResults::Scissors), ERPS::Win),
        ((ERPS::Scissors, EResults::Rock), ERPS::Win),
        ((ERPS::Scissors, EResults::Paper), ERPS::Loss),
    ]);

    let round_score: HashMap<EResults, u32> = HashMap::from([
        (EResults::Draw, 3),
        (EResults::Loss, 0),
        (EResults::Win, 6)
    ]);

    let hand_score: HashMap<ERPS, u32> = HashMap::from([
        (ERPS::Rock, 1),
        (ERPS::Paper, 2),
        (ERPS::Scissors, 3)
    ]);

    let input = fs::read_to_string("rockPaperScissorsInput.txt").unwrap();

    let mut matches: Vec<Round> = Vec::new();
    let mut new_oponent = Oponent::default();
    let mut new_me = Me::default();
    let mut round_score_value:u32 = 0;
    let mut round_id = 0;
    let mut total_score = 0;
    let mut found_white_spaces = 0;

    let mut des_outcome = DesiredOutcome::default();

    for element in input.chars() {
        // Warning: This is OS dependent
        if element.is_whitespace() {
            found_white_spaces = found_white_spaces + 1;
            match found_white_spaces {
                // It's me --> skip
                1 => {
                    continue;
                },
                // New round
                2 => {
                    // Evaluate my hand
                    round_score_value = *hand_score.get(&new_me.m_hand).unwrap();
                    // Evaluate round
                    let new_round_result = round_result.get(&(new_oponent.m_hand, new_me.m_hand)).unwrap();
                    round_score_value = round_score_value + *round_score.get(new_round_result).unwrap();

                    let new_round = Round {
                        m_id: round_id,
                        m_oponent: new_oponent,
                        m_me: new_me,
                        m_score: round_score_value
                    };

                    total_score = total_score + round_score_value;

                    matches.push(new_round);
                    round_id = round_id + 1;
                    round_score_value = 0;
                },
                _ => {
                    panic!("Should not happen!")
                }
            }
        }

        else {
            found_white_spaces = 0;

            match element {
                'A' => {
                    new_oponent.m_hand = ERPS::Rock;
                },
                'B' => {
                    new_oponent.m_hand = ERPS::Paper;
                },
                'C' => {
                    new_oponent.m_hand = ERPS::Scissors;
                },
                'X' => {
                    des_outcome.m_des_outcome = EResults::Loss;
                },
                'Y' => {
                    des_outcome.m_des_outcome = EResults::Draw;
                },
                'Z' => {
                    des_outcome.m_des_outcome = EResults::Win;
                },
                _ => {
                    panic!("Should not happen!");
                }
            }
        }
    }

    print!("Total score: {}", total_score);
}