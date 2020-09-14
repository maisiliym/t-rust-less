use crate::properties::property_label;
use crate::style::ICONS;
use crate::unlocked::UnlockedMessage;
use iced::{button, Align, Button, Element, Length, Row, Text};
use t_rust_less_lib::memguard::weak::{ZeroingString, ZeroingStringExt};

#[derive(Debug)]
pub struct PropertySimpleView {
  block_id: String,
  name: String,
  value: ZeroingString,
  copy_button: button::State,
}

impl PropertySimpleView {
  pub fn new(block_id: &str, name: &str, value: &str) -> Self {
    PropertySimpleView {
      block_id: block_id.to_string(),
      name: name.to_string(),
      value: value.to_zeroing(),
      copy_button: Default::default(),
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    Row::new()
      .align_items(Align::Center)
      .push(Text::new(property_label(&self.name)).width(Length::Units(120)))
      .push(Text::new(self.value.as_str()).width(Length::Fill))
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
