use std::{
	ops::Range,
	sync::{
		mpsc::{
			self,
			Receiver,
			Sender,
			SyncSender,
		},
		Arc,
		Mutex,
	},
	thread,
};

use midir::MidiOutputConnection;
use nodi::{
	Event,
	Moment,
	Sheet,
	Ticker,
	Timer,
};

use crate::{
	note::moment_notes,
	Command,
	Response,
};

pub struct Player {
	output: Sender<Response>,
	con: Arc<Mutex<MidiOutputConnection>>,
	index: usize,
	tpb: usize,
	timer: Arc<Mutex<Ticker>>,
	chunks: usize,
	last_forward: bool,
	sheet: Arc<Sheet>,
	track_len: usize,
}

impl Player {
	pub fn new(
		con: MidiOutputConnection,
		output: Sender<Response>,
		sheet: Sheet,
		tpb: usize,
		chunks: usize,
	) -> Self {
		let timer = Arc::new(Mutex::new(Ticker::new(tpb as u16)));
		let con = Arc::new(Mutex::new(con));
		let track_len = sheet.len();
		let sheet = Arc::new(sheet);
		Self {
			track_len,
			chunks,
			con,
			output,
			timer,
			index: 0,
			tpb,
			sheet,
			last_forward: true,
		}
	}

	pub fn start(mut self, commands: Receiver<Command>) {
		let mut last_sender: Option<SyncSender<_>> = None;
		let mut range = 0..0;
		for c in &commands {
			if let Some(ch) = &last_sender {
				ch.send(true).ok();
			}
			let (cancel_send, cancel) = mpsc::sync_channel(0);
			last_sender = Some(cancel_send);
			match c {
				Command::Next => {
					if let Some(r) = self.play_next(cancel) {
						range = r;
					} else {
						self.output.send(Response::EndOfTrack).unwrap();
					}
				}
				Command::Prev => {
					if let Some(r) = self.play_prev(cancel) {
						range = r;
					} else {
						self.output.send(Response::StartOfTrack).unwrap();
					}
				}
				Command::Replay if range != (0..0) => self.play(range.clone(), cancel),
				Command::Replay => (),
				Command::Silence => self.silence(),
				Command::RewindStart => {
					self.rewind_start();
					range = 0..0;
				}
			};
		}
	}

	fn play_next(&mut self, cancel: Receiver<bool>) -> Option<Range<usize>> {
		if self.index >= self.track_len {
			return None;
		}
		let delta = self.tpb * self.chunks;
		let range = if self.last_forward {
			let start = self.index;
			self.index += delta;
			let end = self.index.min(self.track_len);
			start..end
		} else if self.index + delta >= self.track_len {
			return None;
		} else {
			let start = self.index + delta;
			self.index += delta * 2;
			let end = self.index.min(self.sheet.len());
			start..end
		};

		self.last_forward = true;
		self.play(range.clone(), cancel);
		Some(range)
	}

	fn play_prev(&mut self, cancel: Receiver<bool>) -> Option<Range<usize>> {
		let delta = self.tpb * self.chunks;

		let end = if self.last_forward {
			let end = self.index.checked_sub(delta)?;
			self.index = end.checked_sub(delta)?;
			end
		} else {
			let end = self.index;
			self.index = self.index.checked_sub(delta)?;
			end
		};

		self.last_forward = false;
		let range = self.index..end;
		self.play(range.clone(), cancel);
		Some(range)
	}

	fn rewind_start(&mut self) {
		*self.timer.lock().unwrap() = Ticker::new(self.tpb as u16);
		self.index = 0;
	}

	fn silence(&self) {
		let mut con = self.con.lock().unwrap();
		let _ = con.send(&[0xb0, 120]);
	}

	fn play(&self, range: Range<usize>, cancel: Receiver<bool>) {
		self.silence();
		let output = self.output.clone();
		let con = Arc::clone(&self.con);
		let sheet = Arc::clone(&self.sheet);
		let timer = Arc::clone(&self.timer);

		thread::spawn(move || {
			let mut buf = Vec::new();

			let mut empty_counter = 0_u32;
			let mut con = con.lock().unwrap();
			let mut timer = timer.lock().unwrap();
			let mut played_notes = Vec::new();
			let slice = trim_moments(&sheet[range]);

			for moment in slice {
				if cancel.try_recv().is_ok() {
					return;
				}

				if let Some(notes) = moment_notes(moment) {
					played_notes.push(notes);
				}

				match moment {
					Moment::Events(events) => {
						timer.sleep(empty_counter);
						empty_counter = 0;
						for event in events {
							match event {
								Event::Tempo(val) => timer.change_tempo(*val),
								Event::Midi(msg) => {
									buf.clear();
									let _ = msg.write(&mut buf);
									let _ = con.send(&buf);
								}
								_ => (),
							};
						}
					}
					Moment::Empty => empty_counter += 1,
				};
			}
			output.send(Response::Notes(played_notes)).unwrap();
		});
	}
}

fn trim_moments(slice: &[Moment]) -> &[Moment] {
	let start = slice.iter().take_while(|m| m.is_empty()).count();
	let slice = &slice[start..];
	let end = slice.iter().rev().take_while(|m| m.is_empty()).count();
	&slice[..(slice.len() - end)]
}
