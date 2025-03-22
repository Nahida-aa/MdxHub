"use client"

import type React from "react"
import { createContext, useContext, useEffect, useState } from "react"

interface SearchModalContextType {
  open: boolean
  searchKeyword: string
  setOpen: (open: boolean) => void
  setSearchKeyword: (keyword: string) => void
}

const SearchModalContext = createContext<SearchModalContextType>({
  open: false,
  searchKeyword: "",
  setOpen: () => {},
  setSearchKeyword: () => {},
})

export const useSearchModal = () => useContext(SearchModalContext)

interface SearchProviderProps {
  children: React.ReactNode
}

export function SearchProvider({ children }: SearchProviderProps) {
  const [open, setOpen] = useState(false)
  const [searchKeyword, setSearchKeyword] = useState("")

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setOpen(!open)
      }
    }
    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [open])

  return (
    <SearchModalContext.Provider value={{ open, searchKeyword, setOpen, setSearchKeyword }}>
      {children}
    </SearchModalContext.Provider>
  )
}

