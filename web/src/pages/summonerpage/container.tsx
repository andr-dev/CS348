import Loader from '@components/loader';
import React, { FC, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { SummonerPagePresentation } from './presentation'
import { DbSummoner, SummonerPageProps } from './types'
import axios from 'axios'


export const SummonerPage: FC<SummonerPageProps> = () => {
    const { summonerName } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
    const [summonerInfo, setSummonerInfo] = useState<DbSummoner | null>(null);

    useEffect(() => {
        axios(`http://localhost:8000/api/summoner/${summonerName}`).then((result) => {
            setLoading(false)

            const response = result.data.Ok;
            if (response) {
                setSummonerInfo(response);
                console.log(summonerInfo);
            } else {
                console.log("Axios error")
            }
        })
    }, [])

    return loading ? <Loader /> : <SummonerPagePresentation />
}

