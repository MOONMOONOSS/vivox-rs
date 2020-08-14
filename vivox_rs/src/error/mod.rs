use std::ffi::CStr;
use std::fmt;
use vivox_rs_sys::vx_get_error_string;

#[repr(i32)]
pub enum AccessTokenError {
  ALREADY_USED =          20_120,
  CLAIMS_MISMATCH =       20_123,
  EXPIRED =               20_121,
  INTERNAL_ERROR =        20_125,
  INVALID_SIGNATURE =     20_122,
  ISSUER_MISMATCH =       20_128,
  MALFORMED =             20_124,
  SERVICE_UNAVAILABLE =   20_127,
}

#[repr(i32)]
pub enum VivoxError {
  ACCOUNT_MISCONFIGURED =                           1_059,
  ALREADY_EXIST =                                   1_101,
  ALREADY_INITIALIZED =                             1_085,
  ALREADY_LOGGED_IN =                               1_005,
  ALREADY_LOGGED_OUT =                              1_006,
  BUFSIZE =                                         1_042,
  CALL_CREATION_FAILED =                            1_057,
  CALL_TERMINATED_BAN =                             1_098,
  CALL_TERMINATED_BY_SERVER =                       1_100,
  CALL_TERMINATED_KICK =                            1_099,
  CALL_TERMINATED_NO_ANSWER_LOCAL =                 1_096,
  CALL_TERMINATED_NO_RTP_RXED =                     1_095,
  CAPACITY_EXCEEDED =                               1_084,
  CAPTURE_DEVICE_IN_USE =                           7_006,
  CHANNEL_URI_REQUIRED =                            1_076,
  CHANNEL_URI_TOO_LONG =                            1_097,
  CROSS_DOMAIN_LOGINS_DISABLED =                    1_081,
  DEPRECATED =                                      1_106,
  FAILED =                                          1_004,
  FAILED_TO_CONNECT_TO_SERVER =                     10_007,
  FAILED_TO_CONNECT_TO_VOICE_SERVICE =              1_072,
  FAILED_TO_SEND_REQUEST_TO_VOICE_SERVICE =         1_073,
  FEATURE_DISABLED =                                1_102,
  FILE_CORRUPT =                                    1_044,
  FILE_OPEN_FAILED =                                1_043,
  FILE_WRITE_FAILED =                               1_045,
  FILE_WRITE_FAILED_REACHED_MAX_FILESIZE =          1_049,
  HANDLE_ALREADY_TAKEN =                            1_090,
  HANDLE_IS_RESERVED =                              1_091,
  INSUFFICIENT_PRIVILEGE =                          1_010,
  INVALID_APP_TOKEN =                               1_083,
  INVALID_ARGUMENT =                                1_008,
  INVALID_AUTH_TOKEN =                              1_082,
  INVALID_CAPTURE_DEVICE_FOR_REQUESTED_OPERATION =  1_077,
  INVALID_CAPTURE_DEVICE_SPECIFIER =                7_005,
  INVALID_SDK_HANDLE =                              1_071,
  INVALID_SESSION_STATE =                           1_019,
  INVALID_SUBSCRIPTION_RULE_TYPE =                  1_038,
  INVALID_USERNAME_OR_PASSWORD =                    1_009,
  INVALID_XML =                                     1_000,
  LOGIN_FAILED =                                    1_014,
  LOOP_MODE_RECORDING_NOT_ENABLED =                 1_078,
  MAX_NUM_OF_CALLS_EXCEEDED =                       1_060,
  MAX_CONNECTOR_LIMIT_EXCEEDED =                    1_002,
  MAX_HTTP_DATA_RESPONSE_SIZE_EXCEEDED =            1_075,
  MAX_LOGINS_PER_USER_EXCEEDED =                    1_074,
  MAX_PLAYBACK_SESSIONGROUPS_EXCEEDED =             1_051,
  MAX_SESSION_LIMIT_EXCEEDED =                      1_003,
  MEDIA_DISCONNECT_NOT_ALLOWED =                    1_035,
  NETWORK_ADDRESS_CHANGE =                          1_087,
  NETWORK_DOWN =                                    1_088,
  NOT_IMPL =                                        1_017,
  NOT_INITIALIZED =                                 1_012,
  NOT_LOGGED_IN =                                   1_007,
  NOT_UNINITIALIZED_YET =                           1_086,
  NO_CAPTURE_DEVICES_FOUND =                        7_002,
  NO_EXIST =                                        1_001,
  NO_MESSAGE_AVAILABLE =                            -1,
  NO_MORE_FRAMES =                                  1_047,
  NO_RENDER_DEVICES_FOUND =                         7_001,
  NO_SESSION_PORTS_AVAILABLE =                      1_061,
  NO_SUCH_SESSION =                                 1_011,
  NO_XLSP_CONFIGURED =                              1_092,
  POWER_STATE_CHANGE =                              1_089,
  PRELOGIN_INFO_NOT_RETURNED =                      1_036,
  RECORDING_ALREADY_ACTIVE =                        1_068,
  RECORDING_LOOP_BUFFER_EMPTY =                     1_069,
  RENDER_CONTEXT_DOES_NOT_EXIST =                   1_065,
  RENDER_DEVICE_DOES_NOT_EXIST =                    1_064,
  RENDER_DEVICE_IN_USE =                            7_004,
  RENDER_SOURCE_DOES_NOT_EXIST =                    1_067,
  REQUESTCONTEXT_NOT_FOUND =                        1_013,
  REQUEST_CANCELED =                                1_094, // Oh no no no
  REQUEST_CANCELLED =                               1_018, // Oh he tried it
  REQUEST_NOT_SUPPORTED =                           1_033, // Look at his lips
  REQUEST_TYPE_NOT_SUPPORTED =                      1_032, // Man the back of his neck look like a serloin steak burger
  RTP_SESSION_SOCKET_ERROR =                        1_104,
  SESSIONGROUP_NOT_FOUND =                          1_031,
  SESSIONGROUP_TRANSMIT_NOT_ALLOWED =               1_056,
  SESSION_CHANNEL_TEXT_DENIED =                     1_022,
  SESSION_CREATE_PENDING =                          1_020,
  SESSION_DOES_NOT_HAVE_AUDIO =                     1_028,
  SESSION_DOES_NOT_HAVE_TEXT =                      1_027,
  SESSION_IS_NOT_3D =                               1_030,
  SESSION_MAX =                                     1_015,
  SESSION_MEDIA_CONNECTION_FAILED =                 1_026,
  SESSION_MSG_BUILD_FAILED =                        1_024,
  SESSION_MSG_CONTENT_TYPE_FAILED =                 1_025,
  SESSION_MUST_HAVE_MEDIA =                         1_029,
  SESSION_TERMINATE_PENDING =                       1_021,
  SESSION_TEXT_DENIED =                             1_023,
  SESSION_TEXT_DISABLED =                           1_055,
  SIP_BACKEND_REQUIRED =                            1_105,
  SIZE_LIMIT_REACHED =                              1_103,
  STREAM_READ_FAILED =                              1_070,
  SUBSCRIPTION_NOT_FOUND =                          1_037,
  SUCCESS =                                         0,
  TERMINATE_SESSION_NOT_FOUND =                     1_050,
  TEXT_CONNECT_NOT_ALLOWED =                        1_053,
  TEXT_DISABLED =                                   1_079,
  TEXT_DISCONNECT_NOT_ALLOWED =                     1_052,
  UNABLE_TO_OPEN_CAPTURE_DEVICE =                   7_009,
  VOICE_FONT_NOT_FOUND =                            1_080,
  WRONG_CONNECTOR =                                 1_016,
  XMPP_BACKEND_REQUIRED =                           5_023,
  XNETCONECT_FAILED =                               1_093,
}

impl fmt::Debug for VivoxError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("VivoxError")
      .field("error_code", &(*self as i32))
      .field("explanation", format!("{}", self))
      .finish()
  }
}

impl fmt::Display for VivoxError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let err = vx_get_error_string(*self as i32) as *mut i8;

    let conv_str = unsafe { CStr::from_ptr(err).to_str().unwrap() };

    write!(f, "{}", conv_str)
  }
}
