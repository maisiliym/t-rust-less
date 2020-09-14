use crate::secret_list_element::SecretListElement;
use crate::unlocked::UnlockedMessage;
use iced::{scrollable, Element, Scrollable};
use t_rust_less_lib::api::{Secret, SecretList};

#[derive(Debug)]
pub struct SecretListView {
  scroll: scrollable::State,
  elements: Vec<SecretListElement>,
}

impl SecretListView {
  pub fn new(secret_list: &SecretList) -> Self {
    SecretListView {
      scroll: Default::default(),
      elements: secret_list
        .entries
        .iter()
        .map(|entry| SecretListElement::new(entry.clone()))
        .collect(),
    }
  }

  pub fn update_secret_list(&mut self, secret_list: &SecretList) {
    self.scroll = Default::default();
    self.elements = secret_list
      .entries
      .iter()
      .map(|entry| SecretListElement::new(entry.clone()))
      .collect();
  }

  pub fn update_selected_secret(&mut self, selected_secret: &Option<Secret>) {
    let selected_secret_id = selected_secret.as_ref().map(|secret| secret.id.as_str());
    for element in self.elements.iter_mut() {
      element.update_selected_secret(selected_secret_id);
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    self
      .elements
      .iter_mut()
      .fold(Scrollable::new(&mut self.scroll), |column, task| {
        column.push(task.view())
      })
      .into()
  }
}
