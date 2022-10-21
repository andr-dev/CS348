<div align="center">
  <h1 align="center">CS348 Project</h1>

  <p align="center">
    The source code for our CS348 project written in Rust and integrating the Riot Developer API
  </p>
</div>

## Getting Started

The project has been setup to run on docker. A build and run script is provided in the scripts folder.

### Installation

Install sqlite3 and [https://doc.rust-lang.org/cargo/getting-started/installation.html](Rust)

<!-- USAGE EXAMPLES -->
### Usage

In `scripts/run` add your Riot API key (https://developer.riotgames.com/) to `RG_API_KEY`

Reset the database with `./db/reset.sh` then run `./scripts/run` from the home directory

Once the server is started, navigate to localhost:8000 to view the webapp.

To populate the database with data from the Riot API, navigate to `http://localhost:8000/api/summoner/Dee%20See` with a user's summoner id (i.e. Dee See)

## Testing

Queries and sample output are found in `db/queries`. Run them with `sqlite3 db/store.db < db/queries/feature0.sql`
