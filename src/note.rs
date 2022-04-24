use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};

use midly::MidiMessage;
use nodi::{Event, Moment};

static STYLE: AtomicU8 = AtomicU8::new(1);

pub fn toggle_style() {
	STYLE
		.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
			Some(n.wrapping_add(1) % (NoteStyle::VALUES.len() as u8))
		})
		.unwrap();
}

pub fn style() -> NoteStyle {
	NoteStyle::VALUES[STYLE.load(Ordering::Relaxed) as usize]
}

struct NoteName {
	abc: &'static str,
	doremi: &'static str,
}

const NOTES: [NoteName; 12] = [
	NoteName {
		abc: "C",
		doremi: "Do",
	},
	NoteName {
		abc: "C#",
		doremi: "Do#",
	},
	NoteName {
		abc: "D",
		doremi: "Re",
	},
	NoteName {
		abc: "E♭",
		doremi: "Mi♭",
	},
	NoteName {
		abc: "E",
		doremi: "Mi",
	},
	NoteName {
		abc: "F",
		doremi: "Fa",
	},
	NoteName {
		abc: "F#",
		doremi: "Fa#",
	},
	NoteName {
		abc: "G",
		doremi: "Sol",
	},
	NoteName {
		abc: "A♭",
		doremi: "La♭",
	},
	NoteName {
		abc: "A",
		doremi: "La",
	},
	NoteName {
		abc: "B♭",
		doremi: "Si♭",
	},
	NoteName {
		abc: "B",
		doremi: "Si",
	},
];

#[derive(Copy, Clone, PartialEq, Eq)]
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
		style().display_note(*self, f)
	}
}

pub fn moment_notes(moment: &Moment, shift: i8) -> Option<Vec<Note>> {
	match &moment {
		Moment::Empty => None,
		Moment::Events(events) => {
			let mut buf = Vec::new();
			for e in events {
				if let Event::Midi(m) = e {
					if m.channel == 9 {
						// Drum channel, skip.
						continue;
					}
					match m.message {
						MidiMessage::NoteOn { key, vel } if vel > 0 => {
							let key = key.as_int() as i32 + shift as i32;
							if (0..=127).contains(&key) {
								let k = Note::from(key as u8);
								if !buf.contains(&k) {
									buf.push(k);
								}
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

#[derive(Copy, Clone)]
pub enum NoteStyle {
	/// A, B, C
	Abc,
	/// A3, B2, E7
	AbcN,
	/// Do, Re, Mi
	Doremi,
	/// Do 2, Re 5, Mi 7
	DoremiN,
}

impl NoteStyle {
	pub const VALUES: [Self; 4] = [Self::Abc, Self::AbcN, Self::Doremi, Self::DoremiN];

	#[inline]
	pub fn display_note(self, n: Note, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let display = &NOTES[n.offset as usize];
		match self {
			Self::Abc => f.write_str(display.abc),
			Self::AbcN => write!(f, "{}{}", display.abc, n.octave),
			Self::Doremi => f.write_str(display.doremi),
			Self::DoremiN => write!(f, "{}{}", display.doremi, n.octave),
		}
	}
}
