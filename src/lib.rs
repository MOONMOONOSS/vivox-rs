#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use super::*;
  use std::ffi::CString;
  use std::mem;

  #[test]
  fn sdk_initialization() {
    unsafe {
      let mut default_config: vx_sdk_config_t = mem::zeroed();
      let mut status = vx_get_default_config3(
        &mut default_config,
        mem::size_of::<vx_sdk_config_t>() as size_t
      );

      if status != VX_E_SUCCESS as i32 {
        let err_str = vx_get_error_string(status) as *mut i8;

        println!("vx_get_default_config3() returned {}: {}",
          status,
          CString::from_raw(
            err_str
          ).into_string().expect("Unable to get error string!"));
      }

      status = vx_initialize3(
        &mut default_config,
        mem::size_of::<vx_sdk_config_t>() as size_t
      );

      if status != VX_E_SUCCESS as i32 {
        let err_str = vx_get_error_string(status) as *mut i8;

        println!("vx_initialize3() returned {}: {}",
          status,
          CString::from_raw(
            err_str
          ).into_string().expect("Unable to get error string!"));
      }

      assert_eq!(status, (VX_E_SUCCESS as i32));
    }
  }
}
