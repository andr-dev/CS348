import { Box, Button, Container, Typography, TextField } from "@mui/material";
import React, { FC, KeyboardEvent } from "react";
import { ChampionPagePresentationProps } from "./types";

export const ChampionPagePresentation: FC<ChampionPagePresentationProps> = ({ 
    championPageInfo, 
    searchChampion
}) => {
    console.log(championPageInfo)

    const [champion, setChampion] = React.useState("");

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setChampion(event.target.value);
    };

    const handleKeyPress = (event: KeyboardEvent<HTMLInputElement>) => {
        if (event.key == 'Enter') {
            searchChampion(champion)
        }
    }

    return (
        <Box sx={{color: 'white'}}>
            <>        
                <TextField id="outlined-search" text-co background-color="white" label="Search champion" type="search"  
                onSubmit={()=>searchChampion(champion)}   
                onChange={handleChange}
                InputProps={{
                    onKeyDown: handleKeyPress,
                    style: {
                        background: "white",
                    }
                }}/>
                <Typography>
                    Player name is {championPageInfo?.champion?.cname}.
                </Typography>
            </>
        </Box>
    )
}
