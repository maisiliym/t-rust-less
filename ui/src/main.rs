use iced::{Application, Settings};

mod error_panel;
mod hotkeys;
mod locked;
mod main_frame;
mod properties;
mod property_password_view;
mod property_simple_view;
mod property_totp_view;
mod property_view;
mod secret_list_element;
mod secret_list_view;
mod secret_version_view;
mod secret_versions_select;
mod secret_view;
mod style;
mod unlocked;

use t_rust_less_lib::service::create_service;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut log_builder = env_logger::Builder::from_default_env();

  log_builder.target(env_logger::Target::Stderr);
  log_builder.init();

  let service = create_service()?;

  main_frame::MainFrame::run(Settings {
    flags: service,
    window: Default::default(),
    default_font: None,
    default_text_size: 20,
    antialiasing: false,
  })?;

  Ok(())
}
