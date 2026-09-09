/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for IDT RC323434 CPU.
 */

/* C header guard: _ASM_RC32434_RC32434_H_ */
/* C dependencies: <linux/delay.h>, <linux/io.h> */

pub const IDT_CLOCK_MULT: i32 = 2;

/* cpu pipeline flush */
#[inline]
fn rc32434_sync() {
    unsafe {
        core::arch::asm!("sync", options(nostack, preserves_flags));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
