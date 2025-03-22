import { type NextRequest, NextResponse } from "next/server"
import fs from "fs"
import path from "path"
import { indexMdxDocuments } from "@/lib/search/indexer"
import { SearchEngine } from "@/lib/search/search-engine"
import type { SearchOptions } from "@/lib/search/types"

// 创建搜索引擎实例
let searchEngine: SearchEngine | null = null

// 初始化搜索引擎
async function initializeSearchEngine() {
  if (searchEngine) return searchEngine

  try {
    // 读取文档数据
    const dataDir = path.join(process.cwd(), "public", "data")

    // 读取中文文档
    const zhDocsPath = path.join(dataDir, "zh", 'docs', "index.json")
    const zhDocsData = fs.existsSync(zhDocsPath) ? JSON.parse(fs.readFileSync(zhDocsPath, "utf-8")) : []

    // 读取英文文档
    const enDocsPath = path.join(dataDir, "en", 'docs', "index.json")
    const enDocsData = fs.existsSync(enDocsPath) ? JSON.parse(fs.readFileSync(enDocsPath, "utf-8")) : []

    // 索引文档
    const indexedDocs = indexMdxDocuments([...zhDocsData, ...enDocsData])

    // 创建搜索引擎
    searchEngine = new SearchEngine(indexedDocs)

    return searchEngine
  } catch (error) {
    console.error("Failed to initialize search engine:", error)
    throw error
  }
}

export async function GET(request: NextRequest) {
  try {
    // 获取查询参数
    const searchParams = request.nextUrl.searchParams
    const query = searchParams.get("q") || ""
    const page = Number.parseInt(searchParams.get("page") || "1", 10)
    const pageSize = Number.parseInt(searchParams.get("pageSize") || "10", 10)
    const locale = searchParams.get("locale") || undefined
    const type = searchParams.get("type") || undefined
    const tags = searchParams.get("tags") ? searchParams.get("tags")!.split(",") : undefined
    const sortBy = (searchParams.get("sortBy") as any) || "relevance"
    const sortOrder = (searchParams.get("sortOrder") as any) || "desc"

    // 初始化搜索引擎
    const engine = await initializeSearchEngine()

    // 执行搜索
    const searchOptions: SearchOptions = {
      query,
      page,
      pageSize,
      locale,
      type,
      tags,
      sortBy,
      sortOrder,
    }

    const results = engine.search(searchOptions)

    return NextResponse.json(results)
  } catch (error) {
    console.error("Search API error:", error)
    return NextResponse.json({ error: "Failed to perform search" }, { status: 500 })
  }
}

