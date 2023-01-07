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

use poise::Framework;
use poise::serenity_prelude::{Context, Ready};
use tokio::task::JoinHandle;
use crate::{base, helper};
use crate::helper::{ArcMut, Error, Result};

pub async fn data(
    _: &Context,
    _: &Ready,
    _: &Framework<base::Data, Error>,
) -> Result<ArcMut<Data>> {
    let data = helper::arcmut(Data {
        shutdown: None,
    });

    Ok(data)
}

pub struct Data {
    pub shutdown: Option<JoinHandle<Result<()>>>,
}
