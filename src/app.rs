use clap::{crate_version, App, AppSettings, Arg};

const FOOTER: &str =
	"For the configuration file syntax, please visit https://github.com/insomnimus/midnote";

pub fn new() -> App<'static> {
	let app = App::new("midnote")
		.about("View and play notes in a MIDI track.")
		.after_long_help(FOOTER)
		.setting(AppSettings::UnifiedHelpMessage)
		.version(crate_version!());

	let config = Arg::new("config")
		.short('c')
		.long("config")
		.about("Path of a config file (*.json).")
		.takes_value(true);

	let no_color = Arg::new("no-color")
		.short('C')
		.long("no-color")
		.about("Do not use colored output.");

	let list = Arg::new("list")
		.short('l')
		.long("list")
		.about("List available midi output devices.");

	let file = Arg::new("file")
		.about("The midi file to inspect.")
		.required_unless_present("list");

	let device = Arg::new("device")
		.short('d')
		.long("device")
		.about("The midi device for audio playback.")
		.default_value("0")
		.validator(|s| {
			s.parse::<usize>()
				.map(|_| {})
				.map_err(|_| String::from("the value must be a non-negative integer"))
		});

	app.arg(no_color)
		.arg(config)
		.arg(list)
		.arg(device)
		.arg(file)
}
