import Loader from '@components/loader';
import React, { FC, useCallback, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { SummonerPagePresentation } from './presentation'
import { Kda, SummonerPageInfo } from './types'
import { getSummoner, getSummonerKda } from 'src/api/SummonerAPI';
import { Typography } from '@mui/material';
import { mockMatchesResponse } from './constants';

export const SummonerPage: FC = () => {
    const { summonerName = "" } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
    // Probably important to keep all info fetched from API into one state variable here
    // because otherwise we may re-render several times as we update the state after
    // each API fetch.
    const [summonerPageInfo, setSummonerPageInfo] = useState<SummonerPageInfo>({});

    const fetchSummonerPageInfo = (isUpdatingInfo: boolean = false) => {
        let pageInfo: SummonerPageInfo = {}

        getSummoner(summonerName, isUpdatingInfo).then((result) => {
            const summonerData = result?.data?.Ok
            if (summonerData) {
                pageInfo.summoner = summonerData

                // Better if we could fetch kda by name than id, then no need for chaining -- fix backend implementation.
                getSummonerKda(summonerData.puuid).then((result) => {
                    const summonerKdaArr = result?.data?.Ok
                    if (summonerKdaArr && summonerKdaArr.length) {
                        const summonerKda: Kda = {
                            kills: summonerKdaArr[0],
                            deaths: summonerKdaArr[1],
                            assists: summonerKdaArr[2],
                        }
                        if (pageInfo.summoner) {
                            pageInfo.summoner.kda = summonerKda
                        }
                    }

                    // TODO: handle matches better when backend API for querying summoner's matches implemented,
                    // temporarily mock API response from querying for summoner's matches
                    pageInfo.matches = mockMatchesResponse.matches

                    setSummonerPageInfo(pageInfo)
                    setLoading(false)
                })
            } else {
                console.log('Summoner not found.')
                setLoading(false)
            }
        })
    }

    useEffect(() => {
        fetchSummonerPageInfo(false)
    }, [])

    const handleUpdateSummonerPageInfo = useCallback(() => {
        setLoading(true)
        fetchSummonerPageInfo(true)
    }, [])

    if (loading) {
        return <Loader />
    } else if (summonerPageInfo) {
        return (<SummonerPagePresentation
            summonerPageInfo={summonerPageInfo}
            updateSummonerPageInfo={handleUpdateSummonerPageInfo}
        />)
    } else {
        // MUI theme isn't set up so hard-code to white instead of using "text"
        return <Typography color="white">summoner not found rip</Typography>
    }
}

