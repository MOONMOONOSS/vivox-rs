use crate::helpers::*;
use crate::tokengen::TokenGenerator;

use std::ffi::CStr;
use std::os::raw::c_int;
use std::time::SystemTime;
use vivox_rs_sys::*;

pub struct AddSession<'u> {
  req_ptr: *mut vx_req_sessiongroup_add_session,
  uri: &'u str,
}

#[allow(dead_code)]
impl<'a> AddSession<'a> {
  pub fn new() -> Self {
    use std::mem;

    let mut new_req: Self;

    unsafe {
      new_req = Self {
        req_ptr: mem::zeroed(),
        uri: "",
      };

      vx_req_sessiongroup_add_session_create(&mut new_req.req_ptr);
    }

    new_req
  }

  pub fn account_handle(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).account_handle = strdup(input); }

    self
  }

  pub fn access_token(
    &'a mut self,
    issuer: &str,
    domain: &str,
    generator: &TokenGenerator,
  ) -> &'a mut Self
  {
    if self.uri == "" {
      panic!("uri not set prior to requesting access token!");
    }

    unsafe {
      (*self.req_ptr).access_token = strdup(
        &generator.generate(
          "get_your_own!",
          issuer,
          SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Back from the future")
            .as_secs() + 120,
          "join",
          &format!(
            "sip:{}@{}",
            CStr::from_ptr((*self.req_ptr).account_handle).to_str().unwrap(),
            domain,
          ),
          Some(self.uri.to_string()),
        )
      );
    }

    self
  }

  pub fn connect_audio(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).connect_audio = input; }

    self
  }

  pub fn connect_text(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).connect_text = input; }

    self
  }

  pub fn issue(&'a mut self) {
    unsafe { vx_issue_request(&mut (*self.req_ptr).base); }
  }

  pub fn jitter_compensation(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).jitter_compensation = input; }

    self
  }

  pub fn name(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).name = strdup(input); }

    self
  }

  pub fn password(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).password = strdup(input); }

    self
  }

  pub fn password_hash_algorithm(
    &'a mut self,
    input: vx_password_hash_algorithm_t
  ) -> &'a mut Self {
    unsafe { (*self.req_ptr).password_hash_algorithm = input; }

    self
  }

  pub fn session_font_id(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).session_font_id = input; }

    self
  }

  pub fn session_handle(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).session_handle = strdup(input); }

    self
  }

  pub fn sessiongroup_handle(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).sessiongroup_handle = strdup(input); }

    self
  }

  pub fn uri(&'a mut self, input: &'a str) -> &'a mut Self {
    // Tests without "sip:" and "@vdx5..."
    let start = input.find(':').unwrap_or(0) + 1;
    let end = input.find('@').unwrap_or(1) - 1;
    let sub = &input[start..end];

    let test = sub
      .chars()
      .all(|c|
        c.is_alphanumeric()
        || is_valid_non_alphanumeric(&c)
      )
      && sub.len() <= MAX_CHANNEL_URI_LENGTH as usize;

    if !test {
      panic!("URI validation failed!");
    }

    // Store locally for access_token()
    self.uri = input;

    unsafe { (*self.req_ptr).uri = strdup(input); }

    self
  }
}
