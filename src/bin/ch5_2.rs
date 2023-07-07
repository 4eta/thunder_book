// [世界四連覇AIエンジニアがゼロから教えるゲーム木探索入門]
// chapter5-2を実装
// thunder(@thun_c)さんのコードを参考にしました

#![allow(unused_imports, dead_code, non_snake_case, non_upper_case_globals, unused_doc_comments)]

use rand::{Rng, random};
use std::collections::{BinaryHeap, HashMap};
use std::time;
use std::rc::Rc;

type ScoreType = isize;

const H: usize = 3;
const W: usize = 3;
const END_TURN: usize = 5;

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

    
    fn is_done(&self) -> bool {
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

    fn legal_actions(&self) -> Vec<usize> {
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

    fn is_first(&self) -> bool {
        self.turn % 2 == 0
    }

    fn get_first_player_score_fow_win_rate(&mut self) -> f64 {
        match self.get_winning_status() {
            WinningStates::WIN => {
                if self.is_first() {
                    return 1.0;
                } else {
                    return 0.0;
                };
            },
            WinningStates::LOSE => {
                if self.is_first() {
                    return 0.0;
                } else {
                    return 1.0;
                };
            },
            _ => {
                return 0.5;
            },
        }
    }

    fn get_winning_status(&mut self) -> WinningStates {
        if self.is_done() {
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

    fn get_score(&self) -> ScoreType {
        self.characters[0].game_score - self.characters[1].game_score
    }

    fn to_string(&self) {
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

fn random_action(state: &MazeState) -> usize {
    let actions = state.legal_actions();
    let mut rng = rand::thread_rng();
    let action = rng.gen_range(0, actions.len());
    actions[action]
}


// minimaxのためのスコア計算
fn mini_max_score(state: &MazeState, depth: usize) -> ScoreType {
    if state.is_done() || depth == 0 {
        return state.get_score();
    }
    let legal_actions = state.legal_actions();
    if legal_actions.is_empty() {
        return state.get_score();
    }
    let mut best_score = -1_000_000;
    for action in legal_actions {
        let mut next_state = state.clone();
        next_state.advance(action);
        let score = -mini_max_score(&next_state, depth - 1);
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}

// 深さを指定してminimaxで行動を決定する
fn mini_max_action(state: &MazeState, depth: usize) -> usize {
    let mut best_action = 5;
    let mut best_score = -1_000_000;
    for action in state.legal_actions() {
        let mut next_state = state.clone();
        next_state.advance(action);
        let score = -mini_max_score(&next_state, depth);
        if score > best_score {
            best_action = action;
            best_score = score;
        }
    }
    best_action
}

fn playGame(seed: Option<u64>) -> WinningStates {
    let mut state: MazeState = MazeState::new(seed);
    eprintln!("initial state");
    state.to_string();
    while !state.is_done() {
        eprintln!("1p-----------------------------------");
        //let action = random_action(&state);
        let action = mini_max_action(&state, END_TURN);
        state.advance(action); //ここで1pから2pにターンが移る
        state.to_string();
        if state.is_done() { //2pのターンで終了した場合はここで終了
            match state.get_winning_status() {
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
                },
            }
        }
        eprintln!("2p-----------------------------------");
        let action = random_action(&state);
        state.advance(action); //ここで2pから1pにターンが移る
        state.to_string();
        if state.is_done() { //1pのターンで終了した場合はここで終了
            match state.get_winning_status() {
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
                },
            }
        }
    }
    state.get_winning_status()
}




type AIFunction = dyn Fn(&MazeState) -> usize;
type StringAIPair = (String, Rc<AIFunction>);

// ゲームをgame_number×2(先手後手を交代)回プレイしてaisの0番目のAIの勝率を表示する。
fn test_first_player_win_rate(ais: &[StringAIPair], game_number: usize) {
    let mut first_player_win_rate = 0.0;
    for i in 0..game_number {
        let base_state = MazeState::new(Some(i as u64));
        for j in 0..2 {
            let mut state = base_state.clone();
            let first_ai = &ais[j];
            let second_ai = &ais[(j + 1) % 2];
            loop {
                state.advance(first_ai.1(&state));
                if state.is_done() {
                    break;
                }
                state.advance(second_ai.1(&state));
                if state.is_done() {
                    break;
                }
            }
            let mut win_rate_point = state.get_first_player_score_fow_win_rate();
            if j == 1 {
                win_rate_point = 1.0 - win_rate_point;
            } 
            /*if win_rate_point >= 0.0 {
                state.to_string();
            }*/
            first_player_win_rate += win_rate_point;
        }
        eprintln!("i {} w {}", i, first_player_win_rate / ((i + 1) * 2) as f64);
    }
    first_player_win_rate /= (game_number * 2) as f64;
    println!(
        "Winning rate of {} to {}: {}",
        ais[0].0, ais[1].0, first_player_win_rate
    );
}

fn main() {
    let ais: [StringAIPair; 2] = [
        (
            String::from("min-max"),
            Rc::new(|state| mini_max_action(state, END_TURN)),
        ),
        (
            String::from("random"),
            Rc::new(|state| random_action(state)),
        ),
    ];
    
    test_first_player_win_rate(&ais, 100);
    println!("example");
    playGame(Some(314));
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