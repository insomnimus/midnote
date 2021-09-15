use std::{
	fmt::Write as _Write,
	io::{
		stdout,
		Write,
	},
	sync::mpsc::{
		self,
		Receiver,
	},
	thread,
	time::Duration,
};

use crossterm::{
	event::{
		self,
		Event,
		KeyCode,
	},
	style::{
		Attribute,
		Color,
		Stylize,
	},
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		Clear,
		ClearType,
	},
	ExecutableCommand,
};
use midnote::{
	init,
	player::Player,
	Command,
	Note,
	Response,
};

const CLEAR: Clear = Clear(ClearType::All);

fn run((player, response): (Player, Receiver<Response>)) {
	let (commands, commands_recv) = mpsc::channel();
	thread::spawn(move || {
		player.start(commands_recv);
	});
	start_display(response);

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
	}
}

fn print_notes(notes: &[Vec<Note>]) {
	let mut stdout = stdout();
	let _ = stdout.execute(CLEAR);
	if notes.is_empty() {
		let s = "---"
			.with(Color::Grey)
			.attribute(Attribute::Bold)
			.on(Color::Black);
		writeln!(&mut stdout, "{}", s).unwrap();
		let _ = stdout.flush();
		return;
	}
	let mut buf = String::new();

	for ns in notes {
		for (i, n) in ns.iter().enumerate() {
			if i > 0 {
				buf.push_str(", ");
			}
			write!(&mut buf, "{}", n).unwrap();
		}
		buf.push('\n');
	}

	let s = buf
		.with(Color::Cyan)
		.attribute(Attribute::Bold)
		.on(Color::Black);
	write!(&mut stdout, "{}", s).unwrap();

	let _ = stdout.flush();
}

fn print_clear(s: &str) {
	let mut stdout = stdout();
	let _ = stdout.execute(CLEAR);

	let s = s.on(Color::Black).with(Color::Yellow);
	writeln!(&mut stdout, "{}", s).unwrap();
	let _ = stdout.flush();
}

fn start_display(response: Receiver<Response>) {
	thread::spawn(move || {
		for resp in response {
			match resp {
				Response::StartOfTrack => print_clear("Start of track."),
				Response::EndOfTrack => print_clear("End of track."),
				Response::Notes(notes) => {
					thread::sleep(Duration::from_millis(50));
					print_notes(&notes);
				}
			};
		}
	});
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
