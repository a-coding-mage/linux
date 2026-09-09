/* SPDX-License-Identifier: GPL-2.0 */

// The C header aliases these bit-operation names to architecture-provided
// implementations.  Rust re-exports preserve the same source-level names.
pub use arch___set_bit as ___set_bit;
pub use arch___clear_bit as ___clear_bit;
pub use arch___change_bit as ___change_bit;

pub use arch___test_and_set_bit as ___test_and_set_bit;
pub use arch___test_and_clear_bit as ___test_and_clear_bit;
pub use arch___test_and_change_bit as ___test_and_change_bit;

pub use arch_test_bit as _test_bit;
pub use arch_test_bit_acquire as _test_bit_acquire;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
