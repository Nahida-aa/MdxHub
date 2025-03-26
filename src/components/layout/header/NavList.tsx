"use client"

import Link from "@/components/Link"
import headerNavLinks from "@/data/headerNavLinks"
import { useParams, usePathname } from 'next/navigation'
import { useTranslation } from "react-i18next"


export const NavList = () => {
  const pathname = usePathname()
  // const locale = useParams()?.locale
  const { t } = useTranslation(['common'])
  return <>
    {headerNavLinks.map((link) => (
      <Link
        key={link.title}
        href={link.href}
        className={`block font-medium text-gray-900 hover:text-primary-500 dark:text-gray-100 dark:hover:text-primary-400 ${pathname === link.href ? 'gradient' : ''}`}
      >
        {t(`${link.title}`)}
      </Link>))}
  </>
}