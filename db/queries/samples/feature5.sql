SELECT *
FROM (
    SELECT teamRed.championid AS champion, teamBlue.championid AS counter, 
    AVG(CASE WHEN teamRed.win THEN 1 ELSE 0 END) winrate
    FROM summoner_matches AS teamRed, summoner_matches AS teamBlue
    WHERE teamRed.win != teamBlue.win AND teamRed.matchid = teamBlue.matchid
    GROUP BY teamRed.championid, teamBlue.championid
)
ORDER BY winrate ASC;