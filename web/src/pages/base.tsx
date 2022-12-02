import { motion } from "framer-motion";
import { Outlet } from "react-router-dom";

import styled from "styled-components";

const BasePage: React.FC = (props) => {
    return (
        <motion.div
            style={{ position: "absolute", width: "100%", height: "auto" }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}>
            <FullHeightDiv>
                <Outlet />
            </FullHeightDiv>
        </motion.div>
    );
};

const FullHeightDiv = styled.div`
    height: calc(100vh - ${(props) => props.theme.layout.navbarHeight}px - 32px);
    padding: 32px;
`;

export default BasePage;
