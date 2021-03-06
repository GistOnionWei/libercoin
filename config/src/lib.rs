// Copyright 2018 The Grin Developers
// Copyright 2019 The Libercoin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Crate wrapping up the Libercoin binary and configuration file

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;

use libercoin_core as core;
use libercoin_p2p as p2p;
use libercoin_servers as servers;
use libercoin_util as util;

mod comments;
pub mod config;
pub mod types;

pub use crate::config::initial_setup_server;
pub use crate::types::{ConfigError, ConfigMembers, GlobalConfig};
