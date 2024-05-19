import React from "react";
import { Box } from "@mui/material";
import { Contribute } from "../content/Contribute/Contribute";
import { ECE } from "../content/ECE/ECE";
import { Marketplace } from "../content/Marketplace/Marketplace";

export const BlendedWalletMode = (): JSX.Element => (
    <Box className="flex flex-col items-center justify-center w-full h-screen bg-gray-100">
        <Contribute />
        <ECE />
        <Marketplace />
    </Box>
);