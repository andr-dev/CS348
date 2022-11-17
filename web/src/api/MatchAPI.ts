import axios from 'axios'

const domain = 'http://localhost:8000' + '/api/matches'

export const getMatchById = (matchId: string) => {
    return axios(`${domain}/${matchId}`)
}
