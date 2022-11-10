import { DefaultTheme } from 'styled-components'

const theme: DefaultTheme = {
    layout: {
        navbarHeight: 48,

        sidebarWidth: 192,
        sidebarItemHeight: 56
    },
    colors: {
        primaryLight: "rgb(232, 95, 114)",
        primary: "rgb(232, 64, 87)",
        primaryDark: "",

        secondaryLight: "rgb(65, 84, 126)",
        secondary: "rgb(50, 66, 98)",
        secondaryDark: "rgb(36, 47, 70)",

        text: "rgb(245, 245, 245)",

        neutralDarker: "",
        neutralDark: "",
        neutral: "rgb(255, 255, 255, 0.6)",
        neutralLight: "",
        neutralLighter: "",

        greyDarker: "",
        greyDark: "rgb(28, 28, 31)",
        grey: "rgb(105, 105, 105)",
        greyLight: "",
        greyLighter: "",

        highlight: "rgb(245, 245, 245)",
    },
    fonts: ["sans-serif", "Roboto"],
    fontSizes: {
        small: "1em",
        medium: "2em",
        large: "3em"
    }
};

export default theme;
