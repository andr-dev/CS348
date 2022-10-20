-- CHAMPS

CREATE TABLE champs (
    name VARCHAR(16) UNIQUE NOT NULL,
    id INTEGER PRIMARY KEY NOT NULL
);

-- MATCHES

CREATE TABLE matches (
    id INTEGER PRIMARY KEY NOT NULL,
    gameid INTEGER UNIQUE NOT NULL,
    platformid VARCHAR(4) NOT NULL,
    queueid INTEGER NOT NULL,
    seasonid INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    creation INTEGER NOT NULL,
    -- TODO: rename keyword version
    version VARCHAR(13) NOT NULL
);

-- SUMMONER SPELLS

CREATE TABLE ss (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(1) UNIQUE NOT NULL
);

-- PARTICIPANTS

CREATE TABLE participants (
    id INTEGER PRIMARY KEY NOT NULL,
    matchid INTEGER NOT NULL,
    player INTEGER NOT NULL,
    championid INTEGER NOT NULL,

    -- ss1/ss2 are summoner spell 1/2
    ss1 INTEGER NOT NULL,
    ss2 INTEGER NOT NULL,

    -- TODO: rename role, keyword!
    role VARCHAR(11)
        CHECK ( role IN (
            'NONE',
            'SOLO',
            'DUO',
            'DUO_CARRY',
            'DUO_SUPPORT'
        ) ) NOT NULL,
    
    position VARCHAR(6)
            CHECK ( position IN (
            'TOP',
            'JUNGLE',
            'MID',
            'BOT'
        ) ) NOT NULL,

	FOREIGN KEY (id) REFERENCES stats(id),
    FOREIGN KEY (championid) REFERENCES champs(id),
	FOREIGN KEY (matchid) REFERENCES matches(id),
    FOREIGN KEY (ss1) REFERENCES ss(id),
    FOREIGN KEY (ss2) REFERENCES ss(id)
);

-- STATS

CREATE TABLE stats (
    id INTEGER PRIMARY KEY NOT NULL,
    win BOOLEAN NOT NULL
    -- ...
);

-- TEAMBANS

CREATE TABLE teambans (
    matchid INTEGER NOT NULL,
    teamid BOOLEAN NOT NULL,
    championid INTEGER NOT NULL,
    banturn INTEGER NOT NULL,

    PRIMARY KEY(matchid, banturn),

    FOREIGN KEY (matchid) REFERENCES matches(id),
    FOREIGN KEY (championid) REFERENCES champs(id)
);

-- TEAMSTATS

CREATE TABLE teamstats (
    matchid INTEGER NOT NULL,
    teamid BOOLEAN NOT NULL,
    firstblood BOOLEAN NOT NULL,
    firsttower BOOLEAN NOT NULL,
    firstinhib BOOLEAN NOT NULL,
    firstbaron BOOLEAN NOT NULL,
    firstdragon BOOLEAN NOT NULL,
    firstharry BOOLEAN NOT NULL,
    towerkills INTEGER NOT NULL,
    inhibkills INTEGER NOT NULL,
    baronkills INTEGER NOT NULL,
    dragonkills INTEGER NOT NULL,
    harrykills INTEGER NOT NULL,

    PRIMARY KEY(matchid, teamid)
);
