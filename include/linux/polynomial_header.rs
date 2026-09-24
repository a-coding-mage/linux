/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
 */
//! Safe bounded polynomial descriptors and the shared native-word algorithm.
//!
//! The Rust descriptor borrows a term slice; it does not pretend that a C
//! inline flexible array is a pointer field. Native C callers require the
//! original header's generated bindings and a separate ABI owner.

#[path = "../../lib/math/polynomial.rs"]
mod implementation;

pub use implementation::{
    polynomial_calc, polynomial_finalize, polynomial_term_value, Polynomial, PolynomialTerm,
};

/// Compatibility name for the safe Rust slice descriptor, not a C ABI type.
#[allow(non_camel_case_types)]
pub type polynomial<'a> = Polynomial<'a>;
/// Compatibility name for the unsigned-degree native-word term value.
#[allow(non_camel_case_types)]
pub type polynomial_term = PolynomialTerm;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
