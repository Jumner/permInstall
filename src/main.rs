use clap::{App, Arg};
use perm::Programs;

fn main() {
	let matches = App::new("Perm")
		.about("A nice easy install helper written in rust.")
		.version(env!("CARGO_PKG_VERSION"))
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
		)
		.get_matches();
	let mut programs = Programs::load().unwrap();

	if let Some(value) = matches.value_of("Add") {
		programs.add(value.to_string()).unwrap();
	}
	if let Some(value) = matches.value_of("Remove") {
		programs.remove(value.to_string()).unwrap();
	}
	programs.generate_install().unwrap();
}
