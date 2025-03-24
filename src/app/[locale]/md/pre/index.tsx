"use client"

import React from "react"
import MermaidRenderer from "./mermaid-renderer"
import { PreWithCopy } from "./preWithCopy"

// 定义更具体的类型
interface CodeElementProps {
  className?: string
  "data-language"?: string
  children: React.ReactNode
}

interface LineSpanProps {
  "data-line": string | number
  children: React.ReactNode
}

interface TextSpanProps {
  children: string
}


// 从代码元素中提取 Mermaid 代码的函数
export function extractCodeContent(codeElement: React.ReactElement<any>): string {
  if (!codeElement || !codeElement.props || !codeElement.props.children) {
    return ""
  }

  // 处理直接包含字符串的情况
  if (typeof codeElement.props.children === "string") {
    return codeElement.props.children
  }

  // 处理 rehype-pretty-code 生成的复杂结构
  let mermaidCode = ""

  React.Children.forEach(codeElement.props.children, (lineSpan) => {
    const lineProps = lineSpan.props as LineSpanProps
    // 检查是否是有效的 React 元素并且有 data-line 属性
    if (React.isValidElement(lineSpan) && lineProps && "data-line" in lineProps) {
      const lineSpanElement = lineSpan as React.ReactElement<LineSpanProps>

      React.Children.forEach(lineSpanElement.props.children, (textSpan) => {
        if (React.isValidElement(textSpan) && textSpan.props) {
          const textSpanElement = textSpan as React.ReactElement<TextSpanProps>
          if (typeof textSpanElement.props.children === "string") {
            mermaidCode += textSpanElement.props.children + "\n"
          }
        }
      })
    } else if (typeof lineSpan === "string") {
      // 处理直接是字符串的情况
      mermaidCode += lineSpan + "\n"
    }
  })

  return mermaidCode
}

// 检查是否是 Mermaid 代码块
export function isLanguageCodeBlock(element: React.ReactElement, language: string): boolean {
  if (!element || !element.props) return false

  const props = element.props as CodeElementProps

  return (props.className && props.className.includes(`language-${language}`)) || props["data-language"] === language
}

export const Pre = ({children, ...props}: React.HTMLAttributes<HTMLPreElement>) => {
  console.log('Pre: children:')
  // console.log(children)
  const childArray = React.Children.toArray(children)
  const codeElement = childArray.find((child) => React.isValidElement(child) && child.type === "code") as React.ReactElement<'code'>
  console.log('Pre: codeElement:')
  // console.log(codeElement)
  
  // 如果找到代码元素并且是 Mermaid 代码块
  if (codeElement && isLanguageCodeBlock(codeElement, 'mermaid')) {
    // 提取 代码
    const code = extractCodeContent(codeElement)
    console.log('Pre: mermaid:code:')
    // console.log(code)
    return <MermaidRenderer code={code} />
  }
  return <PreWithCopy {...props}>{children}</PreWithCopy>
}