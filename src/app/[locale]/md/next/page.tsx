import { Suspense } from "react"
import { title } from "@/components/primitives"
import { LoadingS } from "@/components/ui/loading/Loading"
import { MdxComp } from "../types"


export default async function Page({
  params,
}: {
  params: Promise<{ locale: string, slug: readonly string[] }>
}) {
  const { default: Content, frontmatter:metadata } = await import("@/app/[locale]/md/test.mdx") as MdxComp
  console.log("rest: ")
  console.log(metadata)

  return <Suspense fallback={<LoadingS />}>
    <section className='mx-6'>
    <article className="prose dark:prose-invert  mx-auto max-w-full  ">
      <h1 className={`${title()} flex justify-center `}>
        {metadata.title}
        </h1>
      <Content />
    </article>
    </section>
  </Suspense>
}

// export function generateStaticParams() {
//   return [{ slug: 'welcome' }, { slug: 'about' }]
// }

// export const dynamicParams = false