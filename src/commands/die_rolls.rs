use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;

use log::error;
use rand::{thread_rng, Rng};
use std::iter;

fn roll_dice(num_sides: &Vec<u32>) -> u32 {
    let mut rng = thread_rng();

    num_sides
        .iter()
        .fold(0, |accum, num_sides| {
            accum + rng.gen_range(1, num_sides + 1)
        })
}

#[command]
#[description("Roll one or more dice.")]
#[usage("NdM, where N is number of dice and M is number of sides in each dice.")]
#[example("1d6")]
#[help_available]
#[delimiters("d")]
#[min_args(2)]
#[max_args(2)]
pub fn roll(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_dice = args.single::<u32>().unwrap();
    let num_sides = args.single::<u32>().unwrap();

    let sides = iter::repeat(num_sides).take(num_dice as usize).collect();
    let result = roll_dice(&sides);

    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" Result of running")
        .push_bold_safe(format!(" {}", msg.content.replace("~roll ", "")))
        .push(" was")
        .push_bold_safe(format!(" {}", result))
        .build();

    if let Err(reason) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending message: {:?}", reason);
    }
    Ok(())
}
