"use client";
import {Button as UIButton, ButtonProps as UIButtonProps} from "@heroui/button";
import { Search } from "lucide-react";
import {Kbd} from "@heroui/kbd";
import {Modal,ModalContent,ModalHeader,ModalBody,ModalFooter,useDisclosure,} from "@heroui/modal";
import {Command,CommandEmpty,CommandGroup,CommandInput,CommandItem,CommandList,CommandSeparator,CommandShortcut,CommandDialog, CommandDialogProps} from "@/components/ui/command"
import { createContext, useContext, useEffect, useState } from "react";
import { DialogTitle } from "../ui/dialog";
import { useTranslation } from "react-i18next";

const SearchModalContext = createContext({
  open: false,
  searchKeyword: '',
  setOpen: (open: boolean) => {},
  setSearchKeyword: (keyword: string) => {},
});
export const useSearchModal = () => useContext(SearchModalContext);
export const SearchModalProvider = ({ children }) => {
  const [open, setOpen] = useState(false);
  const [searchKeyword, setSearchKeyword] = useState('');

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen(!open);
      }
    };
    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  return (
    <SearchModalContext.Provider value={{ open, searchKeyword, setOpen, setSearchKeyword }}>
      {children}
    </SearchModalContext.Provider>
  );
};

type SearchModalButtonProps = UIButtonProps & {
  text?: string
}
export const SearchModalButton = ({ ...props }: SearchModalButtonProps) => {
  const { setOpen } = useSearchModal();
  return <UIButton onPress={() => setOpen(true)} {...props} className={`h-11 w-60 grid grid-cols-[auto_1fr_auto]  text-left text-gray-950/50  dark:bg-white/5 dark:text-white/50 text-base/4 ${props.className}`} radius="full"
  startContent={<Search size={16} />}  
  endContent={<Kbd keys={["ctrl"]} className="h-5 w-fit rounded-full bg-transparent shadow-none ">K</Kbd>} 
  variant="bordered" >
    {props.text}
  </UIButton>
}

type SearchModalProps = CommandDialogProps

export const SearchModal = ({ ...props }: SearchModalProps) => {
  const { open, setOpen, searchKeyword, setSearchKeyword } = useSearchModal();

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchKeyword(e.target.value)
    console.log("handleSearch: setSearchKeyword", searchKeyword)
  }
  return <>
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