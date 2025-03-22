"use client"
import type { SearchResults } from "@/lib/search/types"
import { SearchResultItemComponent } from "./search-result-item"
import { Button } from "@/components/ui/button"
import { ChevronLeft, ChevronRight } from "lucide-react"

interface SearchResultsProps {
  results: SearchResults
  onPageChange: (page: number) => void
  onResultClick?: () => void
}

export function SearchResultsList({ results, onPageChange, onResultClick }: SearchResultsProps) {
  const { items, totalCount, page, pageSize, totalPages, query } = results

  if (items.length === 0) {
    return (
      <div className="py-8 text-center">
        <p className="text-muted-foreground">{query ? `没有找到与 "${query}" 相关的结果` : "请输入搜索关键词"}</p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <div className="text-sm text-muted-foreground">
        找到 {totalCount} 个结果
        {query && <span> 匹配 "{query}"</span>}
      </div>

      <div className="divide-y divide-border">
        {items.map((item) => (
          <SearchResultItemComponent key={item.id} result={item} onClick={onResultClick} />
        ))}
      </div>

      {totalPages > 1 && (
        <div className="flex items-center justify-between pt-4">
          <Button variant="outline" size="sm" onClick={() => onPageChange(page - 1)} disabled={page <= 1}>
            <ChevronLeft className="h-4 w-4 mr-2" />
            上一页
          </Button>

          <span className="text-sm text-muted-foreground">
            第 {page} 页，共 {totalPages} 页
          </span>

          <Button variant="outline" size="sm" onClick={() => onPageChange(page + 1)} disabled={page >= totalPages}>
            下一页
            <ChevronRight className="h-4 w-4 ml-2" />
          </Button>
        </div>
      )}
    </div>
  )
}

