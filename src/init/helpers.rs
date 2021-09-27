use std::{
	error::Error,
	fmt,
	io::{
		self,
		BufRead,
		Write,
	},
};

use midir::{
	MidiOutput,
	MidiOutputConnection,
};
use midly::{
	MetaMessage,
	TrackEvent,
	TrackEventKind,
};
use nodi::Sheet;

pub fn list_devices() -> Result<(), Box<dyn Error>> {
	let midi_out = MidiOutput::new("nodi")?;

	let out_ports = midi_out.ports();

	if out_ports.is_empty() {
		println!("No active MIDI output device detected.");
	} else {
		for (i, p) in out_ports.iter().enumerate() {
			println!(
				"#{}: {}",
				i,
				midi_out
					.port_name(p)
					.as_deref()
					.unwrap_or("<no device name>")
			);
		}
	}

	Ok(())
}

pub fn get_midi(n: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
	let midi_out = MidiOutput::new("nodi")?;

	let out_ports = midi_out.ports();
	if out_ports.is_empty() {
		return Err("no MIDI output device detected".into());
	}
	if n >= out_ports.len() {
		return Err(format!(
			"only {} MIDI devices detected; run with --list  to see them",
			out_ports.len()
		)
		.into());
	}

	let out_port = &out_ports[n];
	let out = midi_out.connect(out_port, "cello-tabs")?;
	Ok(out)
}

struct Meta {
	name: Option<String>,
	instrument: Option<String>,
}

impl Meta {
	pub fn from_track(events: &[TrackEvent<'_>]) -> Self {
		let mut s = Self {
			name: None,
			instrument: None,
		};
		for e in events {
			match e.kind {
				TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
					s.name = Some(String::from_utf8_lossy(name).to_string());
					if s.instrument.is_some() {
						break;
					}
				}
				TrackEventKind::Meta(MetaMessage::InstrumentName(inst)) => {
					s.instrument = Some(String::from_utf8_lossy(inst).to_string());
					if s.name.is_some() {
						break;
					}
				}
				_ => (),
			};
		}

		s
	}
}

impl fmt::Display for Meta {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = self.name.as_deref().unwrap_or("Unnamed Track");
		if let Some(inst) = &self.instrument {
			write!(f, "{} ({})", name, &inst)
		} else {
			write!(f, "{}", name)
		}
	}
}

pub fn choose_track(tracks: &[Vec<TrackEvent<'_>>]) -> Sheet {
	let names = tracks.iter().map(|t| Meta::from_track(t));

	for (i, name) in names.enumerate() {
		println!("#{}: {}", i, &name);
	}

	loop {
		print!("choose a track (0-{}): ", tracks.len());
		io::stdout().flush().unwrap();
		let stdin = io::stdin();
		let n = stdin.lock().lines().next().unwrap().unwrap();

		let n = match n.parse::<usize>() {
			Err(_) => {
				println!("please enter a number between 0 and {}", tracks.len());
				continue;
			}
			Ok(n) if n >= tracks.len() => {
				println!("please enter a number between 0 and {}", tracks.len());
				continue;
			}
			Ok(n) => n,
		};

		return Sheet::single(&tracks[n]);
	}
}
