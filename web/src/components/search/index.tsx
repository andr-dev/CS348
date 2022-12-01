import styled from "styled-components";
import { useNavigate } from "react-router-dom";
import { useState, KeyboardEvent, FC } from "react";
import { Grid, IconButton, TextField } from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';

interface SearchProps {
    maxWidth?: string
    autoFocus?: boolean
    width?: string
}

const search: FC<SearchProps> = ({ maxWidth = '256px', autoFocus = false, width = '256px' }) => {
    const navigate = useNavigate();
    const [summonerNameInput, setSummonerNameInput] = useState<string>("");

    const doSearch = () => {
        if (summonerNameInput) {
            navigate(`/summoner/${summonerNameInput}`);
        }
    }
    const handleClick = () => {
        doSearch()
    }
    const handleKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
        if (event.key == 'Enter') {
            doSearch()
        }
    }

    return (
        <SearchContainer>
            <Grid container sx={{width: '100%', maxWidth: maxWidth, borderRadius: '6px', background: '#FFF'}}>
                <Grid item xs={10}>
                    <TextField 
                        color='primary'
                        label={summonerNameInput ? "" : "Search Summoner..."}
                        size="small" 
                        onInput={e => setSummonerNameInput((e.target as HTMLInputElement).value)}
                        autoFocus={autoFocus}
                        InputProps={{
                            onKeyDown: handleKeyPress,
                        }}
                        InputLabelProps={{
                            shrink: false,
                            color: "primary",
                        }}
                        sx={{
                            width: width,
                            "& fieldset": { border: 'none' },
                            "& label.Mui-focused": {
                                // Hard-coded cause idk how to use theme here -dc
                                color: 'rgba(0, 0, 0, 0.6)' 
                            },
                        }}
                    />
                </Grid>
                <Grid item xs={2} container alignItems="center" justifyContent="flex-end">
                    <IconButton onClick={handleClick} size="small">
                        <SearchIcon />
                    </IconButton>
                </Grid>
            </Grid>
        </SearchContainer>
    )
}

const SearchContainer = styled.div`
    padding: 8px;
`;

export default search;
