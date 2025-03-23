"use client"

import { useEffect, useRef } from "react"
import mermaid from "mermaid"

interface MermaidProps {
  code: string
  className?: string
}

export default function MermaidRenderer({ code, className = "" }: MermaidProps) {
  const mermaidRef = useRef<HTMLPreElement>(null)

  useEffect(() => {
    // 配置 mermaid
    mermaid.initialize({
      startOnLoad: false,
      theme: "default",
      securityLevel: "loose",
    })

    // 渲染图表
    if (mermaidRef.current) {
      mermaidRef.current.innerHTML = ""
      const id = `mermaid-${Math.random().toString(36).substring(2, 9)}`

      try {
        mermaid.render(id, code).then((result) => {
          if (mermaidRef.current) {
            mermaidRef.current.innerHTML = result.svg
          }
        })
      } catch (error) {
        console.error("Mermaid 渲染错误:", error)
        mermaidRef.current.innerHTML = `<pre class="p-4 text-red-500 border border-red-300 rounded">
          <p>图表渲染错误</p>
          <pre>${error instanceof Error ? error.message : String(error)}</pre>
        </pre>`
      }
    }
  }, [code])

  return <pre className={`mermaid-wrapper bg-transparent my-6 ${className}`} ref={mermaidRef} />
}

