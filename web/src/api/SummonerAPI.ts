import axios from 'axios'

const domain = 'http://localhost:8000' + '/api/summoner'

export const getSummoner = (summonerName: string) => {
    return axios(`${domain}/${summonerName}`)
}

// maybe this should be taking in summoneerName instead of puuid, would require
// backend change
export const getSummonerKda = (puuid: string) => {
    return axios(`${domain}/${puuid}/kda`)
}

// todo: implement this in backend. For now just mock.
export const getSummonerMatches = (summonerName: string) => {
    return axios(`${domain}/${summonerName}/matches`)
}
