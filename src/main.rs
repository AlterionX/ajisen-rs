mod commands;
mod logging;
mod settings;

use log::{error, info};
use serenity::{
    framework::standard::{
        Args,
        StandardFramework,
        CommandGroup, CommandResult,
        HelpOptions, help_commands,
        macros::{group, help},
    },
    model::{
        event::ResumedEvent, gateway::Ready,
        channel::Message, id::UserId
    },
    prelude::*,
};
use std::collections::HashSet;


use commands::{
    die_rolls::*,
};

group!({
    name: "general",
    options: {},
    commands: [roll]
});

#[help]
#[individual_command_tip =
"You can find a list of commands below.\n
To get the details on a specific command, pass the command name as an argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[suggestion_text = "Try `{}` instead."]
#[strikethrough_commands_tip_in_dm(None)]
#[strikethrough_commands_tip_in_guild(None)]
#[max_levenshtein_distance(3)]
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

struct BasicHandler;

impl EventHandler for BasicHandler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    let settings = settings::read().expect("Failed to parse the settings");
    logging::setup(&settings.logging).expect("Failed to setup the logger");

    let mut client = Client::new(&settings.discord.token, BasicHandler).expect("Err creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix("~"))
        .help(&HELP)
        .unrecognised_command(|ctx, msg, unrecognised_command_name| {
            let display_text = format!(
                "Sorry, I didn't recognize this command: `{}`. Could you try again?",
                unrecognised_command_name,
            );
            if let Err(reason) = msg.channel_id.say(&ctx.http, &display_text) {
                error!("{}", reason);
            }
        })
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
