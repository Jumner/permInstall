use std::error::Error;
use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use clap::{App, Arg};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Programs {
	list: Vec<String>,
}

impl Programs {
	fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
		let mut config_dir = config_dir().unwrap();
		config_dir.push("perm_install");
		Programs::check_dir(&config_dir)?;
		Ok(config_dir)
	}

	fn get_config() -> Result<PathBuf, Box<dyn Error>> {
		let mut dir = Programs::get_config_dir()?;
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

	pub fn load() -> Result<Self, Box<dyn Error>> {
		let config = Programs::get_config()?;
		Programs::check_config(&config)?;
		let json = fs::read_to_string(config)?;
		let programs: Programs = serde_json::from_str(&json)?;
		Ok(programs)
	}

	pub fn save(&self) -> Result<(), Box<dyn Error>> {
		println!("Saving State: {:?}", self);
		let config = Programs::get_config()?;
		fs::write(config, serde_json::to_string(&self)?)?;
		Ok(())
	}

	pub fn generate_install(&self) -> Result<(), Box<dyn Error>> {
		let mut dir = Programs::get_config_dir()?;
		dir.push("install.sh");
		println!("{:?}", dir);
		let installs: String = self.list.iter().map(|item| item.clone() + " ").collect();
		let install = format!(
			"
sudo pacman --needed -S base-devel git
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si

yay
yay --noconfirm -S {}
",
			installs
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
}

fn main() {
	let app = App::new("Perm")
		.about("A nice easy install helper written in rust.")
		.version("0.1.0")
		.author("Jumner")
		.arg(
			Arg::with_name("Add")
				.short("S")
				.long("add")
				.takes_value(true)
				.value_name("Program")
				.help("Add a Program to the installer"),
		)
		.arg(
			Arg::with_name("Remove")
				.short("R")
				.long("Remove")
				.takes_value(true)
				.value_name("Program")
				.help("Remove a program from the installer"),
		);
	let matches = app.get_matches();
	let mut programs = Programs::load().unwrap();

	if let Some(value) = matches.value_of("Add") {
		programs.add(value.to_string()).unwrap();
	}
	if let Some(value) = matches.value_of("Remove") {
		programs.remove(value.to_string()).unwrap();
	}
	matches.value_of("Add");
	programs.generate_install().unwrap();
	println!("{:?}", programs);
	println!("Hello, world!");
}
