use t_rust_less_lib::api::{PROPERTY_NOTES, PROPERTY_PASSWORD, PROPERTY_TOTP_URL, PROPERTY_USERNAME};

pub fn property_label(name: &str) -> String {
  match name {
    PROPERTY_PASSWORD => "Password".to_string(),
    PROPERTY_USERNAME => "Username".to_string(),
    PROPERTY_TOTP_URL => "OTP".to_string(),
    PROPERTY_NOTES => "Notes".to_string(),
    "sid" => "SID".to_string(),
    _ => name.to_string(),
  }
}
