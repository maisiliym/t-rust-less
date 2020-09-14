use crate::unlocked::UnlockedMessage;
use iced_native::{keyboard, Clipboard, Element, Event, Layout, Point, Renderer, Widget};

pub struct Hotkeys<'a, Renderer> {
  content: Element<'a, UnlockedMessage, Renderer>,
}

impl<'a, Renderer> Hotkeys<'a, Renderer> {
  pub fn new<E>(child: E) -> Self
  where
    E: Into<Element<'a, UnlockedMessage, Renderer>>,
  {
    Hotkeys { content: child.into() }
  }
}

impl<'a, Renderer> Widget<UnlockedMessage, Renderer> for Hotkeys<'a, Renderer>
where
  Renderer: self::Renderer,
{
  fn width(&self) -> iced::Length {
    self.content.width()
  }

  fn height(&self) -> iced::Length {
    self.content.height()
  }

  fn layout(&self, renderer: &Renderer, limits: &iced_native::layout::Limits) -> iced_native::layout::Node {
    self.content.layout(renderer, limits)
  }

  fn draw(
    &self,
    renderer: &mut Renderer,
    defaults: &Renderer::Defaults,
    layout: iced_native::Layout<'_>,
    cursor_position: iced::Point,
  ) -> Renderer::Output {
    self.content.draw(renderer, defaults, layout, cursor_position)
  }

  fn hash_layout(&self, state: &mut iced_native::Hasher) {
    self.content.hash_layout(state)
  }

  fn on_event(
    &mut self,
    event: Event,
    layout: Layout<'_>,
    cursor_position: Point,
    messages: &mut Vec<UnlockedMessage>,
    renderer: &Renderer,
    clipboard: Option<&dyn Clipboard>,
  ) {
    match event {
      Event::Keyboard(keyboard::Event::KeyPressed {
        key_code: keyboard::KeyCode::Down,
        ..
      }) => messages.push(UnlockedMessage::NextSecret),
      Event::Keyboard(keyboard::Event::KeyPressed {
        key_code: keyboard::KeyCode::Up,
        ..
      }) => messages.push(UnlockedMessage::PrevSecret),
      _ => self
        .content
        .on_event(event, layout, cursor_position, messages, renderer, clipboard),
    }
  }

  fn overlay(&mut self, layout: Layout) -> Option<iced_native::overlay::Element<UnlockedMessage, Renderer>> {
    self.content.overlay(layout)
  }
}

impl<'a, Renderer> Into<Element<'a, UnlockedMessage, Renderer>> for Hotkeys<'a, Renderer>
where
  Renderer: iced_native::Renderer + 'a,
{
  fn into(self) -> Element<'a, UnlockedMessage, Renderer> {
    Element::new(self)
  }
}
