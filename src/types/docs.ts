export type docMeta = {
  title: string; // The page's <h1> title, used for SEO and OG Images. 页面标题 <h1> ，用于 SEO 和 OG 图片
  nav_title?: string; // Overrides the page's title in the navigation. This is useful when the page's title is too long to fit. If not provided, the title field is used. 覆盖页面标题在导航中。当页面标题过长无法适应时很有用。如果未提供，则使用 title 字段。
  description: string; // The page's description, used in the <meta name="description"> tag for SEO. 页面描述，用于 <meta name="description"> 标签的 SEO。
  source?: string; // Pulls content into a shared page. See Shared Pages. 拉取内容到共享页面。见共享页面。
  related?: { // A list of related pages at the bottom of the document. These will automatically be turned into cards. See Related Links. 文档底部相关页面列表。这些将自动转换为卡片。参见相关链接。 Links will be automatically generated for pages that have child pages. For example, the Optimizing section has links to all of its child pages. //链接将自动生成，用于具有子页面的页面。例如，优化部分包含对其所有子页面的链接。
    description?: string; // A description of the related content. 相关内容的描述。
    links: string[]; // A list of links to related content. 相关内容的链接列表。
  }
  version?: string //	A stage of development. e.g. experimental,legacy,unstable,RC
}