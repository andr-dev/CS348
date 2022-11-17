pub const CHAMPION_GET_BY_NAME_QUERY: &'static str = "SELECT * FROM champions WHERE cname = ?";
pub const CHAMPION_GET_BY_ID_QUERY: &'static str = "SELECT * FROM champions WHERE championid = ?";
pub const CHAMPION_IS_TABLE_EMPTY_QUERY: &'static str =
    "SELECT CASE WHEN EXISTS(SELECT 1 FROM champions) THEN 0 ELSE 1 END AS IsEmpty";
pub const CHAMPION_INSERT_QUERY: &'static str = "INSERT INTO champions VALUES (?, ?, ?, ?)";

pub const MATCH_FIND_BY_ID_QUERY: &'static str = "SELECT * FROM matches WHERE matchid = ?";
pub const MATCH_INSERT_QUERY: &'static str =
    "INSERT INTO matches VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

pub const SUMMONER_FIND_BY_PUUID_QUERY: &'static str = "SELECT * FROM summoners WHERE puuid = ?";
pub const SUMMONER_GET_BY_NAME_QUERY: &'static str = "SELECT * FROM summoners WHERE sname = ?";
pub const SUMMONER_INSERT_QUERY: &'static str =
    "INSERT INTO summoners VALUES (?, ?, ?, ?, ?, ?, ?)";
pub const SUMMONER_UPDATE_QUERY: &'static str =
    "UPDATE summoners SET accountid = ? WHERE puuid = ?";
pub const SUMMONER_MATCH_IDS_BY_PUUID_QUERY: &'static str = "SELECT matchid FROM summoner_matches WHERE puuid = ?";
pub const SUMMONER_MATCHES_KDA_QUERY: &'static str = "SELECT AVG(kills), AVG(deaths), AVG(assists) FROM summoner_matches WHERE puuid = ?";

pub const SUMMONER_MATCHES_INSERT_QUERY: &'static str =
    "INSERT INTO summoner_matches VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
pub const CHAMPION_WINRATE_QUERY: &'static str = "
    SELECT cname, AVG(CASE WHEN win THEN 1 ELSE 0 END) winrate FROM champions 
    LEFT JOIN summoner_matches
    ON summoner_matches.championid = champions.championid
    WHERE summoner_matches.matchid IN (
        SELECT matchid FROM
        matches
        WHERE game_duration >= ? AND game_duration <= ?
    ) OR summoner_matches.matchid IS NULL
    GROUP BY champions.championid
    ORDER BY champions.cname";

