import styled from "styled-components";
import Flex from '@ui/flex';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import Typography from "@ui/typography";
import { useNavigate } from "react-router-dom";
import { useState, KeyboardEvent } from "react";
import Button from "@mui/material/Button";
import { Grid, IconButton, Input, TextField } from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';

const search = () => {
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
            {/* <SearchWrapper justify="around" gap={8} align="center" bg="#FFF"> */}
            <Grid container sx={{width: '100%', maxWidth: '256px', borderRadius: '6px'}}>
                <Grid item xs={8}>
                <TextField 
                    color='primary'
                    label={summonerNameInput ? "" : "Search Summoner..."}
                    size="small" 
                    onInput={e => setSummonerNameInput((e.target as HTMLInputElement).value)}
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
                </Grid>
                <Grid item xs={4}>
                <IconButton onClick={handleClick} size="small">
                    <SearchIcon />
                </IconButton>
                </Grid>
            </Grid>
            {/* </SearchWrapper> */}
        </SearchContainer>
    )
}

const SearchContainer = styled.div`
    padding: 8px;
`;

const SearchWrapper = styled(Flex)`
    width: 100%;
    max-width: 256px;

    border-radius: 6px;
`;

const SearchInput = styled(Typography)`
    flex-grow: 1;
`;

export default search;
