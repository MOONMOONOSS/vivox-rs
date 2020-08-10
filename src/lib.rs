#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(async_closure)]

extern crate serde;
extern crate futures;

use serde::{Serialize, Deserialize};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

static mut connected: bool = false;

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

  println!("Logging in...");
  login();
}

fn init() -> i32 {
  use std::ffi::CString;
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

fn login() {
  use std::ffi::CString;
  use std::mem;
  use std::time::SystemTime;

  unsafe {
    let mut req: *mut vx_req_account_anonymous_login_t = mem::zeroed();
    vx_req_account_anonymous_login_create(&mut req);

    (*req).connector_handle = vx_strdup(
      CString::new("c1")
        .expect("Unable to allocate string")
        .as_ptr()
    );

    (*req).acct_name = vx_strdup(
      CString::new(".gmclvivox-gmvivox-w-dev.dunkel.")
        .expect("Unable to allocate string")
        .as_ptr()
    );

    (*req).displayname = vx_strdup(
      CString::new("Dunkel")
        .expect("Unable to allocate string")
        .as_ptr()
    );

    (*req).account_handle = vx_strdup((*req).acct_name);
    (*req).access_token = vx_strdup(
      CString::new(
        vx_generate_token(
          "get_your_own_key!",
          "gmclvivox-gmvivox-w-dev",
          SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Back from the future")
            .as_secs() + 120,
          "login",
          1,
          "sip:.gmclvivox-gmvivox-w-dev.dunkel.@vdx5.vivox.com",
          None,
        )
      )
        .expect("Unable to allocate string")
        .as_ptr()
    );

    vx_issue_request(&mut (*req).base);
  }
}

fn poll_loop() {
  use std::mem;

  unsafe {
    let mut m: *mut vx_message_base_t = mem::zeroed();

    loop {
      let status = vx_get_message(&mut m);

      if status == VX_GET_MESSAGE_AVAILABLE as i32 {
        println!("MSG AVAILABLE!");
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
  use std::ffi::CStr;
  println!("RESP");

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

fn event_handler(evt: *mut vx_evt_base_t) {
  use std::mem::transmute;

  println!("EVT");

  unsafe {
    match (*evt).type_ {
      evt_account_login_state_change => {
        println!("[Vivox] Login state changed");
  
        let act = transmute::<*mut vx_evt_base_t, *mut vx_evt_account_login_state_change>(&mut *evt);

        if (*act).state == vx_login_state_change_state_login_state_logged_in {
          println!("I have logged in!");
        }
      }
    }
  }
}

fn create_connector() {
  use std::ffi::CString;
  use std::mem;

  unsafe {
    let mut req: *mut vx_req_connector_create = mem::zeroed();
    
    // Creates default Connector Create request struct
    vx_req_connector_create_create(&mut req);

    (*req).connector_handle = vx_strdup(
      CString::new("c1")
        .expect("Unable to allocate string")
        .as_ptr()
    );
    (*req).acct_mgmt_server = vx_strdup(
      CString::new("https://vdx5.www.vivox.com/api2")
        .expect("Unable to allocate string")
        .as_ptr()
    );

    vx_issue_request(&mut (*req).base);
  }
}

#[derive(Serialize, Deserialize)]
struct TokenRequest {
  iss: String,
  exp: u64,
  vxa: String,
  vxi: u32,
  t: Option<String>,
  f: String,
}

fn vx_generate_token(
  key: &str,
  issuer: &str,
  exp: u64,
  vxa: &str,
  vxi: u32,
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
    vxi,
    t,
    f: f.to_string(),
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

  // Combine header and payload with signature
  println!("{}.{}", to_sign, signed_payload);

  format!("{}.{}", to_sign, signed_payload)
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
