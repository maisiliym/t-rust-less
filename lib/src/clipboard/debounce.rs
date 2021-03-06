use super::SelectionProvider;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use zeroize::Zeroize;

struct LastContext {
  content: String,
  timestamp: SystemTime,
  initial: bool,
}

impl Drop for LastContext {
  fn drop(&mut self) {
    self.content.zeroize();
  }
}

/// Debounce selections.
///
/// Some clients, like a browser that shall not be named (it's from a company start with G),
/// sent up to 4 selection requests to process a single Crtl-V. There seems to be no other way
/// but to debounce these requests by time. I.e. we consider all requests within 200ms to be part
/// of the same paste-action.
///
pub struct SelectionDebounce {
  underlying: Arc<RwLock<dyn SelectionProvider>>,
  last_content: Option<LastContext>,
  startup_timestamp: SystemTime,
}

impl SelectionDebounce {
  pub fn new(underlying: Arc<RwLock<dyn SelectionProvider>>) -> Self {
    SelectionDebounce {
      underlying,
      last_content: None,
      startup_timestamp: SystemTime::now(),
    }
  }
}

impl SelectionProvider for SelectionDebounce {
  fn current_selection_name(&self) -> Option<String> {
    self.underlying.read().ok()?.current_selection_name()
  }

  fn get_selection(&mut self) -> Option<String> {
    let now = SystemTime::now();
    if let Some(last_content) = self.last_content.take() {
      if last_content.initial {
        self.last_content.replace(LastContext {
          content: last_content.content.clone(),
          timestamp: now,
          initial: false,
        });
        return Some(last_content.content.clone());
      }
      if let Ok(elapsed) = now.duration_since(last_content.timestamp) {
        if elapsed.as_millis() < 200 {
          let content = last_content.content.clone();
          self.last_content.replace(last_content);
          return Some(content);
        }
      }
    }

    match self.underlying.write().ok()?.get_selection() {
      Some(content) => {
        let initial = matches!(now.duration_since(self.startup_timestamp), Ok(elapsed) if elapsed.as_millis() < 200);
        self.last_content = Some(LastContext {
          content: content.clone(),
          timestamp: now,
          initial,
        });
        Some(content)
      }
      _ => {
        self.last_content = None;
        None
      }
    }
  }
}
