use crate::secret_version_view::SecretVersionView;
use crate::style::{ButtonStyle, ICONS};
use crate::unlocked::UnlockedMessage;
use iced::{button, Button, Column, Container, Element, Length, Text};
use t_rust_less_lib::api::{Secret, SecretVersion};

#[derive(Debug)]
pub enum SecretView {
  Empty(button::State),
  View(SecretVersionView),
}

impl SecretView {
  pub fn new() -> Self {
    SecretView::Empty(Default::default())
  }

  pub fn update_selected_secret(&mut self, selected_secret: &Option<Secret>) {
    match selected_secret {
      Some(secret) => *self = SecretView::View(SecretVersionView::new(secret.clone())),
      None => *self = SecretView::Empty(Default::default()),
    }
  }

  pub fn update_selected_secret_version(&mut self, block_id: String, secret_version: &SecretVersion) {
    if let SecretView::View(view) = self {
      view.update_selected_secret_version(block_id, secret_version);
    }
  }

  pub fn reveal_property(&mut self, reveal: bool, property_name: &str) {
    if let SecretView::View(view) = self {
      view.reveal_property(reveal, property_name);
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    match self {
      SecretView::Empty(add_button) => Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .push(
          Button::new(add_button, Text::new("\u{f067}").font(ICONS))
            .style(ButtonStyle::Secondary)
            .on_press(UnlockedMessage::AddSecret),
        )
        .push(
          Container::new(Text::new("Select secret").color([0.5, 0.5, 0.5]))
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .into(),
      SecretView::View(view) => view.view(),
    }
  }
}
