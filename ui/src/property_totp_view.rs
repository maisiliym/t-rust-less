use std::time::{SystemTime, UNIX_EPOCH};

use iced::{button, Align, Button, Column, Element, Length, ProgressBar, Row, Text};
use t_rust_less_lib::otp::OTPAuthUrl;

use crate::{properties::property_label, style::ICONS, unlocked::UnlockedMessage};

#[derive(Debug)]
pub struct PropertyTOPTView {
  block_id: String,
  name: String,
  url: OTPAuthUrl,
  period: u32,
  copy_button: button::State,
}

impl PropertyTOPTView {
  pub fn new(block_id: &str, name: &str, url: OTPAuthUrl, period: u32) -> Self {
    PropertyTOPTView {
      block_id: block_id.to_string(),
      name: name.to_string(),
      url,
      period,
      copy_button: Default::default(),
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let (token, expires_at) = self.url.generate(now);
    let remain = expires_at - now;

    Row::new()
      .align_items(Align::Center)
      .push(Text::new(property_label(&self.name)).width(Length::Units(120)))
      .push(
        Column::new()
          .width(Length::Fill)
          .push(Text::new(token))
          .push(ProgressBar::new(0.0..=(self.period as f32), remain as f32).height(Length::Units(5))),
      )
      .push(
        Button::new(&mut self.copy_button, Text::new("\u{f0c5}").font(ICONS)).on_press(
          UnlockedMessage::CopySecretProperty {
            block_id: self.block_id.clone(),
            property_names: vec![self.name.clone()],
          },
        ),
      )
      .into()
  }
}
