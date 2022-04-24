mod app;
pub(crate) mod bar;
pub mod config;
pub mod init;
mod note;
pub mod player;

use std::fmt;

pub use note::{moment_notes, Note};
pub type Notes = Vec<Vec<Note>>;

pub struct State {
	pub transposition: i8,
	pub index: usize,
	pub length: usize,
	pub solo: bool,
	pub speed: f32,
}

pub enum Command {
	Next,
	Prev,
	Replay,
	Silence,
	Reset,
	Solo,
	/// Transpose(0) will reset it instead
	Transpose(i8),
	Info,
	/// Changes the speed by the value given.
	Speed(f32),
	NoteStyle,
}

pub enum Response {
	EndOfTrack,
	StartOfTrack,
	Notes(Notes),
	State(State),
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{index} / {length}
transposition = {trans:+} | solo = {solo} | speed = {speed}%",
			index = self.index,
			length = self.length,
			trans = self.transposition,
			solo = if self.solo { "on" } else { "off" },
			speed = self.speed * 100.0,
		)
	}
}
