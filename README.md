# apicommand

## Features

## Commands

- Api:
  - lastRun ()
  - specific "from" "to"
  - run
  - get "brand_id"

log full json string on every api request in sqlite db

- `--version`/`-V` Version
- `--help`/`-h` Help/About

## Test

To run test in lib.rs: `cargo test --lib -- --nocapture`

## Structure

The application is split into a binary and library. The binary will support a cli interface and call methods in the library.

### Library

- `lib.rs` provides primary functions
  - get
  - last_run
  - run
  - specific
- `config.rs` provides configuration for api and database
- `database.rs` will contain all code related to database IO
- `network.rs` contains all network request code
- `validate.rs` validates raw inputs from the cli and creates valid structs that can be used in requests to the api

## Cli Usage

```
Usage: apicommand [OPTIONS] <COMMAND>
Commands:
  get       get API request [aliases: g]
  last_run  last run API request [aliases: l]
  run       run API request [aliases: r]
  specific  specific API request [aliases: s]
  help      Print this message or the help of the given subcommand(s)

Options:
  -k, --api_key <api_key>              Optional API authentication key
  -r, --api_root <api_root>            api root for requests [default: https://httpbin.org/anything]
  -d, --database_path <database_path>  Database path [default: test.db]
  -h, --help                           Print help
  -V, --version                        Print version
```
