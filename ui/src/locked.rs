use crate::error_panel::ErrorPanel;
use crate::style::ButtonStyle;
use crate::unlocked::Unlocked;
use iced::{button, pick_list, text_input, Button, Column, Element, Length, PickList, Row, Space, Text, TextInput};
use std::sync::Arc;
use t_rust_less_lib::api::Identity;
use t_rust_less_lib::memguard::SecretBytes;
use t_rust_less_lib::secrets_store::SecretsStore;
use t_rust_less_lib::service::{ServiceResult, TrustlessService};

#[derive(Debug)]
pub struct Locked {
  service: Arc<dyn TrustlessService>,
  store_select: pick_list::State<String>,
  identity_select: pick_list::State<Identity>,
  passphrase_input: text_input::State,
  unlock_button: button::State,
  passphrase: SecretBytes,
  store_names: Vec<String>,
  selected_store: Option<Arc<dyn SecretsStore>>,
  selected_store_name: Option<String>,
  identities: Vec<Identity>,
  selected_identity: Option<Identity>,
  last_error: Option<ErrorPanel>,
}

#[derive(Debug, Clone)]
pub enum LockedMessage {
  SelectStore(String),
  SelectIdentity(Identity),
  PassphraseChanged(String),
  TryUnlock,
}

impl Locked {
  pub fn new_safe(service: Arc<dyn TrustlessService>) -> ServiceResult<Self> {
    let store_names = service.list_stores()?;
    let selected_store_name = service.get_default_store()?;
    let selected_store = selected_store_name
      .as_ref()
      .map(|name| service.open_store(name))
      .transpose()?;
    let identities = selected_store
      .as_ref()
      .map(|store| store.identities())
      .transpose()?
      .unwrap_or_default();
    let selected_identity = identities.first().cloned();

    Ok(Locked {
      service,
      store_names,
      selected_store,
      selected_store_name,
      identities,
      selected_identity,
      store_select: Default::default(),
      identity_select: Default::default(),
      passphrase_input: text_input::State::focused(),
      unlock_button: Default::default(),
      passphrase: SecretBytes::zeroed(0),
      last_error: None,
    })
  }

  pub fn check_unlocked(&self) -> Option<Unlocked> {
    let store = self.selected_store.as_ref()?;
    let status = store.status().ok()?;

    if status.locked {
      None
    } else {
      Some(Unlocked::new(self.service.clone(), store.clone(), status))
    }
  }

  pub fn update(&mut self, message: LockedMessage) -> Option<Unlocked> {
    match message {
      LockedMessage::SelectIdentity(identity) => {
        self.selected_identity = Some(identity);
        None
      }
      LockedMessage::SelectStore(store_name) => {
        match self.service.open_store(&store_name) {
          Ok(store) => {
            self.selected_store_name = Some(store_name);
            self.selected_store = Some(store);
          }
          Err(error) => {
            self.last_error = Some(ErrorPanel::new(error));
          }
        };
        None
      }
      LockedMessage::PassphraseChanged(passphrase) => {
        self.passphrase = SecretBytes::from(passphrase);
        self.last_error = None;
        None
      }
      LockedMessage::TryUnlock => match (&self.selected_store, &self.selected_identity) {
        (Some(store), Some(identity)) => match store.unlock(&identity.id, self.passphrase.clone()) {
          Ok(_) => {
            let status = store.status().ok()?;
            Some(Unlocked::new(self.service.clone(), store.clone(), status))
          }
          Err(error) => {
            self.last_error = Some(ErrorPanel::new(error));
            self.passphrase = SecretBytes::zeroed(0);
            None
          }
        },
        _ => None,
      },
    }
  }

  pub fn view(&mut self) -> Element<LockedMessage> {
    let store_select = PickList::new(
      &mut self.store_select,
      &self.store_names,
      self.selected_store_name.clone(),
      LockedMessage::SelectStore,
    )
    .width(Length::Fill);
    let identity_select = PickList::new(
      &mut self.identity_select,
      &self.identities,
      self.selected_identity.clone(),
      LockedMessage::SelectIdentity,
    )
    .width(Length::Fill);
    let passphrase_input = TextInput::new(
      &mut self.passphrase_input,
      "Passphrase",
      self.passphrase.borrow().as_str(),
      LockedMessage::PassphraseChanged,
    )
    .on_submit(LockedMessage::TryUnlock)
    .width(Length::Fill)
    .password()
    .padding(10);
    let unlock_button = Button::new(
      &mut self.unlock_button,
      Row::new()
        .spacing(10)
        .push(Space::with_width(Length::Fill))
        .push(Text::new('\u{f09c}'.to_string()).font(crate::style::ICONS))
        .push(Text::new("Unlock"))
        .push(Space::with_width(Length::Fill)),
    )
    .on_press(LockedMessage::TryUnlock)
    .width(Length::Fill)
    .padding(10)
    .style(ButtonStyle::Primary);

    let mut column = Column::new()
      .spacing(10)
      .width(Length::FillPortion(3))
      .push(store_select)
      .push(identity_select)
      .push(passphrase_input)
      .push(unlock_button);

    if let Some(error) = &mut self.last_error {
      column = column.push(error.view());
    } else {
      column = column.push(Space::with_height(Length::Units(40)))
    }

    Row::new()
      .push(Space::with_width(Length::Fill))
      .push(column)
      .push(Space::with_width(Length::Fill))
      .into()
  }
}
