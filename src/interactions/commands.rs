

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::{Command, CommandInteraction, CommandOptionType, Context, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse, GuildId};

use crate::handler_struct::GuildInstances;
use crate::messages::scan::download_and_scan_file;

pub async fn generate_commands() -> Vec<CreateCommand> {
	let silence_commands = vec![
		CreateCommand::new("silence")
			.description("Toggles | Silences a use by means of deleting their message.")
			.add_option(
					CreateCommandOption::new(CommandOptionType::User, "user", "The User to be silenced")
						.required(true)
			),
		CreateCommand::new("silencemessage")
			.description("Optional String | Sends an Custom message replying to the user.")
			.add_option(
				CreateCommandOption::new(CommandOptionType::String, "string", "The message to be sent"),
			),
	];


	let alinkmask_commands = vec![
		CreateCommand::new("alinkmasktoggle")
			.description("Toggles | Toggles the activity of the antilinkmask feature."),
		CreateCommand::new("alinkmaskwhitelist")
			.description("User | Toggles a user being whitelisted from the antilinkmask feature.")
			.add_option(
				CreateCommandOption::new(CommandOptionType::User, "user", "User for the whitelist"),
			),
		CreateCommand::new("alinkmaskwhitelistclear")
			.description("None | Clears the antilinkmask whitelist."),
		CreateCommand::new("alinkmaskmaxoffenses")
			.description("Int | Sets the amount of offenses before taking the set action")
			.add_option(
				CreateCommandOption::new(CommandOptionType::Integer, "count", "The count of offenses that will trigger the set action. Default: 4")
					.required(false)
			),
		CreateCommand::new("alinkmaskresetoffenses")
			.description("User | Resets the offense count for a given user")
			.add_option(
				CreateCommandOption::new(CommandOptionType::User, "user", "User to reset the offense count of")
					.required(true)
			),
		CreateCommand::new("alinkmaskresetall")
			.description("None | Resets all users offense counts"),
		CreateCommand::new("alinkmaskaction")
			.description("Action | The action taken against the user after reaching the max offenses.")
			.add_option(
				CreateCommandOption::new(CommandOptionType::String, "action", "The action taken after offenses reached.")
					.add_string_choice("None", "none")
					.add_string_choice("Silence", "silence")
					.add_string_choice("Kick", "kick")
					.add_string_choice("Ban", "ban")
					.add_string_choice("HardBan", "hban")
			),
	];
	let scanner_commands = vec![
		CreateCommand::new("scannertoggle")
			.description("Toggles | Toggles the activity of the scanner functionality"),
		CreateCommand::new("scanfile")
			.description("File | Scans files giving a report for the blocks scanned.")
			.add_option(
				CreateCommandOption::new(CommandOptionType::Attachment, "file", "File to be scanned")
					.required(true)
			)
	];





	let compiled_commands = vec![alinkmask_commands, scanner_commands, silence_commands].concat();

	
	compiled_commands
}

pub async fn handle_commands(gi: std::sync::Arc<tokio::sync::Mutex<GuildInstances>>, ctx: &Context, cmd: &CommandInteraction) {
	let perms = match cmd.data.name.as_str() {
		"scanfile" => true,
		_ => {
			cmd.member.as_ref().unwrap().permissions.unwrap().contains(serenity::all::Permissions::ADMINISTRATOR)
		}
	};
	if !perms {
		if let Err(e) = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content("You do not have the permissions for this command.").ephemeral(true))).await {
			eprintln!("Error sending unprivledged response! {e:?}");
		}
		return;
	}
	match cmd.data.name.as_str() {
		// Start of silence commands
		"silence" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap(); // Bot should auto create settings on join.

			let contained = !ssettings.gs.silence_settings.users.insert(cmd.data.options[0].value.as_user_id().unwrap());
			let response = if contained {
				ssettings.gs.silence_settings.users.remove(&cmd.data.options[0].value.as_user_id().unwrap());
				"User unsilenced!"
			} else {
				"User silenced!"
			};
			 
			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(response).ephemeral(true))).await;
		}
		"silencemessage" => {
			let mut u_gi = gi.lock().await;
			let ssetings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();
			ssetings.gs.silence_settings.msg = if cmd.data.options.len() > 0 {
				cmd.data.options[0].value.as_str().map(|o| o.to_string())
			} else {
				None
			};

			let response = if cmd.data.options[0].value.as_str().is_none() {
				"Silence message removed!"
			} else {
				"Silence message added!"
			};

			let _ = cmd.create_response(ctx.http.clone(),CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(response).ephemeral(true))).await;
		}
		// End of silence commands


		// Start of AntiLinkmask commands 
		"alinkmasktoggle" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			ssettings.gs.lmsettings.active = !ssettings.gs.lmsettings.active;

			let response = if ssettings.gs.lmsettings.active {
				"Antilinkmask is turned on!"
			} else {
				"Antilinkmask is turned off!"
			};

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(response).ephemeral(true))).await;
		}
		"alinkmaskwhitelist" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			let contained = !ssettings.gs.lmsettings.whitelist.insert(cmd.data.options[0].value.as_user_id().unwrap());
			if contained {
				ssettings.gs.lmsettings.whitelist.remove(&cmd.data.options[0].value.as_user_id().unwrap());
			}

			let response = if contained {
				"User removed from whitelist!"
			} else {
				"User added to whitelist!"
			};
			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(response).ephemeral(true))).await;
		}
		"alinkmaskwhitelistclear" => {
			let mut u_gi = gi.lock().await;

			u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap().gs.lmsettings.whitelist.clear();

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Whitelist cleared!").ephemeral(true))).await;
		}
		"alinkmaskmaxoffenses" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			ssettings.gs.lmsettings.maxoffenses = cmd.data.options[0].value.as_i64().or(Some(3)).unwrap();

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Set max offenses!").ephemeral(true))).await;
		} 
		"alinkmaskresetall" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();
			
			ssettings.gd.lmdata.offenses.clear();

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Cleared all offense counts!").ephemeral(true))).await;
		}
		"alinkmaskresetoffenses" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			let _prev = ssettings.gd.lmdata.offenses.insert(cmd.data.options[0].value.as_user_id().unwrap(), 0);
			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Cleared user's offense count!").ephemeral(true))).await;
		}
		"alinkmaskaction" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			use crate::utils::ActionKind;

			let action = match cmd.data.options[0].value.as_str().unwrap() {
				"none" => ActionKind::None,
				"silence" => ActionKind::Silence,
				"kick" => ActionKind::Kick,
				"ban" => ActionKind::Ban,
				"hban" => ActionKind::HardBan,

				_ => unreachable!()
			};

			ssettings.gs.lmsettings.action = action;

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Set antilinkmask action!").ephemeral(true))).await;
		}
		// End Linkmask Commands

		// Start Scanner Commands
		"scannertoggle" => {
			let mut u_gi = gi.lock().await;
			let ssettings = u_gi.instances.get_mut(&cmd.guild_id.unwrap()).unwrap();

			ssettings.gs.scanner_settings.active = !ssettings.gs.scanner_settings.active;

			let response = if ssettings.gs.scanner_settings.active {
				"Scanner turned on!"
			} else {
				"Scanner turned off!"
			};

			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(response).ephemeral(true))).await;

		}
		"scanfile" => {
			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new().content("Scanning...").ephemeral(true))).await;
			tokio::task::yield_now().await;
			let att_id = cmd.data.options[0].value.as_attachment_id().unwrap();
			let att = cmd.data.resolved.attachments.get(&att_id).unwrap();


			match download_and_scan_file(att).await {
				Ok(results) => {
					// Send an embed of the clean, infected, and failed blocks.
					let infected = results.infected > 0;
					let broken = results.total() > 0 && results.failed as f32 / results.total() as f32 > 0.466f32;
					let color = if infected {
						serenity::model::colour::Color::from_rgb(255, 0, 0)
					} else if broken {
						serenity::model::colour::Color::from_rgb(255, 255, 0)
					} else {
						serenity::model::colour::Color::from_rgb(0, 255, 0)
					};
					let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content("Scan complete!").embed(serenity::builder::CreateEmbed::default()
						.title("Scan Results")
						.description(format!("Clean: {}\nInfected: {}\nFailed: {}", results.clean, results.infected, results.failed))
						.color(color)
						
					).ephemeral(true))).await;
					let _ = cmd.edit_response(ctx.http.clone(), EditInteractionResponse::new().content("Scan complete!").embed(serenity::builder::CreateEmbed::default()
						.title("Scan Results")
						.description(format!("Clean: {}\nInfected: {}\nFailed: {}", results.clean, results.infected, results.failed))
						.color(color))
					).await;
				}
				Err(e) => {
					let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content(format!("Error scanning file: {e:?}")).ephemeral(true))).await;
				}
			}
			
		}
		// End Scanner commands

		_ => {
			let _ = cmd.create_response(ctx.http.clone(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::default().content("Unimplemented Command").ephemeral(true))).await;
		}
	}
}