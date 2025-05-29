#![allow(warnings)]

use colored::*;
use rand::Rng;
use std::io;
use std::process::exit;
use std::str::FromStr;

fn Game() -> i32 {
    let numAscii="
  ______                 __               
 |   _  \\.--.--.--------|  |--.-----.----.
 |.  |   |  |  |        |  _  |  -__|   _|
 |.  |   |_____|__|__|__|_____|_____|__|  
 |:  |   |                                
 |::.|   |                                
 `--- ---'";
    let gueAscii:&str="
 _______                         __             
 |   _   .--.--.-----.-----.-----|__.-----.-----.
 |.  |___|  |  |  -__|__ --|__ --|  |     |  _  |
 |.  |   |_____|_____|_____|_____|__|__|__|___  |
 |:  1   |                                |_____|
 |::.. . |                                       
 `-------'\t";
    let gameAscii:&str="
  _______                      
 |   _   .---.-.--------.-----.
 |.  |___|  _  |        |  -__|
 |.  |   |___._|__|__|__|_____|
 |:  1   |                     
 |::.. . |                     
 `-------'";
    println!(
        "{}",
        format!(
            "{} {} {}",
            numAscii.red(),
            gueAscii.yellow(),
            gameAscii.green()
        )
        .on_black()
        .bold()
    );
    println!(
        "{}",
        format!("Please input your guess from {}" ,"1 to 100".italic().bold()).bright_purple().on_truecolor(255,255,255)
       
    );
    let num: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    let mut counter: u8 = 15;
    while counter > 0 {
        guess = input();
        let mut guess_int: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Input a valid number from 1 to 100".bold().red());
                continue;
            }
        };
        if guess_int == num {
            println!(
                "{}",
                format!("You guessed correctly: {}!", guess_int)
                    .bold()
                    .green()
            );
            break; // Exit the loop
        } else if guess_int > num {
            if guess_int - num >= 30 {
                println!(
                    "{} {}",
                    format!("You guessed: {}, ", guess_int).red(),
                    "try way lower".red().italic()
                );
            } else {
                println!(
                    "{} {}",
                    format!("You guessed: {}, ", guess_int).red(),
                    "try a little lower".red().italic()
                );
            }
        } else if guess_int < num {
            if num - guess_int >= 30 {
                println!(
                    "{} {}",
                    format!("You guessed: {}, ", guess_int).red(),
                    "try way higher".red().italic()
                );
            } else {
                println!(
                    "{} {}",
                    format!("You guessed: {}, ", guess_int).red(),
                    "try a little higher".red().italic()
                );
            }
        }

        counter -= 1;
    }
    if counter == 0 {
        println!("{}", "Too many Tries. You Lose".bold().red());
    }
    0
}

fn input() -> String {
    let mut output = String::new();
    io::stdin()
        .read_line(&mut output)
        .expect("Failed to read line");
    output
}

fn main() {
    let exit_code = Game();
    exit(exit_code);
}
