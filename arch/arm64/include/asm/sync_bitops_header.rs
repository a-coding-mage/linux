/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
//   #include <asm/bitops.h>
//   #include <asm/cmpxchg.h>

/* sync_bitops functions are equivalent to the SMP implementation of the
 * original functions, independently from CONFIG_SMP being defined.
 *
 * We need them because _set_bit etc are not SMP safe if !CONFIG_SMP. But
 * under Xen you might be communicating with a completely external entity
 * who might be on another CPU (e.g. two uniprocessor guests communicating
 * via event channels and grant tables). So we need a variant of the bit
 * ops which are SMP safe even on a UP kernel.
 */

#[macro_export]
macro_rules! sync_set_bit {
    ($nr:expr, $p:expr) => { set_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_clear_bit {
    ($nr:expr, $p:expr) => { clear_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_change_bit {
    ($nr:expr, $p:expr) => { change_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_test_and_set_bit {
    ($nr:expr, $p:expr) => { test_and_set_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_test_and_clear_bit {
    ($nr:expr, $p:expr) => { test_and_clear_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_test_and_change_bit {
    ($nr:expr, $p:expr) => { test_and_change_bit!($nr, $p) };
}

#[macro_export]
macro_rules! sync_test_bit {
    ($nr:expr, $addr:expr) => { test_bit!($nr, $addr) };
}

// Equivalent to: #define arch_sync_cmpxchg arch_cmpxchg
pub use arch_cmpxchg as arch_sync_cmpxchg;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
