import styled, { useTheme } from 'styled-components';
import Flex from '@ui/flex';
import SidebarItem from "./item";
import { faHouse } from '@fortawesome/free-solid-svg-icons';
import { faUser } from '@fortawesome/free-regular-svg-icons';

const sidebar = () => {
  const theme = useTheme();

  return (
    <SidebarContainer bg={theme.colors.secondary} column justify='start' align='center'>
      <SidebarLogo justify="center" align='center'>Logo</SidebarLogo>
      <SidebarItem icon={faHouse} label="Home" to="/" />
      <SidebarItem icon={faUser} label="Champs" to="/champs" />
      <SidebarItem icon={faUser} label="Profile" to="/profile" />
      <SidebarItem icon={faUser} label="About" to="/about" />
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

  border-bottom: 1px solid ${props => props.theme.colors.grey};
`;

export default sidebar;
