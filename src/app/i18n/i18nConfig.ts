import { Config } from "next-i18n-router/dist/types";

export const i18nConfig: Config = {
  locales: ['zh', 'en', 
    // 'es', 'fr'
  ],
  defaultLocale: 'zh',
  prefixDefault: false
}
