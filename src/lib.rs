//! # API Response Library
//!
//! This library provides a consistent structure for API responses, including
//! success and error handling, with support for various serialization formats
//! like JSON and Protobuf.
//!
//! ## Modules
//!
//! * `meta`: Contains the meta structures.
//! * `success`: Contains the success response structures.
//! * `error`: Contains the error handling structures.

// -------- rust coding guidelines: https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/ --------
// -------- rustc lint doc: https://doc.rust-lang.org/rustc/lints/listing/index.html --------
// -------- rust-clippy doc: https://rust-lang.github.io/rust-clippy/master/index.html --------

// [REQUIRED] G.VAR.02 Do not use non-ASCII characters in identifiers
#![deny(non_ascii_idents)]
// [REQUIRED]
#![allow(clippy::disallowed_names)]
// [REQUIRED]
#![allow(clippy::blanket_clippy_restriction_lints)]
// [REQUIRED] G.CMT.02 Add Panic documentation in the docs of public APIs that may panic under certain circumstances
#![warn(clippy::missing_panics_doc)]
// [RECOMMENDED] G.VAR.03 Variable shadowing should be used carefully
#![warn(clippy::shadow_reuse, clippy::shadow_same, clippy::shadow_unrelated)]
// [RECOMMENDED] G.CNS.05 Use const fn for functions or methods wherever applicable
#![warn(clippy::missing_const_for_fn)]
// [REQUIRED] G.TYP.01 Prefer safe conversion functions over `as` for type casting
#![warn(
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::ptr_as_ptr
)]
// [RECOMMENDED] G.VAR.01 Avoid using too many meaningless variable names when destructuring tuples with more than four
// variables
#![warn(clippy::many_single_char_names)]
// [RECOMMENDED] G.TYP.02 Explicitly specify the type for numeric literals
#![warn(clippy::default_numeric_fallback)]
// [RECOMMENDED] G.TYP.03 Use `try_from` methods instead of relying on numeric boundaries for safe conversion
#![warn(clippy::checked_conversions)]
// [RECOMMENDED] G.TYP.BOL.02 Use `if` expressions instead of `match` for boolean conditions
#![warn(clippy::match_bool)]
// [RECOMMENDED] G.TYP.BOL.05 Use logical operators (&&/||) instead of bitwise operators (&/|) for boolean operations
// when not necessary
#![warn(clippy::needless_bitwise_bool)]
// [REQUIRED] G.TYP.INT.01 Consider the risks of integer overflow, wrapping, and truncation in integer arithmetic
#![warn(clippy::arithmetic_side_effects)]
// [REQUIRED] G.TYP.INT.02 Avoid `as` casting between signed and unsigned integers; use safe conversion functions
#![deny(clippy::cast_sign_loss)]
// [REQUIRED] G.TYP.INT.03 Avoid using `%` for modulo operations on negative numbers
#![warn(clippy::modulo_arithmetic)]
// [REQUIRED] G.TYP.FLT.02 Avoid precision loss when casting from any numeric type to floating-point; use safe
// conversion functions
#![warn(clippy::cast_precision_loss)]
// [REQUIRED] G.TYP.FLT.03 Be cautious of precision loss in floating-point arithmetic and comparisons
#![warn(clippy::float_arithmetic, clippy::float_cmp, clippy::float_cmp_const)]
// [REQUIRED] G.TYP.FLT.04 Use Rust's built-in methods for floating-point calculations
#![warn(clippy::imprecise_flops, clippy::suboptimal_flops)]
// [OPTIONAL] G.TYP.ARR.01 Use static variables instead of constants for large global arrays
#![warn(clippy::large_stack_arrays)]
// [RECOMMENDED] G.TYP.SCT.01 Add `#[non_exhaustive]` attribute to publicly exported structs
#![warn(clippy::exhaustive_structs)]
// [RECOMMENDED] G.TYP.ENM.05 Add `#[non_exhaustive]` attribute to publicly exported enums
#![warn(clippy::exhaustive_enums)]
// [RECOMMENDED] G.TYP.SCT.02 Consider refactoring when a struct contains more than three boolean fields
#![warn(clippy::struct_excessive_bools)]
// [RECOMMENDED] G.FUD.03 Consider using a custom struct or enum instead of many boolean parameters in function
// signatures
#![warn(clippy::fn_params_excessive_bools)]
// [RECOMMENDED] G.TYP.ENM.04 Avoid using glob imports for enum variants in `use` statements
#![warn(clippy::enum_glob_use)]
// [RECOMMENDED] G.CTF.02 Ensure `else` branches are present whenever `else if` is used
#![warn(clippy::else_if_without_else)]
// [RECOMMENDED] G.STR.02 Use `push_str` method for appending strings
#![warn(clippy::string_add_assign, clippy::string_add)]
// [RECOMMENDED] G.STR.03 Convert string literals containing only ASCII characters to byte sequences using `b"str"`
// syntax instead of `as_bytes()`
#![warn(clippy::string_lit_as_bytes)]
// [RECOMMENDED] G.STR.05 Take care to avoid disrupting UTF-8 encoding when slicing strings at specific positions
#![warn(clippy::string_slice)]
// [RECOMMENDED] G.FUD.02 Prefer passing large values by reference if function parameters implement `Copy`
#![warn(clippy::large_types_passed_by_value)]
// [RECOMMENDED] G.FUD.04 Pass small `Copy` type values by value instead of by reference
#![warn(clippy::trivially_copy_pass_by_ref)]
// [REQUIRED] G.GEN.02 Be cautious to avoid using generic default implementations of some methods from Rust's standard
// library; prefer specific type implementations
#![warn(clippy::inefficient_to_string)]
// [RECOMMENDED] G.TRA.BLN.01 Prefer using the concrete type's `default()` method over calling `Default::default()`
#![warn(clippy::default_trait_access)]
// [REQUIRED] G.TRA.BLN.02 Do not implement the `Copy` trait for iterators
#![warn(clippy::copy_iterator)]
// [RECOMMENDED] G.TRA.BLN.07 Use `copied` method instead of `cloned` for iterable `Copy` types
#![warn(clippy::cloned_instead_of_copied)]
// [RECOMMENDED] G.ERR.01 Avoid using `unwrap` indiscriminately when handling `Option<T>` and `Result<T, E>`
#![warn(clippy::unwrap_used)]
// [RECOMMENDED] G.MOD.03 Avoid using wildcard imports in module declarations
#![warn(clippy::wildcard_imports)]
// [REQUIRED] G.MOD.04 Avoid using different module layout styles within the same project
#![warn(clippy::self_named_module_files)]
// [RECOMMENDED] G.CAR.02 Ensure that necessary metadata is included in the `Cargo.toml` of the crate
#![warn(clippy::cargo_common_metadata)]
// [RECOMMENDED] G.CAR.03 Avoid negative or redundant prefixes and suffixes in feature names
#![warn(clippy::negative_feature_names, clippy::redundant_feature_names)]
// [REQUIRED] G.CAR.04 Avoid using wildcard dependencies in `Cargo.toml`
#![warn(clippy::wildcard_dependencies)]
// [RECOMMENDED] G.MAC.01 Only use the `dbg!()` macro for debugging code
#![warn(clippy::dbg_macro)]
// [REQUIRED] Ensure that locks are released before `await` is called in asynchronous code
#![warn(clippy::await_holding_lock)]
// [REQUIRED] Handle `RefCell` references across `await` points
#![warn(clippy::await_holding_refcell_ref)]
// [RECOMMENDED] G.ASY.04 Avoid defining unnecessary async functions
#![warn(clippy::unused_async)]
// [REQUIRED] G.UNS.SAS.02 Use `assert!` instead of `debug_assert!` to verify boundary conditions in unsafe functions
#![warn(clippy::debug_assert_with_mut_call)]
#![cfg_attr(feature = "try", feature(try_trait_v2))]

#[cfg(feature = "try")]
mod try_trait;

#[cfg(feature = "salvo")]
mod salvo_trait;

mod error;
pub mod error_code;
mod meta;
mod result;
mod success;
mod utils;

use std::{error::Error, fmt::Debug};

pub use prelude::*;
pub mod prelude {
    pub use serde::{Deserialize, Serialize, de::DeserializeOwned};

    pub use crate::{
        ApiResponse, api_err,
        error::{ApiError, ErrorResponse},
        error_code,
        meta::{DefaultMeta, Links},
        result::ApiResult,
        success::{ApiSuccessResponse, SuccessResponse},
        utils::{ErrWrapper, IntoError, MaybeString},
    };
}

/// Enum to represent the overall API response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
#[allow(clippy::exhaustive_enums)]
pub enum ApiResponse<Data, Meta> {
    Success(SuccessResponse<Data, Meta>),
    Error(ErrorResponse<Meta>),
}

impl<Data, Meta> From<SuccessResponse<Data, Meta>> for ApiResponse<Data, Meta> {
    fn from(value: SuccessResponse<Data, Meta>) -> Self {
        Self::Success(value)
    }
}

impl<Data, Meta> From<ErrorResponse<Meta>> for ApiResponse<Data, Meta> {
    fn from(value: ErrorResponse<Meta>) -> Self {
        Self::Error(value)
    }
}

impl<Data, Meta> From<ApiError> for ApiResponse<Data, Meta> {
    fn from(error: ApiError) -> Self {
        ApiResponse::Error(ErrorResponse::from_error(error))
    }
}

impl<Data, Meta> ApiResponse<Data, Meta> {
    #[inline(always)]
    pub const fn new_success(data: Data, meta: Meta) -> Self {
        Self::Success(SuccessResponse::new(data, meta))
    }
    #[inline(always)]
    pub const fn from_success(data: Data) -> Self {
        Self::Success(SuccessResponse::from_data(data))
    }
    #[inline(always)]
    pub const fn from_success_data(data: Data) -> Self {
        Self::Success(SuccessResponse::from_data(data))
    }
    #[inline(always)]
    pub const fn new_error(error: ApiError, meta: Meta) -> Self {
        Self::Error(ErrorResponse::new(error, meta))
    }
    #[inline(always)]
    pub const fn from_error(error: ApiError) -> Self {
        Self::Error(ErrorResponse::from_error(error))
    }
    #[inline(always)]
    pub fn from_error_msg(code: impl Into<i32>, message: impl Into<String>) -> Self {
        Self::Error(ErrorResponse::from_error_msg(code, message))
    }
    #[inline(always)]
    pub fn from_error_source(
        code: impl Into<i32>,
        source: impl Error + Send + Sync + 'static,
        set_source_detail: bool,
        message: Option<String>,
    ) -> Self {
        Self::Error(ErrorResponse::from_error_source(
            code,
            source,
            set_source_detail,
            message,
        ))
    }
    #[inline(always)]
    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.set_meta(meta);
        self
    }
    #[inline(always)]
    pub fn set_meta(&mut self, meta: Meta) -> &mut Self {
        match self {
            ApiResponse::Success(success_response) => {
                success_response.set_meta(meta);
            }
            ApiResponse::Error(error_response) => {
                error_response.set_meta(meta);
            }
        }
        self
    }
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
    pub const fn get_meta(&self) -> Option<&Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response.meta.as_ref(),
            ApiResponse::Error(error_response) => error_response.meta.as_ref(),
        }
    }
    /// # Panics
    /// If it is `ApiResponse::Error`, trigger a panic using the `expect_msg`
    /// parameter.
    pub fn expect(self, expect_msg: &str) -> SuccessResponse<Data, Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response,
            ApiResponse::Error(_) => {
                panic!("{expect_msg}")
            }
        }
    }
    /// # Panics
    /// If it is `ApiResponse::Error`, trigger a panic.
    pub fn unwrap(self) -> SuccessResponse<Data, Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response,
            ApiResponse::Error(_) => {
                panic!("called `ApiResponse::unwrap()` on an `ApiResponse::Error` value")
            }
        }
    }
    /// # Panics
    /// If it is `ApiResponse::Success`, trigger a panic.
    pub fn unwrap_err(self) -> ErrorResponse<Meta> {
        match self {
            ApiResponse::Success(_) => {
                panic!("called `ApiResponse::unwrap_err()` on an `ApiResponse::Success` value")
            }
            ApiResponse::Error(error_response) => error_response,
        }
    }
    pub fn into_result(self) -> ApiResult<Data, Meta> {
        self.into()
    }
    pub fn into_result_data(self) -> Result<Data, ErrorResponse<Meta>> {
        match self {
            ApiResponse::Success(success_response) => Ok(success_response.data),
            ApiResponse::Error(error_response) => Err(error_response),
        }
    }
    pub fn into_result_without_meta(self) -> Result<Data, ApiError> {
        match self {
            ApiResponse::Success(success_response) => Ok(success_response.data),
            ApiResponse::Error(error_response) => Err(error_response.error),
        }
    }
}
