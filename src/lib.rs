#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(async_closure)]

extern crate serde;
extern crate futures;

use serde::{Serialize, Deserialize};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::time::SystemTime;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

fn strdup(input: &str) -> *mut c_char {
  unsafe {
    vx_strdup(CString::new(input)
      .expect("Unable to allocate string")
      .as_ptr()
    )
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

#[derive(Debug, Copy, Clone)]
struct TokenGenerator {
  req_index: u64,
}

impl TokenGenerator {
  pub const fn init() -> Self {
    Self {
      req_index: 0,
    }
  }

  pub fn generate(
    mut self,
    key: &str,
    issuer: &str,
    exp: u64,
    vxa: &str,
    f: &str,
    t: Option<String>,
  ) -> String {
    use data_encoding::BASE64URL_NOPAD;
    use hmac::{Hmac, Mac, NewMac};
    use sha2::Sha256;
    // Header is static - base64url encoded {}
    let header = BASE64URL_NOPAD.encode(b"{}");
  
    // Create payload and base64 encode
    let tr = TokenRequest {
      iss: issuer.to_string(),
      exp,
      vxa: vxa.to_string(),
      vxi: self.req_index,
      f: f.to_string(),
      t,
    };
    let mut obj = serde_json::to_string(&tr)
      .expect("Unable to serialize object!");
  
    obj = BASE64URL_NOPAD.encode(obj.as_bytes());
  
    // Join segments to prepare for signing
    let to_sign = format!("{}.{}", header, obj);
  
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;
  
    let mut mac = HmacSha256::new_varkey(key.as_bytes())
      .expect("HMAC can take key of any size");
  
    mac.update(to_sign.as_bytes());
  
    let res = mac.finalize();
  
    // Sign token with key and HMACSHA256, then base64 encode
    let signed_payload = BASE64URL_NOPAD.encode(&res.into_bytes());

    // Token has been generated. Increment counter

    self.req_index += 1;
  
    // Combine header and payload with signature
    format!("{}.{}", to_sign, signed_payload)
  }  
}

struct AddSession<'u> {
  req_ptr: *mut vx_req_sessiongroup_add_session,
  uri: &'u str,
}

#[allow(dead_code)]
impl<'a> AddSession<'a> {
  pub fn new() -> Self {
    use std::mem;

    unsafe {
      let mut new_req = Self {
        req_ptr: mem::zeroed(),
        uri: "",
      };

      vx_req_sessiongroup_add_session_create(&mut new_req.req_ptr);

      new_req
    }
  }

  pub fn account_handle(&'a mut self, input: &str) -> &'a mut Self {
    unsafe { (*self.req_ptr).account_handle = strdup(input); }

    self
  }

  pub fn access_token(
    &'a mut self,
    issuer: &str,
    domain: &str,
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

fn is_valid_non_alphanumeric(x: &char) -> bool {
  "-_.!~*'()&=+$,;?/".chars().any(|y| y == *x)
}

fn join_echo() {
  AddSession::new()
    .sessiongroup_handle("sg1")
    .session_handle("echotest")
    .uri("sip:confctl-e-gmclvivox-gmvivox-w-dev.echotest@vdx5.vivox.com")
    .connect_audio(1)
    .connect_text(1)
    .account_handle(".gmclvivox-gmvivox-w-dev.dunkel.")
    .access_token("gmclvivox-gmvivox-w-dev", "vdx5.vivox.com")
    .issue();
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

#[derive(Serialize, Deserialize)]
struct TokenRequest {
  iss: String,
  exp: u64,
  vxa: String,
  vxi: u64,
  f: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  t: Option<String>,
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
