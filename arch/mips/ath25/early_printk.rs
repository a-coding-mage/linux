/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2010 Gabor Juhos <juhosg@openwrt.org>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm.h, linux/io.h, linux/serial_reg.h, asm/setup.h,
// devices.h, ar2315_regs.h, and ar5312_regs.h.

use core::ptr;

extern "C" {
    fn is_ar2315() -> bool;
}

// External constants/macros supplied by the surrounding translation.
extern "C" {
    static AR2315_UART0_BASE: usize;
    static AR5312_UART0_BASE: usize;
    static UART_LSR: usize;
    static UART_LSR_THRE: u32;
    static UART_TX: usize;
}

// KSEG1ADDR is supplied by the platform-specific translation.
extern "C" {
    fn KSEG1ADDR(address: usize) -> usize;
}

#[inline]
unsafe fn prom_uart_wr(base: *mut u8, reg: usize, ch: u8) {
    ptr::write_volatile(base.add(4 * reg) as *mut u32, ch as u32);
}

#[inline]
unsafe fn prom_uart_rr(base: *mut u8, reg: usize) -> u8 {
    ptr::read_volatile(base.add(4 * reg) as *const u32) as u8
}

pub unsafe fn prom_putchar(ch: i8) {
    static mut BASE: *mut u8 = ptr::null_mut();

    if BASE.is_null() {
        if is_ar2315() {
            BASE = KSEG1ADDR(AR2315_UART0_BASE) as *mut u8;
        } else {
            BASE = KSEG1ADDR(AR5312_UART0_BASE) as *mut u8;
        }
    }

    while (prom_uart_rr(BASE, UART_LSR) as u32 & UART_LSR_THRE) == 0 {}
    prom_uart_wr(BASE, UART_TX, ch as u8);
    while (prom_uart_rr(BASE, UART_LSR) as u32 & UART_LSR_THRE) == 0 {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
