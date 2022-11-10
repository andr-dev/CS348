import styled from "styled-components";
import Flex from '@ui/flex';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import Typography from "@ui/typography";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import Button from "@mui/material/Button";
import { IconButton, Input, TextField } from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';

const search = () => {
    const navigate = useNavigate();
    const [summonerNameInput, setSummonerNameInput] = useState<string>("");

    const handleClick = () => {
        if (summonerNameInput) {
            navigate(`/summoner/${summonerNameInput}`);
        }
    }

    return (
        <SearchContainer>
            <SearchWrapper justify="around" gap={8} align="center" bg="#FFF">
                <TextField label={summonerNameInput ? "" : "Search..."}
                size="small" 
                onInput={e => setSummonerNameInput((e.target as HTMLInputElement).value)}
                InputLabelProps={{shrink: false}}
                sx={{
                    "& fieldset": { border: 'none' },
                }}
                />
                <IconButton onClick={handleClick} size="small">
                    <SearchIcon />
                </IconButton>
            </SearchWrapper>
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
