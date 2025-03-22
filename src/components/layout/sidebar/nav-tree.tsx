"use client"

import { usePathname } from "next/navigation"
import { SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from "@/components/ui/sidebar"
import { NavNode } from "@/lib/md/get"
import { TreeMenuNode } from "./TreeNode"
import { useEffect, useState } from "react"

type ContentNavTreeProps = {
  currentNavTree: NavNode[]
}
export const ContentNavTree = ({currentNavTree}: ContentNavTreeProps) => {
  const pathname = usePathname()
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(new Set());
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    const savedPaths = localStorage.getItem('vscode-like-expanded-paths');
    if (savedPaths) {
      setExpandedPaths(new Set(JSON.parse(savedPaths)));
    }
  }, []);

  useEffect(() => {
    if (isClient && expandedPaths.size > 0) {
      localStorage.setItem('vscode-like-expanded-paths', JSON.stringify(Array.from(expandedPaths)));
    }
  }, [expandedPaths, isClient]);

  const togglePath = (path: string) => {
    setExpandedPaths(prevPaths => {
      const newPaths = new Set(prevPaths);
      if (newPaths.has(path)) {
        newPaths.delete(path);
      } else {
        newPaths.add(path);
      }
      return newPaths;
    });
  };
  return <>
        <SidebarMenu className="gap-0 px-2">
        {currentNavTree.map((item, index) => (
          <TreeMenuNode key={index} item={item} currentPath={pathname}    
            expandedPaths={expandedPaths}
          togglePath={togglePath}
          isClient={isClient}
          />
        ))}
      </SidebarMenu>
  </>
}