use crate::{Opstamp, SegmentId};

/// A `MergeOperation` has two roles.
/// It carries all of the information required to describe a merge:
/// - `target_opstamp` is the opstamp up to which we want to consume the
/// delete queue and reflect their deletes.
/// - `segment_ids` is the list of segment to be merged.
///
/// The second role is to ensure keep track of the fact that these
/// segments are in merge and avoid starting a merge operation that
/// may conflict with this one.
///
/// This works by tracking merge operations. When considering computing
/// merge candidates, we simply list tracked merge operations and remove
/// their segments from possible merge candidates.
pub struct MergeOperation {
    target_opstamp: Opstamp,
    segment_ids: Vec<SegmentId>,
}

impl MergeOperation {
    pub(crate) fn new(target_opstamp: Opstamp, segment_ids: Vec<SegmentId>) -> MergeOperation {
        MergeOperation {
            target_opstamp,
            segment_ids,
        }
    }

    pub fn target_opstamp(&self) -> Opstamp {
        self.target_opstamp
    }

    pub fn segment_ids(&self) -> &[SegmentId] {
        &self.segment_ids[..]
    }
}
