import { Box } from "@mui/material";
import React, { FC } from "react";
import { SummonerPagePresentationProps } from "./types";

export const SummonerPagePresentation: FC<SummonerPagePresentationProps> = ({summonerInfo}) => {
    console.log(summonerInfo)
    return (
        <Box sx={{color: 'white'}}>
            Player name is {summonerInfo.sname}. 
            Player level is {summonerInfo.summonerlevel}. 
            {summonerInfo.kda && `Summoner KDA: ${summonerInfo.kda.kills}/${summonerInfo.kda.deaths}/${summonerInfo.kda.assists}`}
        </Box>
    )
}
