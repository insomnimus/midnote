mod app;

use std::{
	error::Error,
	fs,
	io::{
		self,
		BufRead,
		Write,
	},
};

use crossterm::{
	event::{
		self,
		Event,
		KeyCode,
	},
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
	},
};
use midir::{
	MidiOutput,
	MidiOutputConnection,
};
use midly::{
	Format,
	MetaMessage,
	Smf,
	Timing,
	TrackEvent,
	TrackEventKind,
};
use nodi::Sheet;
use tabs::Player;

fn get_midi(n: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
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

fn list_devices() -> Result<(), Box<dyn Error>> {
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

fn parse_args() -> Result<Player, Box<dyn Error>> {
	let m = app::new().get_matches();
	if m.is_present("list") {
		list_devices()?;
		std::process::exit(0);
	}

	let data = m.value_of("file").map(|s| fs::read(s)).unwrap()?;
	let device_no = m.value_of("device").unwrap().parse::<usize>()?;
	let con = get_midi(device_no)?;

	let Smf { tracks, header } = Smf::parse(&data)?;

	let tpb = match header.timing {
		Timing::Metrical(n) => u16::from(n),
		_ => return Err("the midi file has an unsupported time format".into()),
	};

	Ok(match header.format {
		Format::Parallel => {
			let meta = Sheet::single(&tracks[0]);
			let mut sheet = choose_track(&tracks[0..]);
			sheet.merge_with(meta);
			Player::new(con, sheet, tpb)
		}
		_ => {
			let sheet = Sheet::sequential(&tracks);
			Player::new(con, sheet, tpb)
		}
	})
}

fn run(player: &mut Player) {
	if let Err(e) = enable_raw_mode() {
		eprintln!("warning: could not enable raw mode: {}", e);
	}

	loop {
		let k = match event::read() {
			Ok(Event::Key(k)) => k,
			_ => continue,
		};

		match k.code {
			KeyCode::Char('r') => player.replay(),
			KeyCode::Char('q') => break,
			KeyCode::Char('s') => player.rewind_start(),
			KeyCode::Left => {
				match player.play_prev() {
					None => println!("At the start."),
					Some(notes) => {
						println!("-");
						for n in notes {
							let n = n.iter().map(|s| s.to_string()).collect::<Vec<_>>();
							println!("{}", n.join(", "));
						}
					}
				};
			}
			KeyCode::Right => {
				match player.play_next() {
					None => println!("End of track, press `s` to seek to the start."),
					Some(notes) => {
						println!("-");
						for n in notes {
							let n = n.iter().map(|s| s.to_string()).collect::<Vec<_>>();
							println!("{}", n.join(", "));
						}
					}
				};
			}
			_ => (),
		};
	}

	disable_raw_mode();
}

fn choose_track(tracks: &[Vec<TrackEvent<'_>>]) -> Sheet {
	let names = tracks
		.iter()
		.map(|t| {
			t.iter()
				.filter_map(|e| match e.kind {
					TrackEventKind::Meta(MetaMessage::TrackName(s)) => {
						Some(String::from_utf8_lossy(s))
					}
					_ => None,
				})
				.next()
				.unwrap_or_else(|| std::borrow::Cow::Borrowed("unnamed track"))
		})
		.collect::<Vec<_>>();

	for (i, name) in names.iter().enumerate() {
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

fn main() {
	let mut player = match parse_args() {
		Err(e) => {
			eprintln!("error: {}", e);
			std::process::exit(2);
		}
		Ok(p) => p,
	};
	run(&mut player);
}
