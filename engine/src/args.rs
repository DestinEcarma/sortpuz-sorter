use clap::{Parser, Subcommand};
use sortpuz::{Color, Puzzle, Sorter};

#[derive(Parser)]
pub struct Args {
	#[command(subcommand)]
	pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
	/// Display all available colors
	Colors,
	/// Check the given puzzle state if it is solvable
	Check {
		/// The puzzle state to check
		///
		/// The puzzle state should be in the format of `rrbb/bbrr/-/`
		///
		/// Note that the las `/` should be included to close the last jar
		///
		/// Use `colors` command to see the available colors
		puzzle: String,
		/// Whether to display the given puzzle state
		#[arg(short, long)]
		display: bool,
	},
	/// Solves the given puzzle state
	Solve {
		/// The puzzle state to check
		///
		/// The puzzle state should be in the format of `rrbb/bbrr/-/`
		///
		/// Note that the las `/` should be included to close the last jar
		///
		/// Use `colors` command to see the available colors
		puzzle: String,
		/// Whether to display the given puzzle state
		#[arg(short, long)]
		display: bool,
	},
}

pub fn colors() {
	println!("Available Colors:");

	for color in Color::LIST {
		println!("{:>4} {color} {color:?}", color.to_char());
	}
}

pub fn check(puzzle: String, display: bool) {
	let mut puzzle = Puzzle::from(puzzle.as_str());

	if display {
		println!("{puzzle}");
	}

	match Sorter::sort(&mut puzzle, false) {
		true => println!("Solvable"),
		false => println!("Unsolvable"),
	}
}

pub fn solve(puzzle: String, display: bool) {
	let mut puzzle = Puzzle::from(puzzle.as_str());

	if display {
		println!("Initial State:\n{puzzle}");
	}

	if Sorter::sort(&mut puzzle, true) && display {
		println!("\nSolved State:\n{puzzle}");
	}
}
