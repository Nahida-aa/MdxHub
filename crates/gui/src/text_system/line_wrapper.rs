// use collections::HashMap;
// use gpui::{FontId, Pixels, SharedString, TextRun, TextSystem, px};
// use std::{borrow::Cow, iter, sync::Arc};

// /// Determines whether to truncate text from the start or end.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum TruncateFrom {
//     /// Truncate text from the start.
//     Start,
//     /// Truncate text from the end.
//     End,
// }

// /// The GPUI line wrapper, used to wrap lines of text to a given width.
// pub struct LineWrapper {
//     text_system: Arc<TextSystem>,
//     pub(crate) font_id: FontId,
//     pub(crate) font_size: Pixels,
//     cached_ascii_char_widths: [Option<Pixels>; 128],
//     cached_other_char_widths: HashMap<char, Pixels>,
// }

// impl LineWrapper {
//     /// The maximum indent that can be applied to a line.
//     pub const MAX_INDENT: u32 = 256;

//     pub(crate) fn new(font_id: FontId, font_size: Pixels, text_system: Arc<TextSystem>) -> Self {
//         Self {
//             text_system,
//             font_id,
//             font_size,
//             cached_ascii_char_widths: [None; 128],
//             cached_other_char_widths: HashMap::default(),
//         }
//     }

//     /// Wrap a line of text to the given width with this wrapper's font and font size.
//     pub fn wrap_line<'a>(
//         &'a mut self,
//         fragments: &'a [LineFragment],
//         wrap_width: Pixels,
//     ) -> impl Iterator<Item = Boundary> + 'a {
//         let mut width = px(0.);
//         let mut first_non_whitespace_ix = None;
//         let mut indent = None;
//         let mut last_candidate_ix = 0;
//         let mut last_candidate_width = px(0.);
//         let mut last_wrap_ix = 0;
//         let mut prev_c = '\0';
//         let mut index = 0;
//         let mut candidates = fragments
//             .iter()
//             .flat_map(move |fragment| fragment.wrap_boundary_candidates())
//             .peekable();
//         iter::from_fn(move || {
//             for candidate in candidates.by_ref() {
//                 let ix = index;
//                 index += candidate.len_utf8();
//                 let mut new_prev_c = prev_c;
//                 let item_width = match candidate {
//                     WrapBoundaryCandidate::Char { character: c } => {
//                         if c == '\n' {
//                             continue;
//                         }

//                         if Self::is_word_char(c) {
//                             if prev_c == ' ' && c != ' ' && first_non_whitespace_ix.is_some() {
//                                 last_candidate_ix = ix;
//                                 last_candidate_width = width;
//                             }
//                         } else {
//                             // CJK may not be space separated, e.g.: `Hello world你好世界`
//                             if c != ' ' && first_non_whitespace_ix.is_some() {
//                                 last_candidate_ix = ix;
//                                 last_candidate_width = width;
//                             }
//                         }

//                         if c != ' ' && first_non_whitespace_ix.is_none() {
//                             first_non_whitespace_ix = Some(ix);
//                         }

//                         new_prev_c = c;

//                         self.width_for_char(c)
//                     }
//                     WrapBoundaryCandidate::Element {
//                         width: element_width,
//                         ..
//                     } => {
//                         if prev_c == ' ' && first_non_whitespace_ix.is_some() {
//                             last_candidate_ix = ix;
//                             last_candidate_width = width;
//                         }

//                         if first_non_whitespace_ix.is_none() {
//                             first_non_whitespace_ix = Some(ix);
//                         }

//                         element_width
//                     }
//                 };

//                 width += item_width;
//                 if width > wrap_width && ix > last_wrap_ix {
//                     if let (None, Some(first_non_whitespace_ix)) = (indent, first_non_whitespace_ix)
//                     {
//                         indent = Some(
//                             Self::MAX_INDENT.min((first_non_whitespace_ix - last_wrap_ix) as u32),
//                         );
//                     }

//                     if last_candidate_ix > 0 {
//                         last_wrap_ix = last_candidate_ix;
//                         width -= last_candidate_width;
//                         last_candidate_ix = 0;
//                     } else {
//                         last_wrap_ix = ix;
//                         width = item_width;
//                     }

//                     if let Some(indent) = indent {
//                         width += self.width_for_char(' ') * indent as f32;
//                     }

//                     return Some(Boundary::new(last_wrap_ix, indent.unwrap_or(0)));
//                 }

//                 prev_c = new_prev_c;
//             }

//             None
//         })
//     }

//     /// Determines if a line should be truncated based on its width.
//     ///
//     /// Returns the truncation index in `line`.
//     pub fn should_truncate_line(
//         &mut self,
//         line: &str,
//         truncate_width: Pixels,
//         truncation_affix: &str,
//         truncate_from: TruncateFrom,
//     ) -> Option<usize> {
//         let mut width = px(0.);
//         let suffix_width = truncation_affix
//             .chars()
//             .map(|c| self.width_for_char(c))
//             .fold(px(0.0), |a, x| a + x);
//         let mut truncate_ix = 0;

//         match truncate_from {
//             TruncateFrom::Start => {
//                 for (ix, c) in line.char_indices().rev() {
//                     if width + suffix_width < truncate_width {
//                         truncate_ix = ix;
//                     }

//                     let char_width = self.width_for_char(c);
//                     width += char_width;

//                     if width.floor() > truncate_width {
//                         return Some(truncate_ix);
//                     }
//                 }
//             }
//             TruncateFrom::End => {
//                 for (ix, c) in line.char_indices() {
//                     if width + suffix_width < truncate_width {
//                         truncate_ix = ix;
//                     }

//                     let char_width = self.width_for_char(c);
//                     width += char_width;

//                     if width.floor() > truncate_width {
//                         return Some(truncate_ix);
//                     }
//                 }
//             }
//         }

//         None
//     }

//     /// Truncate a line of text to the given width with this wrapper's font and font size.
//     pub fn truncate_line<'a>(
//         &mut self,
//         line: SharedString,
//         truncate_width: Pixels,
//         truncation_affix: &str,
//         runs: &'a [TextRun],
//         truncate_from: TruncateFrom,
//     ) -> (SharedString, Cow<'a, [TextRun]>) {
//         if let Some(truncate_ix) =
//             self.should_truncate_line(&line, truncate_width, truncation_affix, truncate_from)
//         {
//             let result = match truncate_from {
//                 TruncateFrom::Start => SharedString::from(format!(
//                     "{truncation_affix}{}",
//                     &line[line.ceil_char_boundary(truncate_ix + 1)..]
//                 )),
//                 TruncateFrom::End => SharedString::from(format!(
//                     "{}{truncation_affix}",
//                     line[..truncate_ix]
//                         .trim_end_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
//                 )),
//             };
//             let mut runs = runs.to_vec();
//             update_runs_after_truncation(&result, truncation_affix, &mut runs, truncate_from);
//             (result, Cow::Owned(runs))
//         } else {
//             (line, Cow::Borrowed(runs))
//         }
//     }

//     /// Any character in this list should be treated as a word character,
//     /// meaning it can be part of a word that should not be wrapped.
//     pub(crate) fn is_word_char(c: char) -> bool {
//         // ASCII alphanumeric characters, for English, numbers: `Hello123`, etc.
//         c.is_ascii_alphanumeric() ||
//         // Latin script in Unicode for French, German, Spanish, etc.
//         // Latin-1 Supplement
//         // https://en.wikipedia.org/wiki/Latin-1_Supplement
//         matches!(c, '\u{00C0}'..='\u{00FF}') ||
//         // Latin Extended-A
//         // https://en.wikipedia.org/wiki/Latin_Extended-A
//         matches!(c, '\u{0100}'..='\u{017F}') ||
//         // Latin Extended-B
//         // https://en.wikipedia.org/wiki/Latin_Extended-B
//         matches!(c, '\u{0180}'..='\u{024F}') ||
//         // Cyrillic for Russian, Ukrainian, etc.
//         // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unicode
//         matches!(c, '\u{0400}'..='\u{04FF}') ||

//         // Vietnamese (https://vietunicode.sourceforge.net/charset/)
//         matches!(c, '\u{1E00}'..='\u{1EFF}') || // Latin Extended Additional
//         matches!(c, '\u{0300}'..='\u{036F}') || // Combining Diacritical Marks

//         // Bengali (https://en.wikipedia.org/wiki/Bengali_(Unicode_block))
//         matches!(c, '\u{0980}'..='\u{09FF}') ||

//         // Some other known special characters that should be treated as word characters,
//         // e.g. `a-b`, `var_name`, `I'm`/`won’t`, '@mention`, `#hashtag`, `100%`, `3.1415`,
//         // `2^3`, `a~b`, `a=1`, `Self::new`, etc. Trailing punctuation like `,`, `.`, `:`, `;`
//         // is included so it stays attached to the preceding word when wrapping.
//         matches!(c, '-' | '_' | '.' | '\'' | '’' | '‘' | '$' | '%' | '@' | '#' | '^' | '~' | ',' | '=' | ':' | ';') ||
//         // `⋯` character is special used in Zed, to keep this at the end of the line.
//         matches!(c, '⋯')
//     }

//     #[inline(always)]
//     fn width_for_char(&mut self, c: char) -> Pixels {
//         if (c as u32) < 128 {
//             if let Some(cached_width) = self.cached_ascii_char_widths[c as usize] {
//                 cached_width
//             } else {
//                 let width = self
//                     .text_system
//                     .layout_width(self.font_id, self.font_size, c);
//                 self.cached_ascii_char_widths[c as usize] = Some(width);
//                 width
//             }
//         } else if let Some(cached_width) = self.cached_other_char_widths.get(&c) {
//             *cached_width
//         } else {
//             let width = self
//                 .text_system
//                 .layout_width(self.font_id, self.font_size, c);
//             self.cached_other_char_widths.insert(c, width);
//             width
//         }
//     }
// }

// fn update_runs_after_truncation(
//     result: &str,
//     ellipsis: &str,
//     runs: &mut Vec<TextRun>,
//     truncate_from: TruncateFrom,
// ) {
//     let mut truncate_at = result.len() - ellipsis.len();
//     match truncate_from {
//         TruncateFrom::Start => {
//             for (run_index, run) in runs.iter_mut().enumerate().rev() {
//                 if run.len <= truncate_at {
//                     truncate_at -= run.len;
//                 } else {
//                     run.len = truncate_at + ellipsis.len();
//                     runs.splice(..run_index, std::iter::empty());
//                     break;
//                 }
//             }
//         }
//         TruncateFrom::End => {
//             for (run_index, run) in runs.iter_mut().enumerate() {
//                 if run.len <= truncate_at {
//                     truncate_at -= run.len;
//                 } else {
//                     run.len = truncate_at + ellipsis.len();
//                     runs.truncate(run_index + 1);
//                     break;
//                 }
//             }
//         }
//     }
// }

// /// A fragment of a line that can be wrapped.
// pub enum LineFragment<'a> {
//     /// A text fragment consisting of characters.
//     Text {
//         /// The text content of the fragment.
//         text: &'a str,
//     },
//     /// A non-text element with a fixed width.
//     Element {
//         /// The width of the element in pixels.
//         width: Pixels,
//         /// The UTF-8 encoded length of the element.
//         len_utf8: usize,
//     },
// }

// impl<'a> LineFragment<'a> {
//     /// Creates a new text fragment from the given text.
//     pub fn text(text: &'a str) -> Self {
//         LineFragment::Text { text }
//     }

//     /// Creates a new non-text element with the given width and UTF-8 encoded length.
//     pub fn element(width: Pixels, len_utf8: usize) -> Self {
//         LineFragment::Element { width, len_utf8 }
//     }

//     fn wrap_boundary_candidates(&self) -> impl Iterator<Item = WrapBoundaryCandidate> {
//         let text = match self {
//             LineFragment::Text { text } => text,
//             LineFragment::Element { .. } => "\0",
//         };
//         text.chars().map(move |character| {
//             if let LineFragment::Element { width, len_utf8 } = self {
//                 WrapBoundaryCandidate::Element {
//                     width: *width,
//                     len_utf8: *len_utf8,
//                 }
//             } else {
//                 WrapBoundaryCandidate::Char { character }
//             }
//         })
//     }
// }

// enum WrapBoundaryCandidate {
//     Char { character: char },
//     Element { width: Pixels, len_utf8: usize },
// }

// impl WrapBoundaryCandidate {
//     pub fn len_utf8(&self) -> usize {
//         match self {
//             WrapBoundaryCandidate::Char { character } => character.len_utf8(),
//             WrapBoundaryCandidate::Element { len_utf8: len, .. } => *len,
//         }
//     }
// }

// /// A boundary between two lines of text.
// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub struct Boundary {
//     /// The index of the last character in a line
//     pub ix: usize,
//     /// The indent of the next line.
//     pub next_indent: u32,
// }

// impl Boundary {
//     fn new(ix: usize, next_indent: u32) -> Self {
//         Self { ix, next_indent }
//     }
// }
