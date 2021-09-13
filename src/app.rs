use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("midnote")
		.about("View and play notes in a MIDI track.")
		.setting(AppSettings::UnifiedHelpMessage)
		.version(crate_version!());

	let chunks = Arg::new("chunks")
		.short('n')
		.long("chunks")
		.about("Chunks per page.")
		.default_value("1")
		.validator(|s| match s.parse::<usize>() {
			Err(_) | Ok(0) => Err(String::from("the value must be a positive integer")),
			_ => Ok(()),
		});

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

	app.arg(chunks).arg(list).arg(device).arg(file)
}
