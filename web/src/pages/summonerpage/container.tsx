import Loader from '@components/loader';
import React, { FC, useCallback, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { SummonerPagePresentation } from './presentation'
import { Kda, SummonerPageInfo } from './types'
import { getSummoner, getSummonerKda, getSummonerMatches } from 'src/api/SummonerAPI';
import { Typography } from '@mui/material';
import { mockMatchesResponse } from './constants';
import { getMatchById } from 'src/api/MatchAPI';

export const SummonerPage: FC = () => {
    const { summonerName = "" } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
    // Probably important to keep all info fetched from API into one state variable here
    // because otherwise we may re-render several times as we update the state after
    // each API fetch.
    const [summonerPageInfo, setSummonerPageInfo] = useState<SummonerPageInfo>({});

    const fetchSummonerPageInfo = (isUpdatingInfo: boolean = false) => {
        let pageInfo: SummonerPageInfo = {}

        // Better if we could fetch kda & matches by name than id, then no need for chaining
        // it after getSummoner().then-- fix backend implementation.
        getSummoner(summonerName, isUpdatingInfo).then((result) => {
            const summonerData = result?.data?.Ok
            if (summonerData) {
                pageInfo.summoner = summonerData
                Promise.all([
                    getSummonerKda(summonerData.puuid),
                    getSummonerMatches(summonerData.puuid)
                ]).then((results) => {
                    // Handle kda
                    const summonerKdaArr = results[0]?.data?.Ok
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
                    // Handle summoner matches, reverse to get most recent to least recent order.
                    const summonerMatchIds = results[1].data.Ok?.reverse()
                    Promise.all(summonerMatchIds.map((matchId: string) => getMatchById(matchId))).then((results) => {
                        pageInfo.matches = results.map((result) => result.data.Ok)

                        // Common
                        setSummonerPageInfo(pageInfo)
                        setLoading(false)
                    })
                })
            } else if (isUpdatingInfo === false) {
                console.log('Summoner not found in existing database. Attempting to fetch from Riot API.')
                fetchSummonerPageInfo(true)
            } else {
                console.log('Summoner not found in existing database nor Riot API.')
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
    } else if (summonerPageInfo?.summoner) {
        return (<SummonerPagePresentation
            summonerPageInfo={summonerPageInfo}
            updateSummonerPageInfo={handleUpdateSummonerPageInfo}
        />)
    } else {
        // MUI theme isn't set up so hard-code to white instead of using "text"
        return (
            <Typography color="white">
                Summoner was not found, are you sure that summoner exists on the NA server?
            </Typography>
        )
    }
}
