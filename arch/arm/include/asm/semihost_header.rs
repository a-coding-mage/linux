/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Adapted for ARM and earlycon:
 * Copyright (C) 2014 Linaro Ltd.
 * Author: Rob Herring <robh@kernel.org>
 */

// The original header depends on the build-time CONFIG_THUMB2_KERNEL option.
#[cfg(CONFIG_THUMB2_KERNEL)]
const SEMIHOST_SWI: u32 = 0xab;

#[cfg(not(CONFIG_THUMB2_KERNEL))]
const SEMIHOST_SWI: u32 = 0x123456;

#[repr(C)]
pub struct uart_port {
    _private: [u8; 0],
}

pub unsafe fn smh_putc(_port: *mut uart_port, c: u8) {
    core::arch::asm!(
        "mov r1, {c}",
        "mov r0, #3",
        "svc {swi}",
        c = in(reg) &c,
        swi = const SEMIHOST_SWI,
        out("r0") _,
        out("r1") _,
        options(nostack),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
