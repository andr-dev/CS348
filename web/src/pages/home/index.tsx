import Search from "@components/search";
import { Grid, Typography } from "@mui/material";
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
			<div style={{height: "16px"}} />
            <Grid item sx={{color: 'rgb(105, 105, 105)'}}>
                <Search maxWidth="100%" autoFocus width='350px'/>
            </Grid>
        </Grid>
    );
};

export default Home;
