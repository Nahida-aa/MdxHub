"use client"

import type React from "react"
import { useEffect, useState } from "react"
import { useSearchModal } from "./search-context"
import { CommandDialog, CommandInput, CommandList, CommandEmpty } from "@/components/ui/command"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Loader2 } from "lucide-react"
import { searchClient } from "@/lib/search/search-client"
import type { SearchResults } from "@/lib/search/types"
import { SearchResultsList } from "./search-results"
import { SearchFilters } from "./search-filters"
import { Kbd } from "@heroui/kbd"
import { DialogTitle } from "../ui/dialog"

export function EnhancedSearchModal() {
  const { open, setOpen, searchKeyword, setSearchKeyword } = useSearchModal()
  const [activeTab, setActiveTab] = useState("results")
  const [isLoading, setIsLoading] = useState(false)
  const [results, setResults] = useState<SearchResults | null>(null)
  const [page, setPage] = useState(1)
  const [locale, setLocale] = useState("all")
  const [sortBy, setSortBy] = useState("relevance")
  const [sortOrder, setSortOrder] = useState("desc")
  const [selectedTags, setSelectedTags] = useState<string[]>([])
  const [availableTags, setAvailableTags] = useState<string[]>([])
  const [debouncedKeyword, setDebouncedKeyword] = useState("")

  // 初始化搜索引擎
  useEffect(() => {
    searchClient.initialize().catch(console.error)
  }, [])

  // 提取所有可用标签
  useEffect(() => {
    if (results && results.items.length > 0) {
      const tags = new Set<string>()
      results.items.forEach((item) => {
        if (item.tags) {
          item.tags.forEach((tag) => tags.add(tag))
        }
      })
      setAvailableTags(Array.from(tags))
    }
  }, [results])

  // 关键词防抖
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedKeyword(searchKeyword)
      setPage(1) // 重置页码
    }, 300)

    return () => clearTimeout(timer)
  }, [searchKeyword])

  // 执行搜索
  useEffect(() => {
    if (!debouncedKeyword.trim()) {
      setResults(null)
      return
    }

    const performSearch = async () => {
      setIsLoading(true)
      try {
        const searchOptions = {
          query: debouncedKeyword,
          page,
          pageSize: 10,
          sortBy: sortBy as any,
          sortOrder: sortOrder as any,
          locale: locale !== "all" ? locale : undefined,
          tags: selectedTags.length > 0 ? selectedTags : undefined,
        }

        const results = await searchClient.search(searchOptions)
        setResults(results)
      } catch (error) {
        console.error("Search error:", error)
      } finally {
        setIsLoading(false)
      }
    }

    performSearch()
  }, [debouncedKeyword, page, locale, sortBy, sortOrder, selectedTags])

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchKeyword(e.target.value)
  }

  const handlePageChange = (newPage: number) => {
    setPage(newPage)
  }

  const handleCloseModal = () => {
    setOpen(false)
    // 延迟重置状态，以避免关闭动画中的闪烁
    setTimeout(() => {
      setSearchKeyword("")
      setResults(null)
      setPage(1)
      setActiveTab("results")
    }, 300)
  }

  return (
    <CommandDialog open={open} onOpenChange={setOpen} contentClassName='max-w-3xl max-h-[90svh]' className="max-w-3xl w-full max-h-[90svh]"  CloseButton={<Kbd  className=" rounded-full h-5 text-xs   ">Esc</Kbd>} >
      <DialogTitle />
      <div className="flex items-center border-b px-3">
        <CommandInput
          placeholder="搜索文档..."
          value={searchKeyword}
          onValueChange={setSearchKeyword}
          className="border-0 focus:ring-0 h-14 text-lg"
        />
        {/* <Kbd className="ml-2">Esc</Kbd> */}
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className=" pt-4">
        <TabsList className="mb-4 mx-4">
          <TabsTrigger value="results">搜索结果</TabsTrigger>
          <TabsTrigger value="filters">筛选</TabsTrigger>
        </TabsList>

        <TabsContent value="results" className="min-h-[300px] h-full max-h-[80vh] overflow-y-auto">
          <CommandList className="max-h-full py-2">
            {isLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-8 w-8 animate-spin text-primary" />
              </div>
            ) : !searchKeyword.trim() ? (
              <CommandEmpty>请输入搜索关键词</CommandEmpty>
            ) : results ? (
              <SearchResultsList results={results} onPageChange={handlePageChange} onResultClick={handleCloseModal} />
            ) : (
              <CommandEmpty>没有找到结果</CommandEmpty>
            )}
          </CommandList>
        </TabsContent>

        <TabsContent value="filters">
          <SearchFilters
            locale={locale}
            setLocale={setLocale}
            sortBy={sortBy}
            setSortBy={setSortBy}
            sortOrder={sortOrder}
            setSortOrder={setSortOrder}
            selectedTags={selectedTags}
            setSelectedTags={setSelectedTags}
            availableTags={availableTags}
          />
        </TabsContent>
      </Tabs>
    </CommandDialog>
  )
}

