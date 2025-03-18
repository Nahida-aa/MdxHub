// type Theme = BuiltinTheme | ThemeRegistrationRaw;
// export interface Options {
//   grid?: boolean;
//   theme?: Theme | Record<string, Theme>;
//   keepBackground?: boolean;
//   bypassInlineCode?: boolean;
//   defaultLang?: string | {
//       block?: string;
//       inline?: string;
//   };
//   tokensMap?: Record<string, string>;
//   transformers?: Array<ShikiTransformer>;
//   filterMetaString?(str: string): string;
//   getHighlighter?(options: BundledHighlighterOptions<any, any>): Promise<Highlighter>;
//   onVisitLine?(element: LineElement): void;
//   onVisitHighlightedLine?(element: LineElement, id: string | undefined): void;
//   onVisitHighlightedChars?(element: CharsElement, id: string | undefined): void;
//   onVisitTitle?(element: Element): void;
//   onVisitCaption?(element: Element): void;
// }