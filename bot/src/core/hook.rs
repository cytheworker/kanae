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

use poise::{FrameworkContext, FrameworkError, Event};
use poise::serenity_prelude as serenity;
use crate::base::Data;
use crate::helper::{Error, Result};

pub async fn event_handler(
    _: &serenity::Context,
    event: &Event<'_>,
    _: FrameworkContext<'_, Data, Error>,
    _: &Data,
) -> Result<()> {
    match event {
        Event::Ready { .. } => tracing::info!("connection ready"),
        Event::Resume { .. } => tracing::info!("session resumed"),
        _ => (),
    };

    Ok(())
}

pub async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            let message = "error building data";
            tracing::info!(message, error);
        }
        FrameworkError::EventHandler { error, event, .. } => {
            let message = "error dispatching handler";
            let event = event.name();
            tracing::info!(message, event, error);
        }
        FrameworkError::Command { error, ctx } => {
            let message = "error invoking command";
            let (name, arguments) = match ctx {
                poise::Context::Application(ctx) => {
                    let name = &ctx.command.qualified_name;
                    let pairs = ctx.args
                        .iter()
                        .map(|option| {
                            let value = match &option.value {
                                Some(value) => value.to_string(),
                                None => "...".to_owned(),
                            };
                            format!("{}={}", option.name, value)
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    let arguments = format!("({})", pairs);
                    (name, arguments)
                }
                poise::Context::Prefix(ctx) => {
                    let name = &ctx.command.qualified_name;
                    let arguments = ctx.args.to_owned();
                    (name, arguments)
                }
            };
            tracing::info!(message, name, arguments, error);
        }
        _ => (),
    };
}
