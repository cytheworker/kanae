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
use poise::serenity_prelude::{self as serenity, CommandDataOption};
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
            tracing::error!(
                message = "error building data",
                error,
            );
        }
        FrameworkError::EventHandler { error, event, .. } => {
            tracing::error!(
                message = "error dispatching handler",
                event = event.name(),
                error,
            );
        }
        FrameworkError::Command { error, ctx } => {
            match ctx {
                poise::Context::Application(context) => {
                    let mut arguments = String::new();
                    for argument in context.args
                        .iter()
                        .map(|CommandDataOption { name, value, .. }| {
                            let value = value
                                .as_ref()
                                .map_or_else(|| "...".to_owned(), |value| value.to_string());
                            format!("{name}={value} ")
                        }) {
                        arguments.push_str(&argument);
                    }

                    tracing::error!(
                        message = "error invoking application command",
                        name = context.command.qualified_name,
                        arguments = format!("({})", arguments.trim_end()),
                        error,
                    );
                }
                poise::Context::Prefix(context) => {
                    tracing::error!(
                        message = "error invoking prefix command",
                        name = context.command.qualified_name,
                        arguments = context.args,
                        error,
                    );
                }
            };
        }
        _ => (),
    };
}
