use crate::encode::settings::GifSettings;
use crate::record::settings::RecordSettings;
use crate::util;
use crate::util::cmd::Command;
use chrono::Local;
use clap::ArgMatches;
use std::str::FromStr;

/* General application settings */
#[derive(Clone, Debug)]
pub struct AppSettings {
	pub args: ArgMatches<'static>,
	pub gif: GifSettings,
	pub record: RecordSettings,
}

impl AppSettings {
	/**
	 * Create a new AppSettings object.
	 *
	 * @param  args
	 * @return AppSettings
	 */
	pub fn new(args: ArgMatches<'static>) -> Self {
		Self {
			args: args.clone(),
			gif: Self::get_gif_settings(args.clone()),
			record: Self::get_record_settings(args),
		}
	}

	/**
	 * Get a Command object from parsed arguments.
	 *
	 * @return Command
	 */
	pub fn get_command(&self) -> Command {
		match self.args.value_of("command") {
			Some(cmd) => {
				let cmd = String::from(cmd);
				if !cmd.contains(' ') {
					Command::new(cmd, Vec::new())
				} else {
					Command::new(String::from("sh"), vec![String::from("-c"), cmd])
				}
			}
			None => panic!("No command specified to run"),
		}
	}

	/**
	 * Get the output file from parsed arguments.
	 *
	 * @return String
	 */
	pub fn get_output_file(&self) -> String {
		match self.args.subcommand_matches("save") {
			Some(matches) => {
				let mut file_name =
					String::from(matches.value_of("output").unwrap_or_default());
				if matches.is_present("prompt") {
					file_name = rprompt::prompt_reply_stdout("Enter file name: ")
						.unwrap_or(file_name);
				}
				if matches.is_present("date") || matches.is_present("timestamp") {
					util::update_file_name(
						file_name,
						if matches.is_present("date") {
							Local::now().format("%Y%m%dT%H%M%S").to_string()
						} else {
							Local::now().timestamp().to_string()
						},
					)
				} else {
					file_name
				}
			}
			None => String::from("t.gif"),
		}
	}

	/**
	 * Get recording settings from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	fn get_record_settings(args: ArgMatches<'static>) -> RecordSettings {
		RecordSettings::from_args(
			args.subcommand_matches("record"),
			u64::from_str_radix(args.value_of("color").unwrap_or("FF00FF"), 16)
				.expect("Failed to parse the color HEX"),
		)
	}

	/**
	 * Get GIF settings from parsed arguments.
	 *
	 * @param  args
	 * @return GifSettings
	 */
	fn get_gif_settings(args: ArgMatches<'static>) -> GifSettings {
		match args.subcommand_matches("gif") {
			Some(matches) => {
				let parser = ArgParser::new(&matches);
				GifSettings::new(
					parser.parse("repeat", -1),
					parser.parse("speed", 10),
				)
			}
			None => GifSettings::default(),
		}
	}
}
