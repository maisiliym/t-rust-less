use crate::property_view::PropertyView;
use crate::secret_versions_select::SecretVersionsSelect;
use crate::unlocked::UnlockedMessage;
use iced::{scrollable, Align, Column, Element, Length, Scrollable, Text};
use t_rust_less_lib::api::{Secret, SecretVersion};

#[derive(Debug)]
pub struct SecretVersionView {
  secret: Secret,
  current_block_id: String,
  secret_version: SecretVersion,
  version_select: SecretVersionsSelect,
  scroll: scrollable::State,
  property_views: Vec<PropertyView>,
}

impl SecretVersionView {
  pub fn new(secret: Secret) -> Self {
    let secret_version = secret.current.clone();
    let current_block_id = secret.current_block_id.clone();
    let property_views = secret_version
      .properties
      .iter()
      .map(|(name, value)| PropertyView::new(&current_block_id, name, value))
      .collect();

    SecretVersionView {
      version_select: SecretVersionsSelect::new(current_block_id.clone(), secret.versions.clone()),
      secret,
      current_block_id,
      secret_version,
      scroll: Default::default(),
      property_views,
    }
  }

  pub fn update_selected_secret_version(&mut self, block_id: String, secret_version: &SecretVersion) {
    self.property_views = secret_version
      .properties
      .iter()
      .map(|(name, value)| PropertyView::new(&self.current_block_id, name, value))
      .collect();
    self.secret_version = secret_version.clone();
    self.current_block_id = block_id.clone();
    self.version_select.update_selected_secret_version(block_id);
  }

  pub fn reveal_property(&mut self, reveal: bool, property_name: &str) {
    for property_view in self.property_views.iter_mut() {
      property_view.reveal_property(reveal, property_name)
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    let mut properties_scroll = Scrollable::new(&mut self.scroll)
      .width(Length::Fill)
      .height(Length::Fill)
      .spacing(5);

    for view in self.property_views.iter_mut() {
      properties_scroll = properties_scroll.push(view.view());
    }

    Column::new()
      .width(Length::Fill)
      .align_items(Align::Center)
      .spacing(5)
      .push(self.version_select.view())
      .push(Text::new(self.secret_version.name.as_str()).size(35))
      .push(properties_scroll)
      .into()
  }
}
