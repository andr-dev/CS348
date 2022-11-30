export interface SummonerPagePresentationProps {
    summonerPageInfo: SummonerPageInfo,
    updateSummonerPageInfo: () => void,
}

export interface SummonerMatchProps {
    match: Match,
}

export interface SummonerChampionStatsProps {
    champWinrates: [string, number][]
}

export interface SummonerPageInfo {
    summoner?: Summoner,
    matches?: Match[],
}

export interface Summoner {
    // accountid: string
    profileiconid: number,
    // lastupdate: string,
    sname: string,
    // id: string,
    puuid: string,
    summonerlevel: number,
    kda?: Kda,
}

export interface Kda {
    kills: number, 
    deaths: number,
    assists: number,
}

export interface Match {
    matchid: string,
    // gameid: string,
    // data_version: string,
    participant0: string,
    participant1: string,
    participant2: string,
    participant3: string,
    participant4: string,
    participant5: string,
    participant6: string,
    participant7: string,
    participant8: string,
    participant9: string,
    // game_creation: number,
    game_duration: number,
    game_start_timestamp: number,
    game_end_timestamp?: number,
}
