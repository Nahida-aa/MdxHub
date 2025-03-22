"use client"

import * as React from "react"
import { useTheme } from "next-themes"
import { motion, AnimatePresence } from "framer-motion"
import { LoadingS } from "@/components/ui/loading/Loading"

export const ModeToggleGradientIconV0 = ({
  className = "",
  size = 100,
}: {
  className?: string
  size?: number
}) => {
  const { setTheme, theme } = useTheme()
  // const [mounted, setMounted] = React.useState(false)

  // React.useEffect(() => {
  //   setMounted(true)
  // }, [])

  // if (!mounted) {
  //   return (
  //     <div className="min-w-9 min-h-9 flex items-center justify-center">
  //       <LoadingS />
  //     </div>
  //   )
  // }

  const isDark = theme === "dark"

  // SVG paths
  const sunPath =
    "M70 49.5C70 60.8218 60.8218 70 49.5 70C38.1782 70 29 60.8218 29 49.5C29 38.1782 38.1782 29 49.5 29C60 29 69.5 38 70 49.5Z"
  const moonPath =
    "M70 49.5C70 60.8218 60.8218 70 49.5 70C38.1782 70 29 60.8218 29 49.5C29 38.1782 38.1782 29 49.5 29C39 45 49.5 59.5 70 49.5Z"

  // Ray animations
  const raysVariants = {
    hidden: {
      opacity: 0,
      scale: 0,
      transition: {
        staggerChildren: 0.05,
        staggerDirection: -1,
      },
    },
    visible: {
      opacity: 1,
      scale: 1,
      transition: {
        staggerChildren: 0.05,
        delayChildren: 0.2,
      },
    },
  }

  return (
    <button
      className="flex items-center justify-center relative gap-0 size-9 p-1.5 [&_svg]:size-6"
      onClick={() => setTheme(isDark ? "light" : "dark")}
      aria-label={isDark ? "Switch to light mode" : "Switch to dark mode"}
    >
      <div className="relative w-full h-full flex items-center justify-center">
        {/* We'll use two separate SVGs for light and dark mode to avoid transform conflicts */}
        <AnimatePresence initial={false} mode="wait">
          {isDark ? (
            <motion.svg
              key="moon"
              strokeWidth="4"
              strokeLinecap="round"
              width={100}
              height={100}
              viewBox="0 0 100 100"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              className="absolute inset-0"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.3 }}
            >
              {/* Moon */}
              <motion.path
                d={moonPath} 
                fill="#60a5fa"
                fillOpacity={0.35}
                stroke="#60a5fa"
                strokeOpacity={1}
                initial={{ pathLength: 0 }}
                animate={{ pathLength: 1 ,
                  rotate: -360,
                  scale: 2,
                }}
                transition={{ duration: 0.5, delay: 0.2 }}
              />

              {/* Shine effect */}
              <motion.circle
                cx="49.5"
                cy="49.5"
                r="25"
                stroke="#60a5fa"
                strokeOpacity={0.3}
                strokeWidth="2"
                fill="none"
                initial={{ scale: 0.8, opacity: 0 }}
                animate={{
                  scale: [0.8, 1.2, 0.8],
                  opacity: [0, 0.5, 0],
                  transition: {
                    repeat: 1,
                    duration: 2,
                    ease: "easeInOut",
                  },
                }}
              />
            </motion.svg>
          ) : (
            <motion.svg
              key="sun"
              strokeWidth="4"
              strokeLinecap="round"
              width={100}
              height={100}
              viewBox="0 0 100 100"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              className="absolute inset-0"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.3 }}
            >
              {/* Sun center */}
              <motion.path
                d={sunPath}
                fill="#ca8a04"
                fillOpacity={0.35}
                stroke="#ca8a04"
                strokeOpacity={1}
                initial={{ pathLength: 0 }}
                animate={{ pathLength: 1 }}
                transition={{ duration: 0.5 }}
              />

              {/* Sun rays */}
              <motion.g
                variants={raysVariants}
                initial="hidden"
                animate="visible"
                className="stroke-yellow-600"
                style={{ strokeLinecap: "round", strokeWidth: 4 }}
              >
                <motion.line x1="50" y1="2" x2="50" y2="11" />
                <motion.line x1="85" y1="15" x2="78" y2="22" />
                <motion.line x1="98" y1="50" x2="89" y2="50" />
                <motion.line x1="85" y1="85" x2="78" y2="78" />
                <motion.line x1="50" y1="98" x2="50" y2="89" />
                <motion.line x1="23" y1="78" x2="16" y2="84" />
                <motion.line x1="11" y1="50" x2="2" y2="50" />
                <motion.line x1="23" y1="23" x2="16" y2="16" />
              </motion.g>
            </motion.svg>
          )}
        </AnimatePresence>
      </div>
    </button>
  )
}

