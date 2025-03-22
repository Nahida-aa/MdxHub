import { SearchEngine } from "./search-engine"
import { indexMdxDocuments } from "./indexer"
import type { SearchOptions, SearchResults } from "./types"

// 单例搜索客户端
class SearchClient {
  private static instance: SearchClient
  private searchEngine: SearchEngine
  private initialized = false

  private constructor() {
    this.searchEngine = new SearchEngine()
  }

  public static getInstance(): SearchClient {
    if (!SearchClient.instance) {
      SearchClient.instance = new SearchClient()
    }
    return SearchClient.instance
  }

  // 初始化搜索引擎
  public async initialize(): Promise<void> {
    if (this.initialized) return

    try {
      // 加载文档数据
      const [zhDocs, enDocs] = await Promise.all([this.fetchDocuments("zh",'docs'), this.fetchDocuments("en",'docs')])

      // 索引文档
      const indexedDocs = indexMdxDocuments([...zhDocs, ...enDocs])
      this.searchEngine.addDocuments(indexedDocs)

      this.initialized = true
      console.log(`Search engine initialized with ${this.searchEngine.getDocumentCount()} documents`)
    } catch (error) {
      console.error("Failed to initialize search engine:", error)
      throw error
    }
  }

  // 获取文档数据
  private async fetchDocuments(locale: string, type:string): Promise<any[]> {
    try {
      const response = await fetch(`/data/${locale}/${type}/index.json`)
      if (!response.ok) {
        throw new Error(`Failed to fetch ${locale}/${type} documents: ${response.statusText}`)
      }
      return await response.json()
    } catch (error) {
      console.error(`Error fetching ${locale} documents:`, error)
      return []
    }
  }

  // 搜索文档
  public async search(options: SearchOptions): Promise<SearchResults> {
    if (!this.initialized) {
      await this.initialize()
    }

    return this.searchEngine.search(options)
  }

  // 重新加载索引
  public async reloadIndex(): Promise<void> {
    this.searchEngine.clearIndex()
    this.initialized = false
    await this.initialize()
  }
}

// 导出搜索客户端实例
export const searchClient = SearchClient.getInstance()

