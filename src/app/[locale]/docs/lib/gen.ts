import { DOCS_DIR } from "@/data/url"
import { DocMeta } from "../../md/types"
import path from "path"
import fs from "fs/promises"
import matter from "gray-matter"

export type NavNode = {
  title: string
  url: string
  order: number
  children?: NavNode[]
}

// e.g: dir 下有
// - 01-dev/index.mdx
// - 01-dev/01-cl.mdx
// - 01-dev/02-md/index.mdx
// - 01-dev/02-md/01-mdx.mdx
// - 01-dev/02-md/02-github.mdx
// - 02-linux/index.mdx
// - 02-linux/01-command/index.mdx
// - 02-linux/02-file_system/index.mdx
// - next/index.mdx
// - next/01-app/index.mdx
// - next/01-app/03-building-your-applications/06-package-bundling.mdx
// - ts/async.mdx
// - ts/tsconfig.mdx
export const generateNav = async (locale: string): Promise<NavNode[]> => {
  const dir = `${DOCS_DIR}/${locale}/`
  // 每个文件夹或文件 可能 写为 01-xxx.mdx 或 02-xxx/index.mdx 或 03-xxx/01-code/index.mdx 等等
  // 需要将
  //   const { data: metadata, content } = matter(rawContent) as unknown as {
      // data: DocMeta;
    //   content: string
    // }
  // title = metadata.nav_title || metadata.title
  // url = `/${locale}/docs/...`

  // 递归函数：构建导航树
  const buildNavTree = async (currentDir: string, baseUrl: string): Promise<NavNode[]> => {
    const entries = await fs.readdir(currentDir, { withFileTypes: true })
    const nodes: NavNode[] = []

    for (const entry of entries) {
      const fullPath = path.join(currentDir, entry.name)
      const relativePath = path.relative(dir, fullPath)
      const cleanRelativePath = removeOrderFromPath(relativePath) // 去掉路径中所有部分的序号
      const cleanName = removeOrderPrefix(entry.name) // 去掉当前文件或文件夹名称的序号
      const url = `/${locale}/docs/${cleanRelativePath.replace(/\\/g, "/").replace(/\/index$/, "")}`

      if (entry.isDirectory()) {
        // 递归处理子目录
        const children = await buildNavTree(fullPath, url)
        if (children.length > 0) {
          nodes.push({
            title: cleanName,
            url,
            order: parseOrder(entry.name),
            children,
          })
        }
      } else if (entry.isFile() && entry.name.endsWith(".mdx")) {
        // 解析文件元数据
        const rawContent = await fs.readFile(fullPath, "utf-8")
        const { data: metadata } = matter(rawContent) as unknown as { data: DocMeta }
        const title = metadata.nav_title || metadata.title || cleanName

        nodes.push({
          title,
          url,
          order: parseOrder(entry.name),
        })
      }
    }

    // 按 order 排序
    return nodes.sort((a, b) => a.order - b.order)
  }
  return buildNavTree(dir, "")
}

// 提取文件或文件夹的排序号
const parseOrder = (name: string): number => {
  const match = name.match(/^(\d+)-/)
  return match ? parseInt(match[1], 10) : 99 // 默认排序号为 9999
}

// 去掉文件或文件夹名称中的序号前缀
const removeOrderPrefix = (name: string): string => {
  return name.replace(/^\d+-/, "")
}
// 去掉路径中所有部分的序号
const removeOrderFromPath = (relativePath: string): string => {
  return relativePath
    .split(path.sep) // 按路径分隔符分割路径
    .map(removeOrderPrefix) // 去掉每一部分的序号
    .join(path.sep) // 重新拼接路径
}