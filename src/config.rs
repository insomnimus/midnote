use std::{
	error::Error,
	fmt,
	fs,
};

use crossterm::{
	event::KeyCode,
	style::{
		Attribute,
		Color,
		Stylize,
	},
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
	pub colors: bool,
	pub keys: Keys,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(default)]
pub struct Keys {
	pub next: KeyCode,
	pub prev: KeyCode,
	pub replay: KeyCode,
	pub silence: KeyCode,
	pub rewind: KeyCode,
	pub exit: KeyCode,
	pub help: KeyCode,
}

impl Default for Keys {
	fn default() -> Self {
		Self {
			next: KeyCode::Right,
			prev: KeyCode::Left,
			silence: KeyCode::Char(' '),
			exit: KeyCode::Esc,
			rewind: KeyCode::Char('s'),
			replay: KeyCode::Char('r'),
			help: KeyCode::Char('h'),
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			colors: true,
			keys: Keys::default(),
		}
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		const W: usize = 7;

		if self.colors {
			let mut format = |name: &str, val: KeyCode| -> fmt::Result {
				let val = key_string(val);
				let mut s = format!("{s:w$}", s = name, w = W);
				s = format!(
					"{} | {}",
					s.with(Color::Yellow).attribute(Attribute::Bold),
					val.with(Color::Grey),
				);
				writeln!(f, "{}", s.on(Color::Black))
			};
			format("next", self.keys.next)?;
			format("prev", self.keys.prev)?;
			format("replay", self.keys.replay)?;
			format("silence", self.keys.silence)?;
			format("rewind", self.keys.rewind)?;
			format("help", self.keys.help)?;
			format("exit", self.keys.exit)
		} else {
			let mut format = |name: &str, val: KeyCode| -> fmt::Result {
				let val = key_string(val);
				writeln!(f, "{s:w$} | {v:}", w = W, s = name, v = val)
			};
			format("next", self.keys.next)?;
			format("prev", self.keys.prev)?;
			format("replay", self.keys.replay)?;
			format("silence", self.keys.silence)?;
			format("rewind", self.keys.rewind)?;
			format("help", self.keys.help)?;
			format("exit", self.keys.exit)
		}
	}
}

impl Config {
	pub fn read_from(p: &str) -> Result<Self, Box<dyn Error>> {
		let data = fs::read_to_string(p)?;
		let c: Self = toml::from_str(&data)?;
		Ok(c)
	}
}

fn key_string(k: KeyCode) -> String {
	type K = KeyCode;
	match k {
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
		K::Char(c) => return c.to_string(),
		K::F(n) => return format!("F{}", n),
	}
	.to_string()
}
