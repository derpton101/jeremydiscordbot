

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum ActionKind {
	None,
	Silence,
	Kick,
	Ban,
	HardBan
}
