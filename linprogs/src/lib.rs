//! # Fundament time series library
//!
//! Time series library to build and evaluate time series expressions.
//!
//! ## Overview
//!
//! Fun TS splits time series into three main parts:
//!
//! - [Time axes] which represents a strictly monotonically increasing sequence of time,
//! - [evaluables] which represents a sequence of points, and
//! - an [interpolation degree] which represent how values between point values are interpreted.
//!
//! A related time axis, array of values, and an interpolation degree can be combined into a
//! [`TimeSeries`]. Time series implements a system for lazily evaluated compute expressions
//! through the [`Evaluable`] trait.
//!
//! [Time axes]: time_axis
//! [interpolation degree]: Interpolation
//!

// warn for missing documentation on public items when building documentation
#![cfg_attr(doc, warn(missing_docs))]
#![deny(unsafe_code)]
#![warn(
    future_incompatible,
    let_underscore,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    nonstandard_style,
    unused,
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::checked_conversions,
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_underscore_must_use,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatching_type_param_order,
    clippy::mut_mut,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_pub_crate,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::self_named_module_files,
    clippy::semicolon_if_nothing_returned,
    clippy::shadow_unrelated,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unseparated_literal_suffix,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies,
    clippy::zero_sized_map_values
)]

pub mod matrix;
pub mod solvers;
mod utils;
