// [世界四連覇AIエンジニアがゼロから教えるゲーム木探索入門] chapter4を実装
// thunder(@thun_c)さんのコードを参考にしました

#![allow(unused_imports, dead_code, non_snake_case, non_upper_case_globals, unused_doc_comments)]

use rand::{Rng, random};
use std::{collections::BinaryHeap, time};

type ScoreType = isize;

const H: usize = 20;
const W: usize = 20;
const END_TURN: usize = 50;
const N_CHARACTER: usize = 3;

const dx: [isize; 4] = [1, 0, -1, 0];
const dy: [isize; 4] = [0, 1, 0, -1];


#[derive(Clone, Copy, Debug, Eq)]
struct Coord {
    y: usize,
    x: usize,
}
impl std::cmp::PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}
impl Coord {
    fn new() -> Self {
        Coord { y: 0, x: 0 }
    }
}


#[derive(Clone,Debug,Eq)]
struct MazeState {
    grid: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Coord>,
    game_score: ScoreType,
    evaluate_score: ScoreType,
}
impl std::cmp::PartialEq for MazeState {
    fn eq(&self, other: &Self) -> bool {
        self.evaluate_score == other.evaluate_score
    }
}
impl std::cmp::PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.evaluate_score.partial_cmp(&other.evaluate_score)
    }
}
impl std::cmp::Ord for MazeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluate_score.cmp(&other.evaluate_score)
    }
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
        let characters: Vec<Coord> = [Coord::new(); N_CHARACTER].to_vec();
        MazeState {
            grid,
            turn: 0,
            characters,
            game_score: 0,
            evaluate_score: 0,
        }
    }

    fn set_character(&mut self, id: usize, y:usize, x:usize) {
        assert!(id < N_CHARACTER);
        self.characters[id] = Coord { y, x };
    }

    fn init(&mut self) {
        ///キャラクターの配置をランダムに初期化する
        let mut rng = rand::thread_rng();
        for id in 0..N_CHARACTER {
            let y = rng.gen_range(0, H);
            let x = rng.gen_range(0, W);
            self.set_character(id, y, x);
        }
    }

    fn transition(&mut self) {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0, N_CHARACTER);
        let y = rng.gen_range(0, H);
        let x = rng.gen_range(0, W);
        self.set_character(id, y, x);
    }

    fn get_score(&self,is_print: bool) -> ScoreType {
        let mut tmp_state = self.clone();
        for character in self.characters.iter () {
            tmp_state.grid[character.y][character.x] = 0;
        }
        if is_print {
            tmp_state.to_string();
        }
        while !tmp_state.is_done() {
            tmp_state.advance();
            if is_print {
                tmp_state.to_string();
            }
        }
        tmp_state.game_score
    }

    fn is_done(&self) -> bool {
        assert!(self.turn <= END_TURN);
        self.turn == END_TURN
    }

    fn move_player(&mut self, id:usize) {
        assert!(id < N_CHARACTER);
        let mut best_point:ScoreType = -1;
        let mut best_action = 0;
        for action in 0..4 {
            let ny = self.characters[id].y as isize + dy[action];
            let nx = self.characters[id].x as isize + dx[action];
            if !Self::isIn(nx, ny) {
                continue;
            }
            let point = self.grid[ny as usize][nx as usize] as ScoreType;
            if point > best_point {
                best_point = point;
                best_action = action;
            }
        }
        self.characters[id].y = (self.characters[id].y as isize + dy[best_action]) as usize;
        self.characters[id].x = (self.characters[id].x as isize + dx[best_action]) as usize;
    }

    fn advance(&mut self) {
        for id in 0..N_CHARACTER {
            self.move_player(id);
        }
        for character in self.characters.iter() {
            self.game_score += self.grid[character.y][character.x] as ScoreType;
            self.grid[character.y][character.x] = 0;
        } 
        self.turn += 1;
    }

    fn isIn(x: isize, y: isize) -> bool {
        x >= 0 && x < W as isize && y >= 0 && y < H as isize
    }

    fn evaluate_score(&mut self) {
        self.evaluate_score = self.game_score;
    }

    fn to_string(&self) {
        eprintln!("turn:{}, score:{}", self.turn, self.game_score);
        let mut str: Vec<Vec<char>> = vec![vec!['.'; W]; H];
        for y in 0..H {
            for x in 0..W {
                str[y][x] = std::char::from_digit(self.grid[y][x] as u32, 10).unwrap();
            }
        }
        for character in self.characters.iter() {
            str[character.y][character.x] = '@';
        }
        for y in 0..H {
            eprintln!("{}", str[y].iter().collect::<String>());
        }
        eprintln!();
    }
}

fn random_action(state: &MazeState) -> MazeState {
    let mut now_state = state.clone();
    let mut rng = rand::thread_rng();
    for id in 0..N_CHARACTER {
        let y = rng.gen_range(0, H);
        let x = rng.gen_range(0, W);
        now_state.set_character(id, y, x);
    }
    now_state
}

fn hill_climb(state: &MazeState, number: usize) -> MazeState {
    let mut now_state = state.clone();
    now_state.init();
    let mut best_score = now_state.get_score(false);
    for _ in 0..number {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        if next_score > best_score {
            now_state = next_state;
            best_score = next_score;
        }
    }
    now_state
}

fn simulated_annealing(state: &MazeState, number:usize, start_temp: f64, end_temp: f64) -> MazeState {
    let mut now_state = state.clone();
    now_state.init();
    let mut now_score = now_state.get_score(false);
    let mut best_score = now_score;
    let mut best_state = now_state.clone();

    let mut rng = rand::thread_rng();

    for loop_cnt in 0..number {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        let diff = next_score - best_score;
        if diff > 0 {
            now_score = next_score;
            now_state = next_state;
        } else {
            let temp = start_temp + (end_temp - start_temp) * loop_cnt as f64 / number as f64;
            //スコアを大きくしたい場合はこう
            let prob = (diff as f64 / temp).exp();
            //スコアを小さくしたい場合はこう
            //let prob = (-diff as f64 / temp).exp();
            if rng.gen::<f64>() < prob {
                now_score = next_score;
                now_state = next_state;
            }
        }
        if now_score > best_score {
            best_score = now_score;
            best_state = now_state.clone();
        }
    }
    best_state
}


fn playGame(seed: Option<u64>) -> ScoreType {
    let mut state: MazeState = MazeState::new(seed);
    //state = random_action(&state);
    //state = hill_climb(&state, 100000);
    state = simulated_annealing(&state, 100000, 100.0, 0.0);
    let score = state.get_score(false);
    score
}

fn test_AI_score(game_number:usize, seed: Option<u64>) -> f64 {
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

fn main() {
    let score = test_AI_score(10, Some(314));
    println!("average score: {}", score);
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