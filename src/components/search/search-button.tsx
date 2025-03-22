"use client";
import {Button as UIButton, ButtonProps as UIButtonProps} from "@heroui/button";
import { useSearchModal } from "./search-context";
import { Search } from "lucide-react";
import { Kbd } from "@heroui/kbd";

type SearchModalButtonProps = UIButtonProps & {
  text?: string
}
export const SearchButton = ({ text, className,...props }: SearchModalButtonProps) => {
  const { setOpen } = useSearchModal();
  return <UIButton onPress={() => setOpen(true)} {...props} className={`h-11 w-60 grid grid-cols-[auto_1fr_auto]  text-left text-gray-950/50  dark:bg-white/5 dark:text-white/50 text-base/4 ${className}`} radius="full"
  startContent={<Search size={16} />}  
  endContent={<Kbd keys={["ctrl"]} className="h-5 w-fit rounded-full bg-transparent shadow-none ">K</Kbd>} 
  variant="bordered" >
    {text}
  </UIButton>
}