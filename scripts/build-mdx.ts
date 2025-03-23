// scripts/build-mdx.ts
import path from "path"
import { generateNavigationList, processAllMdxFiles } from "@/lib/md/mdx-to-json"
import { i18nConfig } from "@/app/i18n/i18nConfig"

// import(`@/../public/data/zh/docs/index.json`)

const contentDir = path.join(process.cwd(), "src", "data")
const outputDir = path.join(process.cwd(), "public", "data") // Note: changed to public

async function buildAllMdx() {
  console.log("Building all MDX files for production...")
  const localeList = i18nConfig.locales
  const typeList = ["docs"]
  const localeAndTypeList: string[][] = localeList.map((locale) => typeList.map((type) => [locale, type])).flat()
  const allDocsList = await Promise.all(localeAndTypeList.map(([locale, type]) => processAllMdxFiles(contentDir, outputDir, locale, type)))

  await generateNavigationList(allDocsList)
  console.log("MDX build complete!")
}

buildAllMdx().catch(console.error)