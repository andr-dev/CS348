#!/bin/bash

# run feature 1 but optimized

# create dense non-clustering index on puuid first
# NOTE: this modifies the database, so you may want to reset it before running other tests/queries
sqlite3 ../store.db < create_index1.sql

# time query execution after creating secondary index
time for i in {1..1000}; do sqlite3 ../store.db < feature1.sql > /dev/null ; done