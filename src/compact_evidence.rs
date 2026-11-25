use crate::{molitem::Molitem, segment_match::SegmentMatch};
use std::collections::HashMap;

pub(crate) type CompactSegmentMatch = SegmentMatch;

#[derive(Default)]
pub(crate) struct CompactMolitem {
    mappings: Option<Vec<CompactSegmentMatch>>,
    transcript_indices: Vec<usize>,
}

impl CompactMolitem {
    /// Intersect sorted transcript indices and append evidence in read order.
    pub(crate) fn add_mapping(&mut self, mut mapping: Vec<CompactSegmentMatch>) {
        debug_assert!(mapping
            .windows(2)
            .all(|pair| transcript_idx(&pair[0]) <= transcript_idx(&pair[1])));
        let mut incoming_indices = Vec::new();
        for matched in &mapping {
            let index = transcript_idx(matched);
            if incoming_indices.last() != Some(&index) {
                incoming_indices.push(index);
            }
        }
        let Some(mut existing) = self.mappings.take() else {
            self.transcript_indices = incoming_indices;
            self.mappings = Some(mapping);
            return;
        };
        let mut retained_indices = Vec::new();
        let (mut old, mut new) = (0, 0);
        while old < self.transcript_indices.len() && new < incoming_indices.len() {
            match self.transcript_indices[old].cmp(&incoming_indices[new]) {
                std::cmp::Ordering::Less => old += 1,
                std::cmp::Ordering::Greater => new += 1,
                std::cmp::Ordering::Equal => {
                    retained_indices.push(self.transcript_indices[old]);
                    old += 1;
                    new += 1;
                }
            }
        }
        if retained_indices.is_empty() {
            existing = Vec::new();
        } else if retained_indices != self.transcript_indices {
            existing.retain(|matched| {
                retained_indices
                    .binary_search(&transcript_idx(matched))
                    .is_ok()
            });
        }
        if retained_indices != incoming_indices {
            mapping.retain(|matched| {
                retained_indices
                    .binary_search(&transcript_idx(matched))
                    .is_ok()
            });
        }
        existing.extend(mapping);
        self.transcript_indices = retained_indices;
        self.mappings = Some(existing);
    }

    pub(crate) fn materialize(self) -> Molitem {
        let Some(segments) = self.mappings else {
            return Molitem {
                mappings_record: None,
            };
        };
        if self.transcript_indices.len() == 1 {
            return Molitem {
                mappings_record: Some(HashMap::from([(self.transcript_indices[0], segments)])),
            };
        }
        let mut mappings = HashMap::new();
        for segment in segments {
            mappings
                .entry(transcript_idx(&segment))
                .or_insert_with(Vec::new)
                .push(segment);
        }
        Molitem {
            mappings_record: Some(mappings),
        }
    }
}

fn transcript_idx(matched: &CompactSegmentMatch) -> usize {
    matched
        .feature
        .transcript_model_idx
        .expect("compact evidence only contains transcript features")
}

#[cfg(test)]
mod tests {
    use super::{CompactMolitem, CompactSegmentMatch};
    use crate::{feature::Feature, segment_match::SegmentMatch};
    use std::collections::HashMap;

    fn segment(
        transcript_idx: usize,
        feature_idx: usize,
        coords: (i64, i64),
    ) -> CompactSegmentMatch {
        SegmentMatch::new(
            coords,
            feature_idx,
            !feature_idx.is_multiple_of(2),
            Feature {
                start: coords.0 - 2,
                end: coords.1 + 2,
                kind: if feature_idx.is_multiple_of(2) {
                    b'e'
                } else {
                    b'i'
                },
                exin_no: transcript_idx as i64,
                is_validated: transcript_idx.is_multiple_of(2),
                transcript_model_idx: Some(transcript_idx),
            },
        )
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
    fn disjoint_mapping_releases_unusable_evidence_capacity() {
        let mut molecule = CompactMolitem::default();
        molecule.add_mapping(vec![
            segment(2, 0, (20, 29)),
            segment(2, 1, (30, 39)),
            segment(2, 2, (40, 49)),
        ]);
        assert!(molecule.mappings.as_ref().unwrap().capacity() >= 3);

        molecule.add_mapping(vec![segment(3, 0, (50, 59))]);
        assert!(molecule.transcript_indices.is_empty());
        assert_eq!(molecule.mappings.as_ref().unwrap().capacity(), 0);

        molecule.add_mapping(vec![segment(2, 3, (60, 69))]);
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
