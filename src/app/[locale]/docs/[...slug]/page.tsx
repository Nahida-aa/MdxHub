import { Suspense } from 'react';
import { LoadingS } from '@/components/ui/loading/Loading';
import { DocSearchValue, MdxComp } from '../../md/types';
import { title } from '@/components/primitives';
import { TailwindBG2 } from '@/components/layout/bg/tailwind';
import { getFile } from '../../md/lib/get';
import { CustomMDX } from '../../md/mdx';
import { notFound } from 'next/navigation';
import { getAllDocs, getDocBySlug } from '@/lib/md/get';
import { DocsToc } from '../_comp/DocsToc';
// import DocsJson from '@/../public/gen/docs.json' 

export const generateStaticParams = async() => {
  const locales = ["zh", "en"]; // 根据支持的语言列表动态加载
  const params: { locale: string; slug: string[] }[] = [];
  for (const locale of locales) {
    const allDocs = await getAllDocs(locale, 'docs')
    for (const doc of allDocs) {
      params.push({ locale, slug: doc.segments})
    }
  }
  // console.log("docs[]:generateStaticParams: ", params)
  return params
}

const getImportPath = (locale: string, slug: string[]) => {
  const dslug = slug.join('/')
  const importPath = dslug.endsWith('.mdx')? `${locale}/${dslug}` : `${locale}/${dslug}/index.mdx`
  return importPath
}

export default async function Page ({
  params,
  // searchParams,
}: {
  params: Promise<{ locale: string, slug: string[] }>,
  // searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const { locale, slug } = await params
  const dslug = slug.join('/')
  // console.log("docs[]:locale: ", locale, "slug: ", slug)
  // const importPath = getImportPath(locale, slug)
  // const { default: Content, frontmatter: metadata } = await import(`@/data/docs/${importPath}`) as MdxComp
  try {
    const docObj = await import(`@/../public/data/${locale}/docs/${dslug}.json`) as DocSearchValue
      // 去掉 .mdx,再加上 .mdx
    const file_path  = docObj.filePath.replace('.mdx', '')
    // const file_path  = `docs/zh/${dslug}/index.mdx`
   // docs/xxx.mdx 
    // console.log("file_path: ", file_path)
    const { default: Content, frontmatter: metadata } = await import(`@/data/${file_path}.mdx`) as MdxComp
    // console.log("Page:import:rest: ", rest)
  // const docObj = await getDocBySlug(locale, 'docs', dslug)
  // const { metadata, content, rawContent } = getFile(file_path)
  // if (!content) return notFound()
  // const {content: JSXContent, frontmatter: metadata} = await CustomMDX({ source: docObj.content })

  // const { page = '1', sort = 'asc', query = '' } = await searchParams
  // 
  return <Suspense fallback={<LoadingS />}>
  <section className='px-4 sm:px-6 w-full min-w-0  flex-1 grid grid-cols-12'>
  {/* col-span-12 lg:col-span-10 xl:col-span-8   */}
    <article className="prose dark:prose-invert  col-span-12 lg:col-span-9 lg:px-4 xl:col-span-9 xl:px-8
    mx-auto w-full min-w-0 max-w-full ">
      <h1 className={`${title()} flex justify-center `}>
        {metadata?.title||'Title'}
        </h1>
        <p>{metadata?.description|| 'description'}</p>
        {/* <TailwindBG2 /> */}
      <Content />
      {/* {JSXContent} */}
      {/* <JSXContent 
      // options={mdxOptions} 
      /> */}
    </article>
    <DocsToc toc={docObj.toc} />
  </section>
</Suspense>
  } catch (error) {
    console.log("docs[]:Page:error: ", error)
    return notFound()
  }
}