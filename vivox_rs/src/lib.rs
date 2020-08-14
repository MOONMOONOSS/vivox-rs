#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(async_closure)]

extern crate serde;
extern crate futures;

pub(crate) use vivox_rs_sys::*;

use std::ffi::{CString, CStr};
use std::os::raw::c_int;
use std::time::SystemTime;

pub mod error;
pub mod sessiongroup;
pub mod tokengen;
pub(crate) mod helpers;

use crate::helpers::*;
use crate::sessiongroup::AddSession;
use crate::tokengen::TokenGenerator;

static mut connected: bool = false;
static mut logged_in: bool = false;
static mut generator: TokenGenerator = TokenGenerator::init();

pub fn hello_vivox() {
  use std::{thread, time::Duration};

  println!("Starting Vivox...");
  if init() == VX_E_SUCCESS as i32 {
    println!("Vivox initialized");
  }

  thread::spawn(move || {
    loop {
      poll_loop();
      thread::sleep(Duration::from_millis(100));
    }
  });
  println!("Started poll loop");

  create_connector();
  println!("Created connector");

  println!("Waiting until connector finishes creation...");
  unsafe {
    loop {
      if connected {
        break;
      }
    }
  }

  login();

  println!("Waiting until login completes...");

  unsafe {
    loop {
      if logged_in {
        break;
      }
    }
  }

  println!("Joining echo channel...");
  join_echo();
}

fn init() -> i32 {
  use std::mem;

  unsafe {
    let mut default_config: vx_sdk_config_t = mem::zeroed();
    let mut status = vx_get_default_config3(
      &mut default_config,
      mem::size_of::<vx_sdk_config_t>() as size_t
    );
  
    if status != VX_E_SUCCESS as i32 {
      let err_str = vx_get_error_string(status) as *mut i8;
  
      panic!("vx_get_default_config3() returned {}: {}",
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
  
      panic!("vx_initialize3() returned {}: {}",
        status,
        CString::from_raw(
          err_str
        ).into_string().expect("Unable to get error string!"));
    }

    status
  }
}

struct AnonymousLogin {
  req_ptr: *mut vx_req_account_anonymous_login_t,
}

#[allow(dead_code)]
impl AnonymousLogin {
  pub fn new() -> Self {
    use std::mem;

    unsafe {
      let mut new_req = Self {
        req_ptr: mem::zeroed(),
      };

      vx_req_account_anonymous_login_create(&mut new_req.req_ptr);

      new_req
    }
  }

  pub fn acct_mgmt_server<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).acct_mgmt_server = strdup(input); }

    self
  }

  pub fn acct_name<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).acct_name = strdup(input); }

    self
  }

  pub fn account_handle<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).account_handle = strdup(input); }

    self
  }

  pub fn access_token<'a>(&'a mut self, issuer: &str, domain: &str) -> &'a mut Self {
    unsafe {
      (*self.req_ptr).access_token = strdup(
        &generator.generate(
          "get_your_own!",
          issuer,
          SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Back from the future")
            .as_secs() + 120,
          "login",
          &format!(
            "sip:{}@{}",
            CStr::from_ptr((*self.req_ptr).acct_name).to_str().unwrap(),
            domain,
          ),
          None,
        )
      );
    }

    self
  }

  pub fn application_override<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).application_override = strdup(input); }

    self
  }

  pub fn application_token<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).application_token = strdup(input); }

    self
  }

  pub fn autopost_crash_dumps<'a>(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).autopost_crash_dumps = input; }

    self
  }

  pub fn buddy_management_mode(self) {
    panic!("Reserved for future use")
  }

  pub fn connector_handle<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).connector_handle = strdup(input); }

    self
  }

  pub fn displayname<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).displayname = strdup(input); }

    self
  }

  pub fn enable_social<'a>(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).enable_buddies_and_presence = input; }

    self
  }

  pub fn issue<'a>(&'a mut self) {
    unsafe { vx_issue_request(&mut (*self.req_ptr).base); }
  }

  pub fn languages<'a>(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).languages = strdup(input); }

    self
  }

  pub fn participant_property_frequency<'a>(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).participant_property_frequency = input; }

    self
  }

  pub fn persist_presence<'a>(&'a mut self, input: c_int) -> &'a mut Self {
    unsafe { (*self.req_ptr).enable_presence_persistence = input; }

    self
  }
}

fn login() {
  let act_name = ".gmclvivox-gmvivox-w-dev.dunkel.";

  AnonymousLogin::new()
    .connector_handle("c1")
    .acct_name(act_name)
    .displayname("Dunkel")
    .account_handle(act_name)
    .access_token("gmclvivox-gmvivox-w-dev", "vdx5.vivox.com")
    .issue();
}

fn join_echo() {
  unsafe {
    AddSession::new()
      .sessiongroup_handle("sg1")
      .session_handle("echotest")
      .uri("sip:confctl-e-gmclvivox-gmvivox-w-dev.echotest@vdx5.vivox.com")
      .connect_audio(1)
      .connect_text(1)
      .account_handle(".gmclvivox-gmvivox-w-dev.dunkel.")
      .access_token("gmclvivox-gmvivox-w-dev", "vdx5.vivox.com", &generator)
      .issue();
  }
}

fn poll_loop() {
  use std::mem;

  unsafe {
    let mut m: *mut vx_message_base_t = mem::zeroed();

    loop {
      let status = vx_get_message(&mut m);

      if status == VX_GET_MESSAGE_AVAILABLE as i32 {
        message_handler(m);
        vx_destroy_message(m);
        // Handle to `m` becomes invalid here
        // Vivox has also taken control of this memory at this point
        mem::forget(m);
      }
      else if status == VX_GET_MESSAGE_FAILURE as i32 {
        println!("Message error encountered!");
      }
      else if status == VX_GET_MESSAGE_NO_MESSAGE as i32 {
        break;
      }
    };
  }
}

fn message_handler(msg: *mut vx_message_base_t) {
  use std::mem::transmute;

  unsafe {
    match (*msg).type_ {
      vx_message_type_msg_response => {
        let resp = transmute::<*mut vx_message_base_t, *mut vx_resp_base_t>(&mut *msg);

        response_handler(resp);
      },
      vx_message_type_msg_event => {
        let evt = transmute::<*mut vx_message_base_t, *mut vx_evt_base_t>(&mut *msg);

        event_handler(evt);
      },
      _ => {},
    }
  }
}

fn response_handler(resp: *mut vx_resp_base_t) {
  unsafe {
    if (*resp).return_code == 1 {
      println!("[Vivox] ERROR {}, {}", (*resp).status_code, CStr::from_ptr(vx_get_error_string((*resp).status_code)).to_str().unwrap());

      return;
    }

    match (*resp).type_ {
      vx_response_type_resp_connector_create => {
        println!("Connected");
        connected = true;
      },
      _ => {},
    }
  }
}

#[allow(unused_variables)]
fn event_handler(evt: *mut vx_evt_base_t) {
  use std::mem::transmute;

  unsafe {
    match (*evt).type_ {
      vx_event_type_evt_account_login_state_change => {
        println!("[Vivox] Login state changed");
  
        let act = transmute::<*mut vx_evt_base_t, *mut vx_evt_account_login_state_change>(&mut *evt);

        if (*act).state == vx_login_state_change_state_login_state_logged_in {
          logged_in = true;
        }
      },
      vx_event_type_evt_media_stream_updated => {
        let session = transmute::<*mut vx_evt_base_t, *mut vx_evt_media_stream_updated>(&mut *evt);

        match (*session).state {
          vx_session_media_state_session_media_connected => println!(
            "Connected to voice channel: {}",
            CStr::from_ptr((*session).session_handle)
              .to_str()
              .expect("Not a valid string")
          ),
          vx_session_media_state_session_media_disconnected => {
            match (*session).status_code {
              0 => println!(
                "Disconnected from voice channel: {}",
                CStr::from_ptr((*session).session_handle)
                  .to_str()
                  .expect("Not a valid string")
              ),
              val => {
                println!(
                  "Disconnected from voice channel: {}, error {}: {}",
                  CStr::from_ptr((*session).session_handle)
                    .to_str()
                    .expect("Not a valid string"),
                  (*session).status_code,
                  CStr::from_ptr(vx_get_error_string((*session).status_code))
                    .to_str()
                    .expect("Not a valid string")
                );
              }
            }
          },
          _ => {},
        }
      },
      _ => {},
    }
  }
}

fn create_connector() {
  use std::mem;

  unsafe {
    let mut req: *mut vx_req_connector_create = mem::zeroed();
    
    // Creates default Connector Create request struct
    vx_req_connector_create_create(&mut req);

    (*req).connector_handle = strdup("c1");
    (*req).acct_mgmt_server = strdup("https://vdx5.www.vivox.com/api2");

    vx_issue_request(&mut (*req).base);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sdk_initialization() {
    let status = init();

    unsafe {
      vx_uninitialize();
    }

    assert_eq!(status, (VX_E_SUCCESS as i32));
  }
}
