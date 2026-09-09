/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm-generic/bitops/generic-non-atomic.h>.

pub use generic___set_bit as arch___set_bit;
pub use generic___clear_bit as arch___clear_bit;
pub use generic___change_bit as arch___change_bit;

pub use generic___test_and_set_bit as arch___test_and_set_bit;
pub use generic___test_and_clear_bit as arch___test_and_clear_bit;
pub use generic___test_and_change_bit as arch___test_and_change_bit;

pub use generic_test_bit as arch_test_bit;
pub use generic_test_bit_acquire as arch_test_bit_acquire;

// Translated from <asm-generic/bitops/non-instrumented-non-atomic.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
