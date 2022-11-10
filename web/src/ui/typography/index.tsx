import classNames from 'classnames'
import React from 'react'
import styled, { AnyStyledComponent, DefaultTheme } from 'styled-components'

type TypographyLevels =
    | 'button'
    | 'input'
    | 'header';

type TypographyPropsType = {
    className?: string
    level?: TypographyLevels
    color?: keyof DefaultTheme['colors']
    children?: React.ReactNode
    onClick?: () => void
    style: any
    placeholder?: string
}

Typography.defaultProps = {
    level: 'button',
    color: undefined,
    className: '',
    children: null,
    onClick: undefined,
    style: undefined,
    placeholder: undefined
}

function Typography({
    level = 'button',
    color,
    className = '',
    children,
    onClick,
    style,
    placeholder,
}: TypographyPropsType): React.ReactElement {
    const componentMap: Record<TypographyLevels, string> = {
        button: 'span',
        input: 'input',
        header: 'h1',
    }

    const colorMap: Record<TypographyLevels, keyof DefaultTheme['colors']> = {
        button: 'text',
        input: 'grey',
        header: 'text',
    }

    const HtmlTag = componentMap[level];
    const c = color ?? colorMap[level];

    const C = Component.withComponent(HtmlTag as AnyStyledComponent);

    return <C color={c} className={classNames(level, className)} style={style} children={children} onClick={onClick} placeholder={placeholder} />
}

interface ComponentProps {
    color: keyof DefaultTheme['colors'];
}

const Component = styled.div<ComponentProps>`
    color: ${(props) => props.theme.colors[props.color]}
`;

export default Typography