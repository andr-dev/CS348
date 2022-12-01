import Loader from '@components/loader';
import React, { FC, useCallback, useEffect, useState } from 'react'
import { useParams } from 'react-router-dom';
import { ChampionPagePresentation } from './presentation'
import { ChampionPageInfo, ChampionMatchupInfo } from './types'
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
    const [championMatchup, setChampionMatchups] = useState<ChampionMatchupInfo>({});

    const getChampionDetails = (cname: string) => {
        axios(`http://localhost:8000/api/champion/champion/${cname}`).then((res) => {
            setChampionDetails({champion: res?.data?.Ok});
            console.log( res?.data?.Ok);
        });
        
    };

    const getChampionMatchups = (cname: string) => {
        axios(`http://localhost:8000/api/champion/matchup/${cname}`).then((res) => {
            setChampionMatchups({matchup: res?.data?.Ok.map((v: any)=> ({cname1: v[0], cname2: v[1], winrate: v[2].toFixed(2)}))});
            console.log( res?.data?.Ok);
        });
        
    };

    const handleUpdateChampionPageInfo = useCallback((cname: string) => {
        setLoading(true)
        getChampionDetails(cname)
    }, [])

    const resetMatchups = useCallback(()=>{
        setChampionMatchups({})
    }, [])

        return (<ChampionPagePresentation
            championPageInfo={championDetails}
            championMatchupInfo={championMatchup}
            searchChampion={getChampionMatchups}
            resetMatchups={resetMatchups}
        />)

}
