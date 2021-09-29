use std::borrow::Cow;

use nodi::{Event, Moment, Sheet, Ticker, Timer};

pub struct Bar {
	pub timer: Ticker,
	pub moments: Vec<Moment>,
}

pub fn bars(sheet: Sheet, tpb: u16) -> Vec<Bar> {
	let mut timer = Ticker::new(tpb);
	let mut buf = Vec::new();
	for bar in sheet.into_bars(tpb) {
		let t = timer;
		// check if we have a tempo event
		for m in &bar {
			match &m {
				Moment::Empty => (),
				Moment::Events(events) => {
					for e in events {
						if let Event::Tempo(n) = e {
							timer.change_tempo(*n);
						}
					}
				}
			}
		}
		buf.push(Bar {
			timer: t,
			moments: bar,
		});
	}
	buf
}

impl<'a> Bar {
	pub fn trim_moments(&self) -> &[Moment] {
		let start = self.moments.iter().take_while(|m| m.is_empty()).count();
		let slice = &self.moments[start..];
		let end = slice.iter().rev().take_while(|m| m.is_empty()).count();
		&slice[..(slice.len() - end)]
	}

	pub fn transposed_moments(&'a self, n: i8) -> Cow<'a, [Moment]> {
		if n == 0 {
			self.trim_moments().into()
		} else {
			let mut moments = self.trim_moments().to_vec();
			for m in &mut moments {
				m.transpose(n, false);
			}
			moments.into()
		}
	}
}
