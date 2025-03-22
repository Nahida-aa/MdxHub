import { Suspense } from 'react';
import { LoadingS } from '@/components/ui/loading/Loading';

export default async function Page ({
  params,
}: {
  params: Promise<{ slug: string[] }>,
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const { slug } = await params
  return <Suspense fallback={<LoadingS />}>
    <h1>Page</h1>
    <p>This is the Page page.</p>
  </Suspense>
}