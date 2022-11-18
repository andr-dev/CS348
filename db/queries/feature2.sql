SELECT cname, AVG(CASE WHEN win THEN 1 ELSE 0 END) winrate 
FROM champions 
LEFT JOIN 
    (SELECT summoner_matches.championid, win
        FROM summoner_matches
        WHERE summoner_matches.matchid IN (
            SELECT matchid FROM
            matches
            WHERE game_duration >= ? AND game_duration <= ?
        )) AS filtered_matches
ON filtered_matches.championid = champions.championid
GROUP BY champions.championid
ORDER BY champions.cname;
