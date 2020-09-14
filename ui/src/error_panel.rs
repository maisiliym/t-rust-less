use crate::style::TextStyle;
use iced::{Container, Length, Text};

#[derive(Debug)]
pub struct ErrorPanel {
  error: Box<dyn std::error::Error>,
}

impl ErrorPanel {
  pub fn new<T>(error: T) -> Self
  where
    T: Into<Box<dyn std::error::Error>>,
  {
    ErrorPanel { error: error.into() }
  }

  pub fn view<M>(&self) -> Container<M> {
    Container::new(Text::new(format!("{}", self.error)))
      .padding(10)
      .width(Length::Fill)
      .height(Length::Units(40))
      .style(TextStyle::Error)
      .into()
  }
}
