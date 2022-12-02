import { Container, Drawer, Typography } from '@mui/material'
import React, { FC } from 'react'
import { useTheme } from 'styled-components';
import { HelpDrawerProps } from './types'

export const HelpDrawer: FC<HelpDrawerProps> = ({ open, onClose }) => {
    const theme = useTheme();

    const containerStyling = {
        height: '100%',
        width: '100%',
        background: theme.colors.secondary,
        padding: '16px',
        color: theme.colors.text,
    }

    return (
        <Drawer anchor='right' open={open} onClose={onClose} PaperProps={{ sx: { width: "40%" } }}>
            <Container sx={containerStyling}>
                <Typography variant="h2">
                    What is GG.OP?
                </Typography>
                <Typography gutterBottom>
                    GG.OP is a League of Legends statistics visualization tool. Through our application, users 
                    (i.e. League of Legends players) can view their League of Legends stats through a variety of media 
                    such as numerical values, tables, and graphs presented on a webapp to draw insights on their strengths 
                    and weaknesses. 
                </Typography>
                <Typography gutterBottom>
                    On top of caching the publicly available data from the Riot API, the backend application 
                    creates and populates several new tables in the database containing statistical summaries of the queried 
                    data. Modifications to the database occur whenever a user queries a player id, in-game champion, or in-game 
                    item which results in the backend application pulling more data from the API into the database and 
                    recalculating statistical summaries. 
                </Typography>
                <Typography variant="caption" gutterBottom>
                    By: Andre Benedito, David Chen, George Chen, and Jason Xiong
                </Typography>
            </Container>
        </Drawer>
    )
}
