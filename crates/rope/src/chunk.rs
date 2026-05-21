use crate::{
    OffsetUtf16,
    Point,
    // PointUtf16,
    TextSummary,
    // Unclipped
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
}
