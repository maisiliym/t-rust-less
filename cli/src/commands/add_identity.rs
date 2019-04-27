use crate::commands::generate_id;
use crate::commands::tui::create_tui;
use crate::error::ExtResult;
use crate::view::PasswordView;
use atty::Stream;
use cursive::event::Key;
use cursive::traits::{Boxable, Identifiable};
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::process;
use std::sync::Arc;
use t_rust_less_lib::api::Identity;
use t_rust_less_lib::secrets_store::SecretsStore;
use t_rust_less_lib::service::TrustlessService;

pub fn add_identity_dialog(siv: &mut Cursive, secrets_store: Arc<SecretsStore>, title: &str) {
  siv.set_user_data(secrets_store);
  siv.add_layer(
    Dialog::around(
      LinearLayout::vertical()
        .child(TextView::new("Id"))
        .child(EditView::new().content(generate_id(40)).disabled().with_id("id"))
        .child(DummyView {})
        .child(TextView::new("Name"))
        .child(EditView::new().with_id("name").fixed_width(50))
        .child(DummyView {})
        .child(TextView::new("Email"))
        .child(EditView::new().with_id("email").fixed_width(50))
        .child(DummyView {})
        .child(TextView::new("Passphrase"))
        .child(PasswordView::new(100).with_id("passphrase")),
    )
    .title(title)
    .button("Create", create_identity)
    .button("Abort", Cursive::quit)
    .padding_left(5)
    .padding_right(5)
    .padding_top(1)
    .padding_bottom(1),
  )
}

pub fn add_identity(service: Arc<TrustlessService>, store_name: String) {
  if !atty::is(Stream::Stdout) {
    println!("Please use a terminal");
    process::exit(1);
  }

  let secrets_store = service
    .open_store(&store_name)
    .ok_or_exit(format!("Failed opening store {}: ", store_name));
  let mut siv = create_tui();

  siv.add_global_callback(Key::Esc, Cursive::quit);

  add_identity_dialog(&mut siv, secrets_store, "Add identity");

  siv.run();
}

fn create_identity(s: &mut Cursive) {
  let identity = Identity {
    id: s.find_id::<EditView>("id").unwrap().get_content().to_string(),
    name: s.find_id::<EditView>("name").unwrap().get_content().to_string(),
    email: s.find_id::<EditView>("email").unwrap().get_content().to_string(),
  };
  let passphrase = s.find_id::<PasswordView>("passphrase").unwrap().get_content();

  if identity.id.is_empty() {
    s.add_layer(Dialog::info("Id must not be empty"));
    return;
  }
  if identity.name.is_empty() {
    s.add_layer(Dialog::info("Name must not be empty"));
    return;
  }
  if identity.email.is_empty() {
    s.add_layer(Dialog::info("Email must not be empty"));
    return;
  }

  let secrets_store: &Arc<SecretsStore> = s.user_data().unwrap();
  match secrets_store.add_identity(identity, passphrase) {
    Ok(_) => s.quit(),
    Err(error) => s.add_layer(Dialog::info(format!("Failed to create identity: {}", error))),
  }
}
