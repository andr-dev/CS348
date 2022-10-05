#!/usr/bin/env bash

rm store.db

cd csv

python process.py

cd ../

sqlite3 store.db < setup/create_tables.sql

sqlite3 store.db < setup/populate_tables.sql
