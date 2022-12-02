import styled, { useTheme } from 'styled-components';
import Flex from '@ui/flex';
import SidebarItem from "./item";
import { faHouse, faHelmetSafety } from '@fortawesome/free-solid-svg-icons';
import { faHardDrive } from '@fortawesome/free-regular-svg-icons';

const sidebar = () => {
  const theme = useTheme();

  return (
    <SidebarContainer bg={theme.colors.secondary} column justify='start' align='center'>
      <SidebarLogoContainer>
        <SidebarLogo justify="center" align='center' grow={1}>GG.OP</SidebarLogo>
      </SidebarLogoContainer>
      <SidebarItem icon={faHouse} label="Home" to="/" />
      <SidebarItem icon={faHardDrive} label="Stats" to="/stats" />
      <SidebarItem icon={faHelmetSafety} label="Champion" to="/champion" />
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

const SidebarLogoContainer = styled.div`
  background: -webkit-linear-gradient(45deg, #818a8a 0%, white 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  `;
// Version with gradient border instead
// const SidebarLogoContainer = styled.div`
//   background: -webkit-linear-gradient(45deg, #30CFD0 0%, #330867 100%);
//   -webkit-background-clip: text;
//   -webkit-text-stroke: 0px transparent;
//   color: white
// `;

const SidebarLogo = styled(Flex)`
  width: 100%;
  height: ${props => props.theme.layout.navbarHeight}px;
  font-weight: bold;
  font-size: 24px;
  border-bottom: 1px solid ${props => props.theme.colors.grey};
`;

export default sidebar;
