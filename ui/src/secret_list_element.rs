use crate::style::ButtonStyle;
use crate::unlocked::UnlockedMessage;
use iced::{button, Button, Element, Length, Text};
use t_rust_less_lib::api::SecretEntryMatch;

#[derive(Debug)]
pub struct SecretListElement {
  entry: SecretEntryMatch,
  state: button::State,
  selected: bool,
}

impl SecretListElement {
  pub fn new(entry: SecretEntryMatch) -> Self {
    SecretListElement {
      entry,
      state: Default::default(),
      selected: false,
    }
  }

  pub fn update_selected_secret(&mut self, selected_secret_id: Option<&str>) {
    self.selected = match selected_secret_id {
      Some(id) => self.entry.entry.id == id,
      None => false,
    };
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    Button::new(&mut self.state, Text::new(self.entry.entry.name.as_str()))
      .width(Length::Fill)
      .style(if self.selected {
        ButtonStyle::Selected
      } else {
        ButtonStyle::Blank
      })
      .on_press(UnlockedMessage::SelectSecret(self.entry.entry.id.to_string()))
      .into()
  }
}
