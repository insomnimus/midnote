mod note;
mod player;

use midly::MidiMessage;
use nodi::{
	Event,
	Moment,
};
pub use note::*;
pub use player::*;

fn notes_moment(moment: &Moment) -> Option<Vec<Note>> {
	match &moment {
		Moment::Empty => None,
		Moment::Events(events) => {
			let mut buf = Vec::new();
			for e in events {
				if let Event::Midi(m) = e {
					match m.message {
						MidiMessage::NoteOn { key, vel } if vel > 0 => {
							buf.push(u8::from(key).into());
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
