use crate::error_panel::ErrorPanel;
use crate::locked::{Locked, LockedMessage};
use crate::unlocked::{Unlocked, UnlockedMessage};
use iced::{executor, time, Align, Application, Command, Container, Element, Length, Row, Subscription};
use std::sync::Arc;
use std::time::Duration;
use t_rust_less_lib::service::TrustlessService;

#[derive(Debug)]
pub enum MainState {
  Locked(Locked),
  Unlocked(Unlocked),
  Error(ErrorPanel),
}

#[derive(Debug)]
pub struct MainFrame {
  state: MainState,
}

#[derive(Debug, Clone)]
pub enum Message {
  Locked(LockedMessage),
  Unlocked(UnlockedMessage),
  Tick,
}

impl Application for MainFrame {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = Arc<dyn TrustlessService>;

  fn new(service: Arc<dyn TrustlessService>) -> (Self, Command<Message>) {
    let state = match Locked::new_safe(service) {
      Ok(locked) => match locked.check_unlocked() {
        Some(unlocked) => MainState::Unlocked(unlocked),
        None => MainState::Locked(locked),
      },
      Err(error) => MainState::Error(ErrorPanel::new(error)),
    };

    (MainFrame { state }, Command::none())
  }

  fn title(&self) -> String {
    "T-Rust-Less".to_string()
  }

  fn update(&mut self, message: Self::Message) -> Command<Message> {
    match message {
      Message::Locked(locked_message) => {
        if let MainState::Locked(locked) = &mut self.state {
          if let Some(unlocked) = locked.update(locked_message) {
            self.state = MainState::Unlocked(unlocked);
          }
        }
      }
      Message::Unlocked(unlocked_message) => {
        if let MainState::Unlocked(unlocked) = &mut self.state {
          if let Some(locked) = unlocked.update(unlocked_message) {
            self.state = MainState::Locked(locked);
          }
        }
      }
      Message::Tick => match &mut self.state {
        MainState::Locked(locked) => {
          if let Some(unlocked) = locked.check_unlocked() {
            self.state = MainState::Unlocked(unlocked);
          }
        }
        MainState::Unlocked(unlocked) => {
          if let Some(locked) = unlocked.check_locked() {
            self.state = MainState::Locked(locked);
          }
        }
        _ => (),
      },
    }
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let content = match &mut self.state {
      MainState::Locked(locked) => locked.view().map(Message::Locked),
      MainState::Unlocked(unlocked) => unlocked.view().map(Message::Unlocked),
      MainState::Error(error) => Row::new().align_items(Align::Center).push(error.view()).into(),
    };

     Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    time::every(Duration::from_millis(500)).map(|_| Message::Tick)
  }
}
