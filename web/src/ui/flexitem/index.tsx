import styled from 'styled-components';
import is from '@lib/is';

interface IProps {
  shrink: number | string;
  grow: number | string;
  basis: number;
  order: number;
}

export default styled.div.withConfig({
  shouldForwardProp: (prop) => ['children'].includes(prop),
})`
  order: 0;
  flex-basis: auto;
  flex-grow: 0;
  flex-shrink: 1;
  display: block;

  ${is('inlineBlock')`
    display: inline-block;
  `};

  ${is('inlineFlex')`
    display: inline-flex;
  `};

  ${is('flex')`
    display: flex;
  `};

  /********************************** order **********************************/
  /****************** http://cssreference.io/property/order ******************/

  ${is('order')`
    order: ${(props: IProps) => props.order};
  `};

  /******************************** flex-basis ********************************/
  /**************** http://cssreference.io/property/flex-basis ****************/

  ${is('basis')`
    flex-basis: ${(props: IProps) => props.basis};
  `};

  /******************************** flex-grow ********************************/
  /**************** http://cssreference.io/property/flex-grow ****************/

  ${is('grow')`
    flex-grow: ${(props: IProps) => props.grow};
  `};

  /******************************* flex-shrink *******************************/
  /*************** http://cssreference.io/property/flex-shrink ***************/

  ${is('shrink')`
    flex-shrink: ${(props: IProps) => props.shrink};
  `};

  ${is('noShrink')`
    flex-shrink: 0;
  `};
`;