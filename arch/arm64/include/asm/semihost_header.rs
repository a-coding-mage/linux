/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Adapted for ARM and earlycon:
 * Copyright (C) 2014 Linaro Ltd.
 * Author: Rob Herring <robh@kernel.org>
 */

// The C header declares this type elsewhere.
pub enum uart_port {}

#[inline]
pub unsafe fn smh_putc(_port: *mut uart_port, c: u8) {
    core::arch::asm!(
        "mov x1, {cptr}",
        "mov x0, #3",
        "hlt 0xf000",
        cptr = in(reg) (&c as *const u8),
        lateout("x0") _,
        lateout("x1") _,
        options(nostack),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
