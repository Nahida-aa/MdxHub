"use client"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Checkbox } from "@/components/ui/checkbox"
import { Label } from "@/components/ui/label"

interface SearchFiltersProps {
  locale: string
  setLocale: (locale: string) => void
  sortBy: string
  setSortBy: (sortBy: string) => void
  sortOrder: string
  setSortOrder: (sortOrder: string) => void
  selectedTags: string[]
  setSelectedTags: (tags: string[]) => void
  availableTags: string[]
}

export function SearchFilters({
  locale,
  setLocale,
  sortBy,
  setSortBy,
  sortOrder,
  setSortOrder,
  selectedTags,
  setSelectedTags,
  availableTags,
}: SearchFiltersProps) {
  const handleTagToggle = (tag: string) => {
    if (selectedTags.includes(tag)) {
      setSelectedTags(selectedTags.filter((t) => t !== tag))
    } else {
      setSelectedTags([...selectedTags, tag])
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-4">
        <div className="w-full sm:w-auto">
          <Label htmlFor="locale-select">语言</Label>
          <Select value={locale} onValueChange={setLocale}>
            <SelectTrigger id="locale-select" className="w-full">
              <SelectValue placeholder="选择语言" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">全部</SelectItem>
              <SelectItem value="zh">中文</SelectItem>
              <SelectItem value="en">English</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="w-full sm:w-auto">
          <Label htmlFor="sort-by-select">排序方式</Label>
          <Select value={sortBy} onValueChange={setSortBy}>
            <SelectTrigger id="sort-by-select" className="w-full">
              <SelectValue placeholder="排序方式" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="relevance">相关度</SelectItem>
              <SelectItem value="date">日期</SelectItem>
              <SelectItem value="title">标题</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="w-full sm:w-auto">
          <Label htmlFor="sort-order-select">排序顺序</Label>
          <Select value={sortOrder} onValueChange={setSortOrder}>
            <SelectTrigger id="sort-order-select" className="w-full">
              <SelectValue placeholder="排序顺序" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="desc">降序</SelectItem>
              <SelectItem value="asc">升序</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {availableTags.length > 0 && (
        <div>
          <Label className="mb-2 block">标签</Label>
          <div className="flex flex-wrap gap-2">
            {availableTags.map((tag) => (
              <div key={tag} className="flex items-center space-x-2">
                <Checkbox
                  id={`tag-${tag}`}
                  checked={selectedTags.includes(tag)}
                  onCheckedChange={() => handleTagToggle(tag)}
                />
                <Label htmlFor={`tag-${tag}`} className="text-sm cursor-pointer">
                  {tag}
                </Label>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}

