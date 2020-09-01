use chrono::Local;
use clap::ArgMatches;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const DEFAULT_FILE_NAME: &str = "t";

/* Information to include in file name */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileInfo<'a> {
	Date(&'a str),
	Timestamp,
}

impl<'a> FileInfo<'a> {
	/**
	 * Create a FileInfo enum from parsed arguments.
	 *
	 * @param  args
	 * @return FileInfo (Option)
	 */
	pub fn from_args(args: &'a ArgMatches<'a>) -> Option<Self> {
		if args.is_present("timestamp") {
			Some(Self::Timestamp)
		} else if args.is_present("date") && args.occurrences_of("date") != 0 {
			Some(Self::Date(args.value_of("date").unwrap_or_default()))
		} else {
			None
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for FileInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				FileInfo::Date(format) => Local::now().format(format).to_string(),
				FileInfo::Timestamp => Local::now().timestamp().to_string(),
			}
		)
	}
}

/* Format of the output file */
#[derive(Debug, PartialEq)]
pub enum FileFormat {
	Any,
	Gif,
	Png,
	Jpg,
	Bmp,
	Ico,
	Tiff,
	Pnm,
	Ff,
}

/* Display implementation for user-facing output */
impl fmt::Display for FileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self != &Self::Any {
			write!(f, "{:?}", self)
		} else {
			write!(f, "*")
		}
	}
}

/* Implementation for parsing FileFormat from a string */
impl FromStr for FileFormat {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"gif" => Ok(Self::Gif),
			"png" => Ok(Self::Png),
			"jpg" => Ok(Self::Jpg),
			"bmp" => Ok(Self::Bmp),
			"ico" => Ok(Self::Ico),
			"tiff" => Ok(Self::Tiff),
			"pnm" => Ok(Self::Pnm),
			"ff" => Ok(Self::Ff),
			_ => Err("Unrecognized file format"),
		}
	}
}

impl FileFormat {
	/**
	 * Create a FileFormat enum fron parsed arguments.
	 *
	 * @param  args
	 * @return FileFormat
	 */
	pub fn from_args<'a>(args: &'a ArgMatches<'a>) -> Self {
		match args.subcommand_matches(if args.is_present("edit") {
			"edit"
		} else if args.is_present("split") {
			"split"
		} else {
			"capture"
		}) {
			Some(matches) => {
				if matches.is_present("gif") {
					Self::Gif
				} else if matches.is_present("ff") {
					Self::Ff
				} else if matches.is_present("tiff") {
					Self::Tiff
				} else if matches.is_present("pnm") {
					Self::Pnm
				} else if matches.is_present("bmp") {
					Self::Bmp
				} else if matches.is_present("ico") {
					Self::Ico
				} else if matches.is_present("jpg") {
					Self::Jpg
				} else {
					Self::Png
				}
			}
			None => Self::Gif,
		}
	}
}

/* Representation of the output file */
#[derive(Debug)]
pub struct File {
	pub path: PathBuf,
	pub format: FileFormat,
}

impl File {
	/**
	 * Create a new File object.
	 *
	 * @param  path
	 * @param  format
	 * @return File
	 */
	pub fn new(path: PathBuf, format: FileFormat) -> Self {
		Self::create_path(&path);
		Self {
			path: Self::get_path_with_extension(path, &format),
			format,
		}
	}

	/**
	 * Create a new File object from file format.
	 *
	 * @param  format
	 * @return File
	 */
	pub fn from_format(format: FileFormat) -> Self {
		Self::new(
			Self::get_default_path(&format!(
				"{}.{}",
				DEFAULT_FILE_NAME,
				format.to_string().to_lowercase()
			)),
			format,
		)
	}

	/**
	 * Get the default path for a file.
	 *
	 * @param  file_name
	 * @param  PathBuf
	 */
	pub fn get_default_path(file_name: &str) -> PathBuf {
		dirs::picture_dir()
			.unwrap_or_else(|| {
				dirs::home_dir()
					.expect("Failed to access the home directory")
					.as_path()
					.join(env!("CARGO_PKG_NAME"))
			})
			.join(file_name)
	}

	/**
	 * Get the path with extension using the given file format.
	 *
	 * @param  path
	 * @param  format
	 * @return PathBuf
	 */
	pub fn get_path_with_extension(path: PathBuf, format: &FileFormat) -> PathBuf {
		match path.extension().and_then(OsStr::to_str) {
			Some("*") | None => {
				path.with_extension(format.to_string().to_lowercase())
			}
			_ => path,
		}
	}

	/**
	 * Create the path if it does not exist.
	 *
	 * @param path
	 */
	fn create_path(path: &Path) {
		if !path.exists() {
			fs::create_dir_all(&path.parent().expect("Failed to get the directory"))
				.expect("Failed to create directory");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg, SubCommand};
	#[test]
	fn test_file() {
		for format in vec!["png", "jpg", "bmp", "ico", "tiff", "pnm", "ff"] {
			let args = App::new("test")
				.subcommand(
					SubCommand::with_name("capture")
						.subcommand(SubCommand::with_name(format)),
				)
				.get_matches_from(vec!["test", "capture", format]);
			assert_eq!(
				File::get_default_path(&format!("t.{}", format))
					.to_str()
					.unwrap(),
				File::from_format(FileFormat::from_args(&args))
					.path
					.to_str()
					.unwrap()
			);
		}
		assert_eq!(
			"Gif",
			FileFormat::from_args(&App::new("test").get_matches_from(vec!["test"]))
				.to_string()
		);
		for info in vec!["", "date", "timestamp"] {
			let args = App::new("test")
				.arg(Arg::with_name(info).long(&format!("--{}", info)))
				.get_matches_from(vec!["test", &format!("--{}", info)]);
			assert_eq!(
				match info {
					"date" => {
						Some(FileInfo::Date(""))
					}
					"timestamp" => {
						Some(FileInfo::Timestamp)
					}
					_ => None,
				},
				FileInfo::from_args(&args)
			);
		}
	}
}
