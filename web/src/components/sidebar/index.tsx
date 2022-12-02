import styled, { useTheme } from 'styled-components';
import Flex from '@ui/flex';
import SidebarItem from "./item";
import { faHouse } from '@fortawesome/free-solid-svg-icons';
import { faUser } from '@fortawesome/free-regular-svg-icons';

const sidebar = () => {
  const theme = useTheme();

  return (
    <SidebarContainer bg={theme.colors.secondary} column justify='start' align='center'>
      <SidebarLogo justify="center" align='center'>GG.OP</SidebarLogo>
      <SidebarItem icon={faHouse} label="Home" to="/" />
      <SidebarItem icon={faUser} label="Stats" to="/stats" />
      <SidebarItem icon={faUser} label="Champion" to="/champion" />
    </SidebarContainer>
  )
}

const SidebarContainer = styled(Flex)`
  position: fixed;
  left: ${props => props.theme.layout.sidebarWidth}px;
  left: 0;
  top: 0;
  z-index: 2000;

  width: ${props => props.theme.layout.sidebarWidth}px;
  height: 100vh;

  border-right: 1px solid ${props => props.theme.colors.grey};
`;

const SidebarLogo = styled(Flex)`
  width: 100%;
  height: ${props => props.theme.layout.navbarHeight}px;
  color: rgb(245, 245, 245);
  font-weight: bold;
  border-bottom: 1px solid ${props => props.theme.colors.grey};
`;

export default sidebar;
