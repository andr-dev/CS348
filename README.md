<div align="center">
  <h1 align="center">CS348 Project</h1>

  <p align="center">
    The source code for our CS348 project written in Rust and Typescript which integrates with the Riot Developer API
  </p>
</div>

## Getting Started

This project contains a backend server which interacts with the database and a frontend package which communicates with the frontend.

### Setup

To run the application, first install sqlite3, [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html), Node.JS and Yarn.

<!-- USAGE EXAMPLES -->

### Sample Usage

To run the sample sqlite3 queries, use the `./db/sample_store.db` file. Open it using the command `sqlite3 ./db/sample_store.db`.

### Production Usage

In `scripts/run` add your Riot API key (https://developer.riotgames.com/) to `RG_API_KEY`

Finally, run `./scripts/run` from the home directory. This will start the backend server using the provided populated production database.

The production database is populated at runtime by fetching live data from the riot apis. To populate the database, we wrote a script (`./db/prodgen.py`) which simulates frontend calls to the backend which subsequently queries data from the riot api, parses it, and stores it in the database. The result of this script is provided in the github repository at `./db/store.db` and is used by the `./scripts/run` script.

Once the server is started, to start the frontend, run `yarn install` inside the web directory followed by `yarn dev`. This will start the frontend on a hot-reloading page at `localhost:3031`.

## Testing

Queries and sample output are found in `db/queries`. Run them with `sqlite3 db/store.db < db/queries/feature0.sql`

## Features

### Working

Backend:
 - [x] Full Riot API implementation (datamodels in `./rito/src/models` and api call implementations in `./rito/src/apis`) with custom rate-limiting client (see `./server/src/rito/client.rs`)
 - [x] summoners (`./server/src/api/summoner/mod.rs` and `./server/src/api/summoner/db.rs`)
   - [x] get by name
   - [x] get matches by puuid
   - [x] kda by puuid
 - [x] matches (`./server/src/api/matches/mod.rs` and `./server/src/api/matches/db.rs`)
   - [x] get by match id
 - [x] champion (`./server/src/api/champion/mod.rs` and `./server/src/api/champion/db.rs`)
   - [x] get by id
   - [x] get by name
   - [x] get winrates by game duration

Frontend:
 - [x] Seach summoners by name
 - [x] Summoner info page with detailed statistics such as KDA (`./web/src/pages/summonerpage/index.tsx`)
 - [x] Champion winrate by game duration page (`./web/src/pages/champs/index.tsx`)