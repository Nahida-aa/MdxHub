use std::ops::Range;

use crate::{
    OffsetUtf16,
    Point,
    PointUtf16,
    TextSummary, // Unclipped
};
use heapless::String as ArrayString;

#[cfg(not(all(test, not(rust_analyzer))))]
pub(crate) type Bitmap = u128;
#[cfg(all(test, not(rust_analyzer)))]
pub(crate) type Bitmap = u16;
pub(crate) const MIN_BASE: usize = MAX_BASE / 2;
pub(crate) const MAX_BASE: usize = Bitmap::BITS as usize;

#[derive(Clone, Debug, Default)]
pub struct Chunk {
    /// If bit[i] is set, then the character at index i is the start of a UTF-8 character in the
    /// text.
    chars: Bitmap,
    /// The number of set bits is the number of UTF-16 code units it would take to represent the
    /// text.
    ///
    /// Bit[i] is set if text[i] is the start of a UTF-8 character. If the character would
    /// take two UTF-16 code units, then bit[i+1] is also set. (Rust chars never take more
    /// than two UTF-16 code units.)
    chars_utf16: Bitmap,
    /// If bit[i] is set, then the character at index i is an ascii newline.
    newlines: Bitmap,
    /// If bit[i] is set, then the character at index i is an ascii tab.
    tabs: Bitmap,
    pub text: ArrayString<MAX_BASE, u8>,
}

#[inline(always)]
const fn saturating_shl_mask(offset: u32) -> Bitmap {
    (1 as Bitmap).unbounded_shl(offset).wrapping_sub(1)
}

#[inline(always)]
const fn saturating_shr_mask(offset: u32) -> Bitmap {
    !Bitmap::MAX.unbounded_shr(offset)
}

impl Chunk {
    #[inline(always)]
    pub fn as_slice(&self) -> ChunkSlice<'_> {
        ChunkSlice {
            chars: self.chars,
            chars_utf16: self.chars_utf16,
            newlines: self.newlines,
            tabs: self.tabs,
            text: &self.text,
        }
    }

    #[inline(always)]
    pub fn slice(&self, range: Range<usize>) -> ChunkSlice<'_> {
        self.as_slice().slice(range)
    }

    #[inline(always)]
    pub fn is_char_boundary(&self, offset: usize) -> bool {
        (1 as Bitmap).unbounded_shl(offset as u32) & self.chars != 0 || offset == self.text.len()
    }

    #[track_caller]
    #[inline(always)]
    pub fn assert_char_boundary<const PANIC: bool>(&self, offset: usize) -> bool {
        if self.is_char_boundary(offset) {
            return true;
        }
        if PANIC || cfg!(debug_assertions) {
            panic_char_boundary(&self.text, offset);
        } else {
            log_err_char_boundary(&self.text, offset);
            false
        }
    }
}

#[cold]
#[inline(never)]
#[track_caller]
fn panic_char_boundary(text: &str, offset: usize) -> ! {
    if offset > text.len() {
        panic!(
            "byte index {} is out of bounds of `{:?}` (length: {})",
            offset,
            text,
            text.len()
        );
    }
    // find the character
    let char_start = text.floor_char_boundary(offset);
    // `char_start` must be less than len and a char boundary
    let ch = text.get(char_start..).unwrap().chars().next().unwrap();
    let char_range = char_start..char_start + ch.len_utf8();
    panic!(
        "byte index {} is not a char boundary; it is inside {:?} (bytes {:?})",
        offset, ch, char_range,
    );
}

#[cold]
#[inline(never)]
#[track_caller]
fn log_err_char_boundary(text: &str, offset: usize) {
    if offset >= text.len() {
        log::error!(
            "byte index {} is out of bounds of `{:?}` (length: {})",
            offset,
            text,
            text.len()
        );
        return;
    }
    // find the character
    let char_start = text.floor_char_boundary(offset);
    // `char_start` must be less than len and a char boundary
    let ch = text.get(char_start..).unwrap().chars().next().unwrap();
    let char_range = char_start..char_start + ch.len_utf8();
    log::error!(
        "byte index {} is not a char boundary; it is inside {:?} (bytes {:?})",
        offset,
        ch,
        char_range,
    );
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkSlice<'a> {
    chars: Bitmap,
    chars_utf16: Bitmap,
    newlines: Bitmap,
    tabs: Bitmap,
    text: &'a str,
}
impl<'a> ChunkSlice<'a> {
    #[inline(always)]
    pub fn is_char_boundary(&self, offset: usize) -> bool {
        (1 as Bitmap).unbounded_shl(offset as u32) & self.chars != 0 || offset == self.text.len()
    }

    #[inline(always)]
    pub fn slice(self, mut range: Range<usize>) -> Self {
        if range.start == MAX_BASE {
            Self {
                chars: 0,
                chars_utf16: 0,
                newlines: 0,
                tabs: 0,
                text: "",
            }
        } else {
            if !self.assert_char_boundary::<false>(range.start) {
                range.start = self.text.ceil_char_boundary(range.start);
            }
            if !self.assert_char_boundary::<false>(range.end) {
                range.end = if range.end < range.start {
                    range.start
                } else {
                    self.text.floor_char_boundary(range.end)
                };
            }
            let mask = (1 as Bitmap)
                .unbounded_shl(range.end as u32)
                .wrapping_sub(1);
            Self {
                chars: (self.chars & mask) >> range.start,
                chars_utf16: (self.chars_utf16 & mask) >> range.start,
                newlines: (self.newlines & mask) >> range.start,
                tabs: (self.tabs & mask) >> range.start,
                text: &self.text[range],
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn assert_char_boundary<const PANIC: bool>(&self, offset: usize) -> bool {
        if self.is_char_boundary(offset) {
            return true;
        }
        if PANIC {
            panic_char_boundary(self.text, offset);
        } else {
            log_err_char_boundary(self.text, offset);
            false
        }
    }
    /// Get the longest row in the chunk and its length in characters.
    /// Calculate the total number of characters in the chunk along the way.
    #[inline(always)]
    pub fn longest_row(&self, total_chars: &mut usize) -> (u32, u32) {
        let mut chars = self.chars;
        let mut newlines = self.newlines;
        *total_chars = 0;
        let mut row = 0;
        let mut longest_row = 0;
        let mut longest_row_chars = 0;
        while newlines > 0 {
            let newline_ix = newlines.trailing_zeros();
            let row_chars = (chars & ((1 << newline_ix) - 1)).count_ones() as u8;
            *total_chars += usize::from(row_chars);
            if row_chars > longest_row_chars {
                longest_row = row;
                longest_row_chars = row_chars;
            }

            newlines >>= newline_ix;
            newlines >>= 1;
            chars >>= newline_ix;
            chars >>= 1;
            row += 1;
            *total_chars += 1;
        }

        let row_chars = chars.count_ones() as u8;
        *total_chars += usize::from(row_chars);
        if row_chars > longest_row_chars {
            (row, row_chars as u32)
        } else {
            (longest_row, longest_row_chars as u32)
        }
    }

    #[inline(always)]
    pub fn text_summary(&self) -> TextSummary {
        let mut chars = 0;
        let (longest_row, longest_row_chars) = self.longest_row(&mut chars);
        TextSummary {
            len: self.len(),
            chars,
            len_utf16: self.len_utf16(),
            lines: self.lines(),
            first_line_chars: self.first_line_chars(),
            last_line_chars: self.last_line_chars(),
            last_line_len_utf16: self.last_line_len_utf16(),
            longest_row,
            longest_row_chars,
        }
    }
    /// Get length in bytes
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.text.len()
    }
    /// Get length in UTF-16 code units
    #[inline(always)]
    pub fn len_utf16(&self) -> OffsetUtf16 {
        OffsetUtf16(self.chars_utf16.count_ones() as usize)
    }
    /// Get point representing number of lines and length of last line
    #[inline(always)]
    pub fn lines(&self) -> Point {
        let row = self.newlines.count_ones();
        let column = self.newlines.leading_zeros() - (Bitmap::BITS - self.text.len() as u32);
        Point::new(row, column)
    }
    /// Get number of chars in first line
    #[inline(always)]
    pub fn first_line_chars(&self) -> u32 {
        (self.chars & saturating_shl_mask(self.newlines.trailing_zeros())).count_ones()
    }
    /// Get number of chars in last line
    #[inline(always)]
    pub fn last_line_chars(&self) -> u32 {
        (self.chars & saturating_shr_mask(self.newlines.leading_zeros())).count_ones()
    }

    /// Get number of UTF-16 code units in last line
    #[inline(always)]
    pub fn last_line_len_utf16(&self) -> u32 {
        (self.chars_utf16 & saturating_shr_mask(self.newlines.leading_zeros())).count_ones()
    }

    #[inline(always)]
    pub fn offset_to_point(&self, offset: usize) -> Point {
        let mask = (1 as Bitmap).unbounded_shl(offset as u32).wrapping_sub(1);
        let row = (self.newlines & mask).count_ones();
        let newline_ix = Bitmap::BITS - (self.newlines & mask).leading_zeros();
        let column = (offset - newline_ix as usize) as u32;
        Point::new(row, column)
    }
    #[inline(always)]
    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        let mask = saturating_shl_mask(offset as u32);
        let row = (self.newlines & saturating_shl_mask(offset as u32)).count_ones();
        let newline_ix = Bitmap::BITS - (self.newlines & mask).leading_zeros();
        let column = if newline_ix as usize == MAX_BASE {
            0
        } else {
            ((self.chars_utf16 & mask) >> newline_ix).count_ones()
        };
        PointUtf16::new(row, column)
    }
}
