// 修改 SearchResultItem 接口中的 matches 类型定义
export interface SearchResultItem {
  id: string
  url: string
  title: string
  description?: string
  content: string
  locale: string
  type: string
  segments: string[]
  tags?: string[]
  score: number
  matches: {
    field: string
    indices: readonly [number, number][] // 添加 readonly 修饰符
    value: string | undefined // 允许 value 为 undefined
  }[]
}

// 其他类型保持不变
export interface SearchResults {
  items: SearchResultItem[]
  totalCount: number
  page: number
  pageSize: number
  totalPages: number
  query: string
}

export interface SearchOptions {
  query: string
  locale?: string
  type?: string
  tags?: string[]
  page?: number
  pageSize?: number
  sortBy?: "relevance" | "date" | "title"
  sortOrder?: "asc" | "desc"
}

export interface IndexedDocument {
  id: string
  url: string
  title: string
  description?: string
  content: string
  locale: string
  type: string
  segments: string[]
  tags?: string[]
  createdAt?: string
  updatedAt?: string
  order: number
  toc: {
    depth: number
    value: string
    slug: string
  }[]
}

