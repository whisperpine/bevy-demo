//! Illustrates how to make a single system from multiple functions running in sequence,
//! passing the output of the first into the input of the next.

// #![cfg_attr(debug_assertions, allow(unused))]

use bevy::log::{Level, LogPlugin};
use bevy::log::{debug, error, info, warn};
use bevy::prelude::*;
use std::num::ParseIntError;

fn main() {
    println!("\n#### system_piping ####\n");

    App::new()
        .insert_resource(Message("666.0".to_owned()))
        .insert_resource(OptionalWaring(Err("yahaha".to_owned())))
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_systems(Update, parse_int.pipe(log_parse_int))
        .add_systems(
            Update,
            (
                data_output.map(|out| debug!(out)),
                data_output.map(|out| info!(out)),
            ),
        )
        .add_systems(
            Update,
            (
                warning_output.map(|out| {
                    if let Err(err) = out {
                        warn!(err)
                    }
                }),
                warning_output.map(|out| {
                    if let Err(err) = out {
                        error!(err)
                    }
                }),
            ),
        )
        .run();
}

#[derive(Resource)]
struct Message(String);

#[derive(Resource)]
struct OptionalWaring(Result<(), String>);

fn data_output(message: Res<Message>) -> String {
    message.0.clone()
}

fn warning_output(warning: Res<OptionalWaring>) -> Result<(), String> {
    warning.0.clone()
}

fn parse_int(message: Res<Message>) -> Result<i32, ParseIntError> {
    message.0.parse::<i32>()
}

fn log_parse_int(In(result): In<Result<i32, ParseIntError>>) {
    if let Err(error) = result {
        error!("{}", error);
    }
}
