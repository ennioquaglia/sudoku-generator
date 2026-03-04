#![allow(unused_imports)]
use std::{env, process::exit};

use itertools::Itertools;
use sudokulib::{
    generator::generate_sudoku,
    grid::{SudokuGrid, to_sudoku_coord},
    solver::solve_sudoku,
};
extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand::{Rng, prelude::*};
use rand_chacha::ChaCha8Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

fn generate_sudoku_exchange_paste_code(s: &SudokuGrid) -> String {
    s.data.iter().join("")
}

fn get_u64_seed_from_string(string_seed: &str) -> u64 {
    string_seed.parse::<u64>().unwrap_or_else(|_| {
        let mut rng: Pcg64 = Seeder::from(string_seed).into_rng();
        rng.random::<u64>()
    })
}

fn help() {
    println!(
        "usage:

sudoku-generator
    Generate a random sudoku.

sudoku-generator [single_seed]
    Generate a random sudoku from a single seed.
    
sudoku-generator [sudoku_seed] [stencil_seed]
    Generate a random sudoku from two seeds.
    "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|s| matches!(s.as_str(), "--help" | "--h")) {
        help();
        return;
    }

    let (sudoku_seed, stencil_seed) = match args.len() {
        0 | 1 => {
            let mut rng = rand::rng();
            (rng.random::<u64>(), rng.random::<u64>())
        }
        2 => {
            let single_seed = &args[1];
            let mut rng: Pcg64 = Seeder::from(single_seed).into_rng();
            (rng.random::<u64>(), rng.random::<u64>())
        }
        3 => (
            get_u64_seed_from_string(&args[1]),
            get_u64_seed_from_string(&args[2]),
        ),
        _ => {
            println!("Invalid number of parameter (got {})", args.len());
            help();
            exit(-1);
        }
    };

    println!(
        "sudoku_seed: {}, stencil_seed: {}",
        sudoku_seed, stencil_seed
    );

    let s = generate_sudoku(sudoku_seed, stencil_seed);

    println!("{}", s);
    println!(
        "https://sudokuexchange.com/play/?s={}\n",
        generate_sudoku_exchange_paste_code(&s)
    );

    assert!(s.has_unique_solution());
    assert!(s.is_incomplete());
}
