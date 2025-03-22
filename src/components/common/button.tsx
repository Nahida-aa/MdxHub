"use client"
import { Button } from "@heroui/react"
import { useRouter } from "next/navigation"

export const ToPrevPageButton = ({ text="Back" }: { text?: string }) => {
  const router = useRouter()
  return (
    <Button onPress={() => router.back()}>{text}</Button>
  )
}
