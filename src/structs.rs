use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Serialize, Deserialize)]
pub struct Glicko {
    rati: f64, // rating
    devi: f64, // rating deviation
    vola: f64, // rating volatility
}

#[derive(Serialize, Deserialize)]
pub struct History {
    wins: usize,
    loss: usize,
    draw: usize,
    old_rate: VecDeque<f64>, // tracks the rating and rank some sessions ago
    old_rank: VecDeque<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    name: String,
    rate: Glicko,  // glicko ranking information
    hist: History, // historical stats
}

impl Glicko {
    pub fn new() -> Self {
        Self {
            rati: (1500.0),
            devi: (350.0),
            vola: (0.06),
        }
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
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            name: (name),
            rate: Glicko::new(),
            hist: History::new(),
        }
    }
}

const DATA_PATH: &str = "src/data.json";
const INIT_PATH: &str = "src/init.txt";

pub fn initialize_characters() -> Vec<Character> {
    let mut characters: Vec<Character> = Vec::new();

    let read = read_characters();

    if !read.is_empty() {
        println!("Read {} Success", DATA_PATH);
        characters = read;
    } else {
        print!("Read {} Failed, Read {}", DATA_PATH, INIT_PATH);
        let file = File::open(&INIT_PATH).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    let chara = Character::new(l);
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

    for (i, c) in characters.iter().enumerate() {
        println!("#{}: {}", i + 1, c.name);
    }
    characters
}

pub fn read_characters() -> Vec<Character> {
    let result = fs::read_to_string(DATA_PATH);
    match result {
        Ok(content) => {
            return serde_json::from_str(&content).unwrap();
        }
        Err(_) => return Vec::new(),
    }
}

pub fn store_characters(characters: Vec<Character>) {
    let serialized = serde_json::to_string(&characters).unwrap();
    fs::write(DATA_PATH, serialized).unwrap();
}
