import NextImage, { ImageProps } from 'next/image'

export const TailwindBG = () => {

  return (<>
  <div className="absolute z-20 top-0 inset-x-0 flex justify-center overflow-hidden pointer-events-none">
    <div className="w-[108rem] flex-none flex justify-end">
      <picture>
        <source srcSet="/docs@30.8b9a76a2.avif" type="image/avif" />
        <NextImage src="/docs@tinypng.d9e4dcdc.png" width={574} height={167} alt="" className="w-[71.75rem] flex-none max-w-none dark:hidden" decoding="async"/>
      </picture>
      <picture>
        <source srcSet="/docs-dark@30.1a9f8cbf.avif" type="image/avif"/>
        <img src="/docs-dark@tinypng.1bbe175e.png" alt="" className="w-[90rem] flex-none max-w-none hidden dark:block" decoding="async"/>
      </picture>
    </div>
  </div>
  {/* <TailwindBG2 /> */}
  </>
  )
}

export const TailwindBG2 = ()=><div className="hidden sm:block absolute z-0 top-0 left-[15%] pt-[40%] 2xl:left-[40%] 2xl:pt-[8%] dark:hidden"><img src="/installation.50c59fdd.jpg" alt="" className="w-[52.6875rem] max-w-none" decoding="async"/></div>