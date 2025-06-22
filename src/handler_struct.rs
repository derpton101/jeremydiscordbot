
use std::{io::Write, sync::Arc};
use serde::Serialize;
use tokio::sync::Mutex;

use std::io::Read;



use crate::{interactions::commands::generate_commands, messages::{handle_message::handle_message, linkmask}};


#[derive(serde::Deserialize, serde::Serialize)]
pub struct GuildSettings {
	pub has_premium: bool,
	pub lmsettings: linkmask::ALinkmaskSettings,
	pub silence_settings: crate::messages::silence::SilenceSettings,
	pub scanner_settings: crate::messages::scan::ScannerSettings,
}


impl GuildSettings {
	fn new() -> Self {
		Self{
			has_premium: false,
			lmsettings: linkmask::ALinkmaskSettings::new(),
			silence_settings: crate::messages::silence::SilenceSettings::new(),
			scanner_settings: crate::messages::scan::ScannerSettings::new(),
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GuildData {
	pub lmdata: linkmask::ALinkmaskData
}

impl GuildData {
	pub fn new() -> Self {
		Self {
			lmdata: linkmask::ALinkmaskData::new()
		}
	}
}


use std::collections::HashMap;
use serenity::{all::{Context, EventHandler, Message, Ready, UnavailableGuild}, async_trait, model::id::GuildId};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GuildInstance {
	pub gd: GuildData,
	pub gs: GuildSettings
}

impl GuildInstance {
	fn new() -> Self {
		Self{gd: GuildData::new(), gs: GuildSettings::new()}
	}
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GuildInstances {
	pub instances: HashMap<GuildId, GuildInstance>
}



impl GuildInstances {
	pub fn new() -> Self {
		Self{ instances: HashMap::new() }
	}
	pub async fn from_file(filename: &str) -> Self {

		if std::fs::exists(filename).expect("Error checking if file exists") {
			let file = std::fs::File::open(filename).expect("Error opening File!");
			let reader = std::io::BufReader::new(file);
			let gd = serde_json::from_reader(reader).expect("Error deserializing config file!");
			gd
		} else {
			Self::new()
		}
	}
	pub async fn save_to_file(&self, filename: &str) {
		let file = std::fs::File::create(filename);
		if let Ok(mut f) = file {
			let serialized = serde_json::to_string(&self);

			match serialized {
				Ok(s) => {
					if let Err(e) = f.write_all(s.as_bytes()) {
						eprintln!("Error writing save to file! {e:?}")
					}
				}
				Err(e) => {
					eprintln!("Error serializing data! {e:?}")
				}
			}

		}
	}
}


pub struct Handler {
	pub guild_instances: Arc<Mutex<GuildInstances>>,
}

use serenity::model::application::Interaction;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if let Some(_gid) =  msg.guild_id.clone() {
			handle_message(self.guild_instances.clone(), &ctx, &msg).await;
		}
		//Handle DMs
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Some(_gid) = &interaction.as_command().unwrap().guild_id {
			if let Some(cmd) = interaction.as_command() {
				crate::interactions::commands::handle_commands(self.guild_instances.clone(), &ctx, cmd).await;
			}
		}
	}

	async fn ready(&self, ctx: Context, _ready: Ready) {
		let _out  = serenity::all::Command::set_global_commands(ctx.http.clone(), generate_commands().await).await;
		if let Err(e) = _out {
			eprintln!("Error settings global commands! {e:?}")
		}
	}


	async fn guild_create(&self, _ctx: Context, guild: serenity::model::guild::Guild, _is_new: Option<bool>) {
		let mut gi = self.guild_instances.lock().await;
		gi.instances.insert(guild.id.clone(), GuildInstance::new());
	}

	async fn guild_delete(&self, _ctx: Context, incomplete: UnavailableGuild, _full: Option<serenity::all::Guild>) {
		self.guild_instances.lock().await.instances.remove(&incomplete.id);
	}
}