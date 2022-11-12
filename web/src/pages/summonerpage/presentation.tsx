import { Box, Container, Typography } from "@mui/material";
import React, { FC } from "react";
import { Match, SummonerPagePresentationProps } from "./types";

export const SummonerPagePresentation: FC<SummonerPagePresentationProps> = ({ summonerPageInfo }) => {
    console.log(summonerPageInfo)

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
        <Box sx={{color: 'white'}}>
            <>
                <Typography>
                    Player name is {summonerPageInfo?.summoner?.sname}.
                </Typography>
                <Typography>
                    Player level is {summonerPageInfo?.summoner?.summonerlevel}.
                </Typography>
                <Typography>
                    {summonerPageInfo?.summoner?.kda && 
                    `Summoner KDA: ${summonerPageInfo?.summoner?.kda.kills}/${summonerPageInfo?.summoner?.kda.deaths}/${summonerPageInfo?.summoner?.kda.assists}`}
                </Typography>

                {summonerPageInfo?.matches?.map((match) => (
                    <Container key={match.matchid} sx={{marginBottom: '10px'}}>
                        <Typography>Match: {match.matchid}</Typography>
                        <Typography>{match.participants}</Typography>
                        <Typography>{getMinutesFromSeconds(match.game_duration)} minutes</Typography>
                        <Typography>{getTimeFromUnixTimestamp(match.game_start_timestamp)}</Typography>
                        {match?.game_end_timestamp 
                        && <Typography>{getTimeFromUnixTimestamp(match.game_end_timestamp)}</Typography>}
                    </Container>
                ))}
            </>
        </Box>
    )
}
