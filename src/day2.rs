use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissor,
}

impl HandShape {
    fn from_opponent_play(c: char) -> Result<HandShape, String> {
        match c {
            'A' => Ok(HandShape::Rock),
            'B' => Ok(HandShape::Paper),
            'C' => Ok(HandShape::Scissor),
            _ => Err(format!("unrecognized opponent HandShape character '{c}'")),
        }
    }

    fn from_our_play(c: char) -> Result<HandShape, String> {
        match c {
            'X' => Ok(HandShape::Rock),
            'Y' => Ok(HandShape::Paper),
            'Z' => Ok(HandShape::Scissor),
            _ => Err(format!("unrecognized opponent HandShape character '{c}'")),
        }
    }

    fn point_value(self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissor => 3,
        }
    }

    fn determine_play(opponent: HandShape, result: GameResult) -> HandShape {
        use GameResult::*;
        use HandShape::*;

        match (opponent, result) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Loss) => Scissor,
            (Scissor, Win) => Rock,
            (Scissor, Draw) => Scissor,
            (Scissor, Loss) => Paper,
            (Paper, Win) => Scissor,
            (Paper, Draw) => Paper,
            (Paper, Loss) => Rock,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn determine(ours: HandShape, opponents: HandShape) -> GameResult {
        use HandShape::*;
        match (ours, opponents) {
            (Rock, Scissor) | (Scissor, Paper) | (Paper, Rock) => GameResult::Win,
            (Scissor, Rock) | (Paper, Scissor) | (Rock, Paper) => GameResult::Loss,
            (Rock, Rock) | (Scissor, Scissor) | (Paper, Paper) => GameResult::Draw,
        }
    }

    fn point_value(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        }
    }

    fn from_expected_result(c: char) -> Result<GameResult, String> {
        match c {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err(format!("unrecognized game result character '{c}'")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Strategy {
    opponent: HandShape,
    our: HandShape,
}

impl Strategy {
    fn score(&self) -> u32 {
        let gr = GameResult::determine(self.our, self.opponent);
        gr.point_value() + self.our.point_value()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct StrategyGameResult {
    opponent: HandShape,
    game_result: GameResult,
}

impl StrategyGameResult {
    fn score(&self) -> u32 {
        let our = HandShape::determine_play(self.opponent, self.game_result);
        self.game_result.point_value() + our.point_value()
    }
}

#[aoc_generator(day2, part1)]
pub fn parse_strategies(input: &str) -> Vec<Strategy> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let opponent = iter.next().expect("no hand shape for opponent");
            let opponent = HandShape::from_opponent_play(opponent).unwrap();

            let space = iter.next();
            if space != Some(' ') {
                panic!("expected space, not '{:?}'", space);
            }

            let our = iter.next().expect("no hand shape for us");
            let our = HandShape::from_our_play(our).unwrap();

            if let Some(c) = iter.next() {
                panic!("expected newline, not '{}'", c);
            }

            Strategy { opponent, our }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn total_score_strategies(input: &[Strategy]) -> u32 {
    input.iter().map(Strategy::score).sum()
}

#[aoc_generator(day2, part2)]
pub fn parse_strategy_game_result(input: &str) -> Vec<StrategyGameResult> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let opponent = iter.next().expect("no hand shape for opponent");
            let opponent = HandShape::from_opponent_play(opponent).unwrap();

            let space = iter.next();
            if space != Some(' ') {
                panic!("expected space, not '{:?}'", space);
            }

            let game_result = iter.next().expect("no game result");
            let game_result = GameResult::from_expected_result(game_result).unwrap();

            if let Some(c) = iter.next() {
                panic!("expected newline, not '{}'", c);
            }

            StrategyGameResult {
                opponent,
                game_result,
            }
        })
        .collect()
}

#[aoc(day2, part2)]
pub fn total_score_strategy_game_results(input: &[StrategyGameResult]) -> u32 {
    input.iter().map(StrategyGameResult::score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "A Y\nB X\nC Z";
        let strategies = parse_strategies(input);
        let score = total_score_strategies(&strategies);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_part_two() {
        let input = "A Y\nB X\nC Z";
        let strategies = parse_strategy_game_result(input);
        let score = total_score_strategy_game_results(&strategies);
        assert_eq!(score, 12);
    }
}
