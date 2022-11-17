import React, { FC } from 'react'
import { Container, Typography } from '@mui/material'
import { SummonerMatchProps } from './types'

export const SummonerMatch: FC<SummonerMatchProps> = ({ match }) => {
    const getMinutesFromSeconds = (ms: number) => {
        return ms / 60
    }
    // todo: there's something wrong with this function oops
    const getTimeFromUnixTimestamp = (timestamp: number) => {
        const months = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec']
    
        const dateObj = new Date(timestamp * 1000)
        const year = dateObj.getFullYear()
        const month = months[dateObj.getMonth()]
        const date = dateObj.getDate()
        const hour = dateObj.getHours()
        const min = dateObj.getMinutes() < 10 ? '0' + dateObj.getMinutes() : dateObj.getMinutes()
    
        return `${hour}:${min} on ${month} ${date}, ${year}`
    }

    return (
        <Container key={match.matchid} sx={{marginBottom: '10px'}}>
            <Typography>Match: {match.matchid}</Typography>
            <Typography>{match.participants}</Typography>
            <Typography>{getMinutesFromSeconds(match.game_duration)} minutes</Typography>
            <Typography>{getTimeFromUnixTimestamp(match.game_start_timestamp)}</Typography>
            {match?.game_end_timestamp 
            && <Typography>{getTimeFromUnixTimestamp(match.game_end_timestamp)}</Typography>}
        </Container>
    )
}
