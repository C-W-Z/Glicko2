use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
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
    // Try read data from file
    let init = read_init_characters();
    let mut read = read_characters();

    if read.is_empty() {
        return init;
    }

    // println!("Read {} Success", DATA_PATH);
    let read_len = read.len();

    let mut init_name_id: HashMap<String, usize> = HashMap::new();
    let mut read_names: HashSet<String> = HashSet::new();
    for c in read.iter() {
        read_names.insert(c.name.clone());
    }
    for c in init.iter() {
        init_name_id.insert(c.name.clone(), c.id);
        if !read_names.contains(&c.name) {
            read.push(c.clone());
        }
    }

    if read.len() != read_len {
        println!("{:-<1$}", "", 36);
        println!("Find new characters in init.txt");
        for c in read[read_len..].iter() {
            println!("#{}: {}", c.id, c.name);
        }
    }

    let mut next_id = init.len();
    for c in read.iter_mut() {
        match init_name_id.get(&c.name) {
            Some(id) => {
                c.id = *id;
            }
            None => {
                c.id = next_id;
                next_id += 1;
            }
        }
    }

    read.sort_by_key(|c| c.id);
    if read.len() != init.len() {
        println!("{:-<1$}", "", 36);
        println!("Find some characters not in init.txt");
        for c in read[init.len()..].iter() {
            println!("#{}: {}", c.id, c.name);
        }
        print!("Do you want to REMOVE them ? (Y/n) ");
        let mut choice: String = String::new();
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut choice);
        if choice.to_uppercase().starts_with("Y") {
            let _ = read.split_off(init.len());
            println!("Remove Success!");
        } else {
            println!("Keep them exists.");
        }
    }

    // for c in read.iter() {
    //     println!("#{}: {}", c.id, c.name);
    // }

    read
}

pub fn read_init_characters() -> Vec<Character> {
    let mut characters: Vec<Character> = Vec::new();

    // Initialize data from file
    let file = match File::open(&INIT_PATH) {
        Ok(f) => f,
        Err(error) => {
            eprintln!("\nError: {}", error);
            exit(1);
        }
    };
    let reader = BufReader::new(file);

    for (id, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                let _l = l.trim().to_owned();
                if _l.is_empty() {
                    continue;
                }
                let chara = Character::new(id, _l);
                characters.push(chara);
            }
            Err(error) => {
                eprintln!("\nError: {}", error);
                exit(1);
            }
        }
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
