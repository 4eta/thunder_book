#![allow(unused_imports, dead_code, non_snake_case, non_upper_case_globals, unused_doc_comments)]

use rand::{Rng, random};
use std::{collections::BinaryHeap, time};

type ScoreType = isize;

const H: usize = 3;
const W: usize = 3;
const END_TURN: usize = 4;

const dx: [isize; 4] = [1, 0, -1, 0];
const dy: [isize; 4] = [0, 1, 0, -1];


#[derive(Clone, Copy, Debug, Eq)]
struct Character {
    y: usize,
    x: usize,
    game_score: ScoreType,
}
impl std::cmp::PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.game_score == other.game_score
    }
}
impl std::cmp::PartialOrd for Character {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.game_score.partial_cmp(&other.game_score)
    }
}
impl std::cmp::Ord for Character {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.game_score.cmp(&other.game_score)
    }
}
impl Character {
    fn new() -> Self {
        Character { y: 0, x: 0, game_score: 0 }
    }
}

enum WinningStates {
    WIN,
    LOSE,
    DRAW,
    CONTINUE,
}


#[derive(Clone,Debug)]
struct MazeState {
    grid: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
}
impl MazeState {

    fn new(seed: Option<u64>) -> Self {
        let mut rng: rand::rngs::StdRng = match seed {
            Some(seed) => rand::SeedableRng::seed_from_u64(seed),
            None => rand::SeedableRng::from_entropy(),
        };
        let mut grid: Vec<Vec<usize>> = vec![vec![0; W]; H];
        for y in 0..H {
            for x in 0..W {                
                grid[y][x] = rng.gen_range(0, 10);
            }
        }
        let characters: Vec<Character> = vec![Character {y: H/2, x: 0, game_score: 0}, Character {y: H/2, x: W-1, game_score: 0}];
        for character in characters.iter() {
            grid[character.y][character.x] = 0;
        }
        MazeState {
            grid,
            turn: 0,
            characters,
        }
    }

    
    fn isDone(&self) -> bool {
        assert!(self.turn <= END_TURN);
        self.turn == END_TURN
    }

    fn advance(&mut self, action:usize) {
        assert!(action < 4);
        let character: &mut Character = &mut self.characters[0];
        let next_y = (character.y as isize + dy[action]) as usize;
        let next_x = (character.x as isize + dx[action]) as usize;
        character.y = next_y;
        character.x = next_x;
        let point = self.grid[next_y][next_x];
        if point > 0 {
            character.game_score += point as ScoreType;
            self.grid[next_y][next_x] = 0;
        }
        self.turn += 1;
        let tmp = self.characters[0].clone();
        self.characters[0] = self.characters[1].clone();
        self.characters[1] = tmp;
    }

    fn isIn(x: isize, y: isize) -> bool {
        x >= 0 && x < W as isize && y >= 0 && y < H as isize
    }

    fn legalActions(&self) -> Vec<usize> {
        let mut actions: Vec<usize> = Vec::new();
        let character: &Character = &self.characters[0];
        for action in 0..4 {
            let x: isize = character.x as isize + dx[action];
            let y: isize = character.y as isize + dy[action];
            if Self::isIn(x, y) {
                actions.push(action);
            }
        }
        actions
    }

    fn getWinningStatus(&mut self) -> WinningStates {
        if self.isDone() {
            if self.characters[0].game_score > self.characters[1].game_score {
                return WinningStates::WIN;
            } else if self.characters[0].game_score < self.characters[1].game_score {
                return WinningStates::LOSE;
            } else {
                return WinningStates::DRAW;
            } 
        }
        WinningStates::CONTINUE
    }

    fn toString(&self) {
        let score_a = if self.turn%2 == 0 {self.characters[0].game_score} else {self.characters[1].game_score};
        let score_b = if self.turn%2 == 0 {self.characters[1].game_score} else {self.characters[0].game_score};
        eprintln!("turn:{}, A:{}, B:{}", self.turn, score_a, score_b);
        let mut str: Vec<Vec<char>> = vec![vec!['.'; W]; H];
        for y in 0..H {
            for x in 0..W {
                if self.grid[y][x] == 0 {
                    continue;
                }
                str[y][x] = std::char::from_digit(self.grid[y][x] as u32, 10).unwrap();
            }
        }
        if self.turn % 2 == 0 {
            str[self.characters[0].y][self.characters[0].x] = 'A';
            str[self.characters[1].y][self.characters[1].x] = 'B';
        } else {
            str[self.characters[0].y][self.characters[0].x] = 'B';
            str[self.characters[1].y][self.characters[1].x] = 'A';
        }

        for y in 0..H {
            eprintln!("{} ", str[y].iter().collect::<String>());
        }
        eprintln!();
    }
}

fn randomAction(state: &MazeState) -> usize {
    let actions = state.legalActions();
    let mut rng = rand::thread_rng();
    let action = rng.gen_range(0, actions.len());
    actions[action]
}


fn playGame(seed: Option<u64>) -> WinningStates {
    let mut state: MazeState = MazeState::new(seed);
    eprintln!("initial state");
    state.toString();
    while !state.isDone() {
        eprintln!("1p-----------------------------------");
        let action = randomAction(&state);
        state.advance(action); //ここで1pから2pにターンが移る
        state.toString();
        if state.isDone() { //2pのターンで終了した場合はここで終了
            match state.getWinningStatus() {
                WinningStates::WIN => {
                    eprintln!("2p win");
                    break;
                },
                WinningStates::LOSE => {
                    eprintln!("1p win");
                    break;
                },
                WinningStates::DRAW => {
                    eprintln!("draw");
                    break;
                },
                _ => {
                    unreachable!();
                }
            }
        }
        eprintln!("2p-----------------------------------");
        let action = randomAction(&state);
        state.advance(action); //ここで2pから1pにターンが移る
        state.toString();
        if state.isDone() { //1pのターンで終了した場合はここで終了
            match state.getWinningStatus() {
                WinningStates::WIN => {
                    eprintln!("1p win");
                    break;
                },
                WinningStates::LOSE => {
                    eprintln!("2p win");
                    break;
                },
                WinningStates::DRAW => {
                    eprintln!("draw");
                    break;
                },
                _ => {
                    unreachable!();
                }
            }
        }
    }
    state.getWinningStatus()
}

/*fn testAiScore(game_number:usize, seed: Option<u64>) -> f64 {
    let mut total_score = 0;
    for cnt in 0..game_number {
        eprintln!("game: {} start", cnt);
        let seed = match seed {
            Some(seed) => Some(seed + cnt as u64),
            None => None,
        };
        let score = playGame(seed);
        total_score += score;
        eprintln!("game: {} end, score:{}", cnt, score);
        eprintln!();
        
    }
    total_score as f64 / game_number as f64
}
*/

fn main() {
    let _s = playGame(Some(314));
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.85 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }

    fn getElapsedTime(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        elapsed_time
    }

}