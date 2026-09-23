// SPDX-License-Identifier: GPL-2.0
//! Safe integer powers, square roots, greatest common divisors and multiples.
//!
//! These allocation-free, constant-evaluable helpers share the canonical
//! translated algorithms with `CONFIG_RUST_INT_MATH` and `CONFIG_RUST_GCD_LCM`.
//! They are available with either native C or Rust implementation selected,
//! require no foreign calls,
//! and define no unmangled symbols. In particular, [`int_sqrt64`] consumes its
//! entire 64-bit input even on 32-bit kernels.
//! GCD and LCM helpers use the configured CPU bit-scan capability without
//! consulting the native implementation's mutable runtime static key. Either
//! GCD algorithm produces the same result; LCM preserves unsigned wrapping.
//!
//! Only these audited integer functions are exposed here. The other macros
//! and declarations in the translated C math header are not imported.

#[path = "../../lib/math/gcd.rs"]
// The shared source also exposes explicit-strategy helpers to native owners.
#[allow(unreachable_pub)]
mod greatest_common_divisor;
#[path = "../../lib/math/lcm.rs"]
#[allow(dead_code, unreachable_pub)]
mod least_common_multiple;
#[path = "../../lib/math/int_pow.rs"]
mod power;
#[path = "../../lib/math/int_sqrt.rs"]
mod square_root;

pub use greatest_common_divisor::gcd;
pub use least_common_multiple::{lcm, lcm_not_zero};
pub use power::int_pow;
pub use square_root::{int_sqrt, int_sqrt32, int_sqrt64};
