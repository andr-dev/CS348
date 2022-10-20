#!/usr/bin/env bash

rm db/store.db

sqlite3 db/store.db < db/setup/create_tables.sql
