import React, { useEffect } from "react";
import { BrowserRouter, Outlet, Route, Routes, useLocation } from "react-router-dom";

import Loader from "@components/loader";
import Navbar from "@components/navbar";

import { INITIAL_APP_STATE, appContext, appReducer } from "./context";
import Champs from "@pages/champs";
import {ChampionPage} from "@pages/champion";
import Home from "@pages/home";

import theme from "@styles/theme";
import styled, { ThemeProvider, useTheme } from "styled-components"
import Sidebar from "@components/sidebar";

import Flex from '@ui/flex';

import { AnimatePresence } from 'framer-motion';
import BasePage from "@pages/base";
import { SummonerPage } from "@pages/summonerpage";

const AppLayout: React.FC<{ isLoading: boolean }> = ({ isLoading }) => {
    const theme = useTheme();
    useEffect(() => { document.body.style.backgroundColor = theme.colors.greyDark }, [])

    const loader = <Loader />;

    if (isLoading) {
        return loader;
    }

    const location = useLocation();

    return (
        <React.Suspense fallback={loader}>
            <Navbar />
            <Sidebar />
            <AppWrapper>
                <AnimatePresence>
                    <Routes location={location} key={location.pathname} >
                        <Route path="/" element={<BasePage />}>
                            <Route index element={<Home />} />
                            <Route path="champs" element={<Champs />} />
                            <Route path="champion" element={<ChampionPage />} />
                            <Route path="summoner/:summonerName" element={<SummonerPage />} />
                        </Route>
                    </Routes>
                </AnimatePresence>
            </AppWrapper>
        </React.Suspense >
    );
};

export const App: React.FC = () => {
    const [state, dispatch] = React.useReducer(appReducer, INITIAL_APP_STATE);

    return (
        <ThemeProvider theme={theme}>
            <BrowserRouter>
                <appContext.Provider value={{ state, dispatch }}>
                    <AppLayout isLoading={false} />
                </appContext.Provider>
            </BrowserRouter>
        </ThemeProvider>
    );
};

const AppWrapper = styled(Flex)`
    position: relative;
    margin-left: ${props => props.theme.layout.sidebarWidth}px;
    margin-top: ${props => props.theme.layout.navbarHeight}px;
`;
