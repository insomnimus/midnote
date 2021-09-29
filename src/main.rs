use std::{
	error::Error,
	fmt::{self, Write as _Write},
	io::{self, stdout, Write},
	sync::mpsc::{self, Receiver},
	thread,
	time::Duration,
};

use crossterm::{
	event::{self, Event},
	style::{Attribute, Color, Stylize},
	terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
	ExecutableCommand,
};
use midnote::{init::Args, Note, Response};

const CLEAR: Clear = Clear(ClearType::All);

fn run(
	Args {
		config,
		player,
		response,
	}: Args,
) -> Result<(), Box<dyn Error>> {
	let (commands, commands_recv) = mpsc::channel();
	thread::spawn(move || {
		player.start(commands_recv);
	});
	start_display(response, config.colors);
	print(&config.keys).unwrap();

	let keys = config.keys;

	loop {
		let k = match event::read() {
			Ok(Event::Key(k)) => k.code,
			_ => continue,
		};

		if let Some(cmd) = keys.command(k) {
			commands.send(cmd)?;
			continue;
		}

		if k == keys.exit {
			break;
		} else if k == keys.help {
			print(&keys).unwrap();
		}
	}

	Ok(())
}

fn print_notes(notes: &[Vec<Note>], colors: bool) {
	if notes.is_empty() {
		if colors {
			let s = "---"
				.with(Color::Grey)
				.attribute(Attribute::Bold)
				.on(Color::Black);
			print(&s)
		} else {
			print("---")
		}
		.unwrap();
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

	if colors {
		let s = buf
			.with(Color::Cyan)
			.attribute(Attribute::Bold)
			.on(Color::Black);
		print(&s)
	} else {
		print(&buf)
	}
	.unwrap();
}

fn print_color(s: &str, colors: bool) {
	if colors {
		let s = s.on(Color::Black).with(Color::Yellow);
		print(&s)
	} else {
		print(s)
	}
	.unwrap();
}

fn start_display(response: Receiver<Response>, colors: bool) {
	thread::spawn(move || {
		for resp in response {
			match resp {
				Response::State(s) => print_color(&s.to_string(), colors),
				Response::StartOfTrack => print_color("Start of track.", colors),
				Response::EndOfTrack => print_color("End of track.", colors),
				Response::Notes(notes) => {
					// This sleep prevents the screen reader from glitching.
					thread::sleep(Duration::from_millis(50));
					print_notes(&notes, colors);
				}
			};
		}
	});
}

fn print<S: fmt::Display>(s: S) -> Result<(), Box<io::Error>> {
	let mut stdout = stdout();
	stdout.execute(CLEAR)?;
	for (i, ln) in s
		.to_string()
		.lines()
		.filter(|s| !s.is_empty() && s != &"\n")
		.enumerate()
	{
		if i > 0 {
			writeln!(&mut stdout)?;
		}
		write!(&mut stdout, "{}\r", ln)?;
		stdout.flush()?;
	}

	Ok(())
}

fn main() {
	let args = match Args::parse_args() {
		Err(e) => {
			eprintln!("error: {}", e);
			std::process::exit(2);
		}
		Ok(a) => a,
	};

	if let Err(e) = enable_raw_mode() {
		eprintln!("warning: could not enable raw mode: {}", e);
	}

	if let Err(e) = run(args) {
		eprintln!("an internal error occurred: {:?}", e);
		std::process::exit(2);
	}

	if let Err(e) = disable_raw_mode() {
		eprintln!("warning: could not disable raw mode: {}", e);
	}
}
