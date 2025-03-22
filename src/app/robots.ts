import { MetadataRoute } from 'next'
import siteMetadata from 'src/data/siteMetadata'

export const dynamic = "force-static";
export const revalidate = 60; // Set revalidation interval or remove if not needed
export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: '*',
      allow: '/',
    },
    sitemap: `${siteMetadata.siteUrl}/sitemap.xml`,
    host: siteMetadata.siteUrl,
  }
}
