export interface SummonerPageProps {

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
export interface SummonerPagePresentationProps {
    summonerInfo: Summoner | null,
}

