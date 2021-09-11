use std::cell::RefCell;

use midir::MidiOutputConnection;
use nodi::{
	Event,
	Moment,
	Sheet,
	Ticker,
	Timer,
};

use crate::{
	notes_moment,
	Note,
};

pub type Notes = Vec<Vec<Note>>;

pub struct Player {
	con: RefCell<MidiOutputConnection>,
	sheet: Sheet,
	timer: RefCell<Ticker>,
	index: usize,
	tpb: u16,
}

impl Player {
	pub fn new(con: MidiOutputConnection, sheet: Sheet, tpb: u16) -> Self {
		let timer = RefCell::new(Ticker::new(tpb));
		Self {
			con: RefCell::new(con),
			sheet,
			index: 0,
			tpb,
			timer,
		}
	}

	pub fn play_next(&mut self) -> Option<Notes> {
		if self.index >= self.sheet.len() {
			return None;
		}
		let end = self.sheet.len().min(self.index + self.tpb as usize);
		let slice = &self.sheet[self.index..end];
		self.index += self.tpb as usize;
		self.play(slice);

		Some(slice.iter().map(notes_moment).flatten().collect())
	}

	pub fn play_prev(&mut self) -> Option<Notes> {
		let end = self.index;
		self.index = self.index.checked_sub(self.tpb as usize)?;

		let slice = &self.sheet[self.index..end];
		self.play(slice);
		Some(slice.iter().map(notes_moment).flatten().collect())
	}

	fn play(&self, part: &[Moment]) {
		let mut buf = Vec::new();
		let mut empty_counter = 0_u32;
		for moment in part {
			match moment {
				Moment::Empty => empty_counter += 1,
				Moment::Events(events) => {
					self.timer.borrow().sleep(empty_counter);
					empty_counter = 0;
					for event in events {
						match event {
							Event::Tempo(val) => self.timer.borrow_mut().change_tempo(*val),
							Event::Midi(msg) => {
								buf.clear();
								let _ = msg.write(&mut buf);
								let _ = self.con.borrow_mut().send(&buf);
							}
							_ => (),
						};
					}
				}
			}
		}
	}

	pub fn replay(&mut self) {
		let start = match self.index.checked_sub(self.tpb as usize) {
			None => return,
			Some(n) => n,
		};

		let slice = &self.sheet[start..self.index];
		self.play(slice);
	}

	pub fn rewind_start(&mut self) {
		self.index = 0;
		self.timer.replace(Ticker::new(self.tpb));
	}
}
