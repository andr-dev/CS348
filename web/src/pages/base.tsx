import { motion } from "framer-motion";
import { Outlet } from "react-router-dom";

const BasePage = () => {
    return (
        <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}>
            <Outlet />
        </motion.div>
    );
};

export default BasePage;