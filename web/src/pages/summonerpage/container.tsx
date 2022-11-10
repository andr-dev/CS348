import Loader from '@components/loader';
import React, { FC, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { SummonerPagePresentation } from './presentation'
import { Summoner, SummonerPageProps } from './types'
import axios from 'axios'
import { getSummoner, getSummonerKda } from 'src/api/SummonerAPI';
import { Box } from '@mui/material';

export const SummonerPage: FC<SummonerPageProps> = () => {
    const { summonerName = "" } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
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
        return <Box sx={{color: 'white'}}>summoner not found rip</Box>
    }
}

