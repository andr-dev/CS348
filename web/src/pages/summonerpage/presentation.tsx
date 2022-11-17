import { Box, Button, Container, Typography } from "@mui/material";
import React, { FC } from "react";
import { SummonerMatch } from "./summoner-match";
import { Match, SummonerPagePresentationProps } from "./types";

export const SummonerPagePresentation: FC<SummonerPagePresentationProps> = ({ 
    summonerPageInfo, 
    updateSummonerPageInfo,
}) => {
    console.log(summonerPageInfo)

    return (
        <Box sx={{color: 'white'}}>
            <>
                <Button variant="contained" onClick={updateSummonerPageInfo}>Fetch latest matches</Button>
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
                    <SummonerMatch match={match} key={match.matchid}/>
                ))}
            </>
        </Box>
    )
}
