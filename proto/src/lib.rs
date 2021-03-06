// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate futures;
extern crate grpcio;
extern crate protobuf;
#[macro_use(slog_o, slog_kv)]
extern crate slog;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;

pub mod testing {
    include!(concat!(env!("OUT_DIR"), "/testing/mod.rs"));
}

pub mod example {
    include!(concat!(env!("OUT_DIR"), "/example/mod.rs"));
}

pub mod util;
