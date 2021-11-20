use std::ops::Range;

/// Return true if two ranges overlap
///
///     assert_eq!(ranges::overlap(0..7, 3..10), true);
///     assert_eq!(ranges::overlap(1..5, 101..105), false);
///
/// If either range is empty, they don't count as overlapping.
///
///     assert_eq!(ranges::overlap(0..0, 0..10), false);
pub fn overlap(a: Range<usize>, b: Range<usize>) -> bool {
    a.start < a.end && b.start < b.end && a.start < b.end && b.start < a.end
}
