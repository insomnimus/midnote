use std::{borrow::Cow, error::Error, fmt, fs};

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

use crate::Command;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
	pub colors: bool,
	pub keys: Keys,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			colors: true,
			keys: Keys::default(),
		}
	}
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(default)]
pub struct Keys {
	pub next: KeyCode,
	pub prev: KeyCode,
	pub transpose_up: KeyCode,
	pub transpose_down: KeyCode,
	pub speed_up: KeyCode,
	pub speed_down: KeyCode,
	pub reset: KeyCode,
	pub replay: KeyCode,
	pub solo: KeyCode,
	pub silence: KeyCode,
	pub rewind: KeyCode,
	pub info: KeyCode,
	pub exit: KeyCode,
	pub help: KeyCode,
}

impl Default for Keys {
	fn default() -> Self {
		Self {
			next: KeyCode::Right,
			prev: KeyCode::Left,
			transpose_up: KeyCode::Up,
			transpose_down: KeyCode::Down,
			speed_up: KeyCode::Char('2'),
			speed_down: KeyCode::Char('1'),
			reset: KeyCode::Char('x'),
			silence: KeyCode::Char(' '),
			exit: KeyCode::Esc,
			solo: KeyCode::Char('s'),
			rewind: KeyCode::Char('p'),
			replay: KeyCode::Char('r'),
			info: KeyCode::Char('i'),
			help: KeyCode::Char('h'),
		}
	}
}

impl Config {
	pub fn read_from(p: &str) -> Result<Self, Box<dyn Error>> {
		let data = fs::read_to_string(p)?;
		let c: Self = serde_json::from_str(&data)?;
		Ok(c)
	}
}

impl fmt::Display for Keys {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let keys = &[
			("next", self.next),
			("previous", self.prev),
			("transpose up", self.transpose_up),
			("transpose down", self.transpose_down),
			("speed up", self.speed_up),
			("speed down", self.speed_down),
			("reset transposition", self.reset),
			("replay", self.replay),
			("solo", self.solo),
			("rewind", self.rewind),
			("silence", self.silence),
			("info", self.info),
			("help", self.help),
			("exit", self.exit),
		];

		let w = keys.iter().map(|x| x.0.len()).max().unwrap();

		for (i, (name, key)) in keys.iter().enumerate() {
			if i > 0 {
				f.write_str("\n")?;
			}
			write!(
				f,
				"{name:w$} | {key}",
				name = name,
				w = w,
				key = key_string(*key)
			)?;
		}
		Ok(())
	}
}

fn key_string(k: KeyCode) -> Cow<'static, str> {
	type K = KeyCode;
	Cow::Borrowed(match k {
		K::Null => "null",
		K::BackTab => "backtab",
		K::Backspace => "backspace",
		K::Delete => "del",
		K::Up => "up",
		K::Down => "down",
		K::Left => "left",
		K::Right => "right",
		K::End => "end",
		K::Enter => "enter",
		K::Esc => "esc",
		K::Home => "home",
		K::Insert => "insert",
		K::PageDown => "pagedown",
		K::PageUp => "pageup",
		K::Tab => "tab",
		K::Char(c) => return Cow::Owned(format!("'{}'", c)),
		K::F(n) => return Cow::Owned(format!("F{}", n)),
	})
}

impl Keys {
	pub fn command(&self, k: KeyCode) -> Option<Command> {
		Some(if k == self.next {
			Command::Next
		} else if k == self.prev {
			Command::Prev
		} else if k == self.transpose_up {
			Command::Transpose(1)
		} else if k == self.transpose_down {
			Command::Transpose(-1)
		} else if k == self.reset {
			Command::Transpose(0)
		} else if k == self.replay {
			Command::Replay
		} else if k == self.silence {
			Command::Silence
		} else if k == self.solo {
			Command::Solo
		} else if k == self.rewind {
			Command::Reset
		} else if k == self.info {
			Command::Info
		} else if k == self.speed_up {
			Command::Speed(0.05)
		} else if k == self.speed_down {
			Command::Speed(-0.05)
		} else {
			return None;
		})
	}
}
