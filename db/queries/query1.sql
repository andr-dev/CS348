SELECT champs.id, AVG(stat.win)
FROM champs, matches, (
	SELECT participants.id AS id, 
		CASE WHEN stats.win THEN 1 ELSE 0 END AS win
	FROM participants, stats
	WHERE participants.id = stats.id
) stat
WHERE champs.id = stat.championid
	AND matches.id = stat.matchid
GROUP BY champs.id;

