SELECT championid, AVG(CASE WHEN win THEN 1 ELSE 0 END) winrate
FROM summoner_matches
GROUP BY championid;