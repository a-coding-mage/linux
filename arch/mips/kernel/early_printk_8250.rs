// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  8250/16550-type serial ports prom_putchar()
 *
 *  Copyright (C) 2010  Yoichi Yuasa <yuasa@linux-mips.org>
 */
// Dependencies corresponding to <linux/io.h>, <linux/serial_core.h>,
// <linux/serial_reg.h>, and <asm/setup.h> are supplied externally.

static mut serial8250_base: *mut u8 = core::ptr::null_mut();
static mut serial8250_reg_shift: u32 = 0;
static mut serial8250_tx_timeout: u32 = 0;

extern "C" {
    fn readb(addr: *mut u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
}

pub unsafe fn setup_8250_early_printk_port(
    base: usize,
    reg_shift: u32,
    timeout: u32,
) {
    serial8250_base = base as *mut u8;
    serial8250_reg_shift = reg_shift;
    serial8250_tx_timeout = timeout;
}

#[inline]
unsafe fn serial_in(offset: i32) -> u8 {
    let displacement = (offset as u32) << serial8250_reg_shift;
    readb(serial8250_base.add(displacement as usize))
}

#[inline]
unsafe fn serial_out(offset: i32, value: i8) {
    let displacement = (offset as u32) << serial8250_reg_shift;
    writeb(value as u8, serial8250_base.add(displacement as usize));
}

pub unsafe fn prom_putchar(c: i8) {
    let mut timeout: u32;
    let mut status: i32;
    let bits: i32;

    if serial8250_base.is_null() {
        return;
    }

    timeout = serial8250_tx_timeout;
    bits = UART_LSR_TEMT | UART_LSR_THRE;

    loop {
        status = serial_in(UART_LSR);

        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            break;
        }
        if (status & bits) == bits {
            break;
        }
    }

    if timeout != 0 {
        serial_out(UART_TX, c);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
