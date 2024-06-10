# Transit Arrival Board

A hobby project that uses the MTA's GTFS static and realtime feeds to get realtime transit info about public transit near our high school, The Bronx High School of Science.

## Environment Variables

Create a `.env` file and add the following variables.

### `MTABUSKEY`

The MTA's BusTime feeds require an API key, which you can obtain [here](http://www.bustime.mta.info/wiki/Developers/Index).

## Configuration

The configuration file is located in `config.ts`. It is currently set to stops located near The Bronx High School of Science, but this file is meant to be edited to display any stop in the MTA network. Stops can be added (either subway or bus) with `StopConfig` objects, which is explained in depth below.

### `stop_ids`

Although each GTFS `stop_id` refers to a single platform, we have anticipated for the need to combine platforms with built-in transfers. The first `stop_id` is used to distinguish the `stop_name` and as a key.

### `walk_time`

Although the Rust backend uses the MTA's GTFS realtime feeds to get information about when a vehicle is scheduled to arrive at a station, it may be unhelpful for the vehicle to arrive faster than someone would be able to "catch" it. `walk_time` is used to remove vehicles from the React.js frontend that are deemed to be unlikely for someone to "catch" it.

## Dependencies

Install the following dependencies.

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download)
- [Protobuf Compiler](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation)

## Deploying

The webpage can be found at http://localhost:3000/ after deploying.

### Development

Open 2 terminals and run the following commands.

```bash
cargo test
cargo run
```

```bash
npm install
npm run dev
```

### Production

Open 2 terminals and run the following commands.

> Note: `start` is a command only found on Windows.

```bash
cargo test
cargo build --release
start .\target\release\transit-board.exe
```

```bash
npm install
npm run build
npm run start
```
