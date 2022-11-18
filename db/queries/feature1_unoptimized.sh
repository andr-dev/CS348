#!/bin/bash

# runs feature 1 without optimizations/creating additional indexes
time for i in {1..1000}; do sqlite3 ../store.db < feature1.sql > /dev/null ; done