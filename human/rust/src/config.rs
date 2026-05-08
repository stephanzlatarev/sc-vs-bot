use std::path::PathBuf;
use sc2_proto::common::Race;

pub struct Config {
	pub sc2_path: PathBuf,
	pub sc2_port: u16,
	pub sc2_version: String,
	pub map_name: String,
	pub player_name: String,
	pub player_race: Race,
	pub local_host: String,
	pub local_server_port: u16,
	pub local_client_port: u16,
	pub remote_host: String,
	pub remote_server_port: u16,
	pub remote_client_port: u16,
}

impl Config {
	pub fn new() -> Self {
		Self {
			sc2_path: PathBuf::from(r"C:\Program Files (x86)\StarCraft II"),
			sc2_port: 10001,
			sc2_version: "Base75689".to_string(),
			map_name: "LeyLinesAIE_v3".to_string(),
			player_name: "Human".to_string(),
			player_race: Race::Random,
			local_host: "127.0.0.1".to_string(),
			local_server_port: 10004,
			local_client_port: 10005,
			remote_host: "209.38.114.125".to_string(),
			remote_server_port: 10044,
			remote_client_port: 10055,
		}
	}
}
