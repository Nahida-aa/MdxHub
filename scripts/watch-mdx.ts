import chokidar from "chokidar"
import path from "path"
import { processMdxToJson, processAllMdxFiles } from "../src/lib/md/mdx-to-json"

const contentDir = path.join(process.cwd(), "src", "data")
const outputDir = path.join(process.cwd(), "public", "data")

// Initial build of all files
async function initialBuild() {
  console.log("Building all MDX files...")
  const localeList = ["en", "zh"]
  const typeList = ["docs"]
  const localeAndTypeList: string[][] = localeList.map((locale) => typeList.map((type) => [locale, type])).flat()
  await Promise.all(localeAndTypeList.map(([locale, type]) => processAllMdxFiles(contentDir, outputDir, locale, type)))

  console.log("Initial build complete!")
}

// Watch for changes
function watchFiles() {
  console.log(`Watching for ${contentDir}/**/*.mdx changes...`)

  const watcher = chokidar.watch(`${contentDir}/**/*.mdx`, {
    persistent: true,
    ignoreInitial: true,
  })

  watcher
    .on("add", async (filePath) => {
      console.log(`File added: ${filePath}`)
      const docData = await processMdxToJson(filePath,  outputDir)
      // Rebuild index after adding a file
      await processAllMdxFiles(contentDir, outputDir, docData.locale, docData.type)
    })
    .on("change", async (filePath) => {
      console.log(`File changed: ${filePath}`)
      await processMdxToJson(filePath,  outputDir)
    })
    .on("unlink", async (filePath) => {
      console.log(`File deleted: ${filePath}`)
      // filePath - contentDir
      const relativePath = path.relative(contentDir, filePath)
      // get 第一个目录
      const [locale, type] = relativePath.split(path.sep)
      // Rebuild index after deleting a file
      await processAllMdxFiles(contentDir, outputDir, locale, type)
    })
}

// Run both initial build and watcher
async function run() {
  await initialBuild()
  watchFiles()
}

run().catch(console.error)

