use anyhow::Result;
use ra_rpc::{Attestation, RpcCall};
use {{app_name}}_rpc::{{app_name}}_server::{ {{app_name | capitalize}}Rpc, {{app_name | capitalize}}Server };

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct RpcHandler {
    state: AppState,
}

impl {{app_name | capitalize}}Rpc for RpcHandler {
    async fn handle_request(self) -> Result<()> {
        Ok(())
    }
}

impl RpcCall<AppState> for RpcHandler {
    type PrpcService = {{app_name | capitalize}}Server<Self>;

    fn into_prpc_service(self) -> Self::PrpcService {
        {{app_name | capitalize}}Server::new(self)
    }

    fn construct(state: &AppState, _attestation: Option<Attestation>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(RpcHandler {
            state: state.clone(),
        })
    }
}

pub fn rpc_methods() -> &'static [&'static str] {
    <{{app_name | capitalize}}Server<RpcHandler>>::supported_methods()
}