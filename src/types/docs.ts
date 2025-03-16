export type docMeta = {
  title: string; // The page's <h1> title, used for SEO and OG Images. 页面标题 <h1> ，用于 SEO 和 OG 图片
  nav_title?: string; // Overrides the page's title in the navigation. This is useful when the page's title is too long to fit. If not provided, the title field is used. 覆盖页面标题在导航中。当页面标题过长无法适应时很有用。如果未提供，则使用 title 字段。
  description: string; // The page's description, used in the <meta name="description"> tag for SEO. 页面描述，用于 <meta name="description"> 标签的 SEO。
  source?: string; // Pulls content into a shared page. See Shared Pages. 拉取内容到共享页面。见共享页面。
}