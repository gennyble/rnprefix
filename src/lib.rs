use std::{
	borrow::Cow,
	ffi::{OsStr, OsString},
	fmt,
	path::{Path, PathBuf},
};

// This doesn't need to be a lib but I needed to store state for the prefix
// resolution so I could write tests, so here we are.
pub struct Rnprefix {
	//FIXME: this is pub for test :(
	pub files: Vec<TypedFile>,
}

impl Rnprefix {
	pub fn new<I, P>(files: I) -> Result<Self, Error>
	where
		I: Iterator<Item = P>,
		P: Into<PathBuf>,
	{
		let mut file_vec = vec![];

		if file_vec.len() == 0 {
			return Err(Error::NoFiles);
		}

		for pbuf in files {
			file_vec.push(TypedFile::new(pbuf.into())?);
		}

		Ok(Self { files: file_vec })
	}

	pub fn prefixes(&self) -> Prefixes {
		Prefixes::new(self)
	}

	pub fn files(&self) -> &[TypedFile] {
		&self.files
	}
}

pub struct Prefixes<'r> {
	rn: &'r Rnprefix,
	// We can't index OsString (or OsStr) so we have this. It's lossyily converted, but the
	// prefixes are presented to the user so perhaps it's fine
	name: &'r str,
	offset: usize,
}

impl<'r> Prefixes<'r> {
	fn new(rn: &'r Rnprefix) -> Self {
		let name = rn.files[0].name.as_str();

		let mut this = Prefixes {
			rn: &rn,
			name,
			offset: name.len(),
		};
		this.eliminate_entirely_similar();

		this
	}

	fn eliminate_entirely_similar(&mut self) {
		'prefix_loop: loop {
			let prefix = match self.next() {
				None => return,
				Some(prefix) => prefix,
			};

			for thing in self.rn.files.iter().skip(1) {
				if thing.name.len() <= prefix.len() {
					continue 'prefix_loop;
				}

				if !thing.name.starts_with(prefix) {
					continue 'prefix_loop;
				}
			}

			// We made it through which means we've reached a prefix that starts
			// every file name!
			// Manually move the iterator back one; the last prefix we consumed
			// was the one we want to present next.
			self.offset += 1;
			break;
		}
	}
}

impl<'r> Iterator for Prefixes<'r> {
	type Item = &'r str;

	fn next(&mut self) -> Option<Self::Item> {
		self.offset = self.offset.saturating_sub(1);

		if self.offset == 0 {
			return None;
		} else {
			Some(&self.name[0..self.offset])
		}
	}
}

pub struct TypedFile {
	pub parent: Option<PathBuf>,
	pub name: String,
}

impl TypedFile {
	pub fn new(pbuf: PathBuf) -> Result<Self, Error> {
		check_path(&pbuf)?;

		let parent = pbuf.parent().map(<_>::to_owned);
		let name = match pbuf.file_name().unwrap().to_str() {
			None => return Err(Error::FileNameNotUtf8 { path: pbuf }),
			Some(name) => name.to_owned(),
		};

		Ok(Self { parent, name })
	}

	pub fn path(&self) -> PathBuf {
		if let Some(parent) = self.parent.as_ref() {
			parent.join(&self.name)
		} else {
			PathBuf::from(&self.name)
		}
	}
}

#[derive(Debug)]
pub enum Error {
	NoFiles,
	FileNameNotUtf8 { path: PathBuf },
	PathIsNotFile { path: PathBuf },
	PathIsDirectory { path: PathBuf },
	Io(std::io::Error),
}
impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::NoFiles => {
				write!(f, "No files")
			}
			Error::FileNameNotUtf8 { path } => write!(
				f,
				"The file at '{}' does not have a UTF8 name",
				path.as_os_str().to_string_lossy()
			),
			Error::PathIsNotFile { path } => write!(
				f,
				"The path '{}' is not a file",
				path.as_os_str().to_string_lossy()
			),
			Error::PathIsDirectory { path } => write!(
				f,
				"The path '{}' is a directory",
				path.as_os_str().to_string_lossy()
			),
			Error::Io(e) => write!(f, "{e}"),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(ioe: std::io::Error) -> Self {
		Error::Io(ioe)
	}
}

fn check_path(path: &Path) -> Result<(), Error> {
	let meta = path.metadata()?;

	if meta.is_dir() {
		Err(Error::PathIsDirectory {
			path: path.to_path_buf(),
		})
	} else if !meta.is_file() {
		Err(Error::PathIsNotFile {
			path: path.to_path_buf(),
		})
	} else {
		Ok(())
	}
}
