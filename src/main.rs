#![allow(dead_code)]

use std::process;

use clap::Parser;
use day_1::solution::Day1;
use day_2::solution::Day2;
use solution::{Act, Solution};
mod day_1;
mod day_2;
mod solution;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RunOptions {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    act: u8,
}

fn get_solution_from_day(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Day1::new_boxed()),
        2 => Some(Day2::new_boxed()),
        _ => None,
    }
}

fn main() {
    let run_options = RunOptions::parse();
    let chosen_solution =
        get_solution_from_day(run_options.day).unwrap_or_else(|| {
        println!("You did not provide a valid day.");
        process::exit(69);
    });
    let act = Act::from_u8(run_options.act).unwrap_or_else(|| {
        println!("You did not provide a valid act.");
        process::exit(420)
    });
    match chosen_solution.execute_act(act) {
        Ok(()) => (),
        Err(error) => {
            println!("The act failed to execute.");
            println!("Error is defined as:");
            println!("{:?}", error);
            process::exit(69420);
        }
    }
}
