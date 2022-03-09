use clap::{arg, crate_version, Arg, Command};

const FOOTER: &str =
	"For the configuration file syntax, visit https://github.com/insomnimus/midnote";

pub fn new() -> Command<'static> {
	Command::new("midnote")
		.about("View and play notes in a MIDI track.")
		.after_long_help(FOOTER)
		.version(crate_version!())
		.args(&[
			arg!(-c --config [PATH] "Path to a config file (*.json)."),
			arg!(-C --"no-color" "Do not use colored output."),
			arg!(-l --list "List available MIDI output devices."),
			arg!(-d --device <NO> "The MIDI output device.")
				.default_value("0")
				.validator(|s| {
					s.parse::<usize>().map(|_| {}).map_err(|_| {
						String::from("the value must be an integer greater than or equal to 0")
					})
				}),
			Arg::new("file")
				.help("The midi file to inspect.")
				.required_unless_present("list"),
		])
}
