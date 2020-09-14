use crate::{properties::property_label, style::ICONS, unlocked::UnlockedMessage};
use iced::{button, Align, Button, Element, Length, Row, Text};
use std::iter;
use t_rust_less_lib::memguard::weak::{ZeroingString, ZeroingStringExt};

#[derive(Debug)]
pub struct PropertyPasswordView {
  block_id: String,
  name: String,
  value: ZeroingString,
  reveal_button: button::State,
  copy_button: button::State,
  revealed: bool,
}

impl PropertyPasswordView {
  pub fn new(block_id: &str, name: &str, value: &str) -> Self {
    PropertyPasswordView {
      block_id: block_id.to_string(),
      name: name.to_string(),
      value: value.to_zeroing(),
      reveal_button: Default::default(),
      copy_button: Default::default(),
      revealed: false,
    }
  }

  pub fn set_revealed(&mut self, revealed: bool, property_name: &str) {
    if self.name == property_name {
      self.revealed = revealed;
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    if self.revealed {
      Row::new()
        .align_items(Align::Center)
        .push(Text::new(property_label(&self.name)).width(Length::Units(120)))
        .push(Text::new(self.value.as_str()).width(Length::Fill))
        .push(
          Button::new(&mut self.reveal_button, Text::new("\u{f070}").font(ICONS)).on_press(
            UnlockedMessage::RevealProperty {
              reveal: false,
              property_name: self.name.clone(),
            },
          ),
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
    } else {
      let hidden: String = iter::repeat("\u{2022}").take(self.value.len()).collect();
      Row::new()
        .align_items(Align::Center)
        .push(Text::new(property_label(&self.name)).width(Length::Units(120)))
        .push(Text::new(hidden).width(Length::Fill))
        .push(
          Button::new(&mut self.reveal_button, Text::new("\u{f06e}").font(ICONS)).on_press(
            UnlockedMessage::RevealProperty {
              reveal: true,
              property_name: self.name.clone(),
            },
          ),
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
}
