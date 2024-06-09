use crate::Puzzle;
use std::collections::HashSet;

pub struct Sorter;

impl Sorter {
	pub fn sort(puzzle: &mut Puzzle, display: bool) -> bool {
		let mut visited = HashSet::new();

		Self::sort_driver(puzzle, &mut visited);

		let is_solved = puzzle.is_solved();

		if display {
			match is_solved {
				true => {
					for (from, to, _) in puzzle.moves.iter() {
						println!("Move: {} -> {}", from + 1, to + 1);
					}
				}
				false => println!("No solution found"),
			}
		}

		is_solved
	}
}

impl Sorter {
	#[inline(always)]
	fn sort_driver(puzzle: &mut Puzzle, visited: &mut HashSet<u64>) {
		let len = puzzle.len();

		visited.insert(puzzle.hash());

		for from in 0..len {
			if puzzle.is_jar_solved(from) {
				continue;
			}

			for to in 0..len {
				if from == to || puzzle.is_jar_solved(to) || !puzzle.make_move((from, to)) {
					continue;
				}

				match visited.contains(&(puzzle.hash())) {
					false => {
						Self::sort_driver(puzzle, visited);

						match puzzle.is_solved() {
							true => return,
							false => puzzle.undo_move(),
						}
					}
					true => puzzle.undo_move(),
				}
			}
		}
	}
}
