
use crate::handler_struct::GuildInstances;

use serenity::client::Context;
use serenity::model::channel::Message;

use tokio::sync::Mutex;

use std::sync::Arc;


use crate::messages::scan::download_and_scan_file;

pub async fn handle_message(gi: Arc<Mutex<GuildInstances>>, ctx: &Context, msg: &Message) {

	// Silence User start
	let mut lcok = gi.lock().await;

	let guild_instance = lcok.instances.get_mut(&msg.guild_id.unwrap()).unwrap();

	super::silence::silence_user(&guild_instance, &msg, ctx.http.clone()).await;

	// Silence User End

	// Fire linkmask handler

	if super::linkmask::contains_masked_links(&msg.content).await {
		super::linkmask::alinkmask_handle(gi.clone(), ctx, &msg).await;
	}
	// End of linkmask handler 


	

	// If a message has attachments, scan them.

	if !msg.attachments.is_empty() {
		for file in &msg.attachments {
			let results = download_and_scan_file(file).await;
			if let Ok(re) = &results {
				let re= re.clone();
				if re.infected > 0 {
					let r = msg.reply(ctx.http.clone(), "Message contains an infected block.").await;
					if let Err(e) = &r {
						eprintln!("Error in scan infected reply. {e:?}");
					}
					if let Err(e) = msg.delete(ctx.http.clone()).await {
						eprintln!("Error in infected message Delete. {e:?}");
					}
				}
				if re.failed as f32 / re.total() as f32 > 0.466f32 {
					if let Err (e) = msg.reply(ctx.http.clone(), "Message contains too many failed blocks from scanning.").await {
						eprintln!("Err in failed scan message reply. {e:?}");
					}
					if let Err(e) = msg.delete(ctx.http.clone()).await {
						eprintln!("Err in failed scan message delete. {e:?}");
					}
				}

			}
		}
	}

	// End of file scanning




}