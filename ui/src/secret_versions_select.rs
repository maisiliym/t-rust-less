use crate::style::{ButtonStyle, ICONS};
use crate::unlocked::UnlockedMessage;
use iced::{button, pick_list, Button, Element, Length, PickList, Row, Space, Text};
use t_rust_less_lib::api::SecretVersionRef;

#[derive(Debug)]
pub struct SecretVersionsSelect {
  current_block_id: String,
  versions: Vec<SecretVersionRef>,
  prev_button: button::State,
  version_pick: pick_list::State<SecretVersionRef>,
  next_button: button::State,
  add_button: button::State,
  edit_button: button::State,
}

impl SecretVersionsSelect {
  pub fn new(current_block_id: String, versions: Vec<SecretVersionRef>) -> Self {
    SecretVersionsSelect {
      current_block_id,
      versions,
      prev_button: Default::default(),
      version_pick: Default::default(),
      next_button: Default::default(),
      add_button: Default::default(),
      edit_button: Default::default(),
    }
  }

  pub fn update_selected_secret_version(&mut self, block_id: String) {
    self.current_block_id = block_id;
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    let selected_idx = self
      .versions
      .iter()
      .position(|version| version.block_id == self.current_block_id);
    let mut prev = Button::new(&mut self.prev_button, Text::new(" \u{f0d9} ").font(ICONS));
    let mut next = Button::new(&mut self.next_button, Text::new(" \u{f0da} ").font(ICONS));
    let mut selected_version_ref = None;

    match selected_idx {
      Some(idx) => {
        selected_version_ref = self.versions.get(idx).cloned();
        if let Some(prev_version_ref) = self.versions.get(idx + 1).cloned() {
          prev = prev.on_press(UnlockedMessage::SelectSecretVersion(prev_version_ref))
        }
        if idx > 0 {
          if let Some(next_version_ref) = self.versions.get(idx - 1).cloned() {
            next = next.on_press(UnlockedMessage::SelectSecretVersion(next_version_ref))
          }
        }
      }
      None => (),
    };

    Row::new()
      .width(Length::Fill)
      .push(
        Button::new(&mut self.add_button, Text::new("\u{f067}").font(ICONS))
          .style(ButtonStyle::Secondary)
          .on_press(UnlockedMessage::AddSecret),
      )
      .push(Space::with_width(Length::Fill))
      .push(prev)
      .push(PickList::new(
        &mut self.version_pick,
        &self.versions,
        selected_version_ref,
        UnlockedMessage::SelectSecretVersion,
      ))
      .push(next)
      .push(Space::with_width(Length::Fill))
      .push(
        Button::new(&mut self.edit_button, Text::new("\u{f044}").font(ICONS))
          .style(ButtonStyle::Secondary)
          .on_press(UnlockedMessage::EditSecret),
      )
      .into()
  }
}
