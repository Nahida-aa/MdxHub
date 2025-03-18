'use client'
import { Check, Copy } from "lucide-react"
import { useRef, useState } from "react"

export const PreWithCopy = (props: React.HTMLAttributes<HTMLPreElement>) => {
  const [copied, setCopied] = useState(false)
  const preRef = useRef<HTMLPreElement>(null)
  // console.log('PreWithCopy: props:')
  // console.log(props)
  const copyToClipboard = async () => {
    if (!preRef.current) return

    // Extract text content from the code element inside pre
    const codeElement = preRef.current.querySelector("code")
    const textToCopy = codeElement?.textContent || ""

    try {
      await navigator.clipboard.writeText(textToCopy)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch (error) {
      console.error("Failed to copy code:", error)
    }
  }
  const { children } = props;
  return (
    <pre ref={preRef} {...props} className={`${props.className || ""} pr-10 group relative`}>
      {children}
      <button
        onClick={copyToClipboard}
        aria-label="Copy code"
        className="absolute right-2 top-2 h-8 w-8 rounded-md bg-[#2d333b] p-1.5 opacity-70 transition-opacity hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#539bf5] hidden group-hover:block"
      >
        {copied ? (
          <Check className="h-full w-full text-green-500" />
        ) : (
          <Copy className="h-full w-full text-[#adbac7]" />
        )}
      </button>
    </pre>
  )
}