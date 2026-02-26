#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Volume(f32);

impl Volume {
	pub fn new(n: f32) -> Option<Self> {
		if (0.0..=1.0).contains(&n) { Some(Volume(n)) } else { None }
	}

	pub fn as_percent(self: &Volume) -> f32 {
		self.0 * 100.0f32
	}
}

impl From<Volume> for f32 {
	fn from(value: Volume) -> Self {
		value.0
	}
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// ----------- VALID values

	#[test]
	fn test_valid_zero() {
		assert_eq!(Volume::new(0.0), Some(Volume(0.0)));
	}

	#[test]
	fn test_valid_zero_point_one() {
		assert_eq!(Volume::new(0.1), Some(Volume(0.1)));
	}

	#[test]
	fn test_valid_zero_point_five() {
		assert_eq!(Volume::new(0.5), Some(Volume(0.5)));
	}

	#[test]
	fn test_valid_zero_point_nine() {
		assert_eq!(Volume::new(0.9), Some(Volume(0.9)));
	}

	#[test]
	fn test_valid_one_point_zero() {
		assert_eq!(Volume::new(1.0), Some(Volume(1.0)));
	}

	// ----------- INVALID values

	#[test]
	fn test_bad_lt_negative_one() {
		assert_eq!(Volume::new(-1.2), None);
	}

	#[test]
	fn test_bad_gt_one() {
		assert_eq!(Volume::new(1.2), None);
	}

	#[test]
	fn test_bad_f32_min() {
		assert_eq!(Volume::new(f32::MIN), None);
	}

	#[test]
	fn test_bad_f32_max() {
		assert_eq!(Volume::new(f32::MAX), None);
	}

	#[test]
	fn test_bad_f32_nan() {
		assert_eq!(Volume::new(f32::NAN), None);
	}
}
