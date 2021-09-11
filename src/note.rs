use std::fmt;

pub const NOTES: [&str; 12] = [
	"C", "C#", "D", "E♭", "E", "F", "F#", "G", "A♭", "A", "B♭", "B",
];

pub struct Note {
	offset: u8,
	octave: u8,
}

impl From<u8> for Note {
	fn from(n: u8) -> Self {
		let offset = n % 12;
		let octave = if offset == 0 { n / 12 + 1 } else { n / 12 };
		Self { offset, octave }
	}
}

impl fmt::Display for Note {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", NOTES[self.offset as usize], self.octave)
	}
}
