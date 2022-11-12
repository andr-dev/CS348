import styled, { useTheme } from 'styled-components';
import Flex from '@ui/flex';
import { faLanguage } from '@fortawesome/free-solid-svg-icons';
import { faCircleQuestion } from '@fortawesome/free-regular-svg-icons';

import NavbarItem from "./item";
import Button from '@components/button';
import Search from '@components/search';
import { useState } from 'react';
import { HelpDrawer } from '@components/help-drawer';

const navbar = () => {
  const theme = useTheme();
  const [openHelpDrawer, setOpenHelpDrawer] = useState<boolean>(false)

  const handleClickSignIn = () => {} // todo
  const handleClickHelp = () => {
    setOpenHelpDrawer(true)
  }
  const handleCloseHelpDrawer = () => {
    setOpenHelpDrawer(false)
  }

  return (
    <>
      <NavbarContainer bg={theme.colors.secondary} justify='between' align='center'>
        <Flex align='center' height="100%" grow={1}>
          <Search />
        </Flex>
        <Flex align='center' height="100%">
          <ButtonWrapper>
            <Button label="SIGN IN" onClick={handleClickSignIn}/>
          </ButtonWrapper>
          <NavbarItem icon={faCircleQuestion} padding={[0, 14]} onClick={handleClickHelp}/>
          <NavbarItem icon={faLanguage} padding={[0, 12]} />
        </Flex>
      </NavbarContainer>
      <HelpDrawer open={openHelpDrawer} onClose={handleCloseHelpDrawer}/>
    </>
  )
}

const NavbarContainer = styled(Flex)`
  position: fixed;
  left: ${props => props.theme.layout.sidebarWidth}px;
  right: 0;
  top: 0;
  z-index: 1000;

  height: ${props => props.theme.layout.navbarHeight}px;
  border-bottom: 1px solid ${props => props.theme.colors.grey};
`;

const ButtonWrapper = styled.div`
  padding: 0 8px;
`;

export default navbar;
