//! Async Server Sent Event parser and encoder.
//!
//! # Example
//!
//! ```norun
//! use tide::Request;
//!
//! #[async_std::main]
//! async fn main() -> http_types::Result<()> {
//!     let mut app = tide::new();
//!     
//!      app.at("/sse").get(|req| async move {
//!         let mut res = tide_compressed_sse::upgrade(req, |_req: Request<()>, sender| async move {
//!             sender.send("message", "foo", None).await?;
//!             
//!             Ok(())
//!         });
//!         
//!         Ok(res)
//!     });
//!     
//!     app.listen("localhost:8080").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! # References
//!
//! - [SSE Spec](https://html.spec.whatwg.org/multipage/server-sent-events.html#concept-event-stream-last-event-id)
//! - [EventSource web platform tests](https://github.com/web-platform-tests/wpt/tree/master/eventsource)

#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples)]

mod decoder;
mod encoder;
mod event;
mod handshake;
mod lines;
mod message;
mod tide;

use encoder::encode;
use event::Event;
use message::Message;
pub use crate::tide::upgrade::upgrade;
pub use crate::tide::Sender;

/// Exports for tests
#[cfg(feature = "__internal_test")]
pub mod internals {
    pub use crate::decoder::{decode, Decoder};
    pub use crate::encoder::{encode, Encoder};
    pub use crate::event::Event;
}

pub(crate) use lines::Lines;
