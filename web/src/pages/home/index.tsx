import Search from "@components/search";
import { Box, Grid, Typography } from "@mui/material";
import React, { FC } from "react";

const Home: FC = () => {
    const containerStyling = {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        width: '100%',
        height: '100%',
    }

    const gradientStyling = {
        background: "-webkit-linear-gradient(45deg, #30CFD0 0%, #330867 100%)",
        WebkitBackgroundClip: "text",
        WebkitTextFillColor: "transparent",
    }

    const logoStyling = {
        fontWeight: '800', 
    }

    return (
        <Grid container direction="column" sx={containerStyling} >
            <Grid item sx={gradientStyling}>
                <Typography variant="h1" sx={logoStyling}>
                    GG.OP
                </Typography>
            </Grid>
            <Grid item sx={{color: 'rgb(105, 105, 105)'}}>
                <Search />
            </Grid>
        </Grid>
    );
};

export default Home;
