



#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScannerSettings {
	pub active: bool,
}

impl ScannerSettings {
	pub fn new() -> Self {
		Self {active: true}
	}
}

#[derive(Clone, Copy)]
pub struct ScanResult {
	pub clean: u32,
	pub infected: u32,
	pub failed: u32,
}

impl ScanResult {
	fn new(c: u32, i: u32, f: u32) -> Self {
		Self {
			clean: c,
			infected: i,
			failed: f,
		}
	}


	pub fn total(&self) -> u32 {
		self.clean + self.infected + self.failed
	}
}

use reqwest::Client;
//Output: Scanner Abnormalities/Plausible virus detection.
pub async fn download_and_scan_file(att: &serenity::all::Attachment) -> Result<ScanResult, Box<dyn std::error::Error + Send + Sync>> {
	let cl = Client::new();

	use futures_util::StreamExt;
	let response = cl.get(att.url.clone()).send().await;


	if response.is_err() {
		let err = response.unwrap_err();
		println!("Failed to get response! : {err:?}");
		Err(err)?;
		unreachable!();
	} 
	let response = response.unwrap();

	if !response.status().is_success() {
		println!("Response was a failure! : {:?}", response.status());
		return Err("Shitty response".into());
	}

	let mut stream = response.bytes_stream();

	let mut blocks = Vec::new();
	while let Some(chunk) = stream.next().await {
		if let Err(e) = chunk {
			println!("Error getting chunk! {e:?}");
			blocks.push(None);
			continue;
		}
		let chunk = chunk.unwrap();
		blocks.push(Some(scan_by_buffer(chunk.to_vec()).await));
	}
	let mut clean = 0;
	let mut infected = 0;
	let mut failed = 0;
	for block in blocks {
		match block {
			Some(r) => {
				match r {
					Ok(e) => {
						if e.is_some() {
							infected = infected + 1
						} else {
							clean = clean + 1
						}
					}
					Err(_e) => {
						failed = failed + 1
					}
				}
			}
			None => {
				failed = failed + 1
			}
		}
	}
	Ok(ScanResult::new(clean, infected, failed))
}


//100mb
pub async fn scan_by_buffer(file_in_buffer: Vec<u8>) -> Result<Option<String>, String> {
	let scan = clamav_client::scan_buffer(file_in_buffer.as_slice(), clamav_client::Socket{ socket_path: "/var/run/clamav/clamd.ctl" }, None);
	if let Err(e) = scan {
		println!("Error scanning file! : {e:?}");
		return Err("Failed to scan!".into());
	}
	let (res, clean) = {
		let scan = scan.unwrap();

		let pretty_scan = std::string::String::from_utf8(scan.clone()).unwrap();
		
		(pretty_scan, clamav_client::clean(scan.as_slice()).unwrap())
	};

	match clean {
		true => {
			Ok(None)
		}
		false => {
			Ok(Some(res))
		}
	}
	
}
