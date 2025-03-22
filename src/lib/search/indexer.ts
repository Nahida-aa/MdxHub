import type { IndexedDocument } from "./types"

// 将原始 MDX 文档数据转换为索引文档
export function indexMdxDocuments(mdxDocs: any[]): IndexedDocument[] {
  return mdxDocs.map((doc) => {
    // 提取文档标题
    const title =
      doc.meta?.title || (doc.toc && doc.toc[0]?.value) || doc.segments[doc.segments.length - 1] || "Untitled"

    // 清理内容中的 Markdown 语法
    const cleanContent = cleanMarkdown(doc.content)

    return {
      id: `${doc.locale}:${doc.slug}`,
      url: doc.url,
      title,
      description: doc.meta?.description || "",
      content: cleanContent,
      locale: doc.locale,
      type: doc.type,
      segments: doc.segments,
      tags: doc.meta?.tags || [],
      createdAt: doc.meta?.created_at,
      updatedAt: doc.meta?.updated_at,
      order: doc.order,
      toc: doc.toc || [],
    }
  })
}

// 清理 Markdown 语法，提取纯文本内容
function cleanMarkdown(markdown: string): string {
  if (!markdown) return ""

  return (
    markdown
      // 移除代码块
      .replace(/```[\s\S]*?```/g, "")
      // 移除内联代码
      .replace(/`[^`]*`/g, "")
      // 移除图片
      .replace(/!\[.*?\]$$.*?$$/g, "")
      // 移除链接，保留文本
      .replace(/\[([^\]]+)\]$$[^)]+$$/g, "$1")
      // 移除标题标记
      .replace(/#{1,6}\s+/g, "")
      // 移除粗体和斜体
      .replace(/(\*\*|__)(.*?)\1/g, "$2")
      .replace(/(\*|_)(.*?)\1/g, "$2")
      // 移除 HTML 标签
      .replace(/<[^>]*>/g, "")
      // 移除多余空白
      .replace(/\s+/g, " ")
      .trim()
  )
}

