use std::ffi::CString;
use std::os::raw::c_char;
use vivox_rs_sys::*;

pub(crate) fn strdup(input: &str) -> *mut c_char {
  unsafe {
    vx_strdup(CString::new(input)
      .expect("Unable to allocate string")
      .as_ptr()
    )
  }
}

pub(crate) fn is_valid_non_alphanumeric(x: &char) -> bool {
  "-_.!~*'()&=+$,;?/".chars().any(|y| y == *x)
}
