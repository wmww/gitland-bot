use std::ffi::OsString;
use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};

#[derive(Debug, PartialEq)]
pub enum Command {
	Show,
}

#[derive(Debug, PartialEq)]
pub struct Arguments
{
	pub server_repo_path: OsString,
	pub command: Command,
}

fn build_clap_app<'a>() -> App<'a, 'a> {
    App::new(crate_name!())
        .version(&crate_version!()[..])
        .author(crate_authors!("\n"))
        .about(crate_description!())
		.setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("SERVER_REPO")
                .short("s")
                .long("server-repo")
                .env("GITLAND_SERVER_REPO")
                .value_name("DIRECTORY")
                .help("Sets the directory to look for the server repo in")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Display the map")
                .version(&crate_version!()[..])
                .author(crate_authors!("\n")),
        )
}

fn parse_matches(matches: &ArgMatches) -> Arguments {
	let server_repo_path = matches.value_of_os("SERVER_REPO").expect("failed to find server repo in arguments").into();
	let command = match matches.subcommand_name() {
		Some("show") => Command::Show,
		Some(cmd) => panic!("unknown subcommand: {}", cmd),
		None => panic!("no subcommand"),
	};
	Arguments {
		server_repo_path,
		command,
	}
}

/// Will panic if anything is wrong
pub fn parse_arguments() -> Arguments {
	let app = build_clap_app();
	let matches = app.get_matches();
	parse_matches(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

	fn try_to_parse(args: &[&str]) {
		let app = build_clap_app();
		let all_args = std::iter::once(&"./self").chain(args.iter());
		let _matches = app.get_matches_from_safe(all_args).expect("failed to get matches");
	}

	fn assert_parses_to(args: &[&str], expected: Arguments) {
		let app = build_clap_app();
		let all_args = std::iter::once(&"./self").chain(args.iter());
		let matches = app.get_matches_from_safe(all_args).expect("failed to get matches");
		let actual = parse_matches(&matches);
		assert_eq!(actual, expected);
	}

	#[test]
    fn show() {
        assert_parses_to(
			&["--server-repo", "foobar", "show"],
			Arguments {
				server_repo_path: "foobar".into(),
				command: Command::Show,
			}
		);
    }

	#[test]
	#[should_panic(expected = "failed to get matches")]
    fn invalid_subcommand() {
		try_to_parse(&["--server-repo", "foobar", "invalid"]);
    }

	#[test]
	#[should_panic(expected = "kind: VersionDisplayed")]
    fn version() {
		try_to_parse(&["--version"]);
    }
}