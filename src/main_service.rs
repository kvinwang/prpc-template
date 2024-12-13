use anyhow::Result;
use {{app_name | snake}}_rpc::{{type_name | snake}}_server::{ {{- type_name }}Rpc, {{type_name}}Server};
use {{app_name | snake}}_rpc::HelloResponse;
use ra_rpc::{Attestation, RpcCall};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::config::AppConfig;

#[derive(Clone)]
pub(crate) struct AppState {
    inner: Arc<Mutex<AppStateInner>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AppStateInner { config })),
        }
    }

    pub fn lock(&self) -> MutexGuard<AppStateInner> {
        self.inner.lock().expect("Failed to lock AppState")
    }
}

pub struct AppStateInner {
    config: AppConfig,
}

pub struct RpcHandler {
    attestation: Option<Attestation>,
    state: AppState,
}

impl {{type_name | capitalize}}Rpc for RpcHandler {
    async fn hello(self) -> Result<HelloResponse> {
        Ok(HelloResponse {
            message: self.state.lock().config.rpc_reply.clone(),
        })
    }
}

impl RpcCall<AppState> for RpcHandler {
    type PrpcService = {{type_name | capitalize}}Server<Self>;

    fn into_prpc_service(self) -> Self::PrpcService {
        {{type_name | capitalize}}Server::new(self)
    }

    fn construct(state: &AppState, attestation: Option<Attestation>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(RpcHandler {
            attestation,
            state: state.clone(),
        })
    }
}

pub fn rpc_methods() -> &'static [&'static str] {
    <{{type_name | capitalize}}Server<RpcHandler>>::supported_methods()
}
