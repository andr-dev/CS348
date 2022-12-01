export interface ChampionPagePresentationProps {
    championPageInfo: ChampionPageInfo,
    championMatchupInfo: ChampionMatchupInfo
    searchChampion: (cname: string)=>void
    resetMatchups: ()=>void
}

export interface ChampionPageInfo {
    champion?: Champion,
}

export interface Champion {
    championid: number,
    cname: string,
    title: string,
    blurb: string
}

export interface ChampionMatchupInfo {
    matchup?: ChampionMatchup[],
}

export interface ChampionMatchup {
    cname1: string,
    cname2: string,
    winrate: number,
}