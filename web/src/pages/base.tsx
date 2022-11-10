import { motion } from "framer-motion";
import { Outlet } from "react-router-dom";

import Flex from '@ui/flex';

const BasePage = () => {
    return (
        <motion.div
            style={{ position: "absolute" }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}>
            <Flex padding={[16]}>
                <Outlet />
            </Flex>
        </motion.div>
    );
};

export default BasePage;