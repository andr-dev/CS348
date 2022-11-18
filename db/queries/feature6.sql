INSERT INTO champions VALUES (1000, 'customchamp', 'this is title', 'this is blurb');

INSERT INTO summoners VALUES ('cs348customaccountid1', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test1', 'cs348customid1', 'cs348custompuuid1', 100);

INSERT INTO summoners VALUES ('cs348customaccountid2', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test2', 'cs348customid2', 'cs348custompuuid2', 100);

INSERT INTO summoners VALUES ('cs348customaccountid3', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test3', 'cs348customid3', 'cs348custompuuid3', 100);

INSERT INTO summoners VALUES ('cs348customaccountid4', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test4', 'cs348customid4', 'cs348custompuuid4', 100);

INSERT INTO summoners VALUES ('cs348customaccountid5', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test5', 'cs348customid5', 'cs348custompuuid5', 100);

INSERT INTO summoners VALUES ('cs348customaccountid6', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test6', 'cs348customid6', 'cs348custompuuid6', 100);

INSERT INTO summoners VALUES ('cs348customaccountid7', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test7', 'cs348customid7', 'cs348custompuuid7', 100);

INSERT INTO summoners VALUES ('cs348customaccountid8', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test8', 'cs348customid8', 'cs348custompuuid8', 100);

INSERT INTO summoners VALUES ('cs348customaccountid9', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test9', 'cs348customid9', 'cs348custompuuid9', 100);

INSERT INTO summoners VALUES ('cs348customaccountid0', 1, '2023-10-21 00:10:05.988846898', 'Riot Dee See Test0', 'cs348customid0', 'cs348custompuuid0', 100);

INSERT INTO matches VALUES ('cs348custommatchid', 'cs348customdataversion', 'cs348custompuuid1', 'cs348custompuuid2', 'cs348custompuuid3', 'cs348custompuuid4', 'cs348custompuuid5', 'cs348custompuuid6', 'cs348custompuuid7', 'cs348custompuuid8', 'cs348custompuuid9', 'cs348custompuuid0', 'cs348customegameid', 0, 1, 0, 1);

INSERT INTO summoner_matches VALUES ('cs348custompuuid1', 'cs348customatchid', 1000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

SELECT * FROM champions WHERE championid=1000;
SELECT * FROM summoners WHERE puuid LIKE 'cs348%';
SELECT * FROM matches WHERE matchid='cs348custommatchid';
SELECT * FROM summoner_matches WHERE puuid='cs348custompuuid1' AND matchid='cs348custommatchid';
