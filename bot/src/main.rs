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

mod base;
mod core;
mod helper;

use poise::{Framework, FrameworkOptions};
use poise::serenity_prelude::{GatewayIntents};
use tokio::runtime::Builder;
use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use crate::base::{Config, Data};

fn main() {
    let (writer, _guards) = {
        let (outwriter, outguard) = tracing_appender::non_blocking(std::io::stdout());
        let outwriter = outwriter.with_min_level(Level::INFO);

        let (errwriter, errguard) = tracing_appender::non_blocking(std::io::stderr());
        let errwriter = errwriter.with_max_level(Level::WARN);

        let writer = outwriter
            .and(errwriter)
            .with_filter(|metadata| metadata
                .module_path()
                .map_or(false, |path| path.contains("bot"))
            );
        let guards = (outguard, errguard);

        (writer, guards)
    };

    let description = time::macros::format_description!(
        "[day]/[month]/[year] \
        [hour repr:24]:[minute]:[second].[subsecond digits:3]"
    );
    let timer = UtcTime::new(description);
    let format = tracing_subscriber::fmt::format()
        .with_ansi(true)
        .with_timer(timer)
        .with_target(false)
        .compact();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(writer)
        .event_format(format)
        .try_init()
        .expect("error initializing logger");

    let runtime = Builder::new_multi_thread()
        .thread_name("bot")
        .enable_all()
        .build()
        .expect("error building runtime");

    runtime.block_on(run());
}

async fn run() {
    let config = Config::open("config.toml").expect("error finding config.toml");
    let intents = GatewayIntents::empty();
    let options = FrameworkOptions {
        on_error: |e| Box::pin(core::on_error(e)),
        event_handler: |c, e, f, d| Box::pin(core::event_handler(c, e, f, d)),
        ..Default::default()
    };

    let framework = Framework::builder()
        .token(&config.core.token)
        .intents(intents)
        .options(options)
        .setup(|c, r, f| Box::pin(async move { Ok(Data::build(c, r, f).await?) }))
        .build().await
        .expect("error building framework");

    let shard_manager = framework.shard_manager().clone();

    tokio::spawn(async move {
        #[cfg(unix)] {
            use tokio::signal::unix::{self, SignalKind};

            let mut hangup = unix::signal(SignalKind::hangup())
                .expect("error listening SIGHUP");
            let mut interrupt = unix::signal(SignalKind::interrupt())
                .expect("error listening SIGINT");
            let mut terminate = unix::signal(SignalKind::terminate())
                .expect("error listening SIGTERM");

            tokio::select!{
                s = hangup.recv() => s.unwrap(),
                s = interrupt.recv() => s.unwrap(),
                s = terminate.recv() => s.unwrap(),
            };
        }

        #[cfg(windows)] {
            use tokio::signal::windows;

            let ctrl_break = windows::ctrl_break()
                .expect("error listening CTRL-BREAK");
            let ctrl_c = windows::ctrl_c()
                .expect("error listening CTRL-C");

            tokio::select!{
                s = ctrl_break.recv() => s.unwrap(),
                s = ctrl_c.recv() => s.unwrap(),
            };
        }

        shard_manager
            .lock().await
            .shutdown_all().await;
    });

    framework
        .start().await
        .expect("error starting framework");
}
