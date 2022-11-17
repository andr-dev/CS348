import { Drawer, Typography } from '@mui/material'
import React, { FC } from 'react'
import { HelpDrawerProps } from './types'

export const HelpDrawer: FC<HelpDrawerProps> = ({ open, onClose }) => {
    return (
        <Drawer anchor='right' open={open} onClose={onClose}>
            <Typography>
                Hello, you have opened the Help Drawer. We'll probably want to give an intro to the app,
                what it does, some key features, and stuff.
            </Typography>
        </Drawer>
    )
}
