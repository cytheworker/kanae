// Copyright 2023 cytheworker
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

use std::{error, result};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::base::Data;

pub type Error = Box<dyn error::Error + Send + Sync>;
pub type Result<T> = result::Result<T, Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ArcMut<T> = Arc<Mutex<T>>;

pub fn arcmut<T>(value: T) -> ArcMut<T> {
    Arc::new(Mutex::new(value))
}
