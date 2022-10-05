.mode csv

.import csv/processed/champs.csv champs

-- Will throw error because of first row headers
.import csv/processed/matches.csv matches

-- SUMMONER SPELLS

-- TODO

-- PARTICIPANTS

.import csv/processed/participants.csv participants

-- STATS

-- .import csv/processed/stats.csv stats

-- TEAMBANS

.import csv/processed/teambans.csv teambans

-- TEAMSTATS

.import csv/processed/teamstats.csv teamstats
