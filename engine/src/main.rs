mod args;

use args::{Args, Command};
use clap::{CommandFactory, Parser};

fn main() {
	let cli = Args::parse();

	match cli.command {
		Some(Command::Colors) => args::colors(),
		Some(Command::Check { puzzle, display }) => args::check(puzzle, display),
		Some(Command::Solve { puzzle, display }) => args::solve(puzzle, display),
		None => Args::command().print_help().unwrap(),
	}
}
