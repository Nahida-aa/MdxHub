import * as React from "react"

import { SearchForm } from "@/components/common/search-form"
import { SideSwitcher } from "@/components/layout/sidebar/side-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar"
import {ScrollShadow} from "@heroui/scroll-shadow";
import { generateNav } from "@/app/[locale]/docs/lib/gen";
import { generateNavigationList, getNavigationList, NavNode } from "@/lib/md/get";
import { ContentNavTree } from "./nav-tree";
import { headers, cookies } from 'next/headers'
import { Button } from "@/components/ui/button";
import { Settings } from "lucide-react";
import { ModeToggleGradientIcon } from "@/components/common/ModeToggle";

const types = ["docs"]

type AppSidebarProps = React.ComponentProps<typeof Sidebar> & {
  locale: string
}
export async function AppSidebar({ locale, ...props }: AppSidebarProps) {
  const cookieStore = await cookies()
  const type = cookieStore.get('type')?.value || types[0]
  console.log("AppSidebar: ", locale, type)
  const navTreeObj = {} as { [key: string]: NavNode[] }
  for (const type of types) {
    navTreeObj[type] = await getNavigationList(locale, type)
  }
  const currentNavTree = navTreeObj[type]
  return (
    <Sidebar {...props} className="Sidebar bg-transparent justify-between">
      <SidebarHeader>
        <SideSwitcher
          list={types}
          defaultItem={types[0]}
        />
        {/* <SearchForm /> */}
      </SidebarHeader>
      <ScrollShadow hideScrollBar className="h-full" >
        <SidebarContent>
          {/* We create a SidebarGroup for each parent. */}
          <ContentNavTree currentNavTree={currentNavTree} />
            
        </SidebarContent>
      </ScrollShadow>
      <SidebarFooter>
      <SidebarMenu className='flex-row justify-between'>
          <SidebarMenuItem className="flex items-center">
            {/* <SidebarMenuButton asChild 
            // onClick={() => {
            //     // toggleSidebar()
            //     // router.push('/setting');
            //     // setOpenMobile(false);
            //   }}
            </SidebarMenuButton>
            > */}
              <button  className='flex size-10 h-10 [&_svg]:size-5 items-center justify-center p-1.5'>
              <Settings />
              </button>
          </SidebarMenuItem>
          <SidebarMenuItem>
            {/* <SidebarMenuButton asChild> */}
              <ModeToggleGradientIcon  className='size-10 active:bg-sidebar-accent [&_svg]:size-5' />
            {/* </SidebarMenuButton> */}
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
