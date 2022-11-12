import { Box, Typography } from "@mui/material";
import React, { FC } from "react";
import { SummonerPagePresentationProps } from "./types";

export const SummonerPagePresentation: FC<SummonerPagePresentationProps> = ({summonerInfo}) => {
    console.log(summonerInfo)
    return (
        <Box sx={{color: 'white'}}>
            <Typography>
                Player name is {summonerInfo?.sname}.
            </Typography>
            <Typography>
                Player level is {summonerInfo?.summonerlevel}.
            </Typography>
            <Typography>
                {summonerInfo?.kda && `Summoner KDA: ${summonerInfo.kda.kills}/${summonerInfo.kda.deaths}/${summonerInfo.kda.assists}`}
            </Typography>
        </Box>
    )
}
