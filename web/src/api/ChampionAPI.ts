import axios from 'axios'

const domain = 'http://localhost:8000' + '/api/champion'

export const getChampionById = (championId: string) => {
    return axios(`${domain}/id/${championId}`)
}
