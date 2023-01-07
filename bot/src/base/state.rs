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
use serde::Deserialize;
use crate::{core, owner};
use crate::helper::{ArcMut, Error, Result};

// ==================== BUILDER ==================== //

pub async fn data(
    context: &Context,
    ready: &Ready,
    framework: &Framework<Data, Error>,
) -> Result<Data> {
    let data = Data {
        owner: owner::data(context, ready, framework).await?,
    };

    Ok(data)
}

pub fn config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;

    Ok(config)
}

// ==================== STRUCT ==================== //

pub struct Data {
    pub owner: ArcMut<owner::Data>,
}

#[derive(Deserialize)]
pub struct Config {
    pub core: core::Config,
}

// ==================== IMPL ==================== //

impl Data {
    pub fn owner(&self) -> ArcMut<owner::Data> {
        self.owner.clone()
    }
}
