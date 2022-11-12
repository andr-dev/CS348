import Loader from '@components/loader';
import React, { FC, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { SummonerPagePresentation } from './presentation'
import { Summoner, SummonerPageProps } from './types'
import axios from 'axios'
import { getSummoner, getSummonerKda } from 'src/api/SummonerAPI';
import { Box, Typography } from '@mui/material';

export const SummonerPage: FC<SummonerPageProps> = () => {
    const { summonerName = "" } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
    // Probably important to keep all info fetched from API into one state variable here
    // beecause otherwise we may re-render several times as we update the state after 
    // each API fetch.
    const [summonerInfo, setSummonerInfo] = useState<Summoner | null>(null);

    useEffect(() => {
        getSummoner(summonerName).then((result) => {
            const summonerData = result?.data?.Ok
            if (summonerData) {
                getSummonerKda(summonerData.puuid).then((result) => {
                    const summonerKdaArr = result?.data?.Ok
                    if (summonerKdaArr && summonerKdaArr.length) {
                        const summonerKda = {
                            kills: summonerKdaArr[0],
                            deaths: summonerKdaArr[1],
                            assists: summonerKdaArr[2],
                        }
                        setSummonerInfo({...summonerData, kda: summonerKda})
                    }
                    setLoading(false)
                })
            } else {
                console.log('Summoner not found.')
                setLoading(false)
            }
        })
    }, [])

    if (loading) {
        return <Loader />
    } else if (summonerInfo) {
        return <SummonerPagePresentation summonerInfo={summonerInfo}/>
    } else {
        // MUI theme isn't set up so hard-code to white instead of using "text"
        return <Typography color="white">summoner not found rip</Typography>
    }
}

