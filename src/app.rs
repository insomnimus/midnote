use clap::{arg, crate_version, App, Arg};

const FOOTER: &str =
	"For the configuration file syntax, please visit https://github.com/insomnimus/midnote";

pub fn new() -> App<'static> {
	App::new("midnote")
		.about("View and play notes in a MIDI track.")
		.after_long_help(FOOTER)
		.version(crate_version!())
		.args(&[
			arg!(-c --config [PATH] "Path to a config file (*.json)."),
			Arg::new("no-color")
				.short('C')
				.long("no-color")
				.help("Do not use colored output."),
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
