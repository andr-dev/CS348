import styled, { useTheme } from "styled-components";
import Flex, { PaddingOptions } from '@ui/flex';
import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

const item = (props: {
    icon: IconProp
    onClick?: () => void
    padding?: PaddingOptions
}) => {
    const theme = useTheme()

    return (
        <Hover align='center'>
            <Flex padding={props.padding ?? [4, 16]} onClick={props.onClick}>
                <FontAwesomeIcon icon={props.icon} size='lg' style={{ color: theme.colors.neutral }} />
            </Flex>
        </Hover>
    )
}

const Hover = styled(Flex)`
    height: 100%;

    :hover {
        background: rgba(255, 255, 255, 0.1);
    }
`;

export default item;
