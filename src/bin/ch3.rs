// [世界四連覇AIエンジニアがゼロから教えるゲーム木探索入門] chapter3を実装
// thunder(@thun_c)さんのコードを参考にしました

#![allow(unused_imports, dead_code, non_snake_case, non_upper_case_globals)]

use rand::{Rng, random};
use std::{collections::BinaryHeap, time};

type ScoreType = isize;

const H: usize = 30;
const W: usize = 30;
const END_TURN: usize = 100;

const dx: [isize; 4] = [1, 0, -1, 0];
const dy: [isize; 4] = [0, 1, 0, -1];


#[derive(Clone, Debug, Eq)]
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
    character: Coord,
    game_score: ScoreType,
    evaluate_score: ScoreType,
    first_action: usize,
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
        let mut character = Coord::new();
        character.x = rng.gen_range(0, W);
        character.y = rng.gen_range(0, H);
        let mut grid: Vec<Vec<usize>> = vec![vec![0; W]; H];
        for y in 0..H {
            for x in 0..W {
                if y == character.y as usize && x == character.x as usize {
                    continue;
                }                 
                grid[y][x] = rng.gen_range(0, 10);
            }
        }
        MazeState {
            grid,
            turn: 0,
            character,
            game_score: 0,
            evaluate_score: 0,
            first_action: 5,
        }
    }

    fn is_done(&self) -> bool {
        assert!(self.turn <= END_TURN);
        self.turn == END_TURN
    }

    fn advance(&mut self, action: usize) {
        assert!(action < 4);
        self.character.x = (self.character.x as isize + dx[action]) as usize;
        self.character.y = (self.character.y as isize + dy[action]) as usize;
        assert!(self.character.x < W && self.character.y < H);
        let point:ScoreType = self.grid[self.character.y][self.character.x] as ScoreType;
        if point > 0 {
            self.game_score += point;
            self.grid[self.character.y][self.character.x] = 0;
        }
        self.turn += 1;
    }

    fn isIn(x: isize, y: isize) -> bool {
        x >= 0 && x < W as isize && y >= 0 && y < H as isize
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions: Vec<usize> = Vec::new();
        for action in 0..4 {
            let x: isize = self.character.x as isize + dx[action];
            let y: isize = self.character.y as isize + dy[action];
            if Self::isIn(x, y) {
                actions.push(action);
            }
        }
        actions
    }

    fn evaluate_score(&mut self) {
        self.evaluate_score = self.game_score;
    }

    fn to_string(&self) {
        eprintln!("turn:{}, score:{}", self.turn, self.game_score);
        for y in 0..H {
            for x in 0..W {
                if y == self.character.y && x == self.character.x {
                    eprint!("@");
                } else {
                    eprint!("{}", self.grid[y][x]);
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn random_action(state: &MazeState) -> usize {
    let legal_actions: Vec<usize> = state.legal_actions();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, legal_actions.len());
    legal_actions[index]
}

fn greedy_action(state: &MazeState) -> usize {
    let mut max_score = 0;
    let mut best_action = 0;
    for action in state.legal_actions() {
        let mut next_state: MazeState = state.clone();
        next_state.advance(action);
        next_state.evaluate_score();
        if next_state.evaluate_score > max_score {
            max_score = next_state.evaluate_score;
            best_action = action;
        }
    }
    best_action
}

fn beam_search_action(state: &MazeState, beam_width: usize, beam_depth: usize) -> usize {
    let mut now_beam: BinaryHeap<MazeState> = BinaryHeap::new();
    let mut state = state.clone();
    state.evaluate_score();
    now_beam.push(state);
    for t in 0..beam_depth {
        let mut next_beam: BinaryHeap<MazeState> = BinaryHeap::new();
        for _w in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let tmp_state: MazeState = now_beam.pop().unwrap();
            for action in tmp_state.legal_actions() {
                let mut next_state: MazeState = tmp_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = action;
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        let best_state = now_beam.peek().unwrap();
        if best_state.is_done() {
            break;
        }
    }
    now_beam.peek().unwrap().first_action
    
}

fn beam_search_action_with_time_threshold(state: &MazeState, beam_width: usize, time_threshold: f64) -> usize {
    let mut now_beam: BinaryHeap<MazeState> = BinaryHeap::new();
    let mut best_state = &MazeState::new(Some(1));
    let mut state = state.clone();
    state.evaluate_score();
    now_beam.push(state);
    let time_keeper = TimeKeeper::new(time_threshold);

    for t in 0.. {
        let mut next_beam: BinaryHeap<MazeState> = BinaryHeap::new();
        for _w in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let tmp_state: MazeState = now_beam.pop().unwrap();
            for action in tmp_state.legal_actions() {
                let mut next_state = tmp_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = action;
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = now_beam.peek().unwrap();
        if best_state.is_done() || time_keeper.isTimeOver(){
            break;
        }
    }
    best_state.first_action
}

fn chokudai_search_action(state: &MazeState, beam_width: usize, beam_depth: usize, beam_number:usize) -> usize {
    let mut beam: Vec<BinaryHeap<MazeState>> = vec![BinaryHeap::new(); beam_depth+1];
    beam[0].push(state.clone());
    for _cnt in 0..beam_number {
        for t in 0..beam_depth {
            for _w in 0..beam_width {
                if beam[t].is_empty() {
                    break;
                }
                let now_state = beam[t].peek().unwrap().clone();
                if now_state.is_done() {
                    break;
                }
                beam[t].pop();

                for action in now_state.legal_actions() {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if t == 0 {
                        next_state.first_action = action;
                    }
                    beam[t+1].push(next_state);
                }
            }
        } 
    }
    for t in (0..=beam_depth).rev() {
        if !beam[t].is_empty() {
            return beam[t].peek().unwrap().first_action;
        }
    }
    0
}

fn chokudai_search_action_with_time_threshold(state: &MazeState, beam_width: usize, beam_depth: usize, time_threshold: f64) -> usize {
    let mut beam: Vec<BinaryHeap<MazeState>> = vec![BinaryHeap::new(); beam_depth+1];
    beam[0].push(state.clone());
    let time_keeper = TimeKeeper::new(time_threshold);
    loop {
        for t in 0..beam_depth {
            for _w in 0..beam_width {
                if beam[t].is_empty() {
                    break;
                }
                let now_state: MazeState = beam[t].peek().unwrap().clone();
                if now_state.is_done() {
                    break;
                }
                beam[t].pop();

                for action in now_state.legal_actions() {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if t == 0 {
                        next_state.first_action = action;
                    }
                    beam[t+1].push(next_state);
                }
            }
        } 
        if time_keeper.isTimeOver() {
            break;
        }
    }
    for t in (0..=beam_depth).rev() {
        if !beam[t].is_empty() {
            return beam[t].peek().unwrap().first_action;
        }
    }
    0
}

fn play_game(seed: Option<u64>) -> ScoreType {
    let mut state: MazeState = MazeState::new(seed);
    //state.to_string();
    while !state.is_done() {
        //let action: usize = random_action(&state);
        //let action: usize = greedy_action(&state);
        //let action: usize = beam_search_action(&state, 10, 10);
        //let action: usize = beam_search_action_with_time_threshold(&state, 5, 0.001);
        //let action: usize = chokudai_search_action(&state, 1, 10, 10);
        let action: usize = chokudai_search_action_with_time_threshold(&state, 1, 10, 0.01);
        state.advance(action);
        //state.to_string();
    }
    state.game_score
}

fn test_AI_score(game_number:usize, seed: Option<u64>) -> f64 {
    let mut total_score = 0;
    for cnt in 0..game_number {
        eprintln!("game: {} start", cnt);
        let seed = match seed {
            Some(seed) => Some(seed + cnt as u64),
            None => None,
        };
        let score = play_game(seed);
        total_score += score;
        eprintln!("game: {} end, score:{}", cnt, score);
        eprintln!();
        
    }
    total_score as f64 / game_number as f64
}

fn main() {
    //let score = _gSome(314));
    //println!("final score: {}", score);
    let score = test_AI_score(10, Some(14));
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