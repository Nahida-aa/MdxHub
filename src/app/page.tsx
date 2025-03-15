// import { sortPosts, allCoreContent } from 'pliny/utils/contentlayer'
// import { allBlogs } from 'contentlayer/generated'
import Main from './Main'

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

export const Content = () => <div>
    <p>
      Sit nulla est ex deserunt exercitation anim occaecat. Nostrud ullamco deserunt aute id
      consequat veniam incididunt duis in sint irure nisi. Mollit officia cillum Lorem ullamco minim
      nostrud elit officia tempor esse quis.
    </p>
    <p>
      Sunt ad dolore quis aute consequat. Magna exercitation reprehenderit magna aute tempor
      cupidatat consequat elit dolor adipisicing. Mollit dolor eiusmod sunt ex incididunt cillum
      quis. Velit duis sit officia eiusmod Lorem aliqua enim laboris do dolor eiusmod. Et mollit
      incididunt nisi consectetur esse laborum eiusmod pariatur proident Lorem eiusmod et. Culpa
      deserunt nostrud ad veniam.
    </p>
    <p>
      Est velit labore esse esse cupidatat. Velit id elit consequat minim. Mollit enim excepteur ea
      laboris adipisicing aliqua proident occaecat do do adipisicing adipisicing ut fugiat.
      Consequat pariatur ullamco aute sunt esse. Irure excepteur eu non eiusmod. Commodo commodo et
      ad ipsum elit esse pariatur sit adipisicing sunt excepteur enim.
    </p>
    <p>
      Incididunt duis commodo mollit esse veniam non exercitation dolore occaecat ea nostrud
      laboris. Adipisicing occaecat fugiat fugiat irure fugiat in magna non consectetur proident
      fugiat. Commodo magna et aliqua elit sint cupidatat. Sint aute ullamco enim cillum anim ex.
      Est eiusmod commodo occaecat consequat laboris est do duis. Enim incididunt non culpa velit
      quis aute in elit magna ullamco in consequat ex proident.
    </p>
    <p>
      Dolore incididunt mollit fugiat pariatur cupidatat ipsum laborum cillum. Commodo consequat
      velit cupidatat duis ex nisi non aliquip ad ea pariatur do culpa. Eiusmod proident adipisicing
      tempor tempor qui pariatur voluptate dolor do ea commodo. Veniam voluptate cupidatat ex nisi
      do ullamco in quis elit.
    </p>
    <p>
      Cillum proident veniam cupidatat pariatur laborum tempor cupidatat anim eiusmod id nostrud
      pariatur tempor reprehenderit. Do esse ullamco laboris sunt proident est ea exercitation
      cupidatat. Do Lorem eiusmod aliqua culpa ullamco consectetur veniam voluptate cillum. Dolor
      consequat cillum tempor laboris mollit laborum reprehenderit reprehenderit veniam aliqua
      deserunt cupidatat consequat id.
    </p>
    <p>
      Est id tempor excepteur enim labore sint aliquip consequat duis minim tempor proident. Dolor
      incididunt aliquip minim elit ea. Exercitation non officia eu id.
    </p>
    <p>
      Ipsum ipsum consequat incididunt do aliquip pariatur nostrud. Qui ut sint culpa labore Lorem.
      Magna deserunt aliquip aute duis consectetur magna amet anim. Magna fugiat est nostrud veniam.
      Officia duis ea sunt aliqua.
    </p>
    <p>
      Ipsum minim officia aute anim minim aute aliquip aute non in non. Ipsum aliquip proident ut
      dolore eiusmod ad fugiat fugiat ut ex. Ea velit Lorem ut et commodo nulla voluptate veniam ea
      et aliqua esse id. Pariatur dolor et adipisicing ea mollit. Ipsum non irure proident ipsum
      dolore aliquip adipisicing laborum irure dolor nostrud occaecat exercitation.
    </p>
    <p>
      Culpa qui reprehenderit nostrud aliqua reprehenderit et ullamco proident nisi commodo non ut.
      Ipsum quis irure nisi sint do qui velit nisi. Sunt voluptate eu reprehenderit tempor consequat
      eiusmod Lorem irure velit duis Lorem laboris ipsum cupidatat. Pariatur excepteur tempor veniam
      cillum et nulla ipsum veniam ad ipsum ad aute. Est officia duis pariatur ad eiusmod id
      voluptate.
    </p>
  </div>

export default async function Page ({
  searchParams,
}: {
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  type TextItem = {
    text: string
    initialDelay?: number
    rootAs: keyof typeof motion
    className?: string
  }
  const gapDelay = 0.05
  const titleTextLs: TextItem[] = [
    { text: "Welcome to ", initialDelay: 0, rootAs: "span" },
    { text: "MdxHub !", initialDelay: 1.5, rootAs: "span", className: "gradient" },
  ]
  // 动态计算每个元素的 initialDelay
  let accumulatedDelay = 0
  const titleTextLsWithDelay = titleTextLs.map((element, index) => {
    const delay = accumulatedDelay
    accumulatedDelay += element.text.length * gapDelay
    return { ...element, initialDelay: delay }
  })
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
  return <Suspense fallback={<LoadingS />}>
    
    <section className="flex flex-col items-center justify-center gap-4 py-8 md:py-10">
      <div className=" max-w-2xl text-start justify-center">
        <h1 className={`${title()}`}>
        Welcome to <span className='gradient'>MdxHub</span>
        </h1>
        <div className={`${subtitle({ class: "mt-4" })}`}>
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
      </div>

      <div className="flex gap-3">
        <Link
          // isExternal
          className={`${buttonStyles({
            // color: "primary",
            radius: "full",
            variant: "shadow",
          })} bg-pink-blue-animated`}
          href={siteConfig.links.docs}
        >
          <span className='gradient'>Get Started</span>
        </Link>
        <SearchModal />
      </div>

      <ModeToggleGradientIcon />
      <div className="mt-8 bg-[calc(var(--color-blue-400))] bg-b">
        <Snippet hideCopyButton hideSymbol variant="bordered">
          <span>
            Get started by editing <Code color="primary">app/page.tsx</Code>
          </span>
        </Snippet>
      </div>
      <Content />
      {/* <ScrollShadow >

</ScrollShadow> */}
      <Content />

    </section>
  </Suspense>
}