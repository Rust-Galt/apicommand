use apicommand::{configuration::Config, get, last_run, run, specific};
use tracing::Level;

use std::path::PathBuf;

use clap::{arg, command, Arg, ArgAction, Command};
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("api_key")
                .short('k')
                .long("api_key")
                .help("Optional API authentication key"),
        )
        .arg(
            Arg::new("api_root")
                .short('r')
                .long("api_root")
                .default_value("https://httpbin.org/anything")
                .help("api root for requests"),
        )
        .arg(
            Arg::new("database_path")
                .short('d')
                .long("database_path")
                .default_value("test.sqlite3")
                .value_parser(clap::value_parser!(PathBuf))
                .help("Database path"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .global(true)
                .help("Increase logging verbosity"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .global(true)
                .conflicts_with("verbose")
                .help("Silences output"),
        )
        .subcommand(
            Command::new("get")
                .visible_alias("g")
                .about("get API request")
                .arg(arg!(<brand_id> "Valid brand id")),
        )
        .subcommand(
            Command::new("last_run")
                .visible_alias("l")
                .about("last run API request")
                .arg(arg!(<brand_id> "Valid brand id"))
                .arg(arg!(<location_id> "Valid location id")),
        )
        .subcommand(
            Command::new("run")
                .visible_alias("r")
                .about("run API request")
                .arg(arg!(<brand_id> "Valid brand id"))
                .arg(arg!(<location_id> "Valid location id")),
        )
        .subcommand(
            Command::new("specific")
                .visible_alias("s")
                .about("specific API request")
                .arg(arg!(<brand_id> "Valid brand id"))
                .arg(arg!(<location_id> "Valid location id"))
                .arg(arg!(<from_date> "Unix timestamp(ms)"))
                .arg(arg!(<to_date> "Unix timestamp(ms) >= `from_date`")),
        )
        .get_matches();

    // Enable tracing subscriber and get verbosity level from cli parameters
    let level = match (matches.get_flag("quiet"), matches.get_count("verbose")) {
        // Default while developing
        (false, 0) => Level::INFO,

        (false, 1) => Level::WARN,
        (false, 2) => Level::INFO,
        (false, 3) => Level::DEBUG,
        (false, _) => Level::TRACE,
        // Always show errors
        (_, _) => Level::ERROR,
    };
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // sets this to be the default, global collector for this application.
        .init();

    // Extract parameters for config
    let config = Config::builder()
        .api_key(matches.get_one::<String>("api_key").cloned())
        .api_root(
            matches
                .get_one::<String>("api_root")
                .expect("Default is set in clap")
                .to_string(),
        )
        .db_path(
            matches
                .get_one::<PathBuf>("database_path")
                .expect("Default is set in clap")
                .to_owned(),
        )
        .build();

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let raw_brand_id = sub_matches
                .get_one::<String>("brand_id")
                .expect("String parses any input")
                .to_owned();
            // Call function in library with raw parameters
            let r = get(&config, raw_brand_id).await?;
            println!("{}", r.url.as_str());
        }
        Some(("last_run", sub_matches)) => {
            let raw_brand_id = sub_matches
                .get_one::<String>("brand_id")
                .expect("String parses any input")
                .to_owned();
            let raw_location_id = sub_matches
                .get_one::<String>("location_id")
                .expect("String parses any input")
                .to_owned();
            // Call function in library with raw parameters
            let r = last_run(&config, raw_brand_id, raw_location_id).await?;
            println!("{}", r.url.as_str());
        }
        Some(("run", sub_matches)) => {
            let raw_brand_id = sub_matches
                .get_one::<String>("brand_id")
                .expect("String parses any input")
                .to_owned();
            let raw_location_id = sub_matches
                .get_one::<String>("location_id")
                .expect("String parses any input")
                .to_owned();
            // Call function in library with raw parameters
            let r = run(&config, raw_brand_id, raw_location_id).await?;
            println!("{}", r.url.as_str());
        }
        Some(("specific", sub_matches)) => {
            let raw_brand_id = sub_matches
                .get_one::<String>("brand_id")
                .expect("String parses any input")
                .to_owned();
            let raw_location_id = sub_matches
                .get_one::<String>("location_id")
                .expect("String parses any input")
                .to_owned();
            let raw_from_date = sub_matches
                .get_one::<String>("from_date")
                .expect("String parses any input")
                .to_owned();
            let raw_to_date = sub_matches
                .get_one::<String>("to_date")
                .expect("String parses any input")
                .to_owned();

            // Call function in library with raw parameters
            let r = specific(
                &config,
                raw_brand_id,
                raw_location_id,
                raw_from_date,
                raw_to_date,
            )
            .await?;
            println!("{}", r.url.as_str());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    Ok(())
}
