use crate::Color;
use std::fmt;

#[derive(Debug)]
pub struct Jar {
	colors: [Color; Self::SIZE],
	count: usize,
}

impl From<[Color; Self::SIZE]> for Jar {
	fn from(value: [Color; Self::SIZE]) -> Self {
		let mut jar = Self::EMPTY;

		for color in value {
			if color != Color::None {
				jar.unchecked_push(color);
			}
		}

		jar
	}
}

impl fmt::Display for Jar {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[")?;

		for i in Self::RANGE {
			write!(f, "{}", self.colors[i])?;

			if i < Self::SIZE - 1 {
				write!(f, ", ")?;
			}
		}

		write!(f, "]")
	}
}
impl Jar {
	pub const SIZE: usize = 4;
	pub const RANGE: std::ops::Range<usize> = 0..Self::SIZE;

	pub const EMPTY: Self = Self {
		colors: [Color::None; Self::SIZE],
		count: 0,
	};
}

impl Jar {
	pub fn push(&mut self, color: Color) -> bool {
		if self.count >= Self::SIZE {
			return false;
		}

		if !self.is_empty() {
			let prev_color = self.colors[self.count - 1];

			if !(prev_color == Color::None || prev_color == color) {
				return false;
			}
		}

		self.colors[self.count] = color;
		self.count += 1;

		true
	}

	pub fn pop(&mut self) -> Option<Color> {
		if self.count > 0 {
			self.count -= 1;
			let color = self.colors[self.count];

			self.colors[self.count] = Color::None;
			return Some(color);
		}

		None
	}
}

impl Jar {
	pub(crate) fn len(&self) -> usize {
		self.count
	}

	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool {
		self.count == 0
	}

	pub(crate) fn iter(&self) -> std::slice::Iter<Color> {
		self.colors.iter()
	}

	pub(crate) fn unchecked_push(&mut self, color: Color) {
		self.colors[self.count] = color;
		self.count += 1;
	}

	pub(crate) fn is_sorted(&self) -> bool {
		if self.count != Self::SIZE {
			return false;
		}

		for i in 1..Self::SIZE {
			if self.colors[i] != self.colors[0] {
				return false;
			}
		}

		true
	}
}
