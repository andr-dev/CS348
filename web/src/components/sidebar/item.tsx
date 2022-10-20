import styled, { useTheme } from "styled-components";
import Flex from '@ui/flex';
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Typography from "@ui/typography";
import { Link, To } from "react-router-dom";

const item = (props: {
    icon: IconProp
    label: string
    to: To
}) => {
    const theme = useTheme()

    return (
        <StyledLink to={props.to}>
            <Hover align='center'>
                <Flex padding={[16]} gap={16} align="center">
                    <FontAwesomeIcon icon={props.icon} size='xl' style={{ color: theme.colors.neutral }} fixedWidth />
                    <Typography level="button" children={props.label} />
                </Flex>
            </Hover>
        </StyledLink>
    )
}

const StyledLink = styled(Link)`
    width: 100%
`;

const Hover = styled(Flex)`
    width: 100%;
    height: ${props => props.theme.layout.sidebarItemHeight}px;

    :hover {
        background: rgba(255, 255, 255, 0.1);
        border-right: 1px solid ${props => props.theme.colors.primary};
    }
`;

export default item;
