import Link from 'src/components/Link'
import initTranslations from '../i18n/i18n';
import { headers, cookies } from 'next/headers'
import { i18nConfig } from '../i18n/i18nConfig';
import { ToPrevPageButton } from '@/components/common/button';
import { button as buttonStyles } from "@heroui/theme";
import { SearchButton } from '@/components/search/search-button';
export default async function NotFound() {
  const cookieStore = await cookies()
  const locale = cookieStore.get('NEXT_LOCALE')?.value || i18nConfig.defaultLocale
  const { t, resources } = await initTranslations(locale, ['common']);
  return (<section className='mx-auto'>
      <SearchButton text={t('Quick_search')} className='w-full' />
    <div className="flex flex-col items-center justify-center md:mt-24 md:flex-row md:items-center md:justify-center md:space-x-6">
        
      <div className="space-x-2 py-8 md:space-y-5">
        <h1 className="text-6xl font-extrabold leading-9 tracking-tight text-gray-900 dark:text-gray-100 md:border-r-2 md:px-6 md:text-8xl md:leading-14">
          404
        </h1>
      </div>
      <div className="max-w-md">
        <p className="mb-4 text-xl text-center font-bold leading-normal md:text-2xl">
        {t('sorryPageNotFound')}
        </p>
        <p className="mb-8">{t('redirectToHomepage')}</p>
        <div className='flex gap-3 justify-center '>
        <Link
          href="/"
          className={`focus:shadow-outline-blue inline rounded-lg border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium leading-5 text-white shadow transition-colors duration-150 hover:bg-blue-700 focus:outline-none dark:hover:bg-blue-500 h-10 ${buttonStyles()} bg-pink-blue-animated`}
        >
          {t('back-to-home')}
        </Link>
        <ToPrevPageButton text={t('back-to-prev')} />
        </div>
      </div>
    </div>
  </section>
  )
}
