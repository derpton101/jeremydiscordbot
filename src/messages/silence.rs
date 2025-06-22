use crate::{handler_struct::GuildInstance, secrets::SUPERWHITELISTED_IDS};
use serenity::builder::CreateMessage;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SilenceSettings {

	pub users: std::collections::HashSet<serenity::all::UserId>,

	pub msg: Option<String>
}

impl SilenceSettings {
	pub fn new() -> Self {
		Self {
			users: std::collections::HashSet::new(),
			msg: None
		}
	}
}

pub async fn silence_user(gi: &GuildInstance, msg: &serenity::all::Message, http: std::sync::Arc<serenity::http::Http>) {
	if gi.gs.silence_settings.users.contains(&msg.author.id) && !SUPERWHITELISTED_IDS.contains(&msg.author.id.get()) {
		if msg.delete(http.clone()).await.is_err() {
			//How did we get here			
		}
		if let Some(message) = &gi.gs.silence_settings.msg {
			let _ = msg.channel_id.send_message(http.clone(), CreateMessage::new().content(message).reference_message(msg)).await;
		}
	}
}