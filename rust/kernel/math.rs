// SPDX-License-Identifier: GPL-2.0
//! Safe integer exponentiation and square roots.
//!
//! These allocation-free, constant-evaluable helpers share the canonical
//! translated algorithms with `CONFIG_RUST_INT_MATH`. They are available with
//! either native C or Rust implementation selected, require no foreign calls,
//! and define no unmangled symbols. In particular, [`int_sqrt64`] consumes its
//! entire 64-bit input even on 32-bit kernels.
//!
//! Only these audited integer functions are exposed here. The other macros
//! and declarations in the translated C math header are not imported.

#[path = "../../lib/math/int_pow.rs"]
mod power;
#[path = "../../lib/math/int_sqrt.rs"]
mod square_root;

pub use power::int_pow;
pub use square_root::{int_sqrt, int_sqrt32, int_sqrt64};
