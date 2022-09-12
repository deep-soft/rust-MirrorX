use crate::{
    api::{
        config::ConfigProperties,
        endpoint::handlers::{
            connect::*, handshake::*, input::*, negotiate_finished::*, negotiate_select_monitor::*,
            negotiate_visit_desktop_params::*,
        },
        signaling::{
            dial::{dial, DialRequest},
            heartbeat::{heartbeat, HeartbeatRequest, HeartbeatResponse},
            key_exchange::{key_exchange, KeyExchangeRequest, KeyExchangeResponse},
            register::{register, RegisterRequest, RegisterResponse},
            subscribe::{subscribe, PublishMessage, SubscribeRequest},
            visit::{visit, VisitRequest, VisitResponse},
        },
    },
    utility::runtime::TOKIO_RUNTIME,
};
use flutter_rust_bridge::StreamSink;

macro_rules! async_block_on {
    ($future:expr) => {{
        let (tx, rx) = crossbeam::channel::bounded(1);

        TOKIO_RUNTIME.spawn(async move { tx.try_send($future.await) });

        let message = rx
            .recv()
            .map_err(|err| anyhow::anyhow!("receive call result failed ({})", err))??;

        Ok(message)
    }};
}

/*
    Init API
*/

pub fn init_logger() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    Ok(())
}

/*
    Config API
*/

pub fn config_read(path: String, key: String) -> anyhow::Result<Option<ConfigProperties>> {
    let model = crate::api::config::read(&path, &key)?;
    Ok(model)
}

pub fn config_save(path: String, key: String, properties: ConfigProperties) -> anyhow::Result<()> {
    crate::api::config::save(&path, &key, &properties)?;
    Ok(())
}

/*
    Signaling API
*/

pub fn signaling_dial(req: DialRequest) -> anyhow::Result<()> {
    async_block_on!(dial(req))
}

pub fn signaling_register(req: RegisterRequest) -> anyhow::Result<RegisterResponse> {
    async_block_on!(register(req))
}

pub fn signaling_subscribe(
    req: SubscribeRequest,
    stream: StreamSink<PublishMessage>,
) -> anyhow::Result<()> {
    async_block_on!(subscribe(req, stream))
}

pub fn signaling_heartbeat(req: HeartbeatRequest) -> anyhow::Result<HeartbeatResponse> {
    async_block_on!(heartbeat(req))
}

pub fn signaling_visit(req: VisitRequest) -> anyhow::Result<VisitResponse> {
    async_block_on!(visit(req))
}

pub fn signaling_key_exchange(req: KeyExchangeRequest) -> anyhow::Result<KeyExchangeResponse> {
    async_block_on!(key_exchange(req))
}

/*
    EndPoint API
*/

pub fn endpoint_connect(req: ConnectRequest) -> anyhow::Result<()> {
    async_block_on!(connect(req))
}

pub fn endpoint_handshake(req: HandshakeRequest) -> anyhow::Result<()> {
    async_block_on!(active_device_handshake(req))
}

pub fn endpoint_negotiate_visit_desktop_params(
    req: NegotiateVisitDesktopParamsRequest,
) -> anyhow::Result<NegotiateVisitDesktopParamsResponse> {
    async_block_on!(negotiate_visit_desktop_params(req))
}

pub fn endpoint_negotiate_select_monitor(
    req: NegotiateSelectMonitorRequest,
) -> anyhow::Result<NegotiateSelectMonitorResponse> {
    async_block_on!(negotiate_select_monitor(req))
}

pub fn endpoint_negotiate_finished(req: NegotiateFinishedRequest) -> anyhow::Result<()> {
    async_block_on!(negotiate_finished(req))
}

pub fn endpoint_input(req: InputReqeust) -> anyhow::Result<()> {
    async_block_on!(input(req))
}
