import Link from 'next/link';
import { Suspense } from 'react';
import { genPageMetadata } from '@/app/seo';
import { LoadingS } from '@/components/ui/loading/Loading';

export const metadata = genPageMetadata({ title: 'Docs' })

export default async function Page ({
  params,
  searchParams,
}: {
  params: Promise<{ locale: string }>,
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const demoList: string[] = [
    'dev/search',
    'dev/todo',
  ]
  const { locale } = await params
  const { page = '1', sort = 'asc', query = '' } = await searchParams
  return <Suspense fallback={<LoadingS />}>
    <h1>Page</h1>
    <p>This is the Page page.</p>
    <ul>
      {demoList.map((item, index) => (
        <li key={index}>
          <Link href={`/docs/${item}`}>{item}</Link>
        </li>
      ))}
    </ul>
  </Suspense>
}