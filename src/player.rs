use std::{
	sync::{
		mpsc::{self, Receiver, Sender, SyncSender},
		Arc, Mutex,
	},
	thread,
};

use midir::MidiOutputConnection;
use nodi::{Event, Moment, Timer};

use crate::{bar::Bar, note::moment_notes, Command, Response, State};

type Bars = Vec<Bar>;

pub struct Player {
	output: Sender<Response>,
	con: Arc<Mutex<MidiOutputConnection>>,
	index: usize,
	last_forward: bool,
	all: Arc<Bars>,
	solo: Arc<Bars>,
	solo_on: bool,
	n_bars: usize,
	transpose: i8,
	speed: f32,
}

impl Player {
	pub fn new(con: MidiOutputConnection, output: Sender<Response>, all: Bars, solo: Bars) -> Self {
		let con = Arc::new(Mutex::new(con));
		let n_bars = all.len();

		let all = Arc::new(all);
		let solo = Arc::new(solo);
		Self {
			n_bars,
			con,
			output,
			index: 0,
			all,
			solo,
			solo_on: false,
			last_forward: true,
			transpose: 0,
			speed: 1.0,
		}
	}

	pub fn start(mut self, commands: Receiver<Command>) {
		let mut last_sender: Option<SyncSender<_>> = None;
		let mut last_played = 0_usize;
		for c in &commands {
			if let Some(ch) = &last_sender {
				ch.send(true).ok();
			}
			let (cancel_send, cancel) = mpsc::sync_channel(0);
			last_sender = Some(cancel_send);

			match c {
				Command::Next => {
					if let Some(n) = self.play_next(cancel) {
						last_played = n;
					} else {
						self.output.send(Response::EndOfTrack).unwrap();
					}
				}
				Command::Prev => {
					if let Some(n) = self.play_prev(cancel) {
						last_played = n;
					} else {
						self.output.send(Response::StartOfTrack).unwrap();
					}
				}
				Command::Replay => self.play(last_played, cancel),
				Command::Silence => self.silence(),
				Command::Reset => {
					self.reset();
					last_played = 0;
				}
				Command::Solo => self.solo_on = !self.solo_on,
				Command::Transpose(n) => {
					self.transpose(n);
					self.state(last_played);
				}
				Command::Info => self.state(last_played),
				Command::Speed(f) => {
					self.change_speed(f);
					self.state(last_played);
				}
			};
		}
	}

	fn play_next(&mut self, cancel: Receiver<bool>) -> Option<usize> {
		if self.index >= self.n_bars || (self.last_forward && self.index + 1 > self.n_bars) {
			return None;
		}

		if self.last_forward {
			self.index += 1;
		} else {
			self.index += 2;
		}

		self.last_forward = true;
		self.play(self.index - 1, cancel);
		Some(self.index - 1)
	}

	fn play_prev(&mut self, cancel: Receiver<bool>) -> Option<usize> {
		if self.last_forward {
			self.index = self.index.checked_sub(2)?;
		} else {
			self.index = self.index.checked_sub(1)?;
		}

		self.last_forward = false;
		self.play(self.index, cancel);
		Some(self.index)
	}

	fn reset(&mut self) {
		self.index = 0;
		self.last_forward = true;
		self.speed = 1.0;
	}

	fn silence(&self) {
		let mut con = self.con.lock().unwrap();
		let _ = con.send(&[0xb0, 123]);
		let _ = con.send(&[0xb0, 120]);
	}

	fn play(&self, n: usize, cancel: Receiver<bool>) {
		self.silence();
		let con = Arc::clone(&self.con);
		let notes = self.solo[n]
			.moments
			.iter()
			.map(|m| moment_notes(m, self.transpose))
			.flatten()
			.collect::<Vec<_>>();
		self.output.send(Response::Notes(notes)).unwrap();

		let bars = if self.solo_on {
			Arc::clone(&self.solo)
		} else {
			Arc::clone(&self.all)
		};

		let speed = self.speed;
		let transpose = self.transpose;

		thread::spawn(move || {
			let mut buf = Vec::new();

			let mut counter = 0_u32;
			let mut con = con.lock().unwrap();
			let mut timer = bars[n].timer;

			for moment in bars[n].transposed_moments(transpose).iter() {
				if cancel.try_recv().is_ok() {
					return;
				}

				match moment {
					Moment::Events(events) if !events.is_empty() => {
						let dur = timer.sleep_duration(counter).div_f64(speed as f64);
						if !dur.is_zero() {
							thread::sleep(dur);
						}
						counter = 0;
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
					_ => (),
				};
				counter += 1;
			}
		});
	}

	fn transpose(&mut self, n: i8) {
		if n == 0 {
			self.transpose = 0;
		} else {
			self.transpose = (self.transpose + n) % 12;
		}
	}

	fn state(&self, index: usize) {
		self.output
			.send(Response::State(State {
				transposition: self.transpose,
				index,
				solo: self.solo_on,
				length: self.solo.len(),
				speed: self.speed,
			}))
			.unwrap();
	}

	fn change_speed(&mut self, f: f32) {
		let f = f + self.speed;
		if f < 0.1 {
			self.speed = 0.1;
		} else {
			self.speed = f;
		}
	}
}
