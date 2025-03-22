import chokidar from "chokidar"
import path from "path"
// pnpm ts-code --project tsconfig.node.json ./tests/scripts/watch.ts
const contentDir = path.join(process.cwd(), "src", "data")
const specificFile = path.join(contentDir, "docs", "zh", "dev", "index.mdx")

console.log(`Testing file watching for: ${specificFile}`)

// Watch the specific file directly
const watcher = chokidar.watch(specificFile, {
  persistent: true,
  usePolling: true,
  interval: 1000,
})

watcher
  .on("ready", () => console.log("Ready to watch specific file"))
  .on("change", (path) => console.log(`File ${path} has been changed`))
  .on("error", (error) => console.log(`Error: ${error}`))

console.log("Watcher started. Try modifying the file now.")

