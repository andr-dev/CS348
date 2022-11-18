import Loader from '@components/loader';
import React, { FC, useCallback, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { ChampionPagePresentation } from './presentation'
import { ChampionPageInfo } from './types'
import { getSummoner, getSummonerKda, getSummonerMatches } from 'src/api/SummonerAPI';
import { Typography } from '@mui/material';
import { getMatchById } from 'src/api/MatchAPI';
import axios from 'axios';

export const ChampionPage: FC = () => {
    const { summonerName = "" } = useParams();

    const [loading, setLoading] = useState<Boolean>(true);
    // Probably important to keep all info fetched from API into one state variable here
    // because otherwise we may re-render several times as we update the state after
    // each API fetch.
    const [championDetails, setChampionDetails] = useState<ChampionPageInfo>({});

    const getChampionDetails = (cname: string) => {
        axios(`http://localhost:8000/api/champion/name/${cname}`).then((res) => {
            setChampionDetails({champion: res?.data?.Ok});
        });
        console.log(championDetails)
    };

    const handleUpdateChampionPageInfo = useCallback((cname: string) => {
        setLoading(true)
        getChampionDetails(cname)
    }, [])


        return (<ChampionPagePresentation
            championPageInfo={championDetails}
            searchChampion={getChampionDetails}
        />)

}
