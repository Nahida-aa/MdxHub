'use client';

import { useRouter } from 'next/navigation';
import { usePathname } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { i18nConfig } from './i18nConfig';

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

// export function SelectDemo() {
//   return (
    
//   )
// }

export default function LanguageChanger() {
  const { i18n } = useTranslation();
  const currentLocale = i18n.language;
  console.log("currentLocale: ", currentLocale);
  const router = useRouter();
  const currentPathname = usePathname();

  // const handleChange = (e: React.ChangeEvent<HTMLSelectElement>
  // ) => {
  //   console.log("e.target.value: ", e.target.value);
  //   const newLocale = e.target.value;
  //   changeLocale(newLocale);
  // };

  const changeLocale =(newLocale: string)=>{
    console.log("changeLocale: 0")
    // set cookie for next-i18n-router
    const days = 30;
    const date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    const expires = date.toUTCString();
    document.cookie = `NEXT_LOCALE=${newLocale};expires=${expires};path=/`;

    // redirect to the new locale path
    if (
      currentLocale === i18nConfig.defaultLocale &&
      !i18nConfig.prefixDefault
    ) {
      router.push('/' + newLocale + currentPathname);
    } else {
      console.log("准备切换路由")
      router.push(
        currentPathname.replace(`/${currentLocale}`, `/${newLocale}`)
      );
    }

    router.refresh();
  }

  return (
    <Select defaultValue={currentLocale} onValueChange={changeLocale}>
      <SelectTrigger className="min-w-10 max-w-10 w-10  gap-1 border-none focus:outline-0 focus:ring-0 p-0">
        <SelectValue  />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {i18nConfig.locales.map((locale) => (
            <SelectItem key={locale} value={locale}>
              {locale}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
    // <select onChange={handleChange} value={currentLocale} className='bg-background' >
    //   {i18nConfig.locales.map((locale) => (
    //     <option key={locale} value={locale}>
    //       {locale}
    //     </option>
    //   ))}
    // </select>
  );
}



