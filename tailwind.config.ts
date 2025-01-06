import typography from '@tailwindcss/typography';
import { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  plugins: [
    typography,
    // twP
  ],
  theme: {
    extend: {},
    colors: {
      white: '#fcfcfc',
      red: {
        DEFAULT: '#ff0342',
      },
      gray: {
        light: '#6c7a93',
        DEFAULT: '#3c404e',
        dark: '#1d1e25',
      },
      black: '#1d1e25',
      primary: '#a6d7e7',
      secondary: '#f5f5f5',
    },
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
