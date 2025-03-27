// import typography from '@tailwindcss/typography';
import { heroui } from "@heroui/react";
import { Config } from 'tailwindcss';

const COLORS = {
  primary: {
    '100': '#EFFDFD',
    '200': '#E0FBFC',
    '300': '#CDF3F7',
    '400': '#BDE7F0',
    '500': '#A6D7E7',
    '600': '#79AEC6',
    '700': '#5386A6',
    '800': '#346085',
    '900': '#1F446E',
    '950': '#122E52',
    '975': '#0A1E3C',
    DEFAULT: '#A6D7E7',
  },
  'black-alpha': {
    50: 'rgba(11, 11, 11, 0.04)',
    100: 'rgba(11, 11, 11, 0.06)',
    200: 'rgba(11, 11, 11, 0.08)',
    300: 'rgba(11, 11, 11, 0.16)',
    400: 'rgba(11, 11, 11, 0.24)',
    500: 'rgba(11, 11, 11, 0.36)',
    600: 'rgba(11, 11, 11, 0.48)',
    700: 'rgba(11, 11, 11, 0.64)',
    800: 'rgba(11, 11, 11, 0.80)',
    900: 'rgba(11, 11, 11, 0.92)',
  },
  'white-alpha': {
    50: 'rgba(255, 255, 255, 0.04)',
    100: 'rgba(255, 255, 255, 0.06)',
    200: 'rgba(255, 255, 255, 0.08)',
    300: 'rgba(255, 255, 255, 0.16)',
    400: 'rgba(255, 255, 255, 0.24)',
    500: 'rgba(255, 255, 255, 0.36)',
    600: 'rgba(255, 255, 255, 0.48)',
    700: 'rgba(255, 255, 255, 0.64)',
    800: 'rgba(255, 255, 255, 0.80)',
    900: 'rgba(255, 255, 255, 0.92)',
  },
} as const;

export default {
  content: [
    './src/**/*.{js,jsx,ts,tsx}',
    "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  plugins: [
    heroui({
      addCommonColors: true,
      themes: {
        light: {
          colors: {
            background: '#fcfcfc',
            foreground: '#1d1e25',
            // @ts-expect-error alpha is not a valid color
            alpha: COLORS['black-alpha'],
            primary: {
              ...COLORS.primary,
              foreground: '#1d1e25',
            },
          },
        },
        dark: {
          colors: {
            background: '#1d1e25',
            foreground: '#fcfcfc',
            // @ts-expect-error alpha is not a valid color
            alpha: COLORS['white-alpha'],
            primary: {
              ...COLORS.primary,
              foreground: '#1d1e25',
              // background: '#1d1e25',
            },
          },
        },
      },
    }),
  ],
  theme: {
    extend: {},
    colors: {
      // red: {
      //   DEFAULT: '#ff0342',
      // },
      // gray: {
      //   light: '#6c7a93',
      //   DEFAULT: '#3c404e',
      //   dark: '#1d1e25',
      // },
      // primary: {
      //   '100': '#EFFDFD',
      //   '200': '#E0FBFC',
      //   '300': '#CDF3F7',
      //   '400': '#BDE7F0',
      //   '500': '#A6D7E7',
      //   '600': '#79AEC6',
      //   '700': '#5386A6',
      //   '800': '#346085',
      //   '900': '#1F446E',
      // },
      // secondary: '#f5f5f5',
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
