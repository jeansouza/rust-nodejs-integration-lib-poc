use napi_derive::napi;
use greeting::{greet, test::Who};

pub mod greeting;

#[napi]
fn hello_world_node_js() {
  greet("Hello", Who::You);
}
