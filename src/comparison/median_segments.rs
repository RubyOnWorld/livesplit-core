//! Defines the Comparison Generator for calculating the Median Segments of a
//! Run. The Median Segments are calculated through a weighted median that gives
//! more recent segments a larger weight so that the Median Segments are more
//! suited to represent the current performance of a runner.

use super::ComparisonGenerator;
use crate::{platform::prelude::*, Attempt, Segment, TimeSpan, TimingMethod};

/// The Comparison Generator for calculating the Median Segments of a Run. The
/// Median Segments are calculated through a weighted median that gives more
/// recent segments a larger weight so that the Median Segments are more suited
/// to represent the current performance of a runner.
#[derive(Copy, Clone, Debug)]
pub struct MedianSegments;

/// The short name of this comparison. Suitable for situations where not a lot
/// of space for text is available.
pub const SHORT_NAME: &str = "Median";
/// The name of this comparison.
pub const NAME: &str = "Median Segments";

const WEIGHT: f64 = 0.75;

fn generate(segments: &mut [Segment], medians: &mut Vec<(f64, TimeSpan)>, method: TimingMethod) {
    let mut accumulated = Some(TimeSpan::zero());

    let mut previous_segment: Option<&Segment> = None;
    for segment in segments {
        if let Some(accumulated_val) = &mut accumulated {
            medians.clear();

            let mut current_weight = 1.0;

            for &(id, time) in segment.segment_history().iter_actual_runs().rev() {
                if let Some(time) = time[method] {
                    // Skip all the combined segments
                    let skip = catch! {
                        previous_segment?.segment_history().get(id)?[method].is_none()
                    }
                    .unwrap_or(false);

                    if !skip {
                        medians.push((current_weight, time));
                        current_weight *= WEIGHT;
                    }
                }
            }

            if medians.is_empty() {
                accumulated = None;
            } else {
                medians.sort_unstable_by_key(|&(_, time)| time);
                let mut total_weights = 0.0;
                for (weight, _) in medians.iter_mut() {
                    *weight += total_weights;
                    total_weights = *weight;
                }
                let median_weight = 0.5 * total_weights;
                let found_index = medians.partition_point(|&(weight, _)| weight < median_weight);
                *accumulated_val += medians[found_index].1;
            }
        }
        segment.comparison_mut(NAME)[method] = accumulated;
        previous_segment = Some(&*segment);
    }
}

impl ComparisonGenerator for MedianSegments {
    fn name(&self) -> &str {
        NAME
    }

    fn generate(&mut self, segments: &mut [Segment], _: &[Attempt]) {
        let medians = &mut Vec::new();
        generate(segments, medians, TimingMethod::RealTime);
        generate(segments, medians, TimingMethod::GameTime);
    }
}
