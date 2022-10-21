import React from 'react';
import { GapOption, SizeOption } from '@style';

type JustifyOptions = 'start' | 'end' | 'center' | 'between' | 'around';

export type PaddingOptions = [SizeOption, SizeOption?, SizeOption?, SizeOption?];

type BorderOptions = [1 | 3, string];

interface IProps {
  column?: boolean;
  justify?: JustifyOptions
  align?: JustifyOptions
  grow?: number
  gap?: GapOption;
  bg?: string;
  padding?: PaddingOptions;
  borderBottom?: BorderOptions;

  height?: string

  children: React.ReactNode;
  className?: string;
}

const flex: React.FC<IProps> = ({ column, justify, align, grow, gap, bg, padding, borderBottom, height, className, children }) => {
  return (
    <div style={{
      display: "flex",
      flexDirection: column ? "column" : "row",
      flexWrap: "nowrap",
      flexGrow: grow,
      justifyContent: getJustify(justify),
      alignItems: getJustify(align),
      gap: getGap(gap),
      backgroundColor: bg,

      height,

      padding: getPadding(padding),
      borderBottom: getBorder(borderBottom)
    }} className={className}>
      {children}
    </div>
  );
};

const getJustify = (justify?: JustifyOptions): string => {
  switch (justify) {
    case 'start':
    case 'end':
      return "flex-" + justify;
    case 'between':
    case 'around':
      return "space-" + justify;
    case 'center':
      return "center"
  };

  return "flex-start";
}

const getGap = (gap?: GapOption): string | undefined => {
  if (gap === undefined) {
    return;
  }

  return gap + "px";
}

const getPadding = (padding?: PaddingOptions): string | undefined => {
  if (padding === undefined) {
    return;
  }

  return padding.map((pad) => pad + "px").join(" ");
}

const getBorder = (border?: BorderOptions): string | undefined => {
  if (border == undefined) {
    return;
  }

  return border[0] + "px solid " + border[1];
}

export default flex;