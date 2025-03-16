"use client";
import {Button as UIButton, ButtonProps as UIButtonProps} from "@heroui/button";
import { Search } from "lucide-react";
import {Kbd} from "@heroui/kbd";
import {Modal,ModalContent,ModalHeader,ModalBody,ModalFooter,useDisclosure,} from "@heroui/modal";
import {Command,CommandEmpty,CommandGroup,CommandInput,CommandItem,CommandList,CommandSeparator,CommandShortcut,CommandDialog} from "~/components/ui/command"
import { useEffect, useState } from "react";
import { DialogTitle } from "../ui/dialog";

const SearchModalButton = ({ ...props }: UIButtonProps) => {
  return <UIButton {...props} className="h-11 w-60 grid grid-cols-[auto_1fr_auto]  text-left text-gray-950/50  dark:bg-white/5 dark:text-white/50" radius="full" startContent={<Search size={16} />}  endContent={<Kbd keys={["ctrl"]} className="h-5 rounded-full bg-transparent shadow-none ">K</Kbd>} variant="bordered" >Quick search</UIButton>
}

export const SearchModal = ({ ...props }: React.ComponentProps<"button">) => {
  const [open, setOpen] = useState(false)
  const [searchKeyword, setSearchKeyword] = useState("");
  const onOpen = () => setOpen(true)
  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }
    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [])
  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchKeyword(e.target.value)
    console.log("handleSearch: setSearchKeyword", searchKeyword)
  }
  return <><SearchModalButton onPress={onOpen} />

  <CommandDialog open={open} onOpenChange={setOpen}  className="bg-popover/60"  CloseButton={<Kbd  className=" rounded-full h-5 text-xs   ">Esc</Kbd>}  >
    <DialogTitle />
    <CommandInput placeholder="What are you searching for?" className="border-0 focus:shadow-none  focus:ring-0"
    value={searchKeyword}
    onChangeCapture={handleSearch}
    />
    <CommandList>
      <CommandEmpty>No results found.</CommandEmpty>
      <CommandGroup >
        <CommandItem>
          <span>Calendar</span>
        </CommandItem>
        <CommandItem>
          <span>Search Emoji</span>
        </CommandItem>
        <CommandItem>
          <span>Calculator</span>
        </CommandItem>
      </CommandGroup>
      <CommandSeparator />
    </CommandList>
      {/* <button role="button"  aria-label="Close" className="absolute appearance-none select-none top-1 end-1 p-2 text-foreground-500 rounded-full hover:bg-default-100 active:bg-default-200 tap-highlight-transparent outline-none data-[focus-visible=true]:z-10 data-[focus-visible=true]:outline-2 data-[focus-visible=true]:outline-focus data-[focus-visible=true]:outline-offset-2" type="button"><svg aria-hidden="true" className="fill-current" fill="none" focusable="false" height="1em" role="presentation" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" viewBox="0 0 24 24" width="1em"><path d="M18 6L6 18M6 6l12 12"></path></svg></button> */}
  </CommandDialog>
  </>
}