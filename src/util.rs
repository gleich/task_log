use chrono::Duration;

pub fn format_duration(duration: Duration) -> String {
	let nano_duration = duration.num_nanoseconds().unwrap();
	let suffix;
	let factor;
	if nano_duration < 1000 {
		suffix = "ns";
		factor = 1.0;
	} else if nano_duration < 1e6 as i64 {
		suffix = "µs";
		factor = 1000.0;
	} else if nano_duration < 1e9 as i64 {
		suffix = "ms";
		factor = 1e6;
	} else if nano_duration < 6e10 as i64 {
		suffix = "s ";
		factor = 1e9;
	} else if nano_duration < 3.60e12 as i64 {
		suffix = "m ";
		factor = 6e10;
	} else if nano_duration < 8.64e13 as i64 {
		suffix = "h ";
		factor = 3.60e12;
	} else if nano_duration < 6.048e14 as i64 {
		suffix = "d ";
		factor = 8.64e13
	} else if nano_duration < 2.628e15 as i64 {
		suffix = "w ";
		factor = 6.048e14;
	} else if nano_duration < 3.154e16 as i64 {
		suffix = "mo";
		factor = 2.628e15;
	} else {
		suffix = "y ";
		factor = 3.154e16;
	}
	format!("{}{}", nano_duration / factor as i64, suffix)
}

#[cfg(test)]
mod tests {
	use std::time::Duration;

	use anyhow::Result;

	use crate::util::format_duration;

	#[test]
	fn duration_formatter() -> Result<()> {
		assert_eq!(
			String::from("0ns"),
			format_duration(chrono::Duration::from_std(Duration::from_secs(0))?)
		);
		for i in 1..7 {
			assert_eq!(
				format!("{}ns", i),
				format_duration(chrono::Duration::from_std(Duration::from_nanos(i))?)
			);
			assert_eq!(
				format!("{}µs", i),
				format_duration(chrono::Duration::from_std(Duration::from_micros(i))?)
			);
			assert_eq!(
				format!("{}ms", i),
				format_duration(chrono::Duration::from_std(Duration::from_millis(i))?)
			);
			assert_eq!(
				format!("{}s ", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(i))?)
			);
			assert_eq!(
				format!("{}m ", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(i * 60))?)
			);
			assert_eq!(
				format!("{}h ", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(i * 3600))?)
			);
			assert_eq!(
				format!("{}d ", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(i * 86400))?)
			);
			assert_eq!(
				format!("{}mo", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(
					i * 2.628e6 as u64
				))?)
			);
			assert_eq!(
				format!("{}y ", i),
				format_duration(chrono::Duration::from_std(Duration::from_secs(
					i * 3.154e7 as u64
				))?)
			);
		}
		Ok(())
	}
}
