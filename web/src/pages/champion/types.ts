export interface ChampionPagePresentationProps {
    championPageInfo: ChampionPageInfo,
    searchChampion: (cname: string)=>void
}

export interface ChampionPageInfo {
    champion?: Champion,
}

export interface Champion {
    championid: number,
    cname: string,
    title: string,
    bluebr: string
}
