use device_query::{DeviceQuery, DeviceState as DevState, Keycode};
use std::fmt;

/* State of the mouse and keyboard */
pub struct DeviceState {
	state: DevState,
	mouse: Vec<bool>,
	keys: Vec<Keycode>,
}

impl fmt::Debug for DeviceState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("DeviceState")
			.field("mouse", &self.mouse)
			.field("keys", &self.keys)
			.finish()
	}
}

impl DeviceState {
	/**
	 * Create a new DeviceState object.
	 *
	 * @return DeviceState
	 */
	pub fn new() -> Self {
		Self {
			state: DevState::new(),
			mouse: Vec::new(),
			keys: Vec::new(),
		}
	}

	/**
	 * Check if the mouse left/right buttons are clicked.
	 *
	 * @return bool
	 */
	pub fn check_mouse_clicked(&mut self) -> bool {
		self.mouse = self.state.get_mouse().button_pressed;
		self.mouse[1] || self.mouse[3]
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_pressed(&mut self) -> bool {
		self.keys = self.state.get_keys();
		self.keys.contains(&Keycode::Escape)
			|| (self.keys.contains(&Keycode::LControl)
				&& self.keys.contains(&Keycode::D))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_device_mod() {
		let mut device_state = DeviceState::new();
		device_state.update();
		assert!(!device_state.check_mouse_clicked());
		assert!(!device_state.check_cancel_keys_pressed());
	}
}
