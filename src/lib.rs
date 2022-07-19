use std::error::Error;
use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Programs {
	list: Vec<String>,
}

impl Programs {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		let config = get_config()?;
		check_config(&config)?;
		let json = fs::read_to_string(config)?;
		let programs: Programs = serde_json::from_str(&json)?;
		Ok(programs)
	}

	pub fn save(&self) -> Result<(), Box<dyn Error>> {
		let config = get_config()?;
		fs::write(config, serde_json::to_string(&self)?)?;
		Ok(())
	}

	pub fn generate_install(&self) -> Result<(), Box<dyn Error>> {
		let mut dir = get_config_dir()?;
		dir.push("install.sh");
		let mut post_install = get_config_dir()?;
		post_install.push("postInstall.sh");
		let installs: String = self.list.iter().map(|item| item.clone() + " ").collect();
		let install = format!(
			"
#!/bin/sh
sudo pacman --needed -S base-devel git
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si

yay
yay --noconfirm -S {}

#=== Post install script ===#

{}
",
			installs,
			fs::read_to_string(post_install)?
		);
		fs::write(&dir, install)?;
		let mut perms = fs::metadata(&dir)?.permissions();
		perms.set_mode(0o777);
		fs::set_permissions(dir, perms)?;
		Ok(())
	}

	pub fn add(&mut self, value: String) -> Result<(), Box<dyn Error>> {
		self.list = Programs::load()?.list;
		self.list.push(value);
		self.save()?;
		Ok(())
	}
	pub fn remove(&mut self, value: String) -> Result<(), Box<dyn Error>> {
		self.list = Programs::load()?.list;
		self.list = self
			.list
			.iter()
			.filter_map(|item| {
				if item.clone() != value {
					return Some(item.clone());
				}
				None
			})
			.collect();
		self.save()?;
		Ok(())
	}

	pub fn list(&self) {
		println!("Current Commands are:");
		for (index, program) in self.list.iter().enumerate() {
			println!("    {:<4}{}", index, program);
		}
	}
}

fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
	let mut config_dir = config_dir().unwrap();
	config_dir.push("perm_install");
	check_dir(&config_dir)?;
	Ok(config_dir)
}

fn get_config() -> Result<PathBuf, Box<dyn Error>> {
	let mut dir = get_config_dir()?;
	dir.push("config");
	dir.set_extension("json");
	Ok(dir)
}

fn check_dir(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.exists() {
		return Ok(());
	} else {
		fs::create_dir_all(dir)?;
	}
	Ok(())
}
fn check_config(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.exists() {
		return Ok(());
	} else {
		Programs { list: vec![] }.save()?;
	}
	Ok(())
}
