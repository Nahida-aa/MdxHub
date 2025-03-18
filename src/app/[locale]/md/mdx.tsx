// "use server"
// "use client"
import { 
  MDXRemote, 
  compileMDX,
  MDXRemoteProps } from 'next-mdx-remote/rsc';

import { MdxComponents } from './mdxComponents'

import remarkGfm from 'remark-gfm'
import rehypePrettyCode, { Options as RehypePrettyCode_options } from 'rehype-pretty-code';
// import moonlightTheme from './assets/moonlight-ii.json' with { type: 'json' };

import remarkMath from 'remark-math' // 用于将 math 标记为 code and pre/code
import rehypeKatex from 'rehype-katex'
import rehypeMathjax from 'rehype-mathjax'
// import rehypeMdxCodeProps from 'rehype-mdx-code-props'

// import { remarkMark } from './remark/highlightMark'
// import { remarkMark } from 'remark-mark-highlight'

import rehypeAutolinkHeadings from 'rehype-autolink-headings';
import rehypeSlug from 'rehype-slug';
import { SerializeOptions } from 'node_modules/next-mdx-remote/dist/types';
import mdxErrorHandler from './plugins/error-handler';
import rehypeCallouts from 'rehype-callouts'
import remarkBreaks from 'remark-breaks'

const rehypePrettyCode_options:RehypePrettyCode_options = {
  // keepBackground: false, // 是否继承背景色
  defaultLang: "plaintext",
  // theme: moonlightTheme,
  
  tokensMap: {
    fn: "entity.name.function",
  },
};

// mdx2html: 工作流程是先将 MDX 源码解析为 JSX, 然后再根据需要切换和渲染自定义组件
export async function CustomMDX(props: MDXRemoteProps) {

  // console.log(`CustomMDX: ${JSON.stringify(props.options, null, 2)}`)
  const options: SerializeOptions = {
    // made available to the arguments of any custom MDX component
    // scope: {},
    // MDX's available options, see the MDX docs for more info.
    // https://mdxjs.com/packages/mdx/#compilefile-options
    mdxOptions: {
      // jsx: true,
      remarkPlugins: [  // 处理 md 插件, remarkRehype: Transform to HTML AST
        mdxErrorHandler,
        remarkGfm,
        remarkBreaks,// 处理换行符
        remarkMath, // 将 math 标记为 code and pre/code
        // remarkMark
      ],
      rehypePlugins: [ // 处理 html 插件
        rehypeCallouts,
        [rehypePrettyCode, rehypePrettyCode_options],
        // rehypeAutolinkHeadings, // 不知道为什么不起作用, 然后自己实现
        rehypeSlug, // 为标题添加 id
        // rehypeKatex, // 需要 css
        rehypeMathjax, // 不需要 css
        // 下面的插件必须最后使用
        // rehypeMdxCodeProps
      ],
      format: props.options?.mdxOptions?.format || 'mdx', // 'mdx' or 'md'
    },
    // Indicates whether or not to parse the frontmatter from the MDX source
    parseFrontmatter: true,  // default: false
  }
  // console.log(`CustomMDX: ${JSON.stringify(options, null, 2)}`)
  const components = {
    ...MdxComponents,
    // pre: Pre,
    // h1: createHeading(1),
    // Image,
    ...(props.components || {})
  };
  const mdxRemoteProps = {
    source: props.source, 
    options,
    components
  }

  // const { content, frontmatter } = await compileMDX<{ title: string }>(mdxRemoteProps)
  // const { content } = await compileMDX(mdxRemoteProps)
  // console.log(`content:`)
  // console.log(content)

  // return content
  // return <><h1>{frontmatter.title}</h1>
  // {content}</>
  return <MDXRemote {...mdxRemoteProps}
    // {...props}
    // options={{...options, ...(props.options||{}) }}
    // components={components}
  />
}