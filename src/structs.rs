use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader},
    process::exit,
};

pub enum BattleStat {
    Next,
    End,
    Undo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MatchResult {
    AWin,
    BWin,
    Draw,
    BothLose,
}

// A matchup between two characters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Match {
    pub a: usize,         // the id of first character
    pub b: usize,         // the id of first character
    pub res: MatchResult, // result of the match
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rank {
    pub rati: f64, // rating
    pub devi: f64, // rating deviation
    pub vola: f64, // rating volatility
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub wins: usize,
    pub loss: usize,
    pub draw: usize,
    pub old_rate: VecDeque<f64>, // tracks the rating and rank some sessions ago
    pub old_rank: VecDeque<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: usize,
    pub name: String,
    pub rank: Rank,    // glicko ranking information
    pub hist: History, // historical stats
}

impl Rank {
    pub fn new() -> Self {
        Self {
            rati: (1500.0),
            devi: (350.0),
            vola: (0.06),
        }
    }
    pub fn glicko_1_to_2_scale(&mut self) {
        self.rati = (self.rati - 1500.0) / 173.7178;
        self.devi = self.devi / 173.7178;
    }
    pub fn glicko_2_to_1_scale(&mut self) {
        self.rati = self.rati * 173.7178 + 1500.0;
        self.devi = self.devi * 173.7178;
    }
}

impl History {
    pub fn new() -> Self {
        Self {
            wins: (0),
            loss: (0),
            draw: (0),
            old_rate: VecDeque::new(),
            old_rank: VecDeque::new(),
        }
    }
    pub fn battles(&self) -> usize {
        self.wins + self.loss + self.draw
    }
}

impl Character {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id: (id),
            name: (name),
            rank: Rank::new(),
            hist: History::new(),
        }
    }
}

const DATA_PATH: &str = "src/data.json";
const INIT_PATH: &str = "src/init.txt";

pub fn initialize_characters() -> Vec<Character> {
    let mut characters: Vec<Character> = Vec::new();

    // Try read data from file
    let read = read_characters();

    if !read.is_empty() {
        println!("Read {} Success", DATA_PATH);
        characters = read;
    } else {
        print!("Read {} Failed, Read {}", DATA_PATH, INIT_PATH);

        // Initialize data from file
        let file = File::open(&INIT_PATH).unwrap();
        let reader = BufReader::new(file);

        for (id, line) in reader.lines().enumerate() {
            match line {
                Ok(l) => {
                    let chara = Character::new(id, l);
                    characters.push(chara);
                }
                Err(error) => {
                    println!(" Failed");
                    eprintln!("\nError: {}", error);
                    exit(1);
                }
            }
        }
        println!(" Success");
    }

    for c in characters.iter() {
        println!("#{}: {}", c.id, c.name);
    }
    characters
}

pub fn read_characters() -> Vec<Character> {
    // Read data from file
    let result = fs::read_to_string(DATA_PATH);
    match result {
        Ok(content) => {
            // Deserialize from json string
            let objs = serde_json::from_str(&content);
            match objs {
                Ok(ret) => return ret,
                Err(_) => return Vec::new(),
            }
        }
        Err(_) => return Vec::new(),
    }
}

pub fn store_characters(characters: &[Character]) {
    // Serialize to json string
    let serialized = serde_json::to_string(&characters).unwrap();
    // Write string to file
    fs::write(DATA_PATH, serialized).unwrap();
}
