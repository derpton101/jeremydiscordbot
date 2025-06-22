

use regex::Regex;
use serenity::all::CacheHttp;

use crate::{handler_struct::GuildInstances, utils::ActionKind, secrets::SUPERWHITELISTED_IDS as superwhitelisted_ids};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
// Per guild settings for the Anti Link Mask functions
pub struct ALinkmaskSettings {
	pub active: bool,
	pub whitelist: std::collections::HashSet<serenity::all::UserId>,
	pub maxoffenses: i64,
	pub action: ActionKind
}

impl ALinkmaskSettings {
	pub fn new() -> Self {
		Self {
			active: true,
			whitelist: std::collections::HashSet::new(),
			maxoffenses: 3,
			action: ActionKind::Kick,
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ALinkmaskData {
	pub offenses: std::collections::HashMap<serenity::all::UserId, i64>
}

impl ALinkmaskData {
	pub fn new() -> Self {
		Self {
			offenses: std::collections::HashMap::new()
		}
	}
}

pub async fn contains_masked_links(msgtext: &str) -> bool {
	let re = Regex::new(r"\[\s*([^\]]*?)\s*]\(\s*(https?://[^\s)]+)\s*\)").unwrap();
	if re.is_match(msgtext) {
		true
	} else {
		false
	}
}



pub async fn alinkmask_handle(guild_instances: std::sync::Arc<tokio::sync::Mutex<GuildInstances>>, ctx: &serenity::client::Context, msg: &serenity::model::prelude::Message) {
	let mut lcock = guild_instances.lock().await;
	let guild_instance = lcock.instances.get_mut(&msg.guild_id.unwrap()).unwrap();
	
	let settings = guild_instance.gs.lmsettings.clone();

	if !guild_instance.gs.lmsettings.active {
		return;
	}
	if superwhitelisted_ids.contains(&msg.author.id.get()) {
		return;
	}
	{
		let off = guild_instance.gd.lmdata.offenses.entry(msg.author.id.clone()).or_insert(0);
		*off = *off + 1;
	}
	let count = guild_instance.gd.lmdata.offenses.get(&msg.author.id).unwrap_or(&0).clone();

	if count > settings.maxoffenses {
		let gid = msg.guild_id.unwrap();
		let guild = ctx.cache.guild(gid).unwrap().clone();
		match settings.action {
			ActionKind::None => (),
			ActionKind::Silence => {
				guild_instance.gs.silence_settings.users.insert(msg.author.id);
			},			
			ActionKind::Kick => guild.kick(ctx.http.clone(), msg.author.id.clone()).await.unwrap_or(()),
			ActionKind::Ban => guild.ban_with_reason(ctx.http.clone(), msg.author.id.clone(), 0, "User most likely posting phishing links").await.unwrap_or_else(|_e| {/* Yeah, don't really give a shit, user probably just doeSn't have perms*/()}),
			ActionKind::HardBan => guild.ban_with_reason(ctx.http.clone(), msg.author.id.clone(), 7, "User most likely posting phishing links").await.unwrap_or_else(|_e| {/* Yeah, don't really give a shit, user probably just doeSn't have perms*/()}),
		}	
	}
}

