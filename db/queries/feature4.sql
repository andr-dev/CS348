SELECT championid, AVG(CASE WHEN win THEN 1 ELSE 0 END) winrate
FROM summoner_matches
WHERE puuid = 'WnrBeH4eVnhsUT43Qbgqm9FY9Y2akj9-uOwFrqEdYCWxvAttFM__dLXKYmdzLqDdJhuTXW8tFJSVvg' -- puuid for user: Dee See
GROUP BY championid
ORDER BY winrate DESC;
