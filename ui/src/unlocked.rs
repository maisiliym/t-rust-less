use crate::error_panel::ErrorPanel;
use crate::hotkeys::Hotkeys;
use crate::locked::Locked;
use crate::secret_list_view::SecretListView;
use crate::secret_view::SecretView;
use crate::style::ButtonStyle;
use iced::{
  button, pane_grid, scrollable, text_input, Button, Column, Element, Length, PaneGrid, Row, Space, Text, TextInput,
};
use std::env;
use std::sync::Arc;
use t_rust_less_lib::api::{Secret, SecretList, SecretListFilter, SecretVersion, SecretVersionRef, Status};
use t_rust_less_lib::secrets_store::SecretsStore;
use t_rust_less_lib::service::TrustlessService;

#[derive(Debug)]
pub enum UnlockedPane {
  List(SecretListView),
  View(SecretView),
}

impl UnlockedPane {
  fn update_secret_list(&mut self, secret_list: &SecretList) {
    if let UnlockedPane::List(view) = self {
      view.update_secret_list(secret_list);
    }
  }

  fn update_selected_secret(&mut self, selected_secret: &Option<Secret>) {
    match self {
      UnlockedPane::List(view) => view.update_selected_secret(selected_secret),
      UnlockedPane::View(view) => view.update_selected_secret(selected_secret),
    }
  }

  fn update_selected_secret_version(&mut self, block_id: String, secret_version: &SecretVersion) {
    if let UnlockedPane::View(view) = self {
      view.update_selected_secret_version(block_id, secret_version);
    }
  }

  fn reveal_property(&mut self, reveal: bool, property_name: &str) {
    if let UnlockedPane::View(view) = self {
      view.reveal_property(reveal, property_name);
    }
  }

  fn view(&mut self) -> Element<UnlockedMessage> {
    match self {
      UnlockedPane::List(view) => view.view(),
      UnlockedPane::View(view) => view.view(),
    }
  }
}

#[derive(Debug)]
pub struct Unlocked {
  service: Arc<dyn TrustlessService>,
  store: Arc<dyn SecretsStore>,
  status: Status,
  selected_secret: Option<Secret>,
  secret_list_filter: SecretListFilter,
  secret_list: SecretList,
  filter_name_input: text_input::State,
  lock_button: button::State,
  secret_list_scroll: scrollable::State,
  pane_state: pane_grid::State<UnlockedPane>,
  last_error: Option<ErrorPanel>,
  editing: bool,
}

#[derive(Debug, Clone)]
pub enum UnlockedMessage {
  SelectSecret(String),
  SecretFilterNameChanged(String),
  NextSecret,
  PrevSecret,
  SelectSecretVersion(SecretVersionRef),
  CopySecretProperty {
    block_id: String,
    property_names: Vec<String>,
  },
  RevealProperty {
    reveal: bool,
    property_name: String,
  },
  AddSecret,
  EditSecret,
  Resized(pane_grid::ResizeEvent),
  Lock,
}

impl Unlocked {
  pub fn new(service: Arc<dyn TrustlessService>, store: Arc<dyn SecretsStore>, status: Status) -> Self {
    let secret_list_filter = Default::default();
    let secret_list = store.list(&secret_list_filter).unwrap_or_default();
    let pane_state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
      axis: pane_grid::Axis::Vertical,
      ratio: 0.3,
      a: Box::new(pane_grid::Configuration::Pane(UnlockedPane::List(SecretListView::new(
        &secret_list,
      )))),
      b: Box::new(pane_grid::Configuration::Pane(UnlockedPane::View(SecretView::new()))),
    });

    Unlocked {
      service,
      store,
      status,
      selected_secret: None,
      secret_list_filter,
      secret_list,
      filter_name_input: text_input::State::focused(),
      lock_button: Default::default(),
      secret_list_scroll: Default::default(),
      pane_state,
      last_error: None,
      editing: false,
    }
  }

  pub fn check_locked(&mut self) -> Option<Locked> {
    let status = self.store.status().ok()?;

    if status.locked {
      Some(Locked::new_safe(self.service.clone()).ok()?)
    } else {
      self.status = status;
      None
    }
  }

  pub fn update(&mut self, message: UnlockedMessage) -> Option<Locked> {
    match message {
      UnlockedMessage::SelectSecret(secret_id) => {
        self.select_secret(secret_id);
        None
      }
      UnlockedMessage::SecretFilterNameChanged(name) => {
        self.secret_list_filter.name = if name.is_empty() { None } else { Some(name) };
        match self.store.list(&self.secret_list_filter) {
          Ok(secret_list) => {
            for (_, pane) in self.pane_state.iter_mut() {
              pane.update_secret_list(&secret_list);
            }
            self.secret_list = secret_list;
            self.selected_secret = None;
          }
          Err(error) => self.last_error = Some(ErrorPanel::new(error)),
        };
        None
      }
      UnlockedMessage::NextSecret => {
        match &self.selected_secret {
          Some(secret) => {
            if let Some(idx) = self
              .secret_list
              .entries
              .iter()
              .position(|entry| entry.entry.id == secret.id)
            {
              if let Some(next_id) = self
                .secret_list
                .entries
                .get(idx + 1)
                .map(|entry| entry.entry.id.clone())
              {
                self.select_secret(next_id);
              }
            }
          }
          None => {
            if let Some(first_id) = self.secret_list.entries.first().map(|entry| entry.entry.id.clone()) {
              self.select_secret(first_id);
            }
          }
        }
        None
      }
      UnlockedMessage::PrevSecret => {
        if let Some(secret) = &self.selected_secret {
          if let Some(idx) = self
            .secret_list
            .entries
            .iter()
            .position(|entry| entry.entry.id == secret.id)
          {
            if idx > 0 {
              if let Some(prev_id) = self
                .secret_list
                .entries
                .get(idx - 1)
                .map(|entry| entry.entry.id.clone())
              {
                self.select_secret(prev_id);
              }
            }
          }
        }
        None
      }
      UnlockedMessage::SelectSecretVersion(secret_version_ref) => {
        match self.store.get_version(&secret_version_ref.block_id) {
          Ok(secret_version) => {
            for (_, pane) in self.pane_state.iter_mut() {
              pane.update_selected_secret_version(secret_version_ref.block_id.clone(), &secret_version);
            }
            None
          }
          Err(error) => {
            self.last_error = Some(ErrorPanel::new(error));
            None
          }
        }
      }
      UnlockedMessage::CopySecretProperty {
        block_id,
        property_names,
      } => {
        let store_name = self.store.name();
        let property_names: Vec<&str> = property_names.iter().map(|s| s.as_str()).collect();
        if let Err(error) = self.service.secret_to_clipboard(
          &store_name,
          &block_id,
          &property_names,
          &env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string()),
        ) {
          self.last_error = Some(ErrorPanel::new(error));
        }
        None
      }
      UnlockedMessage::RevealProperty { reveal, property_name } => {
        for (_, pane) in self.pane_state.iter_mut() {
          pane.reveal_property(reveal, &property_name);
        }
        None
      }
      UnlockedMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
        self.pane_state.resize(&split, ratio);
        None
      }
      UnlockedMessage::Lock => {
        self.store.lock().ok()?;
        Some(Locked::new_safe(self.service.clone()).ok()?)
      }
      _ => None,
    }
  }

  fn select_secret(&mut self, secret_id: String) {
    match self.store.get(&secret_id) {
      Ok(secret) => {
        let selected_secret = Some(secret);
        self
          .pane_state
          .iter_mut()
          .for_each(|(_, pane)| pane.update_selected_secret(&selected_secret));
        self.selected_secret = selected_secret;
      }
      Err(error) => self.last_error = Some(ErrorPanel::new(error)),
    }
  }

  pub fn view(&mut self) -> Element<UnlockedMessage> {
    let header = Row::new()
      .width(Length::Fill)
      .push(
        TextInput::new(
          &mut self.filter_name_input,
          "Search",
          self
            .secret_list_filter
            .name
            .as_ref()
            .map(|name| name.as_str())
            .unwrap_or(""),
          UnlockedMessage::SecretFilterNameChanged,
        )
        .padding(5)
        .width(Length::Fill),
      )
      .push(Space::with_width(Length::Fill))
      .push(
        Button::new(
          &mut self.lock_button,
          Row::new()
            .spacing(10)
            .push(Text::new('\u{f023}'.to_string()).font(crate::style::ICONS))
            .push(Text::new("Lock")),
        )
        .style(ButtonStyle::Destructive)
        .padding(5)
        .on_press(UnlockedMessage::Lock),
      );

    let pane = PaneGrid::new(&mut self.pane_state, move |_, pane, _| {
      pane_grid::Content::new(pane.view())
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .spacing(10)
    .on_resize(10, UnlockedMessage::Resized);

    let secrets = Row::new().push(pane);

    let mut main = Column::new().height(Length::Fill).push(header);

    if let Some(error) = &self.last_error {
      main = main.push(error.view())
    }

    main = main.push(secrets).padding(5).spacing(5);

    if self.editing {
      main.into()
    } else {
      Hotkeys::new(main).into()
    }
  }
}
