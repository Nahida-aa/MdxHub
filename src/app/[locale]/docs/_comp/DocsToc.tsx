"use client"
import { Card } from '@/components/ui/card';
import Link from 'next/link';
import { Toc } from '../../md/types';
import {
  Listbox,
  ListboxSection,
  ListboxItem
} from "@heroui/listbox";
import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import clsx from 'clsx';

interface DocsTocProps {
  toc: Toc[];
}

export const DocsToc = ({ toc }: DocsTocProps) => {
  const [activeSlug, setActiveSlug] = useState<string | null>(null);
  const [isClient, setIsClient] = useState(false); // 标记是否在客户端渲染
  useEffect(() => {
    setIsClient(true); // 标记为客户端渲染
    console.log("useEffect: ", toc)
    const handleIntersection = (entries: IntersectionObserverEntry[]) => {
      console.log("DocsToc:entries: ", entries)
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          console.log("DocsToc:isIntersecting: ", entry)
          const id = entry.target.getAttribute("id");
          if (id) {
            console.log("DocsToc:id: ", id)
            setActiveSlug(id);
            // console.log("DocsToc:activeSlug: ", activeSlug)
          }
        }
      });
    };

    const observer = new IntersectionObserver(handleIntersection, {
      root: null, // 默认是视口
      rootMargin: "0px 0px -80% 0px", // 提前触发
      threshold: 0.1, // 元素可见 10% 时触发
    });

    // 监控所有标题
    const headingElements = document.querySelectorAll("h2, h3, h4, h5, h6");
    headingElements.forEach((el) => observer.observe(el));

    return () => {
      headingElements.forEach((el) => observer.unobserve(el));
    };
  }, []);
  useEffect(() => {
    console.log("DocsToc:activeSlug updated: ", activeSlug);
  }, [activeSlug]);
  // 生成标题序号
  const generateHeadingNumbers = (toc: Toc[]) => {
    console.log("generateHeadingNumbers: ", toc)
    const counters: number[] = []; // 用于记录每个层级的计数器
    return toc.map((item) => {
      const { depth } = item;

      // 确保计数器数组的长度与当前深度一致(不考虑h1)
      while (counters.length < depth - 1) {
        counters.push(0);
      }

      // 重置更深层级的计数器
      counters.splice(depth - 1);

      // 当前层级计数器加 1
      counters[depth - 2]++;

      // 生成序号字符串
      const number = counters.join(".");

      return {
        ...item,
        number, // 添加序号到每个标题项
      };
    });
  };

  const numberedToc = generateHeadingNumbers(toc);
  return <>
    <section className=' hidden space-y-2 lg:block lg:sticky lg:top-20 lg:col-span-3 xl:col-span-3  w-full pt-4'>
      <div className='fixed'>

      <p className='font-medium'>在这个页面</p>
      <ul>
        {numberedToc.map((item, index) => <li key={index}>
          <Link className={clsx(
            `text-muted-foreground hover:text-primary ${item.depth===3 ?'pl-3':''} ${item.depth===4 ?'pl-6':''} ${item.depth===5 ?'pl-9':''} ${item.depth===6 ?'pl-12':''} h-5 `,
            {
              "text-primary font-bold": isClient && activeSlug === item.slug,
            }
          )}
            href={`#${item.slug}`}
          >
              {item.number}  {item.value}
          </Link>
        </li>)}
      </ul>
      </div>
    </section>
  </>
}