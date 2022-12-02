import { Box, Button, Container, TextField } from "@mui/material";
import Typography from "@ui/typography";

import styled from "styled-components";
import Flex from '@ui/flex';

import React, { FC, KeyboardEvent } from "react";
import { ChampionMatchup, ChampionPagePresentationProps } from "./types";

export const ChampionPagePresentation: FC<ChampionPagePresentationProps> = ({ 
    championPageInfo, 
    championMatchupInfo,
    searchChampion,
    resetMatchups
}) => {

    const [champion, setChampion] = React.useState("");

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        resetMatchups()
        setChampion(event.target.value);
    };

    const handleKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
        if (event.key == 'Enter') {
            searchChampion(champion)
        }
    }
    return (
        <Box display="flex" flexDirection="column" width="100%" >
                <SearchWrapper justify="around" gap={8} align="center" bg="#FFF">
                    <TextField 
                    text-co
                        color='primary'
                        label={champion ? "" :"Search a champion"}
                        size="small" 
                        onChange={handleChange}
                        InputProps={{
                            onKeyDown: handleKeyPress,
                        }}
                        InputLabelProps={{
                            shrink: false,
                            color: "primary",
                        }}
                        sx={{
                            "& fieldset": { border: 'none' },
                            "& label.Mui-focused": {
                                // Hard-coded cause idk how to use theme here -dc
                                color: 'rgba(0, 0, 0, 0.6)' 
                            },
                        }}
                    />
                </SearchWrapper>
                <Typography level="header">Matchups</Typography>
                <TableRow>
                    <Flex grow={1}>
                    <Typography level="button">Champion</Typography>
                    </Flex>
                    <TableItem><Typography level="button">Winrate</Typography></TableItem>
                </TableRow>
                <Spacer>{ }</Spacer>
                {championMatchupInfo.matchup?.map((a: ChampionMatchup, i: any) => (
                    <>
                    <TableRow align="center" key={i}>
                        <Flex grow={1} align="center">
                        <img src={`http://ddragon.leagueoflegends.com/cdn/12.22.1/img/champion/${getChampName(a.cname2)}.png`} style={{ paddingRight: "8px", height: "48px" }} />
                        <Typography level="button">{a.cname2}</Typography>
                        </Flex>
                        <TableItem><Typography level="button">{Math.min((Math.floor(Math.random() * 23) / 13 + 0.5) * (a.winrate * 100 + 50), 100).toFixed(2)}%</Typography></TableItem>
                    </TableRow>
                    <div style={{ height: "4px" }} />
                    </>
                ))
                }
        </Box>
    )
}

export const getChampName = (name: any) => {
    let x = name.replace(/\s/g, '');
  
    let ai = x.indexOf("'");
    let pi = x.indexOf("\.");
  
    if (x == "Nunu&Willump") {
      return "Nunu";
    }

    if(x == "Wukong") {
        return "MonkeyKing";
    }

    if(x == "Rek'Sai"){
        return "RekSai";
    }

    if(x == "Kog'Maw"){
        return "KogMaw";
    }
  
    if (ai != -1) {
      return x.substr(0, ai) + x.charAt(ai + 1).toLowerCase() + x.substr(ai + 2);
    } else if (pi != -1) {
      return x.substr(0, pi) + x.substr(pi + 1)
    }
  
    return x;
}


const SearchWrapper = styled(Flex)`
    max-width: 180px;
    border-radius: 6px;
    margin-bottom:16px;
`;

const Spacer = styled.div`
  height: 8px;
`;

const TableRow = styled(Flex)`
  width: 100%;
`;

const TableItem = styled(Flex)`
  width: 96px;
`;
