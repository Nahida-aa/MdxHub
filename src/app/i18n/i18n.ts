import { createInstance, i18n, Resource } from 'i18next';
import { initReactI18next } from 'react-i18next/initReactI18next';
import resourcesToBackend from 'i18next-resources-to-backend';
import { i18nConfig } from './i18nConfig';

// export type ResourceKey =
//   | string
//   | {
//       [key: string]: any;
//     };
// export interface ResourceLanguage {
//   [namespace: string]: ResourceKey;
// }
// export interface Resource {
//   [language: string]: ResourceLanguage;
// }

// type InitTranslationsParams = {
//   locale: string;
//   namespaces: string[];
//   i18nInstance?: i18n
//   resources?: Resource;
// }
// type InitTranslationsFn = (params: InitTranslationsParams) => Promise<{
//   i18n: i18n;
//   resources: Record<string, Record<string, unknown>>;
//   t: i18n['t'];
// }>;

export default async function initTranslations(
  locale: string,
  namespaces: string[],
  i18nInstance?: i18n,
  resources?: Resource
) {
  i18nInstance = i18nInstance || createInstance();

  i18nInstance.use(initReactI18next);

  if (!resources) {
    i18nInstance.use(
      resourcesToBackend(
        (language, namespace) =>
          import(`~/app/i18n/locales/${language}/${namespace}.json`)
      )
    );
  }

  await i18nInstance.init({
    lng: locale,
    resources,
    fallbackLng: i18nConfig.defaultLocale,
    supportedLngs: i18nConfig.locales,
    defaultNS: namespaces[0],
    fallbackNS: namespaces[0],
    ns: namespaces,
    preload: resources ? [] : i18nConfig.locales
  });

  return {
    i18n: i18nInstance,
    resources: { [locale]: i18nInstance.services.resourceStore.data[locale] },
    t: i18nInstance.t
  };
}