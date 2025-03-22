"use client"

import type React from "react"
import Link from "next/link"
import type { SearchResultItem } from "@/lib/search/types"

interface SearchResultItemProps {
  result: SearchResultItem
  onClick?: () => void
}

// 修改 HighlightedText 组件以处理可能为 undefined 的 value
function HighlightedText({ text, indices }: { text: string | undefined; indices: readonly [number, number][] }) {
  // 如果文本为 undefined 或没有索引，直接返回空或原文本
  if (!text || !indices || indices.length === 0) {
    return <span>{text || ""}</span>
  }

  // 排序并合并重叠的区间
  const sortedIndices = [...indices].sort((a, b) => a[0] - b[0])
  const mergedIndices: [number, number][] = []

  let current = sortedIndices[0]
  for (let i = 1; i < sortedIndices.length; i++) {
    if (current[1] >= sortedIndices[i][0] - 1) {
      current[1] = Math.max(current[1], sortedIndices[i][1])
    } else {
      mergedIndices.push(current)
      current = sortedIndices[i]
    }
  }
  mergedIndices.push(current)

  // 构建高亮文本
  const parts: React.ReactNode[] = []
  let lastIndex = 0

  mergedIndices.forEach(([start, end], i) => {
    // 添加非高亮部分
    if (start > lastIndex) {
      parts.push(<span key={`normal-${i}`}>{text.substring(lastIndex, start)}</span>)
    }

    // 添加高亮部分
    parts.push(
      <span key={`highlight-${i}`} className="bg-yellow-200 dark:bg-yellow-800 rounded px-0.5">
        {text.substring(start, end + 1)}
      </span>,
    )

    lastIndex = end + 1
  })

  // 添加最后一个非高亮部分
  if (lastIndex < text.length) {
    parts.push(<span key={`normal-last`}>{text.substring(lastIndex)}</span>)
  }

  return <>{parts}</>
}

// 提取匹配的内容片段
function extractContentSnippet(
  content: string,
  matches: SearchResultItem["matches"],
): { text: string; indices: [number, number][] } {
  const contentMatches = matches.find((m) => m.field === "content")

  if (!contentMatches || !contentMatches.indices.length) {
    // 如果没有匹配，返回前150个字符
    return {
      text: content.substring(0, 150) + (content.length > 150 ? "..." : ""),
      indices: [],
    }
  }

  // 找到第一个匹配位置
  const firstMatch = contentMatches.indices[0]
  const matchCenter = Math.floor((firstMatch[0] + firstMatch[1]) / 2)

  // 计算片段的起始和结束位置
  const snippetStart = Math.max(0, matchCenter - 75)
  const snippetEnd = Math.min(content.length, matchCenter + 75)

  // 提取片段
  let snippet = content.substring(snippetStart, snippetEnd)

  // 添加省略号
  if (snippetStart > 0) {
    snippet = "..." + snippet
  }
  if (snippetEnd < content.length) {
    snippet = snippet + "..."
  }

  // 调整匹配索引以适应新的片段
  const adjustedIndices = contentMatches.indices
    .filter(([start, end]) => {
      // 只保留在片段范围内的匹配
      return end >= snippetStart && start <= snippetEnd
    })
    .map(([start, end]) => {
      // 调整索引位置
      return [
        Math.max(0, start - snippetStart + (snippetStart > 0 ? 3 : 0)),
        Math.min(snippet.length - 1, end - snippetStart + (snippetStart > 0 ? 3 : 0)),
      ] as [number, number]
    })

  return { text: snippet, indices: adjustedIndices }
}

export function SearchResultItemComponent({ result, onClick }: SearchResultItemProps) {
  // 提取标题匹配
  const titleMatch = result.matches.find((m) => m.field === "title")
  const titleIndices = titleMatch?.indices || []

  // 提取内容片段
  const { text: contentSnippet, indices: contentIndices } = extractContentSnippet(result.content, result.matches)

  return (
    <Link
      href={result.url}
      onClick={onClick}
      className="block p-4 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
    >
      <div className="flex items-start justify-between">
        <h3 className="text-lg font-medium text-primary">
          <HighlightedText text={result.title} indices={titleIndices} />
        </h3>
        <span className="text-sm text-muted-foreground ml-2">{Math.round(result.score * 100)}%</span>
      </div>

      {result.description && <p className="text-sm text-muted-foreground mt-1">{result.description}</p>}

      <p className="text-sm mt-2 line-clamp-2">
        <HighlightedText text={contentSnippet} indices={contentIndices} />
      </p>

      <div className="flex items-center mt-2 space-x-2">
        <span className="text-xs px-2 py-1 bg-primary/10 rounded-full">
          {result.locale === "zh" ? "中文" : "English"}
        </span>

        {result.tags && result.tags.length > 0 && (
          <div className="flex items-center space-x-1">
            {result.tags.slice(0, 3).map((tag) => (
              <span key={tag} className="text-xs px-2 py-1 bg-secondary/20 rounded-full">
                {tag}
              </span>
            ))}
            {result.tags.length > 3 && <span className="text-xs text-muted-foreground">+{result.tags.length - 3}</span>}
          </div>
        )}
      </div>
    </Link>
  )
}

