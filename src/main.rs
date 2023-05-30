use std::{io::{self, Read, Write}, cmp::Ordering, fs::File, path::Path};
use rand::Rng;
use serde_derive::{Serialize, Deserialize};

const SAVE_FILE_PATH: &str = "save_data.bin";

#[derive(Serialize, Deserialize)]
struct GameState {
    game_count: u32,
    high_score: u32
}

fn main() {
    println!("\n=====[GUESS THE NUMBER BETWEEN 1 AND 100]=====");

    let save_state: GameState = init_game_state().expect("An error occured while loading your progress.");

    println!("Session number #{}", save_state.game_count + 1);
    
    if save_state.high_score != 0 {
        println!("Current High Score: {}", save_state.high_score);
    }

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    let mut count_guesses: u32 = 0;
    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        count_guesses += 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win with {count_guesses} guesses!");
                break;
            }
        }
    }

    let final_score: u32 = process_high_score(save_state.high_score, count_guesses);

    let new_save_state: GameState = GameState {
        high_score: final_score,
        game_count: save_state.game_count + 1
    };

    save_game_state(&new_save_state).expect("An error occured while saving your progress.");

    println!("\nPress enter to exit...");
    let mut buffer: [u8; 1] = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer);
}

fn process_high_score(old_score: u32, new_score: u32) -> u32 {
    if old_score == 0 {
        println!("\nGZ for your first run. I will save your High Score of {new_score} for you to beat!");
        return new_score;
    } else {
        match new_score.cmp(&old_score) {
            Ordering::Less => {
                println!("\nGG, you beat your old highscore of {old_score} and your new High Score is: {new_score}!");
                return new_score;
            },
            Ordering::Equal => {
                println!("\nLmao, you achieved the same score as your High Score ({old_score}) but didn't beat it, what a shame...");
                return old_score
            },
            Ordering::Greater => {
                println!("\nBruh, you could'nt even beat your old High Score of {old_score}... I'm disappointed to say the least...");
                return old_score
            }
        }
    }
}

fn init_game_state() -> Result<GameState, Box<dyn std::error::Error>> {
    if Path::new(SAVE_FILE_PATH).exists() {
        load_game_state()
    } else {
        Ok(GameState {
            game_count: 0,
            high_score: 0
        })
    }
}

fn save_game_state(state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = bincode::serialize(state).expect("An error occured while serializing your save game.");
    let mut file = File::create(SAVE_FILE_PATH).expect("An error occured while creating the save game file.");
    file.write_all(&serialized).expect("An error occured while writing the save game file.");
    Ok(())
}

fn load_game_state() -> Result<GameState, Box<dyn std::error::Error>> {
    let mut file = File::open(SAVE_FILE_PATH).expect("An error occured while opening the save game file.");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("An error occured while reading the save game file.");
    let deserialized: GameState = bincode::deserialize(&contents).expect("An error occured while deserializing your save game.");
    Ok(deserialized)
}