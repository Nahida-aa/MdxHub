// export const generateDocsJson = async (locale: string) => {
//   const dir = `${DOCS_DIR}/${locale}/`
//   const docs: KVDocSearchValue = {}

//   // 并行处理目录和文件
//   const processDir = async (currentDir: string) => {
//     const entries = await fs.readdir(currentDir, { withFileTypes: true });

//     // 使用 Promise.all 并行处理所有条目
//     await Promise.all(
//       entries.map(async (entry) => {
//         const fullPath = path.join(currentDir, entry.name);
//         if (entry.isDirectory()) {
//           // 并行处理子目录
//           await processDir(fullPath);
//         } else if (entry.isFile() && entry.name.endsWith(".mdx")) {
//           // 并行处理文件
//           const fileContent = await fs.readFile(fullPath, "utf-8");
//           const relativePath = fullPath.replace(`${DATA_DIR}/`, "")
//           const [toc, {data, content}, {type,url, slug, segments, order} ] = await Promise.all([
//             processMdxGenTocList(fileContent),
//             matter(fileContent),
//             processSlug(relativePath)
//           ]);
//           docs[slug] = {locale,type,
//             url,
//             slug,
//             segments,
//             filePath: relativePath,
//             order,
//             meta: data as DocMeta,
//             content: content,
//             toc,
//           };
//         }
//       })
//     );
//   };

//   // 开始处理根目录
//   await processDir(dir);
//   const outputFilePath = path.join(OUTPUT_DIR, `docs-${locale}.json`);
//   await fs.writeFile(outputFilePath, JSON.stringify(docs, null, 2));
//   console.log(`Docs JSON generated at: ${outputFilePath}, docs: `);
// }