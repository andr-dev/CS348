import { Box, Grid, Typography } from '@mui/material'
import { getChampName } from '@pages/champion/presentation'
import Flex from '@ui/flex'
import React, { FC } from 'react'
import { SummonerChampionStatsProps } from './types'

interface ChampStatsProps {
    championName: string
    winrate: number
}

const ChampStats: FC<ChampStatsProps> = ({ championName, winrate }) => {
    return (
        <Grid
            container
            item
            direction="row"
            justifyContent="space-between"
            alignItems="flex-start"
        >
            <Grid item>
                <Flex align='center'>
                    <img src={`http://ddragon.leagueoflegends.com/cdn/12.22.1/img/champion/${getChampName(championName)}.png`} style={{ paddingRight: "8px", height: "48px" }} />
                    <Flex grow={1}>{championName}</Flex>
                </Flex>
            </Grid>
            <Grid item>
                {winrate * 100}%
            </Grid>
        </Grid>
    )
}

export const SummonerChampionStats: FC<SummonerChampionStatsProps> = ({ champWinrates }) => {
    return (
        <Grid
            container
            direction="column"
            justifyContent="flex-start"
            alignItems="flex-start"
            sx={{ width: '300px' }}
        >
            {/* background colour should be 'theme.grey' */}
            <Typography variant="h6">
                Champion Winrates
            </Typography>
            {champWinrates.map((champWinrate) => <ChampStats championName={champWinrate[0]} winrate={champWinrate[1]} />)}
        </Grid>
    )
}
