mod api;
mod cli;
mod secret_store;
#[allow(dead_code)]
mod secret_store_capnp {
  include!(concat!(env!("OUT_DIR"), "/src/secret_store/secret_store_capnp.rs"));
}
mod clipboard;
mod memguard;
mod store;

fn main() {
  cli::cli_run()
}
