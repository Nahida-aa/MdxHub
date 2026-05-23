mod offset_utf16;
mod point;
use std::{cmp, ops};

pub use offset_utf16::OffsetUtf16;
mod chunk;
pub use chunk::{
    Chunk,
    // ChunkSlice
};
use sum_tree::{
    // Bias, Dimension, Dimensions,
    Bias,
    Dimension,
    SumTree,
};

pub use point::Point;

use crate::chunk::ChunkSlice;

#[derive(Clone, Default)]
pub struct Rope {
    chunks: SumTree<Chunk>,
}
impl Rope {
    pub fn len(&self) -> usize {
        self.chunks.extent(())
    }
    #[track_caller]
    #[inline(always)]
    pub fn assert_char_boundary<const PANIC: bool>(&self, offset: usize) -> bool {
        if self.chunks.is_empty() && offset == 0 {
            return true;
        }
        let (start, _, item) = self.chunks.find::<usize, _>((), &offset, Bias::Left);
        match item {
            Some(chunk) => {
                let chunk_offset = offset - start;
                chunk.assert_char_boundary::<PANIC>(chunk_offset)
            }
            None if PANIC => {
                panic!(
                    "byte index {} is out of bounds of rope (length: {})",
                    offset,
                    self.len()
                );
            }
            None => {
                log::error!(
                    "byte index {} is out of bounds of rope (length: {})",
                    offset,
                    self.len()
                );
                false
            }
        }
    }
    pub fn floor_char_boundary(&self, index: usize) -> usize {
        if index >= self.len() {
            self.len()
        } else {
            let (start, _, item) = self.chunks.find::<usize, _>((), &index, Bias::Left);
            let chunk_offset = index - start;
            let lower_idx = item.map(|chunk| chunk.text.floor_char_boundary(chunk_offset));
            lower_idx.map_or_else(|| self.len(), |idx| start + idx)
        }
    }
    pub fn ceil_char_boundary(&self, index: usize) -> usize {
        if index > self.len() {
            self.len()
        } else {
            let (start, _, item) = self.chunks.find::<usize, _>((), &index, Bias::Left);
            let chunk_offset = index - start;
            let upper_idx = item.map(|chunk| chunk.text.ceil_char_boundary(chunk_offset));
            upper_idx.map_or_else(|| self.len(), |idx| start + idx)
        }
    }

    pub fn cursor(&self, offset: usize) -> Cursor<'_> {
        Cursor::new(self, offset)
    }
}

impl sum_tree::Item for Chunk {
    type Summary = ChunkSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        ChunkSummary {
            text: self.as_slice().text_summary(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChunkSummary {
    text: TextSummary,
}
impl sum_tree::ContextLessSummary for ChunkSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.text += &summary.text;
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for usize {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += summary.text.len;
    }
}

/// Summary of a string of text.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    /// Length in bytes.
    pub len: usize,
    /// Length in UTF-8.
    pub chars: usize,
    /// Length in UTF-16 code units
    pub len_utf16: OffsetUtf16,
    /// A point representing the number of lines and the length of the last line.
    ///
    /// In other words, it marks the point after the last byte in the text, (if
    /// EOF was a character, this would be its position).
    pub lines: Point,
    /// How many `char`s are in the first line
    pub first_line_chars: u32,
    /// How many `char`s are in the last line
    pub last_line_chars: u32,
    /// How many UTF-16 code units are in the last line
    pub last_line_len_utf16: u32,
    /// The row idx of the longest row
    pub longest_row: u32,
    /// How many `char`s are in the longest row
    pub longest_row_chars: u32,
}
impl<'a> ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        let joined_chars = self.last_line_chars + other.first_line_chars;
        if joined_chars > self.longest_row_chars {
            self.longest_row = self.lines.row;
            self.longest_row_chars = joined_chars;
        }
        if other.longest_row_chars > self.longest_row_chars {
            self.longest_row = self.lines.row + other.longest_row;
            self.longest_row_chars = other.longest_row_chars;
        }

        if self.lines.row == 0 {
            self.first_line_chars += other.first_line_chars;
        }

        if other.lines.row == 0 {
            self.last_line_chars += other.first_line_chars;
            self.last_line_len_utf16 += other.last_line_len_utf16;
        } else {
            self.last_line_chars = other.last_line_chars;
            self.last_line_len_utf16 = other.last_line_len_utf16;
        }

        self.chars += other.chars;
        self.len += other.len;
        self.len_utf16 += other.len_utf16;
        self.lines += other.lines;
    }
}

pub trait TextDimension:
    'static + Clone + Copy + Default + for<'a> Dimension<'a, ChunkSummary> + std::fmt::Debug
{
    fn from_text_summary(summary: &TextSummary) -> Self;
    fn from_chunk(chunk: ChunkSlice) -> Self;
    fn add_assign(&mut self, other: &Self);
}

pub struct Cursor<'a> {
    rope: &'a Rope,
    chunks: sum_tree::Cursor<'a, 'static, Chunk, usize>,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(rope: &'a Rope, offset: usize) -> Self {
        let mut chunks = rope.chunks.cursor(());
        chunks.seek(&offset, Bias::Right);
        Self {
            rope,
            chunks,
            offset,
        }
    }

    pub fn summary<D: TextDimension>(&mut self, end_offset: usize) -> D {
        assert!(
            end_offset >= self.offset,
            "cannot summarize backward from {} to {}",
            self.offset,
            end_offset
        );
        assert!(
            end_offset <= self.rope.len(),
            "cannot summarize past end of rope"
        );

        let mut summary = D::zero(());
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end()) - self.chunks.start();
            summary.add_assign(&D::from_chunk(start_chunk.slice(start_ix..end_ix)));
        }

        if end_offset > self.chunks.end() {
            self.chunks.next();
            summary.add_assign(&self.chunks.summary(&end_offset, Bias::Right));
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                summary.add_assign(&D::from_chunk(end_chunk.slice(0..end_ix)));
            }
        }

        self.offset = end_offset;
        summary
    }
}
