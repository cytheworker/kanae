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

use poise::serenity_prelude::{Activity, OnlineStatus};
use tokio::time::Duration;
use crate::helper::{Context, Result};
use crate::owner::{ActivityType, StatusType};

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn presence(
    context: Context<'_>,
    status: StatusType,
    activity: Option<ActivityType>,
    #[rest] name: Option<String>,
) -> Result<()> {
    let activity = match (activity, name) {
        (Some(activity), Some(name)) => {
            Some(match activity {
                ActivityType::Playing => Activity::playing(name),
                ActivityType::Listening => Activity::listening(name),
                ActivityType::Watching => Activity::watching(name),
                ActivityType::Competing => Activity::competing(name),
            })
        }
        (Some(activity), None) => {
            Some(match activity {
                ActivityType::Playing => Activity::playing("..."),
                ActivityType::Listening => Activity::listening("..."),
                ActivityType::Watching => Activity::watching("..."),
                ActivityType::Competing => Activity::competing("..."),
            })
        }
        _ => None,
    };

    let status = match status {
        StatusType::Dnd => OnlineStatus::DoNotDisturb,
        StatusType::Idle => OnlineStatus::Idle,
        StatusType::Invisible => OnlineStatus::Invisible,
        StatusType::Offline => OnlineStatus::Offline,
        StatusType::Online => OnlineStatus::Online,
    };

    context.say("setting presence...").await?;
    context
        .serenity_context()
        .set_presence(activity, status).await;

    Ok(())
}

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn shutdown(context: Context<'_>, after: Option<u64>) -> Result<()> {
    let framework = context.framework();
    let owner = framework.user_data().await.owner();
    let mut owner = owner.lock().await;
    let mut response = String::new();

    if let Some(shutdown) = owner.shutdown.take() {
        response.push_str("aborting existing shutdown task!");
        shutdown.abort();
    }

    let shard_manager = framework.shard_manager();

    if let Some(after) = after {
        if !(1..=60).contains(&after) {
            response.push_str("\"after\" parameter must be in between 1 and 60");
            context.say(response).await?;

            return Ok(())
        }

        let channel_id = context.channel_id();
        let http = context.serenity_context().http.clone();

        owner.shutdown.replace(tokio::spawn(async move {
            response.push_str(&format!("\nshutting down in about {after} minutes!"));
            channel_id.say(&http, response).await?;

            tokio::time::sleep(Duration::from_secs(after * 60)).await;

            channel_id.say(&http, "shutting down...").await?;
            shard_manager
                .lock().await
                .shutdown_all().await;

            Result::Ok(())
        }));
    } else {
        response.push_str("\nshutting down...");
        context.say(response).await?;

        shard_manager
            .lock().await
            .shutdown_all().await;
    }

    Ok(())
}
