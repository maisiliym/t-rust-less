use iced::{button, container, Background, Color, Font, Vector};

pub enum TextStyle {
  Error,
}

impl container::StyleSheet for TextStyle {
  fn style(&self) -> container::Style {
    match self {
      TextStyle::Error => container::Style {
        background: Color::from_rgb(0.8, 0.2, 0.2).into(),
        text_color: Some(Color::WHITE),
        ..Default::default()
      },
    }
  }
}

pub const ICONS: Font = Font::External {
  name: "font-awesome",
  bytes: include_bytes!("../font/fa-solid-900.ttf"),
};

pub enum ButtonStyle {
  Primary,
  Secondary,
  Destructive,
  Blank,
  Selected,
}

impl button::StyleSheet for ButtonStyle {
  fn active(&self) -> button::Style {
    match self {
      ButtonStyle::Blank => button::Style {
        background: None,
        text_color: Color::BLACK,
        ..button::Style::default()
      },
      ButtonStyle::Selected => button::Style {
        background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
        text_color: Color::WHITE,
        border_radius: 2,
        ..button::Style::default()
      },
      ButtonStyle::Primary => button::Style {
        background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
        border_radius: 5,
        shadow_offset: Vector::new(1.0, 1.0),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      ButtonStyle::Secondary => button::Style {
        background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
        border_radius: 5,
        shadow_offset: Vector::new(1.0, 1.0),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      ButtonStyle::Destructive => button::Style {
        background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
        border_radius: 5,
        shadow_offset: Vector::new(1.0, 1.0),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
    }
  }
}
