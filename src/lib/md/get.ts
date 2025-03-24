import { DocMeta, DocSearchValue } from "@/app/[locale]/md/types"

import path from "path";
import fs from "fs/promises"
import { MenuItemType } from "@/components/layout/sidebar/TreeNode";

// Get all documents
export const getAllDocs = async (locale:string, type:string) => {
  const module = await import(`@/../public/data/${locale}/${type}/index.json`);
  return module.default as DocSearchValue[];
}

// Get a document by slug
export const getDocBySlug = async (locale:string, type:string, slug:string) => {
  const module = await import(`@/../public/data/${locale}/${type}/${slug}.json`)
  return module.default as DocSearchValue;
}

export type NavNode = {
  segment: string
  title: string
  url?: string
  order: number
  type?: MenuItemType
  children?: NavNode[]
}
type DocSearchValueBase ={
  title: string
  url: string
  order: number
  segments: string[]
  filePath: string
}
// 构建嵌套导航结构
const buildNavTree = (docs: DocSearchValueBase[]): NavNode[] => {
    // 创建一个映射来存储所有节点
    const nodeMap: Record<string, NavNode> = {}
    const root: NavNode[] = []
    const [_, locale, type] = docs[0].url!.split("/").slice(0, 3)
  
    // 首先，为每个文档创建一个节点
    docs.forEach((doc) => {
      const segments = doc.segments
  
      // 为每个段落创建一个路径
      for (let i = 0; i < segments.length; i++) {
        const pathSegments = segments.slice(0, i + 1)
        const path = pathSegments.join("/")
        const segment = segments[i]
  
        // 如果节点已存在，跳过
        if (nodeMap[path]) continue
  
        // 创建新节点
        const node: NavNode = {
          segment,
          title: segment, // 使用段落名作为标题
          // url: '', // 默认为空URL
          order: 99, // 默认排序
          children: [],
        }
  
        // 如果是完整路径，使用文档信息
        if (i === segments.length - 1) {
          node.title = doc.title || segment
          node.url = doc.url
          node.order = doc.order
          // 判断是否是文件还是目录
          if (doc.filePath.endsWith("index.mdx")) {
            node.type = "dir";
          } else {
            node.type = "file";
          }
        } else { // 否则说明 url 不存在, 不存在的 url 一定是 button (dir)
          // console.log('buildNavTree:locale: ', locale, 'type: ', type, 'path: ', path)
          // console.log('buildNavTree:url: ', `$/{locale}/${type}/${path}`)
          node.url = `/${locale}/${type}/${path}`
          node.type = "dir"
        }
  
        // 将节点添加到映射中
        nodeMap[path] = node
  
        // 将节点添加到树中
        if (i === 0) {
          // 如果是第一个段落，添加到根节点
          root.push(node)
        } else {
          // 否则，添加到父节点
          const parentPath = pathSegments.slice(0, -1).join("/")
          const parentNode = nodeMap[parentPath]
  
          if (parentNode) {
            if (!parentNode.children) {
              parentNode.children = []
            }
            parentNode.children.push(node)
          }
        }
      }
    })
  
    // 递归排序树
    const sortTree = (nodes: NavNode[]): NavNode[] => {
      nodes.sort((a, b) => a.order - b.order)
      nodes.forEach((node) => {
        if (node.children && node.children.length > 0) {
          node.children = sortTree(node.children)
        }
      })
      return nodes
    }
  
    return sortTree(root)
};

// 生成导航列表
export const getNavigationList = async (allDocs:DocSearchValue[]) => {
  // const contentDir = path.join(process.cwd(), "src", "data");
  // const outputDir = path.join(process.cwd(), "public", "data");

  // 处理所有 MDX 文件，生成文档数据
  // const allDocs = await processAllMdxFiles(contentDir, outputDir, locale, type);
  // const allDocs = await getAllDocs(locale, type);

  // 构建导航树
  const navTree = buildNavTree(
    allDocs.map((doc) => ({
      title: doc.meta.nav_title || doc.meta.title,
      url: doc.url,
      order: doc.order,
      segments: doc.segments,
      filePath: doc.filePath,
    }))
  );

  // console.log("Navigation list generated:", navTree);
  // // 输出导航列表到文件
  // const navFilePath = path.join(outputDir, locale, "nav.json");
  // await fs.writeFile(navFilePath, JSON.stringify(navTree, null, 2));
  // console.log(`Navigation list generated at: ${navFilePath}`);
  return navTree;
};


