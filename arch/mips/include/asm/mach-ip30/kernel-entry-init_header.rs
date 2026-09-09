/* SPDX-License-Identifier: GPL-2.0 */

// Corresponds to the C preprocessor header guard:
// __ASM_MACH_IP30_KERNEL_ENTRY_H

/// Equivalent of the empty `kernel_entry_setup` assembly macro.
#[macro_export]
macro_rules! kernel_entry_setup {
    () => {};
}

/// Equivalent of the `smp_slave_setup` assembly macro.
#[macro_export]
macro_rules! smp_slave_setup {
    () => {
        unsafe {
            core::arch::asm!("move gp, a0");
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
