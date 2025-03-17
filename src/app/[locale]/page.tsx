// import { sortPosts, allCoreContent } from 'pliny/utils/contentlayer'
// import { allBlogs } from 'contentlayer/generated'
import Main from '../Main'

// export default async function Page(){
//   // const sortedPosts = sortPosts(allBlogs)
//   // const posts = allCoreContent(sortedPosts)
//   // return <Main posts={posts} />
// }
import { Suspense } from 'react';
import { LoadingS } from '~/components/ui/loading/Loading';
import { Snippet } from "@heroui/snippet";
import { Link } from "@heroui/link";
import { Code } from "@heroui/code";
import { button as buttonStyles } from "@heroui/theme";
import {ScrollShadow} from "@heroui/scroll-shadow";
import { siteConfig } from "~/config/site";
import { title, subtitle } from "~/components/primitives";
import { GithubIcon } from "~/components/icons";
import { TextUpView } from '~/components/ui/transition/TextUpView';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { ModeToggleGradientIcon } from '~/components/common/ModeToggle';
import {Button as UIButton} from "@heroui/react";
import { SearchModal } from '~/components/common/search';
import { StartedButton } from '../_comp/Started';
import initTranslations from '../i18n/i18n';
import TranslationsProvider from '../i18n/TranslationsProvider';



const i18nNamespaces = ['home'];
export default async function Page ({
  params,
  searchParams,
}: {
  params: Promise<{ locale: string }>,
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const { locale } = await params
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  type TextItem = {
    text: string
    initialDelay?: number
    rootAs: keyof typeof motion
    className?: string
  }
  const gapDelay = 0.05
  // 动态计算每个元素的 initialDelay
  let accumulatedDelay = 0
  // console.log(titleTextLsWithDelay)
  const subTitleTextLs: TextItem[] = [
    { text: "Beautiful, fast, and modern. 探索多领域の文档中心", rootAs: "span" },
    { text: "建议使用 firefox core 的浏览器", rootAs: "p" },
  ]
  const subTitleTextLsWithDelay = subTitleTextLs.map((element, index) => {
    const delay = accumulatedDelay
    accumulatedDelay += element.text.length * gapDelay
    return { ...element, initialDelay: delay }
  })

  // const { page = '1', sort = 'asc', query = '' } = await searchParams
  return <Suspense fallback={<LoadingS />}><TranslationsProvider
  namespaces={i18nNamespaces}
  locale={locale}
  resources={resources}>
    <section className="flex flex-1 max-h-full flex-col items-center justify-center gap-4 py-8 md:py-10">
      <div className=" max-w-2xl  flex flex-col items-center justify-center gap-6">
        <h1 className={`${title()} `}>
        {t('Welcome_to')} <span className='gradient'>MdxHub</span>
        </h1>
        <div className={`${subtitle()} flex flex-col items-center justify-center`}>
          {subTitleTextLsWithDelay.map((v: TextItem, i: number) => {
            return <TextUpView
              rootAs={v.rootAs}
              className={`${v.className ?? ""}`}
              key={v.text} as="span"
              initialDelay={v.initialDelay} eachDelay={gapDelay}
            >
              {v.text}
            </TextUpView>
          })}
        </div>
        <div className="flex gap-4 max-w-2xl justify-center w-full">
          <StartedButton text={t('Started')} href={siteConfig.links.docs} />
          <SearchModal />
        </div>
      </div>


      <ModeToggleGradientIcon />
      {/* <div className="mt-8 bg-[calc(var(--color-blue-400))] bg-b">
        <Snippet hideCopyButton hideSymbol variant="bordered">
          <span>
            Get started by editing <Code color="primary">app/page.tsx</Code>
          </span>
        </Snippet>
      </div> */}
    </section>
  </TranslationsProvider>
  </Suspense>
}