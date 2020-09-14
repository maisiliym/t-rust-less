use iced::Element;

use crate::{property_password_view::PropertyPasswordView, property_totp_view::PropertyTOPTView};
use crate::{property_simple_view::PropertySimpleView, unlocked::UnlockedMessage};
use t_rust_less_lib::{
  api::{PROPERTY_PASSWORD, PROPERTY_TOTP_URL},
  otp::OTPAuthUrl,
  otp::OTPType,
};

#[derive(Debug)]
pub enum PropertyView {
  Simple(PropertySimpleView),
  Password(PropertyPasswordView),
  TOTP(PropertyTOPTView),
}

impl PropertyView {
  pub fn new(block_id: &str, name: &str, value: &str) -> PropertyView {
    match name {
      PROPERTY_PASSWORD => PropertyView::Password(PropertyPasswordView::new(block_id, name, value)),
      PROPERTY_TOTP_URL => match OTPAuthUrl::parse(value) {
        Ok(url) => match url.otp_type {
          OTPType::TOTP { period } => PropertyView::TOTP(PropertyTOPTView::new(block_id, name, url, period)),
          _ => PropertyView::Simple(PropertySimpleView::new(block_id, name, value)),
        },
        _ => PropertyView::Simple(PropertySimpleView::new(block_id, name, value)),
      },
      _ => PropertyView::Simple(PropertySimpleView::new(block_id, name, value)),
    }
  }

  pub fn reveal_property(&mut self, reveal: bool, property_name: &str) {
    if let PropertyView::Password(view) = self {
      view.set_revealed(reveal, property_name);
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    match self {
      PropertyView::Simple(view) => view.view(),
      PropertyView::Password(view) => view.view(),
      PropertyView::TOTP(view) => view.view(),
    }
  }
}
