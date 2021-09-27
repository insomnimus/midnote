use nodi::{
	Event,
	Moment,
	Sheet,
};

pub fn extract_meta_events(sheet: &Sheet) -> Sheet {
	let mut sheet = sheet.clone();
	for m in sheet.iter_mut() {
		match m {
			Moment::Empty => {}
			Moment::Events(events) => {
				events.retain(|e| !matches!(e, Event::Midi { .. }));
				if events.is_empty() {
					*m = Moment::Empty;
				}
			}
		}
	}

	sheet
}
