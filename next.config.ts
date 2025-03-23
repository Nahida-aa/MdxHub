import type { NextConfig } from "next";
import createMDX, { NextMDXOptions } from '@next/mdx'
import dotenv from 'dotenv';
import fs from 'fs';
// import remarkGfm from "remark-gfm";
// import remarkMath from "remark-math";
// import rehypeMathjax from 'rehype-mathjax'

const rehypePrettyCode_options = {
  // keepBackground: false, // 是否继承背景色
  defaultLang: "plaintext",
  // theme: moonlightTheme,
  
  tokensMap: {
    var: "variable",
    str: "string",
    fn: "entity.name.function",
    cls: "entity.name.class",
  },
};

// import { withContentlayer } from 'next-contentlayer2';

const isDev = process.env.NODE_ENV === "development";

const withBundleAnalyzer = require('@next/bundle-analyzer')({
  enabled: process.env.ANALYZE === 'true',
})

// You might need to insert additional domains in script-src if you are using external services
const ContentSecurityPolicy = `
  default-src 'self';
  script-src 'self' 'unsafe-eval' 'unsafe-inline' giscus.app analytics.umami.is;
  style-src 'self' 'unsafe-inline';
  img-src * blob: data:;
  media-src *.s3.amazonaws.com;
  connect-src *;
  font-src 'self';
  frame-src giscus.app
`

const securityHeaders = [
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP
  {
    key: 'Content-Security-Policy',
    value: ContentSecurityPolicy.replace(/\n/g, ''),
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy
  {
    key: 'Referrer-Policy',
    value: 'strict-origin-when-cross-origin',
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options
  {
    key: 'X-Frame-Options',
    value: 'DENY',
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options
  {
    key: 'X-Content-Type-Options',
    value: 'nosniff',
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-DNS-Prefetch-Control
  {
    key: 'X-DNS-Prefetch-Control',
    value: 'on',
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security
  {
    key: 'Strict-Transport-Security',
    value: 'max-age=31536000; includeSubDomains',
  },
  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Feature-Policy
  {
    key: 'Permissions-Policy',
    value: 'camera=(), microphone=(), geolocation=()',
  },
]

const withMDX = createMDX({
  // Add markdown plugins here, as desired
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [
      // [remarkGfm, {}],
      ['remark-gfm'],
      ['@vcarl/remark-headings'],
      ['remark-frontmatter', {type: 'yaml', marker: '-'}], // 解析 frontmatter 到 语法树
      ['remark-mdx-frontmatter'], // 导出 frontmatter
      // ['remark-mermaidjs'],
      // [remarkMath,{}],
      ['remark-math', {}]
    ],
    rehypePlugins: [
      ['rehype-callouts'],
      // ['rehype-katex', { strict: true, throwOnError: true }]
      // [rehypeMathjax,{}],
      // ['rehype-mermaid'],
      ['rehype-pretty-code', rehypePrettyCode_options],
      ["rehype-mathjax"],
    ],
  },
} as NextMDXOptions)

let plugins: any[] = []

// 检查是否在 GitHub 环境中
console.log('process.env:', process.env);
if (process.env.GITHUB_ACTIONS) { // ‘true
  const envPath = '.env.github';
  console.log('GitHub Actions detected, loading environment variables from:', envPath);
  if (fs.existsSync(envPath)) {
    dotenv.config({ path: envPath });
  }
  plugins = [
    withMDX, // pnpm add @next/mdx, 仅需安装这个来实现
  ]
} else {
  plugins = [
    withMDX, // pnpm add @next/mdx, 仅需安装这个来实现
    withBundleAnalyzer, 
  ]
}

const output = process.env.EXPORT ? 'export' : undefined
const basePath = process.env.BASE_PATH || undefined
const unoptimized = process.env.UNOPTIMIZED ? true : undefined

const nextConfig: NextConfig = {
  output,
  basePath,
  reactStrictMode: true,
  pageExtensions: ['ts', 'tsx', 'js', 'jsx', 'md', 'mdx'],
  eslint: {
    dirs: ['app', 'components', 'layouts', 'scripts'],
  },
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'picsum.photos',
      },
      {
        hostname: 'avatar.vercel.sh',
      },
      {
        hostname: 'avatars.githubusercontent.com',
      },
      {
        hostname: 'raw.githubusercontent.com',
      },
      {
        hostname: 'utfs.io',
      },
    ],
    unoptimized,
  },
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: securityHeaders,
      },
    ]
  },
  // webpack: (config, options) => {
  //   config.module.rules.push({
  //     test: /\.svg$/,
  //     use: ['@svgr/webpack'],
  //   })

    // return config
  // },
  experimental: {
    turbo: {
      rules: {
        '*.svg': {
          loaders: ['@svgr/webpack'],
          as: '*.js',
        },
      },
    }, // 启用 Turbopack
  },
  transpilePackages: ['next-mdx-remote'],
}

export default plugins.reduce((prev, item) => item(prev), nextConfig)