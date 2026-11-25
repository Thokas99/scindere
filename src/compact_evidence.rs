use crate::{feature::Feature, molitem::Molitem, segment_match::SegmentMatch};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct CompactSegmentMatch {
    pub(crate) transcript_idx: usize,
    pub(crate) segment: (i64, i64),
    pub(crate) feature_idx: usize,
    pub(crate) is_spliced: bool,
    pub(crate) feature: Feature,
}

#[derive(Default)]
pub(crate) struct CompactMolitem {
    mappings: Option<Vec<CompactSegmentMatch>>,
}

impl CompactMolitem {
    /// Intersect sorted transcript indices and append segment evidence for matches.
    pub(crate) fn add_mapping(&mut self, mapping: Vec<CompactSegmentMatch>) {
        debug_assert!(mapping
            .windows(2)
            .all(|pair| pair[0].transcript_idx <= pair[1].transcript_idx));
        let Some(existing) = self.mappings.take() else {
            self.mappings = Some(mapping);
            return;
        };
        debug_assert!(existing
            .windows(2)
            .all(|pair| pair[0].transcript_idx <= pair[1].transcript_idx));
        let mut existing = existing.into_iter();
        let mut incoming = mapping.into_iter();
        let mut old = existing.next();
        let mut new = incoming.next();
        let mut merged = Vec::new();
        // The inputs are owned, so retained evidence moves without a per-molecule map rebuild.
        while let (Some(old_match), Some(new_match)) = (old.as_ref(), new.as_ref()) {
            let old_idx = old_match.transcript_idx;
            let new_idx = new_match.transcript_idx;
            match old_idx.cmp(&new_idx) {
                std::cmp::Ordering::Less => {
                    while old.as_ref().is_some_and(|m| m.transcript_idx == old_idx) {
                        old = existing.next();
                    }
                }
                std::cmp::Ordering::Greater => {
                    while new.as_ref().is_some_and(|m| m.transcript_idx == new_idx) {
                        new = incoming.next();
                    }
                }
                std::cmp::Ordering::Equal => {
                    while old.as_ref().is_some_and(|m| m.transcript_idx == old_idx) {
                        merged.push(old.take().unwrap());
                        old = existing.next();
                    }
                    while new.as_ref().is_some_and(|m| m.transcript_idx == new_idx) {
                        merged.push(new.take().unwrap());
                        new = incoming.next();
                    }
                }
            }
        }
        self.mappings = Some(merged);
    }

    pub(crate) fn materialize(&self) -> Molitem {
        let mappings_record = self.mappings.as_ref().map(|segments| {
            let mut mappings = HashMap::new();
            for segment in segments {
                mappings
                    .entry(segment.transcript_idx)
                    .or_insert_with(Vec::new)
                    .push(SegmentMatch {
                        segment: segment.segment,
                        feature_idx: segment.feature_idx,
                        is_spliced: segment.is_spliced,
                        feature: segment.feature.clone(),
                    });
            }
            mappings
        });
        Molitem { mappings_record }
    }
}

#[cfg(test)]
mod tests {
    use super::{CompactMolitem, CompactSegmentMatch};
    use crate::feature::Feature;
    use std::collections::HashMap;

    fn segment(
        transcript_idx: usize,
        feature_idx: usize,
        coords: (i64, i64),
    ) -> CompactSegmentMatch {
        CompactSegmentMatch {
            transcript_idx,
            segment: coords,
            feature_idx,
            is_spliced: feature_idx % 2 == 1,
            feature: Feature {
                start: coords.0 - 2,
                end: coords.1 + 2,
                kind: if feature_idx % 2 == 0 { b'e' } else { b'i' },
                exin_no: transcript_idx as i64,
                is_validated: transcript_idx % 2 == 0,
                transcript_model_idx: Some(transcript_idx),
            },
        }
    }

    #[test]
    fn add_mapping_intersects_sorted_transcripts_and_concatenates_segments() {
        let mut molecule = CompactMolitem::default();
        molecule.add_mapping(vec![segment(2, 0, (20, 29)), segment(4, 0, (40, 49))]);
        molecule.add_mapping(vec![
            segment(1, 0, (10, 19)),
            segment(2, 1, (21, 30)),
            segment(4, 1, (41, 50)),
            segment(7, 0, (70, 79)),
        ]);

        let mappings = molecule.materialize().mappings_record.unwrap();
        assert_eq!(mappings.len(), 2);
        assert_eq!(
            mappings[&2].iter().map(|m| m.segment).collect::<Vec<_>>(),
            [(20, 29), (21, 30)]
        );
        assert_eq!(
            mappings[&4].iter().map(|m| m.segment).collect::<Vec<_>>(),
            [(40, 49), (41, 50)]
        );
    }

    #[test]
    fn add_mapping_drops_transcripts_missing_from_the_next_read() {
        let mut molecule = CompactMolitem::default();
        molecule.add_mapping(vec![segment(2, 0, (20, 29)), segment(3, 0, (30, 39))]);
        molecule.add_mapping(vec![segment(3, 1, (31, 40))]);

        let mappings = molecule.materialize().mappings_record.unwrap();
        assert_eq!(mappings.len(), 1);
        assert!(mappings.contains_key(&3));
        assert!(!mappings.contains_key(&2));
    }

    #[test]
    fn empty_mapping_stays_empty_after_later_reads() {
        let mut molecule = CompactMolitem::default();
        molecule.add_mapping(vec![segment(2, 0, (20, 29))]);
        molecule.add_mapping(Vec::new());
        molecule.add_mapping(vec![segment(2, 1, (21, 30))]);

        assert!(molecule.materialize().mappings_record.unwrap().is_empty());
    }

    #[test]
    fn materialize_preserves_segment_and_feature_values() {
        let mut molecule = CompactMolitem::default();
        molecule.add_mapping(vec![segment(6, 3, (60, 69))]);

        let mappings: HashMap<_, _> = molecule.materialize().mappings_record.unwrap();
        let matched = &mappings[&6][0];
        assert_eq!(matched.segment, (60, 69));
        assert_eq!(matched.feature_idx, 3);
        assert!(matched.is_spliced);
        assert_eq!(matched.feature.start, 58);
        assert_eq!(matched.feature.end, 71);
        assert_eq!(matched.feature.kind, b'i');
        assert_eq!(matched.feature.exin_no, 6);
        assert!(matched.feature.is_validated);
        assert_eq!(matched.feature.transcript_model_idx, Some(6));
    }
}
