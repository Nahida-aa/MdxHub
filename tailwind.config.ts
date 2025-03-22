import type { Config } from "tailwindcss";
// @ts-check
// const { fontFamily } = require('tailwindcss/defaultTheme')
import { fontFamily } from 'tailwindcss/defaultTheme'
import colors from 'tailwindcss/colors'
import {heroui} from "@heroui/theme"

const config: Config = {
  darkMode: ['class'],
  content: [
    './node_modules/pliny/**/*.js',
    './src/app/**/*.{js,ts,jsx,tsx}',
    './src/pages/**/*.{js,ts,tsx}',
    './src/components/**/*.{js,ts,tsx}',
    './src/layouts/**/*.{js,ts,tsx}',
    './src/data/**/*.mdx',
    "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}"
  ],
  theme: {
  	extend: {
  		lineHeight: {
  			'11': '2.75rem',
  			'12': '3rem',
  			'13': '3.25rem',
  			'14': '3.5rem'
  		},
  		fontFamily: {
  			sans: [
  				// 'var(--font-space-grotesk)',
          ...fontFamily.sans
        ]
  		},
  		colors: {
  			// gray: 'colors.gray',
  			background: 'hsl(var(--background))',
  			foreground: 'hsl(var(--foreground))',
  			card: {
  				DEFAULT: 'hsl(var(--card))',
  				foreground: 'hsl(var(--card-foreground))'
  			},
  			popover: {
  				DEFAULT: 'hsl(var(--popover))',
  				foreground: 'hsl(var(--popover-foreground))'
  			},
  			primary: {
  				DEFAULT: 'hsl(var(--primary))',
  				foreground: 'hsl(var(--primary-foreground))'
  			},
  			secondary: {
  				DEFAULT: 'hsl(var(--secondary))',
  				foreground: 'hsl(var(--secondary-foreground))'
  			},
  			muted: {
  				DEFAULT: 'hsl(var(--muted))',
  				foreground: 'hsl(var(--muted-foreground))'
  			},
  			accent: {
  				DEFAULT: 'hsl(var(--accent))',
  				foreground: 'hsl(var(--accent-foreground))'
  			},
  			destructive: {
  				DEFAULT: 'hsl(var(--destructive))',
  				foreground: 'hsl(var(--destructive-foreground))'
  			},
  			border: 'hsl(var(--border))',
  			input: 'hsl(var(--input))',
  			ring: 'hsl(var(--ring))',
  			chart: {
  				'1': 'hsl(var(--chart-1))',
  				'2': 'hsl(var(--chart-2))',
  				'3': 'hsl(var(--chart-3))',
  				'4': 'hsl(var(--chart-4))',
  				'5': 'hsl(var(--chart-5))'
  			},
  			sidebar: {
  				DEFAULT: 'hsl(var(--sidebar-background))',
  				foreground: 'hsl(var(--sidebar-foreground))',
  				primary: 'hsl(var(--sidebar-primary))',
  				'primary-foreground': 'hsl(var(--sidebar-primary-foreground))',
  				accent: 'hsl(var(--sidebar-accent))',
  				'accent-foreground': 'hsl(var(--sidebar-accent-foreground))',
  				border: 'hsl(var(--sidebar-border))',
  				ring: 'hsl(var(--sidebar-ring))'
  			}
  		},
  		zIndex: {
  			'60': '60',
  			'70': '70',
  			'80': '80'
  		},
  		// typography: ({ theme }) => ({
      //   DEFAULT: {
      //     css: {
      //       a: {
      //         color: theme('colors.primary.500'),
      //         '&:hover': {
      //           color: `${theme('colors.primary.600')}`,
      //         },
      //         code: { color: theme('colors.primary.400') },
      //       },
      //       'h1,h2': {
      //         fontWeight: '700',
      //         letterSpacing: theme('letterSpacing.tight'),
      //       },
      //       h3: {
      //         fontWeight: '600',
      //       },
      //       code: {
      //         color: theme('colors.indigo.500'),
      //       },
      //     },
      //   },
      //   invert: {
      //     css: {
      //       a: {
      //         color: theme('colors.primary.500'),
      //         '&:hover': {
      //           color: `${theme('colors.primary.400')}`,
      //         },
      //         code: { color: theme('colors.primary.400') },
      //       },
      //       'h1,h2,h3,h4,h5,h6': {
      //         color: theme('colors.gray.100'),
      //       },
      //     },
      //   },
      // }),
  		borderRadius: {
  			lg: 'var(--radius)',
  			md: 'calc(var(--radius) - 2px)',
  			sm: 'calc(var(--radius) - 4px)'
  		}
  	}
  },
  plugins: [require('@tailwindcss/forms'), require('@tailwindcss/typography'), heroui(), require("tailwindcss-animate")],
};
export default config;