import axios from 'axios'

const domain = 'http://localhost:8000' + '/api/summoner'

export const getSummoner = (summonerName: string) => {
    return axios(`${domain}/${summonerName}`)
}

export const getSummonerKda = (puuid: string) => {
    return axios(`${domain}/${puuid}/kda`)
}
