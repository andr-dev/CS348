import React, { FC, useEffect, useState } from 'react'
import { Typography } from '@mui/material'
import { Match, SummonerMatchProps } from './types'
import Flex from "@ui/flex";
import { getSummoner, getSummonerByPuuid } from 'src/api/SummonerAPI';

export const SummonerMatch: FC<SummonerMatchProps> = ({ match }) => {
    const getDurationString = (ms: number) => {
        const minutes = Math.floor(ms / 60)
        const seconds = (ms % 60)
        return `${minutes}m ${seconds}s`
    }
    // todo: there's something wrong with this function oops
    const getTimeFromUnixTimestamp = (timestamp: number) => {
        const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']

        const dateObj = new Date(timestamp)
        const year = dateObj.getFullYear()
        const month = months[dateObj.getMonth()]
        const date = dateObj.getDate()
        const hour = dateObj.getHours() === 0 ? 12 : dateObj.getHours()
        const min = dateObj.getMinutes() < 10 ? '0' + dateObj.getMinutes() : dateObj.getMinutes()

        return `${hour}:${min} on ${month} ${date}, ${year}`
    }

    return (
        <Flex key={match.matchid} align="center" justify="between" gap={32}>
            <Flex column gap={4}>
                <Typography>Match: {match.matchid}</Typography>
                <Typography>Duration: {getDurationString(match.game_duration)}</Typography>
                <Typography>Game Start Time: {getTimeFromUnixTimestamp(match.game_start_timestamp)}</Typography>
                {match?.game_end_timestamp
                    && <Typography>Game End Time: {getTimeFromUnixTimestamp(match.game_end_timestamp)}</Typography>}
                <div style={{ width: "512px" }} />
            </Flex>
            <Flex gap={16}>
                <Flex column>
                    {[0, 1, 2, 3, 4].map((x, i) => GameParticipant(match, x, i))}
                </Flex>
                <Flex column>
                    {[0, 1, 2, 3, 4].map((x, i) => GameParticipant(match, x + 5, i))}
                </Flex>
            </Flex>
        </Flex>
    )
}

const GameParticipant = (match: any, x: number, key: number) => {
    const [summonerInfo, setSummonerInfo] = useState<any>(null);

    const fetchSummonerId = () => {
        getSummonerByPuuid(match[`participant${x}`], false).then((result) => {
            setSummonerInfo(result.data.Ok)
        })
    }

    useEffect(() => {
        fetchSummonerId()
    }, [])

    return (
        <Flex key={key} align="center">
            {summonerInfo != null ?
                <>
                    <img
                        src={`http://ddragon.leagueoflegends.com/cdn/12.22.1/img/profileicon/${summonerInfo.profileiconid}.png`}
                        style={{ height: "48px" }}
                    />
                    <div style={{ width: "192px" }}><Typography>{summonerInfo.sname}</Typography></div>
                </>
                : null
            }
        </Flex>
    );
}
