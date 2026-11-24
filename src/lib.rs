#![allow(unused_variables, unused_imports, unused_mut, non_snake_case)]

pub mod analysis;
#[cfg(feature = "bam")]
#[allow(
    clippy::byte_char_slices,
    clippy::needless_borrow,
    clippy::too_many_arguments
)]
pub mod bam_sort;
#[allow(
    clippy::byte_char_slices,
    clippy::double_ended_iterator_last,
    clippy::match_like_matches_macro,
    clippy::needless_splitn,
    clippy::too_many_arguments,
    clippy::writeln_empty_string
)]
pub mod commands;
pub mod constants;
pub mod count_checkpoint;
pub mod count_output;
#[cfg(all(test, feature = "bam"))]
mod count_regression;
// shortcut: these allowlisted Clippy lints remain in inherited modules; remove each allowance when that module gets a focused, tested cleanup.
#[allow(
    clippy::collapsible_if,
    clippy::for_kv_map,
    clippy::items_after_test_module,
    clippy::lines_filter_map_ok,
    clippy::manual_is_multiple_of,
    clippy::manual_strip,
    clippy::needless_splitn,
    clippy::ptr_arg,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_map_or
)]
pub mod counter;
#[allow(
    clippy::assign_op_pattern,
    clippy::needless_range_loop,
    clippy::useless_conversion
)]
pub mod diffusion;
pub mod estimation;
#[allow(clippy::len_without_is_empty)]
pub mod feature;
pub mod gene_info;
#[allow(clippy::unused_enumerate_index)]
pub mod indexes;
pub mod logic;
#[allow(
    clippy::lines_filter_map_ok,
    clippy::manual_split_once,
    clippy::needless_splitn,
    clippy::unnecessary_unwrap,
    clippy::useless_conversion
)]
pub mod metadata;
pub mod molitem;
#[allow(clippy::op_ref)]
pub mod neighbors;
#[allow(clippy::too_many_arguments)]
pub mod read;
pub mod segment_match;
#[allow(clippy::manual_strip)]
pub mod serialization;
#[allow(clippy::needless_range_loop)]
pub mod speedboosted;
pub(crate) mod telemetry;
pub mod transcript_model;
#[allow(clippy::while_let_loop)]
pub mod utils;
