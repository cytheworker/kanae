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

use poise::serenity_prelude::{Activity, GuildId, OnlineStatus};
use tokio::time::Duration;
use crate::helper::{Context, Result};
use crate::owner::{ActivityType, StatusType};

#[poise::command(
    prefix_command, owners_only, guild_only,
    rename = "owner",
    subcommands("avatar", "presence", "register", "shutdown"),
)]
pub async fn group(_: Context<'_>) -> Result<()> {
    Ok(())
}

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn avatar(context: Context<'_>) -> Result<()> {
    let Context::Prefix(prefix_context) = context else { unreachable!() };
    let attachments = &prefix_context.msg.attachments;

    if attachments.is_empty() {
        context.say("attachment is required!").await?;
        return Ok(())
    }

    let serenity_context = context.serenity_context();
    let base64 = attachments[0].download().await.map(base64::encode)?;
    let avatar = format!("data:image/png;base64,{base64}");

    context.say("setting avatar...").await?;
    serenity_context.cache
        .current_user()
        .edit(&serenity_context, |profile| profile.avatar(Some(&avatar))).await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn presence(
    context: Context<'_>,
    status: StatusType,
    activity: Option<ActivityType>,
    #[rest] name: Option<String>,
) -> Result<()> {
    let name = name.unwrap_or_else(|| "...".to_owned());
    let activity = activity.map(|activity| {
        match activity {
            ActivityType::Playing => Activity::playing(name),
            ActivityType::Listening => Activity::listening(name),
            ActivityType::Watching => Activity::watching(name),
            ActivityType::Competing => Activity::competing(name),
        }
    });
    let status = match status {
        StatusType::Dnd => OnlineStatus::DoNotDisturb,
        StatusType::Idle => OnlineStatus::Idle,
        StatusType::Invisible => OnlineStatus::Invisible,
        StatusType::Offline => OnlineStatus::Offline,
        StatusType::Online => OnlineStatus::Online,
    };

    context.say("setting presence...").await?;
    context.serenity_context().set_presence(activity, status).await;

    Ok(())
}

#[poise::command(prefix_command, owners_only, guild_only)]
pub async fn register(context: Context<'_>, scope: String) -> Result<()> {
    let serenity_context = context.serenity_context();
    let commands = &context.framework().options().commands;

    match scope.as_str() {
        "local" => {
            context.say("registering commands locally...").await?;
            let guild_id = context.guild_id().unwrap();
            poise::builtins::register_in_guild(&serenity_context, commands, guild_id).await?;
        }
        "global" => {
            context.say("registering commands globally...").await?;
            poise::builtins::register_globally(&serenity_context, commands).await?;
        }
        scope => {
            let Ok(guild_id) = scope.parse::<u64>().map(GuildId) else {
                context.say("\"scope\" parameter must be \"local\", \"global\", or GUILD_ID!").await?;
                return Ok(())
            };

            let response = format!("registering commands for {guild_id}...");
            context.say(response).await?;
            poise::builtins::register_in_guild(&serenity_context, commands, guild_id).await?;
        }
    }

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

    let Some(after) = after else {
        response.push_str("\nshutting down...");
        context.say(response).await?;
        shard_manager.lock().await.shutdown_all().await;

        return Ok(())
    };

    if !(1..=60).contains(&after) {
        response.push_str("\n\"after\" parameter must be in between 1 and 60!");
        context.say(response).await?;

        return Ok(())
    }

    let channel_id = context.channel_id();
    let http = context.serenity_context().http.clone();

    let shutdown = tokio::spawn(async move {
        let text = format!("\nshutting down in about {after} minutes!");
        response.push_str(&text);
        channel_id.say(&http, response).await?;

        let duration = Duration::from_secs(after * 60);
        tokio::time::sleep(duration).await;

        channel_id.say(&http, "shutting down...").await?;
        shard_manager.lock().await.shutdown_all().await;

        Result::Ok(())
    });
    owner.shutdown.replace(shutdown);

    Ok(())
}
