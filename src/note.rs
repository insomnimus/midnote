use std::fmt;

use midly::MidiMessage;
use nodi::{Event, Moment};

pub const NOTES: [&str; 12] = [
	"C", "C#", "D", "E♭", "E", "F", "F#", "G", "A♭", "A", "B♭", "B",
];

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Note {
	offset: u8,
	octave: u8,
}

impl From<u8> for Note {
	fn from(n: u8) -> Self {
		let offset = n % 12;
		let octave = if offset == 0 { n / 12 } else { n / 12 + 1 };
		Self { offset, octave }
	}
}

impl fmt::Display for Note {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", NOTES[self.offset as usize], self.octave)
	}
}

pub fn moment_notes(moment: &Moment) -> Option<Vec<Note>> {
	match &moment {
		Moment::Empty => None,
		Moment::Events(events) => {
			let mut buf = Vec::new();
			for e in events {
				if let Event::Midi(m) = e {
					match m.message {
						MidiMessage::NoteOn { key, vel } if vel > 0 => {
							let k: Note = u8::from(key).into();
							if !buf.contains(&k) {
								buf.push(k);
							}
						}
						_ => {}
					};
				}
			}

			if buf.is_empty() {
				None
			} else {
				Some(buf)
			}
		}
	}
}
