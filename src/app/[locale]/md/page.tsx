import { Suspense } from 'react';
import { LoadingS } from '~/components/ui/loading/Loading';
import { CustomMDX } from './mdx';
import { SerializeOptions } from 'node_modules/next-mdx-remote/dist/types';
import { getFile } from './lib/get';
import { notFound } from 'next/navigation';
import { title } from '~/components/primitives';
// import 'katex/dist/katex.min.css';

export default async function Page ({
  params,
  searchParams,
}: {
  params: Promise<{ slug: string }>,
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const { slug } = await params
  const { page = '1', sort = 'asc', query = '' } = await searchParams
  const file_path = `test.mdx`
  const { metadata, content, rawContent } = getFile(file_path)
  // console.log("metadata:", metadata, "content:", content, "rawContent:", rawContent)
  if (!content) return notFound()
    
  const format = 'mdx';
  const mdxOptions: SerializeOptions = {
    mdxOptions: {
      format: format as 'md' | 'mdx', // 确保 format 是 'md' 或 'mdx'
    },
  };
  return <Suspense fallback={<LoadingS />}>
    <section className='mx-6'>
    <article className="prose dark:prose-invert  mx-auto max-w-full  ">
      <h1 className={`${title()} flex justify-center `}>{metadata.title}</h1>
      <CustomMDX source={rawContent} options={mdxOptions} />
    </article>
    </section>
  </Suspense>
}