// src/lib.rs
use worker::*;
#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
  match (req.method(), req.path().as_str()) {
    (&Method::Get, "/ping") => Response::ok("pong"),
    (&Method::Get, "/") => Response::ok("Hello from Rust Worker! 🌍"),
    _ => Response::error("Not found", 404),
  }
}
