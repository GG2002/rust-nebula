/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(warnings)]

use std::i32;

macro_rules! bail_err {
    ($e:expr) => {
        return Err(From::from($e))
    };
}

macro_rules! ensure_err {
    ($cond:expr, $e:expr) => {
        if !$cond {
            bail_err!($e);
        }
    };
}

use anyhow::Result;

#[macro_use]
pub mod protocol;

pub mod application_exception;
pub mod binary_protocol;
pub mod binary_type;
pub mod builtin_types;
pub mod compact_protocol;
pub mod context_stack;
pub mod deserialize;
pub mod export;
pub mod framing;
pub mod processor;
pub mod request_context;
pub mod serialize;
pub mod simplejson_protocol;
pub mod thrift_protocol;
pub mod ttype;
pub mod uri;

mod bufext;
mod client;
mod errors;
mod exceptions;
mod varint;

#[doc(hidden)]
pub mod help;

pub mod types {
    // Define ApplicationException as if it were a normal generated type to make things simpler
    // for codegen.
    pub use super::application_exception::ApplicationException;
}

pub use super::fbthrift::application_exception::ApplicationException;
pub use super::fbthrift::application_exception::ApplicationExceptionErrorCode;
pub use super::fbthrift::binary_protocol::BinaryProtocol;
pub use super::fbthrift::bufext::BufExt;
pub use super::fbthrift::bufext::BufMutExt;
pub use super::fbthrift::client::ClientFactory;
pub use super::fbthrift::client::Transport;
pub use super::fbthrift::compact_protocol::CompactProtocol;
pub use super::fbthrift::context_stack::ContextStack;
pub use super::fbthrift::context_stack::DummyContextStack;
pub use super::fbthrift::context_stack::SerializedMessage;
pub use super::fbthrift::deserialize::Deserialize;
pub use super::fbthrift::errors::NonthrowingFunctionError;
pub use super::fbthrift::errors::ProtocolError;
pub use super::fbthrift::exceptions::ExceptionInfo;
pub use super::fbthrift::exceptions::ResultInfo;
pub use super::fbthrift::exceptions::ResultType;
pub use super::fbthrift::framing::Framing;
pub use super::fbthrift::framing::FramingDecoded;
pub use super::fbthrift::framing::FramingEncoded;
pub use super::fbthrift::framing::FramingEncodedFinal;
pub use super::fbthrift::help::NoopSpawner;
pub use super::fbthrift::processor::NullServiceProcessor;
pub use super::fbthrift::processor::ReplyState;
pub use super::fbthrift::processor::SerializedStreamElement;
pub use super::fbthrift::processor::ServiceProcessor;
pub use super::fbthrift::processor::ThriftService;
pub use super::fbthrift::protocol::Field;
pub use super::fbthrift::protocol::Protocol;
pub use super::fbthrift::protocol::ProtocolDecoded;
pub use super::fbthrift::protocol::ProtocolEncoded;
pub use super::fbthrift::protocol::ProtocolEncodedFinal;
pub use super::fbthrift::protocol::ProtocolReader;
pub use super::fbthrift::protocol::ProtocolWriter;
pub use super::fbthrift::request_context::DummyRequestContext;
pub use super::fbthrift::request_context::RequestContext;
pub use super::fbthrift::serialize::Serialize;
pub use super::fbthrift::simplejson_protocol::SimpleJsonProtocol;
pub use super::fbthrift::thrift_protocol::MessageType;
pub use super::fbthrift::thrift_protocol::ProtocolID;
pub use super::fbthrift::ttype::GetTType;
pub use super::fbthrift::ttype::TType;
pub use super::fbthrift::uri::GetUri;

pub trait ThriftEnum: Sized {
    fn enumerate() -> &'static [(Self, &'static str)];

    fn variants() -> &'static [&'static str];

    fn variant_values() -> &'static [Self];
}

/// Set the default ID's for unknown exceptions and fields.
/// When reading off the wire, these default values will be
/// overridden with the unrecognized id (which must be nonnegative).
// ---
// Keep in sync with the UNKNOWN_ID constant in //common/rust/thrift/ast.
pub const __UNKNOWN_ID: i32 = i32::MIN;
