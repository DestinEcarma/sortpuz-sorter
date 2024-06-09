use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	None,
	Red,
	Pink,
	Blue,
	Gray,
	Brown,
	Peach,
	Green,
	Purple,
	Yellow,
	Orange,
	BrightBlue,
	BrightGreen,
}

impl From<char> for Color {
	fn from(c: char) -> Self {
		match c {
			'-' => Color::None,
			'r' => Color::Red,
			'p' => Color::Pink,
			'b' => Color::Blue,
			'a' => Color::Gray,
			'n' => Color::Brown,
			'c' => Color::Peach,
			'g' => Color::Green,
			'P' => Color::Purple,
			'y' => Color::Yellow,
			'o' => Color::Orange,
			'B' => Color::BrightBlue,
			'G' => Color::BrightGreen,
			_ => panic!("Invalid color character: {}", c),
		}
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Color::None => write!(f, "{}", "----".truecolor(128, 128, 128)),
			Color::Red => write!(f, "{}", "====".truecolor(255, 0, 0)),
			Color::Pink => write!(f, "{}", "====".truecolor(255, 20, 147)),
			Color::Blue => write!(f, "{}", "====".truecolor(0, 0, 255)),
			Color::Gray => write!(f, "{}", "====".truecolor(128, 128, 128)),
			Color::Brown => write!(f, "{}", "====".truecolor(139, 69, 19)),
			Color::Peach => write!(f, "{}", "====".truecolor(255, 192, 203)),
			Color::Green => write!(f, "{}", "====".truecolor(0, 100, 0)),
			Color::Purple => write!(f, "{}", "====".truecolor(153, 50, 204)),
			Color::Yellow => write!(f, "{}", "====".truecolor(255, 255, 0)),
			Color::Orange => write!(f, "{}", "====".truecolor(255, 128, 0)),
			Color::BrightBlue => write!(f, "{}", "====".truecolor(0, 191, 255)),
			Color::BrightGreen => write!(f, "{}", "====".truecolor(0, 255, 0)),
		}
	}
}

impl Color {
	pub const SIZE: usize = 12;
	pub const LIST: [Color; 12] = [
		Color::Red,
		Color::Pink,
		Color::Blue,
		Color::Gray,
		Color::Brown,
		Color::Peach,
		Color::Green,
		Color::Purple,
		Color::Yellow,
		Color::Orange,
		Color::BrightBlue,
		Color::BrightGreen,
	];
}

impl Color {
	pub fn to_char(&self) -> char {
		match self {
			Color::None => '-',
			Color::Red => 'r',
			Color::Pink => 'p',
			Color::Blue => 'b',
			Color::Gray => 'a',
			Color::Brown => 'n',
			Color::Peach => 'c',
			Color::Green => 'g',
			Color::Purple => 'P',
			Color::Yellow => 'y',
			Color::Orange => 'o',
			Color::BrightBlue => 'B',
			Color::BrightGreen => 'G',
		}
	}
}

impl Color {
	pub(crate) fn index(&self) -> usize {
		*self as u8 as usize - 1
	}
}
