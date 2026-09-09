/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Nothing to see here yet
 */

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    static mut irq_err_count: c_ulong;
    fn pr_crit(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn ack_bad_irq(irq: c_int) {
    irq_err_count = irq_err_count.wrapping_add(1);
    pr_crit(b"unexpected IRQ trap at vector %02x\n\0".as_ptr() as *const c_char, irq);
}

pub const ARCH_IRQ_INIT_FLAGS: usize = IRQ_NOREQUEST | IRQ_NOPROBE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
