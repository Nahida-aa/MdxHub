import Fuse, { IFuseOptions } from "fuse.js"
import type { IndexedDocument, SearchOptions, SearchResults, SearchResultItem } from "./types"

export class SearchEngine {
  private fuse: Fuse<IndexedDocument>
  private documents: IndexedDocument[] = []

  constructor(documents: IndexedDocument[] = []) {
    this.documents = documents

    // 配置 Fuse.js 搜索选项
    const options: IFuseOptions<IndexedDocument> = {
      includeScore: true,
      includeMatches: true,
      threshold: 0.3,
      location: 0,
      distance: 100,
      minMatchCharLength: 2,
      keys: [
        { name: "title", weight: 2 },
        { name: "description", weight: 1.5 },
        { name: "content", weight: 1 },
        { name: "tags", weight: 1.5 },
      ],
    }

    this.fuse = new Fuse(this.documents, options)
  }

  // 添加文档到索引
  public addDocuments(documents: IndexedDocument[]): void {
    this.documents = [...this.documents, ...documents]
    this.fuse.setCollection(this.documents)
  }

  // 清空索引
  public clearIndex(): void {
    this.documents = []
    this.fuse.setCollection(this.documents)
  }

  // 搜索文档
  public search(options: SearchOptions): SearchResults {
    const { query, locale, type, tags, page = 1, pageSize = 10, sortBy = "relevance", sortOrder = "desc" } = options

    // 如果查询为空，返回空结果
    if (!query.trim()) {
      return {
        items: [],
        totalCount: 0,
        page,
        pageSize,
        totalPages: 0,
        query,
      }
    }

    // 执行搜索
    let results = this.fuse.search(query)

    // 应用过滤器
    if (locale) {
      results = results.filter((result) => result.item.locale === locale)
    }

    if (type) {
      results = results.filter((result) => result.item.type === type)
    }

    if (tags && tags.length > 0) {
      results = results.filter((result) => {
        if (!result.item.tags) return false
        return tags.some((tag) => result.item.tags!.includes(tag))
      })
    }

    // 应用排序
    if (sortBy !== "relevance") {
      results.sort((a, b) => {
        if (sortBy === "date") {
          const dateA = a.item.updatedAt || a.item.createdAt || ""
          const dateB = b.item.updatedAt || b.item.createdAt || ""
          return sortOrder === "asc" ? dateA.localeCompare(dateB) : dateB.localeCompare(dateA)
        } else if (sortBy === "title") {
          return sortOrder === "asc"
            ? a.item.title.localeCompare(b.item.title)
            : b.item.title.localeCompare(a.item.title)
        }
        return 0
      })
    }

    // 计算分页
    const totalCount = results.length
    const totalPages = Math.ceil(totalCount / pageSize)
    const startIndex = (page - 1) * pageSize
    const endIndex = startIndex + pageSize
    const paginatedResults = results.slice(startIndex, endIndex)

    // 格式化结果 - 直接使用 Fuse.js 返回的类型，不做转换
    const items: SearchResultItem[] = paginatedResults.map((result) => ({
      id: result.item.id,
      url: result.item.url,
      title: result.item.title,
      description: result.item.description,
      content: result.item.content,
      locale: result.item.locale,
      type: result.item.type,
      segments: result.item.segments,
      tags: result.item.tags,
      score: result.score ? 1 - result.score : 1, // 转换为 0-1 分数，1 为最高
      matches: result.matches
        ? result.matches.map((match) => ({
            field: match.key || "",
            indices: match.indices,
            value: match.value,
          }))
        : [],
    })) 
    // as SearchResultItem[] // 使用类型断言

    return {
      items,
      totalCount,
      page,
      pageSize,
      totalPages,
      query,
    }
  }

  // 获取文档数量
  public getDocumentCount(): number {
    return this.documents.length
  }
}

