#!/bin/bash

time for i in {1..1000}; do sqlite3 ../store.db < feature3.sql > /dev/null ; done