use std::{
	sync::mpsc::{
		self,
		Receiver,
	},
	thread,
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
use tabs::{
	init,
	player::Player,
	Command,
	Note,
	Response,
};

fn run((player, response): (Player, Receiver<Response>)) {
	let (commands, commands_recv) = mpsc::channel();
	thread::spawn(move || {
		player.start(commands_recv);
	});

	loop {
		let k = match event::read() {
			Ok(Event::Key(k)) => k,
			_ => continue,
		};

		match k.code {
			KeyCode::Char(' ') => commands.send(Command::Silence),
			KeyCode::Char('r') => commands.send(Command::Replay),
			KeyCode::Char('q') => break,
			KeyCode::Char('s') => commands.send(Command::RewindStart),
			KeyCode::Left => commands.send(Command::Prev),
			KeyCode::Right => commands.send(Command::Next),
			_ => Ok(()),
		}
		.unwrap();

		for resp in response.try_iter() {
			match resp {
				Response::StartOfTrack => println!("start of track"),
				Response::EndOfTrack => println!("end of track"),
				Response::Notes(notes) => print_notes(&notes),
			};
		}
	}
}

fn print_notes(notes: &[Vec<Note>]) {
	println!("-");
	let notes = notes.iter().map(|ns| ns.iter().map(|n| n.to_string()));
	for ns in notes {
		let ns = ns.collect::<Vec<_>>();
		println!("{}", ns.join(", "));
	}
}

fn main() {
	let player = match init::parse_args() {
		Err(e) => {
			eprintln!("error: {}", e);
			std::process::exit(2);
		}
		Ok(p) => p,
	};

	if let Err(e) = enable_raw_mode() {
		eprintln!("warning: could not enable raw mode: {}", e);
	}

	run(player);

	if let Err(e) = disable_raw_mode() {
		eprintln!("warning: could not disable raw mode: {}", e);
	}
}
