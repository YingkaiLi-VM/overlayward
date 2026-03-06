extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Marks a function as an API handler with automatic guardian check,
/// error mapping, and response wrapping.
///
/// Applied to axum handler functions to eliminate per-handler boilerplate.
/// Full implementation deferred to Phase 2 (REST API).
#[proc_macro_attribute]
pub fn api_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Phase 2: will inject guardian.check() + ApiResponse wrapping
    item
}

/// Derive macro for generating `From` conversions between ow-types
/// and protobuf generated types.
///
/// Full implementation deferred to Phase 3 (gRPC).
#[proc_macro_derive(ProtoConvert, attributes(proto))]
pub fn proto_convert(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
