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

#[derive(Copy, Clone)]
struct Die(u32);
impl Die {
    fn roll<R: Rng>(self, rng: &mut R) -> u32 {
        rng.gen_range(1, self.0 + 1)
    }
}
fn roll_dice<D: iter::Iterator<Item = Die>>(dice: D) -> u32 {
    let rng = &mut thread_rng();

    dice.fold(0, |accum, die| accum + die.roll(rng))
}

#[command]
#[description("
**Roll**
`~roll NdM`, where `N` is number of dice and `M` is number of sides in each dice.
")]
#[example("1d6")]
#[help_available]
#[delimiters("d")]
#[min_args(2)]
#[max_args(2)]
pub fn roll(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_dice = args.single::<u32>().unwrap();
    let num_sides = args.single::<u32>().unwrap();

    let dice = iter::repeat(Die(num_sides)).take(num_dice as usize);
    let result = roll_dice(dice);

    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" Result of running ")
        .push_bold_safe(msg.content.replace("~roll ", ""))
        .push(" was ")
        .push_bold_safe(result)
        .build();

    if let Err(reason) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending response for command `{}`: {:?}", msg.content, reason);
    }
    Ok(())
}
