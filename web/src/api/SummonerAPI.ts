import axios from 'axios'

const domain = 'http://localhost:8000' + '/api/summoner'

export const getSummoner = (summonerName: string, isUpdate: boolean) => {
    return axios(`${domain}/name/${summonerName}?from_rito=${isUpdate.toString()}`)
}

export const getSummonerByPuuid = (puuid: string, isUpdate: boolean) => {
    return axios(`${domain}/${puuid}?from_rito=${isUpdate.toString()}`)
}

// maybe this should be taking in summonerName instead of puuid, would require
// backend change
export const getSummonerKda = (puuid: string) => {
    return axios(`${domain}/${puuid}/kda`)
}

// maybe this should be taking in summonerName instead of puuid, would require
// backend change
export const getSummonerMatches = (puuid: string) => {
    return axios(`${domain}/${puuid}/matches`)
}

export const getSummonerChampWinrates = (puuid: string) => {
    return axios(`${domain}/${puuid}/champion_winrates`)
}
