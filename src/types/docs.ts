import { IsoDateTimeString } from "./commont";

export type DocMeta = {
  title: string; // The page's <h1> title, used for SEO and OG Images. 页面标题 <h1> ，用于 SEO 和 OG 图片
  nav_title?: string; // Overrides the page's title in the navigation. This is useful when the page's title is too long to fit. If not provided, the title field is used. 覆盖页面标题在导航中。当页面标题过长无法适应时很有用。如果未提供，则使用 title 字段。
  description: string; // The page's description, used in the <meta name="description"> tag for SEO. 页面描述，用于 <meta name="description"> 标签的 SEO。
  source?: string; // Pulls content into a shared page. See Shared Pages. 拉取内容到共享页面。见共享页面。
  related?: { // A list of related pages at the bottom of the document. These will automatically be turned into cards. See Related Links. 文档底部相关页面列表。这些将自动转换为卡片。参见相关链接。 Links will be automatically generated for pages that have child pages. For example, the Optimizing section has links to all of its child pages. //链接将自动生成，用于具有子页面的页面。例如，优化部分包含对其所有子页面的链接。
    description?: string; // A description of the related content. 相关内容的描述。
    links: string[]; // A list of links to related content. 相关内容的链接列表。
  }
  version?: string //	A stage of development. e.g. experimental,legacy,unstable,RC
}

export type DocumentContentType = 'markdown' | 'mdx' | 'data'

namespace Local {
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