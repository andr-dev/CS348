import Typography from "@ui/typography";
import styled from "styled-components";

const button = (props: {
    label: string
    pill?: boolean
}) => {
    return (
        <StyledButton style={{ padding: props.pill ? "1px 10px" : "6px 14px", margin: props.pill ? "6px" : 0 }}>
            <Typography level='button' children={props.label} />
        </StyledButton>
    )
}

const StyledButton = styled.button`
    position: relative;
    border-radius: 4px;
    border: none;
    background-color: ${props => props.theme.colors.primary};
    cursor: pointer;

    :hover {
        background: rgba(255, 255, 255, 0.1);
    }
`;

export default button;
