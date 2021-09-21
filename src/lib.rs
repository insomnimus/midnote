mod app;
pub(crate) mod bar;
pub mod config;
pub mod init;
mod note;
pub mod player;

pub use note::{moment_notes, Note};
pub type Notes = Vec<Vec<Note>>;

pub enum Command {
	Next,
	Prev,
	Replay,
	Silence,
	RewindStart,
	Solo,
}

pub enum Response {
	EndOfTrack,
	StartOfTrack,
	Notes(Notes),
}
