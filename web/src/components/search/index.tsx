import styled from "styled-components";
import Flex from '@ui/flex';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';
import Typography from "@ui/typography";
import Button from "@components/button";

const search = () => {
    return (
        <SearchContainer>
            <SearchWrapper justify="around" gap={8} align="center" bg="#FFF">
                <FontAwesomeIcon icon={faMagnifyingGlass} style={{ color: "rgba(0, 0, 0, 0.4)", marginLeft: "8px" }} size="sm" />
                <SearchInput level="input" placeholder="Search..." />
                <Button label="NA" pill />
            </SearchWrapper>
        </SearchContainer>
    )
}

const SearchContainer = styled.div`
    padding: 8px;
`;

const SearchWrapper = styled(Flex)`
    width: 100%;
    max-width: 256px;

    border-radius: 6px;
`;

const SearchInput = styled(Typography)`
    flex-grow: 1;
`;

export default search;