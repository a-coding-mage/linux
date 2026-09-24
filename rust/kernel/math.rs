// SPDX-License-Identifier: GPL-2.0
//! Safe integer arithmetic, polynomials, fixed-point coordinates and division.
//!
//! These allocation-free helpers share the canonical
//! translated algorithms with `CONFIG_RUST_INT_MATH`, `CONFIG_RUST_GCD_LCM`,
//! `CONFIG_RUST_RATIONAL`, `CONFIG_RUST_RECIPROCAL_DIV`, `CONFIG_RUST_INT_LOG`,
//! `CONFIG_RUST_DIV64`, `CONFIG_RUST_CORDIC`, and `CONFIG_RUST_POLYNOMIAL`.
//! They are constant-evaluable except for the iterative
//! divider, which preserves its runtime optimization barrier.
//! They are available with either native C or Rust implementation selected,
//! require no foreign calls,
//! and define no unmangled symbols. In particular, [`int_sqrt64`] consumes its
//! entire 64-bit input even on 32-bit kernels.
//! GCD and LCM helpers use the configured CPU bit-scan capability without
//! consulting the native implementation's mutable runtime static key. Either
//! GCD algorithm produces the same result; LCM preserves unsigned wrapping.
//! Rational approximation preserves native-word wrapping and returns its two
//! outputs by value rather than exposing the native C pointer interface.
//! Reciprocal constructors and division return `None` for input outside the
//! original C's defined arithmetic domain, without emitting kernel warnings.
//! Q24 logarithms preserve the original table approximation and return `None`
//! for zero, without the warning and zero return of the native C interface.
//! Division helpers return `None` for zero divisors and signed quotient
//! overflow. Their rounding and multiplication preserve wrapping and the
//! generic headers' documented configuration-dependent shift domains.
//! CORDIC preserves the original fixed-point conversion before angle
//! normalization, arithmetic shifts and signed 32-bit wrapping.
//! Polynomial descriptors borrow bounded term slices and preserve native-long
//! factor redistribution. Evaluated invalid divisions or a missing constant
//! terminator return `None`; unused and trailing terms are not evaluated.
//!
//! Only these audited integer functions are exposed here. The other macros
//! and declarations in the translated C math header are not imported.

#[allow(dead_code, unreachable_pub, unused_imports)]
#[path = "../../include/linux/cordic_header.rs"]
mod coordinates;
#[allow(unreachable_pub)]
#[path = "../../include/linux/math64_header.rs"]
mod division;
#[path = "../../lib/math/rational.rs"]
mod fractions;
#[path = "../../lib/math/gcd.rs"]
// The shared source also exposes explicit-strategy helpers to native owners.
#[allow(unreachable_pub)]
mod greatest_common_divisor;
#[path = "../../lib/math/lcm.rs"]
#[allow(dead_code, unreachable_pub)]
mod least_common_multiple;
#[path = "../../lib/math/int_log.rs"]
mod logarithms;
#[allow(dead_code, unreachable_pub, unused_imports)]
#[path = "../../include/linux/polynomial_header.rs"]
mod polynomials;
#[path = "../../lib/math/int_pow.rs"]
mod power;
#[path = "../../lib/math/reciprocal_div.rs"]
mod reciprocals;
#[path = "../../lib/math/int_sqrt.rs"]
mod square_root;

pub use coordinates::{
    cordic_calc_iq, cordic_fixed, cordic_float, CordicIq, CORDIC_ANGLE_GEN, CORDIC_NUM_ITER,
    CORDIC_PRECISION_SHIFT,
};
pub use division::*;
pub use fractions::rational_best_approximation;
pub use greatest_common_divisor::gcd;
pub use least_common_multiple::{lcm, lcm_not_zero};
pub use logarithms::{intlog10, intlog2};
pub use polynomials::{
    polynomial_calc, polynomial_finalize, polynomial_term_value, Polynomial, PolynomialTerm,
};
pub use power::int_pow;
pub use reciprocals::{
    reciprocal_divide, reciprocal_value, reciprocal_value_adv, ReciprocalValue, ReciprocalValueAdv,
};
pub use square_root::{int_sqrt, int_sqrt32, int_sqrt64};
