mod helpers;

use std::{
	error::Error,
	fs,
	sync::mpsc::{
		self,
		Receiver,
	},
};

use midly::{
	Format,
	Smf,
	Timing,
};
use nodi::Sheet;

use crate::{
	app,
	config::Config,
	player::Player,
	Response,
};

pub struct Args {
	pub config: Config,
	pub player: Player,
	pub response: Receiver<Response>,
}

impl Args {
	pub fn parse_args() -> Result<Self, Box<dyn Error>> {
		let m = app::new().get_matches();
		if m.is_present("list") {
			helpers::list_devices()?;
			std::process::exit(0);
		}

		let mut config = m
			.value_of("config")
			.map(Config::read_from)
			.unwrap_or_else(|| Ok(Config::default()))?;

		if m.is_present("no-color") {
			config.colors = false;
		}

		let data = m.value_of("file").map(fs::read).unwrap()?;
		let device_no = m.value_of("device").unwrap().parse::<usize>()?;
		let con = helpers::get_midi(device_no)?;

		let Smf { tracks, header } = Smf::parse(&data)?;

		let tpb = match header.timing {
			Timing::Metrical(n) => u16::from(n),
			_ => return Err("the midi file has an unsupported time format".into()),
		};

		let (sender, receiver) = mpsc::channel();

		let sheet = match header.format {
			Format::Parallel => {
				let meta = Sheet::single(&tracks[0]);
				let mut sheet = helpers::choose_track(&tracks[0..]);
				sheet.merge_with(meta);
				sheet
			}
			_ => Sheet::sequential(&tracks),
		};

		let player = Player::new(con, sender, sheet.into_bars(tpb).collect(), tpb);

		Ok(Self {
			player,
			response: receiver,
			config,
		})
	}
}
