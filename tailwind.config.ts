import { Config } from 'tailwindcss';
import twPlugin from 'tailwindcss/plugin';
import typography from '@tailwindcss/typography';
import { type CSSRuleObject } from 'tailwindcss/types/config';

export default {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  plugins: [
    typography,
    // twP
  ],
  theme: {
    extend: {},
    // borderRadius: tokens.borderRadius,
    // boxShadow: tokens.boxShadow,
    // colors: {
    //   ...tokens.colors,
    //   // Semantic colors
    //   primary: tokens.colors.blue,
    //   success: tokens.colors.green,
    //   warning: tokens.colors.red,
    //   notification: tokens.colors.yellow,
    // },
    // fontFamily: tokens.fontFamily,p
    // fontWeight: tokens.fontWeight,
    // fontSize: tokens.fontSize,
    // lineHeight: tokens.lineHeight,
    // screens: { ...tokens.breakpoints },
  },
} satisfies Config;
