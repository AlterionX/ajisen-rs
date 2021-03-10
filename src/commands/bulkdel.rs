use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;

use log::error;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::iter;

async fn check_member_perm(ctx: &mut Context, member: &Member, p: Permissions) -> bool {
	for role in &member.roles {
		if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(p)) {
			return true;
		}
	}
	false
}

fn two_weeks_ago() -> DateTime {
	Utc::now() - Duration::weeks(2)
}

// TOOD Perhaps make this return (u64, CommandResult)?
async fn delete_bulk_and_single(ctx: &mut Context, chan: &ChannelId, range: (Message, Message)) -> CommandResult<u64> {
	if (range.0.id == range.1.id) {
		chan.delete_message(&ctx.http, m_to_del).await?;
		return Ok(1);
	}

	let time_lim = two_weeks_ago();
	let mut amount_deleted = 0;

	let msgs_after_start = chan.messages(&ctx.http, |r| r.after(range.0).limit(100)).await?;

	let mut msgs_in_range = vec![];
	let mut latest_msg = range.0;
	// Reversed, since I'm pretty sure that it's chronological... but not certain. Needs test to see if should actually reverse.
	// API doesn't guarantee ordering, but I think that an ordered list of messages is the path of least resistance.
	for m in msgs_after_start.into_iter().rev() {
		if m.timestamp > range.1.timestamp {
			continue;
		} else if m.timestamp > latest_msg.timestamp {
			msgs_in_range.push(latest_msg);
			latest_msg = m;
		} else {
			msgs_in_range.push(m);
		}
	}
	
	let mut bulk_del_msgs = vec![];
	for m in msgs_in_range {
		if m.timestamp < time_lim {
			chan.delete_message(&ctx.http, m).await?; // TODO possibly batch this at the end?
			amount += 1;
		} else if m.timestamp < range.1.timestamp {
			bulk_del_msgs.push(m);
		}
	}

	if bulk_del_msgs.len() == 0 {
		// ...?
	} else if bulk_del_msgs.len() == 1 {
		chan.delete_message(&ctx.http, bulk_del_msgs[0]).await?;
		amount += 1;
	} else {
		chan.delete_messages(&ctx.http, bulk_del_msgs).await?;
		amount += bulk_del_msgs.len();
	}
	
	amount += delete_bulk_and_single(ctx, chan, (latest_msg, range.1))?;
	Ok(amount)
}

// TODO Need to consider rate limiting.
async fn bulkdel_no_cleanup(ctx: &mut Context, msg: &Message, mut args: Args) -> (Vec<MessageId>, CommandResult) {
	let mut sent = vec![msg.id];
	
	let agent = if let Some(member) = &msg.member {
		member
	} else {
		return (sent, Ok(()));
	};

	if !check_member_perm(ctx, agent, Permissions::MANAGE_MESSAGES) {
		let notification = MessageBuilder::new()
			.mention(&msg.author)
			.push(" lacks permissions.")
			.build();
		match msg.channel_id.say(&ctx.http, &notification).await {
			Err(reason) => error!("Error sending lacking permissions notification for command `{}`: {:?}", msg.content, reason);
			Ok(msg) => sent.push(msg.id);
		}
		return (sent, Ok(()));
	}
	
	let status = {
		let output = MessageBuilder::new()
			.mention(&msg.author)
			.push(", we are working on your bulk deletion request, please hold on...")
			.build();
		match msg.channel_id.say(&ctx.http, &output).await {
			Ok(m) => {
				sent.push(m.id);
				m
			},
			Err(reason) => return (sent, Ok(()));
		}
	};

	let range = {
		let bookend_0 = match msg.channel_id.message(&ctx.http, args.single::<u64>()) {
			Ok(m) => m,
			Err(reason) => return (sent, Ok(()));
		};
		let bookend_1 = match msg.channel_id.message(&ctx.http, args.single::<u64>()) {
			Ok(m) => m,
			Err(reason) => return (sent, Ok(()));
		};
		// We technically could have compared the ids directly, but there's no guarantee that they won't change into UUIDs.
		if bookend_0.timestamp < bookend_1.timestamp {
			(bookend_0, bookend_1)
		} else {
			(bookend_1, bookend_0)
		}
	};

	let amount = match delete_bulk_and_single(ctx, msg, range) {
		Ok(n) => n,
		Err(reason) => return (send, Ok(()));
	};

	let completion_message = format!("Request processed. Deleted {} messages. Cleaning up...", amount);
	if let Err(reason) = status.edit(ctx, |m| m.content(completion_message)).await {
		error!("Failed to edit message. Attempting to send new message.");
		unimplemented!("How do I send this message? (aka I'm lazy af.)");
	}

	(sent, Ok(()))
}

#[command]
#[description("
**Bulk Delete**
`~del A B`, where `A` is one end of the range of messages and `B` is the other end. It does not matter which one was sent earlier as long as both are in the same channel. This range is inclusive.
")]
#[max_levenshtein_distance(0)]
#[help_available]
#[min_args(2)]
#[max_args(2)]
// TODO Proper error handling for intermediate discord API calls.
// TODO Consider a "confirmation" message.
pub async fn bulkdel(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
	tokio::time::sleep(Duration::seconds(20)).await;
	let msg_ids = [msg.id, err_msg.id];
	if let Err(res) = msg.channel_id.delete_messages(ctx, &[msg, err_msg]) {
		unimplemented!("Log error.");
	}

	Ok(())
}

// TODO create `bulkdel` group with subcommands `time` and `id`. Current impl is for `id`.