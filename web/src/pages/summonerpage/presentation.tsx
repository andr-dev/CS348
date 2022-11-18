import { Box, Button, Typography } from "@mui/material";
import React, { FC } from "react";
import { SummonerMatch } from "./summoner-match";
import { SummonerPagePresentationProps } from "./types";
import Flex from '@ui/flex';

export const SummonerPagePresentation: FC<SummonerPagePresentationProps> = ({
    summonerPageInfo,
    updateSummonerPageInfo,
}) => {
    console.log(summonerPageInfo)

    return (
        <Box sx={{ color: 'white' }}>
            <Flex gap={16} padding={[16, 0]}>
                <img src={`http://ddragon.leagueoflegends.com/cdn/12.22.1/img/profileicon/${summonerPageInfo.summoner?.profileiconid}.png`} />

                <Flex column gap={8} grow={1} justify="between">
                    <Flex column gap={8} grow={1}>
                        <Typography>
                            Player Name: {summonerPageInfo?.summoner?.sname}.
                        </Typography>

                        <Typography>
                            Player Level: {summonerPageInfo?.summoner?.summonerlevel}.
                        </Typography>

                        <Typography>
                            {summonerPageInfo?.summoner?.kda &&
                                `Summoner KDA: ${summonerPageInfo?.summoner?.kda.kills}/${summonerPageInfo?.summoner?.kda.deaths}/${summonerPageInfo?.summoner?.kda.assists}`}
                        </Typography>
                    </Flex>

                    <Button variant="contained" onClick={updateSummonerPageInfo}>Fetch latest matches</Button>
                </Flex>
            </Flex>

            <div style={{ height: "8px" }} />

            <Flex column gap={32}>
                {summonerPageInfo?.matches?.map((match) => (
                    <SummonerMatch match={match} key={match.matchid} />
                ))}
            </Flex>
        </Box>
    )
}
