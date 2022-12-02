use std::path::PathBuf;
use std::process;
use std::{
	env,
	io::{self, Write},
};

use rnprefix::{Rnprefix, TypedFile};

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.len() < 2 {
		eprintln!("Usage: rnprefix FILE FILE...\n");
		process::exit(1);
	}

	let rn = Rnprefix::new(args.into_iter()).unwrap();
	let prefixes = rn.prefixes();

	for prefix in prefixes {
		match rename_is_okay(rn.files(), prefix) {
			Ok(do_rename) => {
				println!();
				if do_rename {
					rename_files(rn.files(), prefix);
					return;
				}
			}
			Err(e) => {
				panic!("{}", e)
			}
		}
	}

	println!("Could not find a prefix!");
}

fn rename_is_okay(files: &[TypedFile], prefix: &str) -> io::Result<bool> {
	// Find the longest filename for dispaly purposes
	let mut longest_stem = 0;
	for file in files {
		longest_stem = longest_stem.max(file.name.len())
	}

	for file in files {
		println!(
			"{:min$} => {}",
			file.name,
			file.name.strip_prefix(prefix).unwrap(),
			min = longest_stem
		)
	}
	println!("Prefix is '{}'", prefix);

	let mut buffer = String::new();
	loop {
		print!("Are these names okay? (y/n) ");
		io::stdout().flush().expect("Failed to flush stdout!");

		match io::stdin().read_line(&mut buffer) {
			Ok(_) => (),
			Err(e) => {
				return Err(e);
			}
		}

		let input = buffer.to_lowercase();
		let input = input.trim();

		if input == "y" {
			return Ok(true);
		} else if input == "n" {
			return Ok(false);
		} else {
			println!("Please answer with a single 'y' for yes, or 'n' for no");
		}
	}
}

fn rename_files(files: &[TypedFile], prefix: &str) {
	for file in files {
		let new_name = file.name.strip_prefix(prefix).unwrap();
		let new_path = if let Some(parent) = file.parent.as_ref() {
			parent.clone().join(new_name)
		} else {
			PathBuf::from(new_name)
		};

		match std::fs::rename(file.path(), &new_path) {
			Ok(_) => {
				println!(
					"Moved {} to {}",
					file.path().to_str().unwrap(),
					new_path.to_str().unwrap()
				)
			}
			Err(e) => {
				println!(
					"Failed to move {}!\nError: {}",
					file.path().to_str().unwrap(),
					e
				)
			}
		}
	}
}

#[cfg(test)]
mod test {
	use rnprefix::{Rnprefix, TypedFile};

	fn make_rn() -> Rnprefix {
		macro_rules! typed {
			($name:literal) => {
				TypedFile {
					parent: None,
					name: String::from($name),
				}
			};
		}

		let typed = vec![
			typed!("PREFIX One"),
			typed!("PREFIX Two"),
			typed!("PREFIX Three"),
		];

		Rnprefix { files: typed }
	}

	#[test]
	fn finds_prefix() {
		let rn = make_rn();

		let prefixes = vec!["PREFIX ", "PREFIX", "PREFI", "PREF", "PRE", "PR", "P"];
		let actual: Vec<&str> = rn.prefixes().collect();

		assert_eq!(prefixes, actual)
	}
}
