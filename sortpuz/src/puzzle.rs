use crate::{Color, Jar};
use rand::Rng;
use std::fmt;

pub struct Puzzle {
	pub(crate) jars: Vec<Jar>,
	pub(crate) moves: Vec<(usize, usize, usize)>,

	zobrist_table: Vec<Vec<[u64; 4]>>,
}

impl From<&str> for Puzzle {
	fn from(value: &str) -> Self {
		let mut jars = Vec::new();
		let mut jar = Jar::EMPTY;

		for ch in value.chars() {
			match ch {
				'/' => {
					jars.push(jar);
					jar = Jar::EMPTY;
				}
				'-' => jar = Jar::EMPTY,
				_ => match jar.len() < Jar::SIZE {
					true => jar.unchecked_push(Color::from(ch)),
					false => panic!("Invalid String :: Jar must not go beyond {}", Jar::SIZE),
				},
			}
		}

		Self::new(jars)
	}
}

impl fmt::Debug for Puzzle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (index, jar) in self.jars.iter().enumerate() {
			writeln!(f, "{index:<02}: {jar}, {}", jar.len())?;
		}

		write!(f, "")
	}
}

impl fmt::Display for Puzzle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (index, jar) in self.jars.iter().enumerate() {
			write!(f, "{:<02}: {jar}\n", index + 1)?;
		}

		write!(f, "")
	}
}

impl Puzzle {
	pub fn new(jars: Vec<Jar>) -> Self {
		let mut zobrist_table = vec![vec![[0; 4]; Color::SIZE]; jars.len()];
		let mut rng = rand::thread_rng();

		for i in 0..jars.len() {
			for color in Color::LIST {
				for j in Jar::RANGE {
					zobrist_table[i][color.index()][j] =
						rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
				}
			}
		}

		Self {
			jars,
			moves: Vec::new(),

			zobrist_table,
		}
	}
}

impl Puzzle {
	#[inline(always)]
	pub fn is_solved(&self) -> bool {
		self.jars
			.iter()
			.all(|jar| jar.is_empty() || jar.is_sorted())
	}
}

impl Puzzle {
	pub fn len(&self) -> usize {
		self.jars.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &Jar> {
		self.jars.iter()
	}
}

impl Puzzle {
	#[inline(always)]
	pub(crate) fn is_jar_solved(&self, index: usize) -> bool {
		self.jars[index].is_sorted()
	}

	#[inline(always)]
	pub(crate) fn hash(&self) -> u64 {
		let mut hash = 0;

		for (i, jar) in self.jars.iter().enumerate() {
			for (j, color) in jar.iter().enumerate() {
				if *color == Color::None {
					break;
				}

				hash ^= self.zobrist_table[i][color.index()][j];
			}
		}

		hash
	}
}

impl Puzzle {
	#[inline(always)]
	#[cfg(not(feature = "ball"))]
	pub fn make_move(&mut self, (from, to): (usize, usize)) -> bool {
		let mut valid = false;
		let mut count = 0;

		for _ in Jar::RANGE {
			match self.jars[from].pop() {
				Some(color) => {
					if !self.jars[to].push(color) {
						self.jars[from].unchecked_push(color);
						break;
					}

					valid = true;
					count += 1;
				}
				None => break,
			}
		}

		if valid {
			self.moves.push((from, to, count));
		}

		valid
	}

	#[inline(always)]
	pub fn undo_move(&mut self) {
		if let Some((from, to, count)) = self.moves.pop() {
			for _ in 0..count {
				match self.jars[to].pop() {
					Some(color) => self.jars[from].unchecked_push(color),
					None => panic!("Invalid Move: {} -> {}\n{self:?}", from + 1, to + 1),
				}
			}
		}
	}
}

impl Puzzle {
	#[inline(always)]
	#[cfg(feature = "ball")]
	pub fn make_move(&mut self, (from, to): (usize, usize)) -> bool {
		match self.jars[from].pop() {
			Some(color) => {
				if !self.jars[to].push(color) {
					self.jars[from].unchecked_push(color);
					return false;
				}

				self.moves.push((from, to, 1));
				return true;
			}
			None => {
				return false;
			}
		}
	}
}
