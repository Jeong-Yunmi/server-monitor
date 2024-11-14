// src/websocket/handler.rs
use actix::{Actor, StreamHandler, ActorContext, AsyncContext};
use actix_web_actors::ws;
use serde_json::json;
use std::time::{Duration, Instant};
use crate::monitoring::MonitoringService;
use tokio::sync::mpsc;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketConnection {
    monitoring_service: MonitoringService,
    last_heartbeat: Instant,
    server_id: Option<String>,
}

impl WebSocketConnection {
    pub fn new(monitoring_service: MonitoringService) -> Self {
        Self {
            monitoring_service,
            last_heartbeat: Instant::now(),
            server_id: None,
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        
        // Set up metrics stream
        let (tx, mut rx) = mpsc::channel(32);
        let addr = ctx.address();
        
        tokio::spawn(async move {
            while let Some(metrics) = rx.recv().await {
                addr.do_send(metrics);
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if let Ok(cmd) = serde_json::from_str::<serde_json::Value>(&text) {
                    match cmd.get("type").and_then(|t| t.as_str()) {
                        Some("subscribe") => {
                            if let Some(server_id) = cmd.get("server_id").and_then(|s| s.as_str()) {
                                self.server_id = Some(server_id.to_string());
                            }
                        }
                        Some("unsubscribe") => {
                            self.server_id = None;
                        }
                        _ => {}
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}