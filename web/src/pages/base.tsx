import { motion } from "framer-motion";
import { Outlet } from "react-router-dom";

import Flex from '@ui/flex';

const BasePage = () => {
    return (
        <motion.div
            style={{ position: "absolute", width: "100%" }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}>
            <Flex padding={[16]} grow={1}>
                <Outlet />
            </Flex>
        </motion.div>
    );
};

export default BasePage;