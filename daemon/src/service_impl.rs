use crate::error::ExtResult;
use crate::secrets_store_impl::SecretsStoreImpl;
use capnp::capability::Promise;
use t_rust_less_lib::api_capnp::{secrets_store, service};
use t_rust_less_lib::service::local::LocalTrustlessService;
use t_rust_less_lib::service::TrustlessService;

pub struct ServiceImpl {
  service: LocalTrustlessService,
}

impl ServiceImpl {
  pub fn new() -> Self {
    ServiceImpl {
      service: LocalTrustlessService::new().ok_or_exit("Open local store"),
    }
  }
}

impl service::Server for ServiceImpl {
  fn list_stores(
    &mut self,
    _: service::ListStoresParams,
    mut results: service::ListStoresResults,
  ) -> Promise<(), capnp::Error> {
    let store_names = stry!(self.service.list_stores());
    let mut result = results.get().init_store_names(store_names.len() as u32);

    for (idx, store_name) in store_names.into_iter().enumerate() {
      result.set(idx as u32, &store_name);
    }

    Promise::ok(())
  }

  fn set_store_config(
    &mut self,
    _: service::SetStoreConfigParams,
    _: service::SetStoreConfigResults,
  ) -> Promise<(), ::capnp::Error> {
    Promise::err(::capnp::Error::unimplemented("method not implemented".to_string()))
  }

  fn get_store_config(
    &mut self,
    _: service::GetStoreConfigParams,
    _: service::GetStoreConfigResults,
  ) -> Promise<(), ::capnp::Error> {
    Promise::err(::capnp::Error::unimplemented("method not implemented".to_string()))
  }

  fn get_default_store(
    &mut self,
    _: service::GetDefaultStoreParams,
    mut results: service::GetDefaultStoreResults,
  ) -> Promise<(), ::capnp::Error> {
    let mut result = results.get().init_default_store();

    match stry!(self.service.get_default_store()) {
      Some(default_store) => {
        let text = stry!(capnp::text::new_reader(default_store.as_bytes()));
        stry!(result.set_some(text))
      }
      None => result.set_none(()),
    }

    Promise::ok(())
  }

  fn set_default_store(
    &mut self,
    params: service::SetDefaultStoreParams,
    _: service::SetDefaultStoreResults,
  ) -> Promise<(), ::capnp::Error> {
    let default_store = stry!(stry!(params.get()).get_default_store());

    stry!(self.service.set_default_store(default_store));

    Promise::ok(())
  }

  fn open_store(
    &mut self,
    params: service::OpenStoreParams,
    mut results: service::OpenStoreResults,
  ) -> Promise<(), ::capnp::Error> {
    let store_name = stry!(stry!(params.get()).get_store_name());
    let store = stry!(self.service.open_store(store_name));

    results
      .get()
      .set_store(secrets_store::ToClient::new(SecretsStoreImpl::new(store)).into_client::<capnp_rpc::Server>());

    Promise::ok(())
  }
}
