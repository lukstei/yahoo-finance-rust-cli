# yahoo-finance-rust-cli

[![asciicast](https://asciinema.org/a/4QZc1Owwe6LJJaW6zhu8eN5eo.svg)](https://asciinema.org/a/4QZc1Owwe6LJJaW6zhu8eN5eo)

Displays your configurable stock portfolio using Yahoo Finance data. It uses the WebSocket of the website to get real time (but delayed) data updates.
The Yahoo Finance websocket uses an undocumented ProtoBuf format to encode the data - see `proto/yahoo.proto` for details.

## Usage

- Configure your portfolio and settings in the `Config` struct
- run with `cargo run`

## Advanced features

- Sync to InfluxDB: Use the `docker-compose.yaml` file to startup the DB and configure it in `Config`

## License

MIT