use std::{env, io::{self, Write}, str::FromStr};
use std::path::PathBuf;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.len() < 2 {
		println!("Usage: rnprefix FILE FILE...\n");
		return;
	}

	let paths: Vec<PathBuf> = args.into_iter().map(|filename| {
		PathBuf::from_str(&filename).expect(&format!("'{}' is not a valid filename!", filename))
	}).collect();

	let paths_stems: Vec<(PathBuf, String)> = paths.into_iter().map(|path| {
		if !path.exists() {
			println!("'{}' does not exist or cannot be accessed due to permissions!", path.to_str().unwrap());
		}

		if path.is_dir() {
			println!("'{}' is a directory! We only want files, sorry!", path.to_str().unwrap());
		}

		if !path.is_file() {
			println!("'{}' is not a file, not a directory, but exists? What's going on?", path.to_str().unwrap());
		}

		let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
		(
			path,
			stem
		)
	}).collect();

	let first_name = paths_stems[0].1.clone();

	'name_loop: for upper in (1..=first_name.len()).rev() {
		let test_str = &first_name[0..upper];
			println!("{}", test_str);
		for (_, stem) in paths_stems.iter().skip(1) {
			if stem.len() <= test_str.len() {
				break 'name_loop;
			}

			if !stem.starts_with(test_str) {
				continue 'name_loop;
			}
		}

		match rename_is_okay(&paths_stems, test_str) {
			Ok(do_rename) => {
				if do_rename {
					rename_files(paths_stems, test_str);
					return;
				}
			},
			Err(e) => {
				panic!("{}", e)
			}
		}
	}

	println!("Could not find a prefix!");
}

fn rename_is_okay(paths_stems: &Vec<(PathBuf, String)>, prefix: &str) -> io::Result<bool> {
	// Find the longest filename for dispaly purposes
	let mut longest_stem = 0;
	for (_, stem) in paths_stems {
		longest_stem = longest_stem.max(stem.len())
	}

	for (_, stem) in paths_stems {
		println!("{:min$} => {}", stem, stem.strip_prefix(prefix).unwrap(), min = longest_stem)
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

fn rename_files(paths_stems: Vec<(PathBuf, String)>, prefix: &str) {
	for (path, _) in paths_stems {
		let new_name = path.file_name().unwrap().to_str().to_owned().unwrap().strip_prefix(prefix).unwrap();
		let mut new_path = path.clone();
		new_path.set_file_name(new_name);
		match std::fs::rename(&path, &new_path) {
			Ok(_) => {
				println!("Moved {} to {}", path.to_str().unwrap(), new_path.to_str().unwrap())
			},
			Err(e) => {
				println!("Failed to move {}!\nError: {}", path.to_str().unwrap(), e)
			}
		}
	}
}