import { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  plugins: [require('@tailwindcss/typography')],
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
