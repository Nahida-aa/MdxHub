import fs from "fs/promises"
import path from "path"
import matter from "gray-matter"
// import {compile} from '@mdx-js/mdx' // pnpm add @mdx-js/mdx
import { unified } from "unified" // pnpm add -D unified
import remarkParse from "remark-parse";
import remarkStringify from "remark-stringify"
import remarkFrontmatter from 'remark-frontmatter'
import remarkHeadings from '@vcarl/remark-headings';
import { text2slug } from "@/lib/utils/slug"
import { DocMeta, DocSearchValue, Toc } from "@/app/[locale]/md/types"
import { getNavigationList } from "./get";
import { outputDir } from "../path";


const DATA_DIR = path.join(process.cwd(), "src",'data')
const DOCS_DIR = path.join(process.cwd(), "src",'data', 'docs')
const OUTPUT_DIR = path.join(process.cwd(), "public", "gen")
// type ListDocSearchValue = DocSearchValue[]
type KVDocSearchValue = { [key: string]: DocSearchValue }

// filePath
export async function processMdxToJson(filePath: string, outputDir: string): Promise<DocSearchValue> {
  const fileContent = await fs.readFile(filePath, "utf-8");
  const relativePath = filePath.replace(`${DATA_DIR}/`, "")
  const [toc, {data, content}, {locale,type,url, slug, segments, order} ] = await Promise.all([
    processMdxGenTocList(fileContent),
    matter(fileContent),
    processSlug(relativePath)
  ]);
  const docData: DocSearchValue = {locale,
    type,
    url: url,
    slug: slug,
    segments: segments,
    filePath: relativePath,
    meta: data as DocMeta,
    order: order,
    content: content,
    toc: toc
  }
  // Create output directory if it doesn't exist
  const outputPath = path.join(outputDir, `${url}.json`)
  console.log(`processMdxToJson: outputPath: `, outputPath)
  const outputDirPath = path.dirname(outputPath)

  try {
    await fs.access(outputDirPath);
  } catch {
    await fs.mkdir(outputDirPath, { recursive: true });
  }

  // Write JSON file
  await fs.writeFile(outputPath, JSON.stringify(docData, null, 2))
  return docData
}
// Process all MDX files in a directory
export const  processAllMdxFiles = async(contentDir: string, outputDir: string, locale:string , type: string) => {
  const allDocs: DocSearchValue[] = []

  // Recursive function to process all files in a directory
  const processDir = async (dirPath: string) => {
    const entries = await fs.readdir(dirPath, { withFileTypes: true });
  
    // 使用 Promise.all 并行处理所有条目
    await Promise.all(entries.map(async (entry) => {
      const fullPath = path.join(dirPath, entry.name);
      if (entry.isDirectory()) {// 并行处理子目录
        await processDir(fullPath);
      } else if (entry.name.endsWith(".mdx")) {
        try {
          // 并行处理文件
          const docData = await processMdxToJson(fullPath, outputDir);
          allDocs.push(docData);
        } catch (error) {
          console.error(`Error processing ${fullPath}:`, error);
        }
      }
    }));
  };
  
  await processDir(path.join(contentDir, type, locale));

  // Generate index file with all documents
  const indexPath = path.join(outputDir, locale, type, "index.json")
  await fs.writeFile(indexPath, JSON.stringify(allDocs, null, 2))

  return allDocs
}
// mdxhub/source-files: Dependencies: chokidar
// filePath -> slug
// docs/zh/01-dev/index.mdx -> dev
// docs/zh/01-dev/01-cl.mdx -> dev/cl
// docs/zh/01-dev/02-hf/index.mdx -> dev/hf


// docs/zh/01-dev/index.mdx
const processSlug = async (relativePath: string): Promise<{locale:string,
  type: string // docs, 指定文档的类型
  url: string // /zh/docs/dev/cl, 指定文档的 URL
  // /zh/docs/dev/cl, 指定文档的 URL
  slug: string // dev/cl, 指定文档的 slug, 用于 静态生成
  // dev/cl, 指定文档的 slug, 用于 静态生成
  segments: string[] // ['dev', 'cl'], 指定文档的 slug, 用于 静态生成
  // ['dev', 'cl'], 指定文档的 slug, 用于 静态生成
  order: number // 99, 指定文档的排序
}> => {
  // 分割路径
  const segments = relativePath.split("/");
  const type = segments.shift()!;
  const locale = segments.shift()!;
  // 如果最后一个段是 index，则去掉
  let lastSegment = segments.pop()!;
  if (lastSegment === "index.mdx") {
    lastSegment = segments.pop()!;}
  // 去掉最后一个段的 .mdx 扩展名
  lastSegment = lastSegment.replace(/\.mdx$/, "");
  // 提取排序数字并清理每个路径段
  const cleanedSegments = segments.map((segment) => {
    const match = segment.match(/^(\d+)-(.+)$/);
    return match ? match[2] : segment; // 去掉数字前缀
  });
  // 提取排序数字（从最后一个有效路径段中提取）
  const match = lastSegment.match(/^(\d+)-(.+)$/);
  const order = match ? parseInt(match[1], 10) : 99; // 如果有数字前缀，提取为 order，否则默认 99
  const cleanedLastSegment = match ? match[2] : lastSegment; // 去掉数字前缀后的路径段
  // 重新组合 slug
  const slug = [...cleanedSegments, cleanedLastSegment].join("/");
  const url = `/${locale}/${type}/${slug}`;
  return {
    locale,
    type,
    url,
    slug,
    segments: [...cleanedSegments, cleanedLastSegment],
    order,
  };
};

const mdProcessor = unified()
.use(remarkParse)
.use(remarkStringify)
.use(remarkFrontmatter)
.use(remarkHeadings);

const processMdxGenTocList = async (content: string): Promise<Toc[]> => {
  const vfile = await mdProcessor.process(content);
  // Place to store custom info (default: {}).
  // 翻译: 存储自定义信息的地方（默认值：{}）。
  // It’s OK to store custom data directly on the file but moving it to data is recommended.
  // 翻译: 将自定义数据直接存储在文件上是可以的，但建议将其移动到 data 上。
  const data = vfile.data as {
    headings: {depth: number
      value: string}[]
  }
  const toc = data.headings.map((heading) => ({
    depth: heading.depth,
    value: heading.value,
    slug: text2slug(heading.value)
  }))
  // console.log(`processMdxGenTocList:toc: `, toc)
  return toc
}

export const generateNavigationList = async (allDocsList:DocSearchValue[][]) => {
  const locales = ["zh", "en"];
  // const types = ["docs"];
  // console.log("generateNavigationList: ", navList);
  // 输出导航列表到文件

  for (let i = 0; i < allDocsList.length; i++) {
    const allDocs = allDocsList[i];
    const locale = locales[i];
    const navList = await getNavigationList(allDocs);
    const navFilePath = path.join(outputDir, locale, "nav.json");
    await fs.writeFile(navFilePath, JSON.stringify(navList, null, 2));
    // for (const type of types) {
    // }
  }
  // await fs.writeFile(navFilePath, JSON.stringify(navList, null, 2));
}