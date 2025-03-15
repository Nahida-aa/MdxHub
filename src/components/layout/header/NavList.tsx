"use client"

import Link from "~/components/Link"
import headerNavLinks from "~/data/headerNavLinks"
import { usePathname } from 'next/navigation'


export const NavList = () => {
  const pathname = usePathname()
  return <>
    {headerNavLinks.map((link) => (
      <Link
        key={link.title}
        href={link.href}
        className={`block font-medium text-gray-900 hover:text-primary-500 dark:text-gray-100 dark:hover:text-primary-400 ${pathname === link.href ? 'gradient' : ''}`}
      >
        {link.title}
      </Link>))}
  </>
}