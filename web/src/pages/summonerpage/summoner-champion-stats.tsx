import { Box, Grid, Typography } from '@mui/material'
import React, { FC } from 'react'
import { SummonerChampionStatsProps } from './types'

interface ChampStatsProps {
    championName: string
    winrate: number
}

const ChampStats: FC<ChampStatsProps> = ( {championName, winrate} ) => {
    return (
        <Grid
            container
            item
            direction="row"
            justifyContent="space-between"
            alignItems="flex-start"
        >
            <Grid item>
                {championName}
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
            sx={{width: '300px'}}
        >
            {/* background colour should be 'theme.grey' */}
            <Typography variant="h6">
                Champion Winrates
            </Typography>
            {champWinrates.map((champWinrate) => <ChampStats championName={champWinrate[0]} winrate={champWinrate[1]} />)}
        </Grid>
    )
}
