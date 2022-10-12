import 'styled-components';

export type GapOption = 4 | 6 | 8 | 10 | 12 | 14 | 16 | 20 | 24 | 28 | 32;
export type SizeOption = 0 | GapOption | 40 | 48 | 56 | 64 | 96 | 128 | 192 | 256;

interface IPalette {
    main: string
    contrastText: string
}

declare module 'styled-components' {
    export interface DefaultTheme {
        layout: {
            navbarHeight: SizeOption,

            sidebarWidth: SizeOption,
            sidebarItemHeight: SizeOption,
        },
        colors: {
            primaryLight: string,
            primary: string,
            primaryDark: string,

            secondaryLight: string,
            secondary: string,
            secondaryDark: string,

            text: string,

            neutralDarker: string,
            neutralDark: string,
            neutral: string,
            neutralLight: string,
            neutralLighter: string,

            greyDarker: string,
            greyDark: string,
            grey: string,
            greyLight: string,
            greyLighter: string,

            highlight: string,
        },
        fonts: string[],
        fontSizes: {
            small: string,
            medium: string,
            large: string
        }
    }
}