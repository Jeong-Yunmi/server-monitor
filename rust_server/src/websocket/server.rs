// server/src/websocket/server.rs
use actix::{AsyncContext, ActorContext, Actor, StreamHandler, SpawnHandle, WrapFuture};
use actix::ActorFutureExt;
use actix_web_actors::ws;
use actix_web::web;
use tokio::time::{Duration, Instant};
use serde_json::json;
use std::collections::HashMap;
use crate::monitoring;
use crate::monitoring::MonitoringService;
use std::sync::{Mutex, Arc};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketConnection {
    last_heartbeat: Instant,
    monitoring_service: web::Data<MonitoringService>,
    subscription_handles: Arc<Mutex<HashMap<String, SpawnHandle>>>,
    server_id: Option<String>,
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.schedule_heartbeat(ctx);
        self.start_metrics_stream(ctx);
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
                if let Ok(command) = serde_json::from_str::<serde_json::Value>(&text) {
                    self.handle_command(command, ctx);
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

impl WebSocketConnection {
    pub fn new(monitoring_service: web::Data<MonitoringService>) -> Self {
        Self {
            last_heartbeat: Instant::now(),
            monitoring_service,
            subscription_handles: Arc::new(Mutex::new(HashMap::new())),
            server_id: None,
        }
    }

    fn schedule_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn start_metrics_stream(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let monitoring = monitoring::MonitoringService::new();
        let handle = ctx.run_interval(Duration::from_secs(1), move |actor, ctx| {
            let monitoring = monitoring.clone();
            
            let fut = async move {
                if let Some(metrics) = monitoring.get_current_metrics().await {
                    let message = json!({
                        "type": "metrics",
                        "data": metrics
                    });
                    message.to_string()
                } else {
                    "".to_string()
                }
            }
            .into_actor(actor) // Use actor parameter instead of self
            .map(|result, _, ctx| {
                if !result.is_empty() {
                    ctx.text(result);
                }
            });
            
            ctx.wait(fut);
        });

        self.subscription_handles.lock().unwrap().insert("global".to_string(), handle);
    }

    fn handle_command(&mut self, command: serde_json::Value, ctx: &mut ws::WebsocketContext<Self>) {
        if let Some(cmd_type) = command.get("type").and_then(|v| v.as_str()) {
            match cmd_type {
                "subscribe" => {
                    if let Some(server_id) = command.get("server_id").and_then(|v| v.as_str()) {
                        // Cancel existing subscription
                        match self.subscription_handles.lock() {
                            Ok(handles) => {
                                if let Some(handle) = handles.get(server_id) {
                                    ctx.cancel_future(*handle);
                                }
                            },
                            Err(e) => {
                                log::error!("Failed to lock subscription handles: {}", e);
                            }
                        }

                        let _monitoring_service = monitoring::MonitoringService::new(); // or similar initialization
                        let server_id = self.server_id.clone();
                        
                        let handle = ctx.run_interval(Duration::from_secs(1), move |actor, ctx| {
                            let monitoring = monitoring::MonitoringService::new();
                            let server_id = server_id.clone();
                            
                            let fut = async move {
                                if let Some(server_id_str) = server_id {  // Unwrap Option<String>
                                    if let Some(metrics) = monitoring.get_server_metrics(&server_id_str).await {
                                        let message = json!({
                                            "type": "server_metrics",
                                            "server_id": server_id_str,
                                            "data": metrics
                                        });
                                        message.to_string()
                                    } else {
                                        "".to_string()
                                    }
                                } else {
                                    "".to_string()
                                }
                            }
                            .into_actor(actor)  // Changed from self to actor
                            .map(|result, _, ctx| {
                                if !result.is_empty() {
                                    ctx.text(result);
                                }
                            });
                            
                            ctx.wait(fut);
                        });

                        if let Some(server_id_str) = server_id {
                            if let Ok(mut handles) = self.subscription_handles.lock() {
                                handles.insert(server_id_str.to_string(), handle);
                            } else {
                                log::error!("Failed to lock subscription handles");
                            }
                        } else {
                            log::error!("Server ID is None");
                        }
                    }
                }
                "unsubscribe" => {
                    if let Some(server_id) = command.get("server_id").and_then(|v| v.as_str()) {
                        if let Ok(mut handles) = self.subscription_handles.lock() {
                            if let Some(handle) = handles.remove(server_id) {
                                ctx.cancel_future(handle);
                            }
                        } else {
                            log::error!("Failed to lock subscription handles during unsubscribe");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}