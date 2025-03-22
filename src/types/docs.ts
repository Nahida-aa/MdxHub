import { IsoDateTimeString } from "./commont";



export type DocumentContentType = 'markdown' | 'mdx' | 'data'

export namespace Local {
export type RawDocumentData = {
  /** Relative to `contentDirPath` */
  sourceFilePath: string
  sourceFileName: string
  /** Relative to `contentDirPath` */
  sourceFileDir: string
  contentType: DocumentContentType
  /** A path e.g. useful as URL paths based on `sourceFilePath` with file extension removed and `/index` removed. */
  flattenedPath: string
}
}

/** Nested types */
export type Series = {
  /** File path relative to `contentDirPath` */
  _id: string
  _raw: Local.RawDocumentData
  type: 'Series'
  title: string
  order: number
}

export type MDX = {
  /** Raw MDX source */
  raw: string
  /** Prebundled via mdx-bundler */
  code: string
}

export type Doc = {
  /** File path relative to `contentDirPath` */
  _id: string // e.g: docs/en/code-sample.mdx
  _raw: Local.RawDocumentData
  type: 'Dcos'
  title: string
  series?: Series | undefined
  date: IsoDateTimeString
  // language: string
  tags: string[]
  lastmod?: IsoDateTimeString | undefined
  featured?: boolean | undefined
  draft?: boolean | undefined
  summary?: string | undefined
  images?: any | undefined
  authors: string[]
  layout?: string | undefined
  bibliography?: string | undefined
  canonicalUrl?: string | undefined
  /** MDX file body */
  body: MDX
  readingTime: JSON
  slug: string
  path: string
  filePath: string
  toc: string
  structuredData: JSON
}  