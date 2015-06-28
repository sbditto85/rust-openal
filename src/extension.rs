use std::ffi::CString;

use ffi::*;

pub fn is_supported(name: &str) -> bool {
	unsafe {
		alIsExtensionPresent(CString::new(name).unwrap().as_ptr()) == AL_TRUE
	}
}

pub mod device {
	use std::ptr;
	use std::ffi::CString;

	use ffi::*;
	use ::traits::Device;

	pub fn is_supported(name: &str) -> bool {
		unsafe {
			alcIsExtensionPresent(ptr::null(), CString::new(name).unwrap().as_ptr()) == AL_TRUE
		}
	}

	pub fn is_supported_by<T: Device>(device: &T, name: &str) -> bool {
		unsafe {
			alcIsExtensionPresent(device.as_ptr(), CString::new(name).unwrap().as_ptr()) == AL_TRUE
		}
	}
}
