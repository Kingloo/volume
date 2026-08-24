#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Volume(f32);

#[derive(Debug, Clone)]
pub enum VolumeError {
	OutOfRange(f32),
	ParseFailed(String),
}

impl Volume {
	pub fn new(n: f32) -> Result<Self, VolumeError> {
		if (0.0..=1.0).contains(&n) {
			Ok(Volume(n))
		} else {
			Err(VolumeError::OutOfRange(n))
		}
	}

	pub fn as_percent(self: &Volume) -> f32 {
		self.0 * 100.0f32
	}

	pub fn add(self: &Volume, rhs: f32) -> Result<Self, VolumeError> {
		Volume::new((self.0 + rhs).clamp(0.0f32, 1.0f32))
	}

	pub fn sub(self: &Volume, rhs: f32) -> Result<Self, VolumeError> {
		Volume::new((self.0 - rhs).clamp(0.0f32, 1.0f32))
	}
}

impl From<Volume> for f32 {
	fn from(value: Volume) -> Self {
		value.0
	}
}

impl TryFrom<f32> for Volume {
	type Error = VolumeError;

	fn try_from(value: f32) -> Result<Self, Self::Error> {
		if (0.0..=1.0).contains(&value) {
			Ok(Volume(value))
		} else {
			Err(VolumeError::OutOfRange(value))
		}
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
		assert!(Volume::new(0.0).is_ok());
	}

	#[test]
	fn test_valid_zero_point_one() {
		assert!(Volume::new(0.1).is_ok());
	}

	#[test]
	fn test_valid_zero_point_five() {
		assert!(Volume::new(0.5).is_ok());
	}

	#[test]
	fn test_valid_zero_point_nine() {
		assert!(Volume::new(0.9).is_ok());
	}

	#[test]
	fn test_valid_one_point_zero() {
		assert!(Volume::new(1.0).is_ok());
	}

	// ----------- INVALID values

	#[test]
	fn test_bad_lt_negative_one() {
		assert!(Volume::new(-1.2).is_err());
	}

	#[test]
	fn test_bad_gt_one() {
		assert!(Volume::new(1.2).is_err());
	}

	#[test]
	fn test_bad_f32_min() {
		assert!(Volume::new(f32::MIN).is_err());
	}

	#[test]
	fn test_bad_f32_max() {
		assert!(Volume::new(f32::MAX).is_err());
	}

	#[test]
	fn test_bad_f32_nan() {
		assert!(Volume::new(f32::NAN).is_err());
	}
}
