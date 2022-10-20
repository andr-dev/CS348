-- CHAMPIONS

CREATE TABLE champions (
    championid INT PRIMARY KEY NOT NULL,
    cname VARCHAR(32) NOT NULL,
    title VARCHAR(64) NOT NULL,
    blurb TEXT NOT NULL
);

-- SUMMONERS

CREATE TABLE summoners (
    accountid VARCHAR(56) UNIQUE NOT NULL,
    profileiconid INT NOT NULL,
    lastupdate TIMESTAMP NOT NULL,
    sname VARCHAR(32) NOT NULL,
    id  VARCHAR(63) NOT NULL,
    puuid VARCHAR(78) PRIMARY KEY NOT NULL,
    summonerlevel BIGINT NOT NULL
);

-- MATCHES

CREATE TABLE matches (
    matchid VARCHAR(64) PRIMARY KEY NOT NULL,
    data_version VARCHAR(32) NOT NULL,
    participant0 VARCHAR(78) REFERENCES summoners(puuid),
    participant1 VARCHAR(78) REFERENCES summoners(puuid),
    participant2 VARCHAR(78) REFERENCES summoners(puuid),
    participant3 VARCHAR(78) REFERENCES summoners(puuid),
    participant4 VARCHAR(78) REFERENCES summoners(puuid),
    participant5 VARCHAR(78) REFERENCES summoners(puuid),
    participant6 VARCHAR(78) REFERENCES summoners(puuid),
    participant7 VARCHAR(78) REFERENCES summoners(puuid),
    participant8 VARCHAR(78) REFERENCES summoners(puuid),
    participant9 VARCHAR(78) REFERENCES summoners(puuid),
    gameid BIGINT NOT NULL,
    game_creation BIGINT NOT NULL,
    game_duration BIGINT NOT NULL,
    game_start_timestamp BIGINT NOT NULL,
    game_end_timestamp BIGINT
);

-- SUMMONER MATCHES

CREATE TABLE summoner_matches (
    puuid VARCHAR(78) REFERENCES summoners(puuid),
    matchid VARCHAR(64) REFERENCES matches(matchid),

    championid INT REFERENCES champions(championid),
    win BOOLEAN NOT NULL,
    team_position VARCHAR(32) NOT NULL,
    champ_level INT NOT NULL,
    champ_experience INT NOT NULL,
    vision_score INT NOT NULL,
    gold_earned INT NOT NULL,
    gold_spent INT NOT NULL,

    kills INT NOT NULL,
    double_kills INT NOT NULL,
    triple_kills INT NOT NULL,
    quadra_kills INT NOT NULL,
    penta_kills INT NOT NULL,
    deaths INT NOT NULL,
    assists INT NOT NULL,

    item0 INT NOT NULL,
    item1 INT NOT NULL,
    item2 INT NOT NULL,
    item3 INT NOT NULL,
    item4 INT NOT NULL,
    item5 INT NOT NULL,
    item6 INT NOT NULL,
    
    first_blood_assist BOOLEAN NOT NULL,
    first_blood_kill BOOLEAN NOT NULL,
    first_tower_assist BOOLEAN NOT NULL,
    first_tower_kill BOOLEAN NOT NULL,
    
    dragon_kills INT NOT NULL,
    baorn_kills INT NOT NULL,

    turret_kills INT NOT NULL,
    turret_takedowns INT NOT NULL,
    turrets_lost INT NOT NULL,

    inhibitor_kills INT NOT NULL,
    inhibitor_takedowns INT NOT NULL,
    inhibitors_list INT NOT NULL,
    
    damage_dealt_to_buildings INT NOT NULL,
    damage_dealt_to_objectives INT NOT NULL,
    damage_dealt_to_turrets INT NOT NULL,
    
    consumables_purchased INT NOT NULL,
    wards_placed INT NOT NULL,
    wards_killed INT NOT NULL,

    PRIMARY KEY (puuid, matchid)
);

