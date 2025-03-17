import '~/css/index.css'
// import 'pliny/search/algolia.css'
// import 'remark-github-blockquote-alert/alert.css'
import { Space_Grotesk } from 'next/font/google'
// import { Analytics, AnalyticsConfig } from 'pliny/analytics'
// import { SearchProvider, SearchConfig } from 'pliny/search'
import {Header} from '~/components/layout/header/Header'
import SectionContainer from 'src/components/SectionContainer'
import Footer from 'src/components/Footer'
import siteMetadata from 'src/data/siteMetadata'
import { ThemeProviders } from '../../components/providers/theme-providers'
import { Metadata } from 'next'
import { Providers } from '~/components/providers'
import { SidebarInset, SidebarProvider } from '~/components/ui/sidebar'
import { AppSidebar } from '~/components/layout/sidebar'
import { ModeToggleGradientIcon } from '~/components/common/ModeToggle'
import {ScrollShadow} from "@heroui/scroll-shadow";
import { Toaster } from "~/components/ui/sonner"
import { ProgressBar, ProgressBarWithSuspense } from "~/components/layout/header/ProgressBar";
import { dir } from 'i18next'
import initTranslations from '~/app/i18n/i18n'
import TranslationsProvider from '~/app/i18n/TranslationsProvider'
import { i18nConfig } from '~/app/i18n/i18nConfig'


const space_grotesk = Space_Grotesk({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-space-grotesk',
})

export const metadata: Metadata = {
  metadataBase: new URL(siteMetadata.siteUrl),
  title: {
    default: siteMetadata.title,
    template: `%s | ${siteMetadata.title}`,
  },
  description: siteMetadata.description,
  openGraph: {
    title: siteMetadata.title,
    description: siteMetadata.description,
    url: './',
    siteName: siteMetadata.title,
    images: [siteMetadata.socialBanner],
    locale: 'en_US',
    type: 'website',
  },
  alternates: {
    canonical: './',
    types: {
      'application/rss+xml': `${siteMetadata.siteUrl}/feed.xml`,
    },
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  twitter: {
    title: siteMetadata.title,
    card: 'summary_large_image',
    images: [siteMetadata.socialBanner],
  },
}

export async function generateStaticParams() {
  return i18nConfig.locales.map((locale) => ({ locale }))
}

export default async function RootLayout({params, children }: { params: Promise<{ locale: string }>,
  children: React.ReactNode }) {
  const { locale } = await params
  const { t, resources } = await initTranslations(locale, ['common']);
  const basePath = process.env.BASE_PATH || ''

  return <html lang={locale} dir={dir(locale)} className={`${space_grotesk.variable} scroll-smooth`} suppressHydrationWarning>
    {/* // 为 iOS 设备指定应用图标 */}
    <link rel="apple-touch-icon" sizes="76x76" href={`${basePath}/icon/cat-76.webp`}/>
    {/* 如果没有指定 <link rel="icon">，浏览器会自动向根目录发起请求，尝试加载 favicon.ico */} 
    {/* // 书签栏可能需要稍大一些的图标（如 32x32） */}
    <link rel="icon" type="image/webp"sizes="64x64"href={`${basePath}/icon/cat-64-round-50.webp`}/>
    {/* // 书签栏可能需要稍大一些的图标（如 32x32） */}
    <link rel="icon" type="image/webp" sizes="32x32"href={`${basePath}/icon/cat-32-round-50.webp`}/>
    <link
      rel="icon" // 浏览器标签页通常使用较小的图标（如 16x16
      type="image/webp"
      sizes="16x16"
      href={`${basePath}/icon/cat-16-round-50.webp`}
    />
    {/* 指定网站的 Web 应用清单文件 */}
    <link rel="manifest" href={`${basePath}/site.webmanifest`} />
    <link
      rel="mask-icon" // 为 Safari 浏览器指定图标和颜色
      href={`${basePath}/static/favicons/safari-pinned-tab.svg`}
      color="#5bbad5"
    />
    <meta name="msapplication-TileColor" content="#000000" />
    <meta name="theme-color" media="(prefers-color-scheme: light)" content="#fff" />
    <meta name="theme-color" media="(prefers-color-scheme: dark)" content="#000" />
    <link rel="alternate" type="application/rss+xml" href={`${basePath}/feed.xml`} />
    <body className={`antialiased max-h-screen`}><TranslationsProvider
      namespaces={['common']}
      locale={locale}
      resources={resources}>
      <Providers attribute="class" defaultTheme={siteMetadata.theme} enableSystem>
        {/* <Analytics analyticsConfig={siteMetadata.analytics as AnalyticsConfig} /> */}
        {/* <SectionContainer> */}
          {/* <SearchProvider searchConfig={siteMetadata.search as SearchConfig}> */}
          <SidebarProvider>
            <AppSidebar />
            {/* <ScrollShadow className='h-svh' > */}
            <SidebarInset className='bg-background/20 '>
              <Header  />

              {/* <ScrollShadow  className='max-h-screen' > */}

              {/* <section className=" flex flex-1 flex-col max-h-full">
              </section> */}
                {children}
    
              <Footer />
              {/* </ScrollShadow> */}
            </SidebarInset>
{/* </ScrollShadow> */}
          </SidebarProvider>
          {/* <ModeToggleGradientIcon /> */}
          {/* </SearchProvider> */}
        {/* </SectionContainer> */}
        <ProgressBarWithSuspense />
        <Toaster position="top-right" richColors   />
      </Providers>
    </TranslationsProvider>
    </body>
  </html>
}
