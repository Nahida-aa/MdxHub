import path from "path"
import fs from "fs"
import matter from 'gray-matter';
import { DocMeta } from "../types";

export const getFile = (file_path: string) => {
  // console.log(`getFile: ${file_path}`)
  const filePath = path.join(process.cwd(),`${file_path}`)
  let rawContent = '';

  if (!fs.existsSync(filePath)) {
    console.log(`File not found: ${filePath}`);
    return { metadata: null, content: null };
  } 

  try {
    rawContent = fs.readFileSync(filePath, 'utf8');
    // 判断空文件
    if (!rawContent) {
      console.log(`Empty file: ${filePath}`);
      return { metadata: null, content: null
      };
    }
  } catch (err) {
    console.error(`Failed to read file: ${filePath}`, err);
    return { metadata: null, content: null };
  }

  const { data: metadata, content } = matter(rawContent) as unknown as {
    data: DocMeta;
    content: string
  }
  return { metadata, content, rawContent }
};