/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022 tinylab.org
 * Author: Bin Meng <bmeng@tinylab.org>
 */

// C forward declaration: struct uart_port;
#[repr(C)]
pub struct uart_port {
    _private: [u8; 0],
}

/// C equivalent of `static inline void smh_putc(struct uart_port *port,
/// unsigned char c)`.
#[inline]
pub unsafe fn smh_putc(_port: *mut uart_port, c: u8) {
    core::arch::asm!(
        "addi    a1, {cptr}, 0",
        "addi    a0, zero, 3",
        ".balign 16",
        ".option push",
        ".option norvc",
        "slli    zero, zero, 0x1f",
        "ebreak",
        "srai    zero, zero, 0x7",
        ".option pop",
        cptr = in(reg) &c,
        out("a0") _,
        out("a1") _,
        options(volatile),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
