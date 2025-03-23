import { MetadataRoute } from 'next'
// import { allBlogs } from 'contentlayer/generated'
import siteMetadata from 'src/data/siteMetadata'

export const dynamic = process.env.EXPORT ? 'force-static' : undefined
export const revalidate = process.env.EXPORT ? 60 : undefined; // revalidate every 60 seconds

export default function sitemap(): MetadataRoute.Sitemap {
  const siteUrl = siteMetadata.siteUrl

  // const blogRoutes = allBlogs
  //   .filter((post) => !post.draft)
  //   .map((post) => ({
  //     url: `${siteUrl}/${post.path}`,
  //     lastModified: post.lastmod || post.date,
  //   }))

  const routes = ['', 'blog', 'projects', 'tags'].map((route) => ({
    url: `${siteUrl}/${route}`,
    lastModified: new Date().toISOString().split('T')[0],
  }))

  return [...routes, 
    // ...blogRoutes
  ]
}
