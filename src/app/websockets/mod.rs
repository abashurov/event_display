use crate::actix::*;
use actix_web::*;

use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct Ws {
  hb: Instant,
  access_token: String,
};

impl Actor for Ws {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);
  }
}

impl Ws {
  fn new() -> Self {
    Self { 
      hb: Instant::now(),
      access_token: "".to_string(),
    }
  }

  fn hb(&self, ctx: &mut <Self as Actor>::Context) {
    ctx.run_interval(HEARTBEAT_INTERVAL, | act, ctx | {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        info!("Client has disconnected: Timeout reached");
        ctx.stop();
        return
      }
      ctx.ping("");
    })
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}