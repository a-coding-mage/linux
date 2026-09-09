// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2008-2010 Thomas Chou <thomas@wytron.com.tw>
 */

// Translated from C. The original preprocessor conditions select the
// corresponding implementation at build time.

extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn readw(addr: *const core::ffi::c_void) -> u16;
    fn writeb(value: i32, addr: *mut core::ffi::c_void);
    fn writew(value: u16, addr: *mut core::ffi::c_void);
    fn writel(value: u32, addr: *mut core::ffi::c_void);
}

#[cfg(any(feature = "serial_altera_jtaguart_console", feature = "serial_altera_uart_console"))]
unsafe fn my_ioremap(physaddr: u64) -> *mut core::ffi::c_void {
    // CONFIG_NIOS2_IO_REGION_BASE is supplied by the build configuration.
    (physaddr | CONFIG_NIOS2_IO_REGION_BASE as u64) as *mut core::ffi::c_void
}

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
const ALTERA_JTAGUART_SIZE: usize = 8;
#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
const ALTERA_JTAGUART_DATA_REG: usize = 0;
#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
const ALTERA_JTAGUART_CONTROL_REG: usize = 4;
#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
const ALTERA_JTAGUART_CONTROL_AC_MSK: u32 = 0x00000400;
#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
const ALTERA_JTAGUART_CONTROL_WSPACE_MSK: u32 = 0xFFFF0000;

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
static mut uartbase: *mut core::ffi::c_void = core::ptr::null_mut();

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base", feature = "serial_altera_jtaguart_console_bypass"))]
unsafe fn jtag_putc(ch: i32) {
    if readl(uartbase.add(ALTERA_JTAGUART_CONTROL_REG)) & ALTERA_JTAGUART_CONTROL_WSPACE_MSK != 0 {
        writeb(ch, uartbase.add(ALTERA_JTAGUART_DATA_REG));
    }
}

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base", not(feature = "serial_altera_jtaguart_console_bypass")))]
unsafe fn jtag_putc(ch: i32) {
    while readl(uartbase.add(ALTERA_JTAGUART_CONTROL_REG)) & ALTERA_JTAGUART_CONTROL_WSPACE_MSK == 0 {}
    writeb(ch, uartbase.add(ALTERA_JTAGUART_DATA_REG));
}

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
unsafe fn putchar(ch: i32) -> i32 {
    jtag_putc(ch);
    ch
}

#[cfg(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"))]
unsafe fn console_init() {
    uartbase = my_ioremap(JTAG_UART_BASE as u64);
    writel(ALTERA_JTAGUART_CONTROL_AC_MSK, uartbase.add(ALTERA_JTAGUART_CONTROL_REG));
}

#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
const ALTERA_UART_SIZE: usize = 32;
#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
const ALTERA_UART_TXDATA_REG: usize = 4;
#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
const ALTERA_UART_STATUS_REG: usize = 8;
#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
const ALTERA_UART_DIVISOR_REG: usize = 16;
#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
const ALTERA_UART_STATUS_TRDY_MSK: u16 = 0x40;
#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
static mut uartbase: u64 = 0;

#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
unsafe fn uart_putc(ch: i32) {
    let mut i = 0;
    while i < 0x10000 {
        if readw((uartbase + ALTERA_UART_STATUS_REG as u64) as *const _) & ALTERA_UART_STATUS_TRDY_MSK != 0 { break; }
        i += 1;
    }
    writeb(ch, (uartbase + ALTERA_UART_TXDATA_REG as u64) as *mut _);
}

#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
unsafe fn putchar(ch: i32) -> i32 {
    uart_putc(ch);
    if ch == b'\n' as i32 { uart_putc(b'\r' as i32); }
    ch
}

#[cfg(all(feature = "serial_altera_uart_console", feature = "uart0_base"))]
unsafe fn console_init() {
    uartbase = my_ioremap(UART0_BASE as u64) as u64;
    let baud = CONFIG_SERIAL_ALTERA_UART_BAUDRATE;
    let baudclk = UART0_FREQ / baud;
    writew(baudclk as u16, (uartbase + ALTERA_UART_DIVISOR_REG as u64) as *mut _);
}

#[cfg(not(any(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"), all(feature = "serial_altera_uart_console", feature = "uart0_base"))))]
fn putchar(ch: i32) -> i32 { ch }

#[cfg(not(any(all(feature = "serial_altera_jtaguart_console", feature = "jtag_uart_base"), all(feature = "serial_altera_uart_console", feature = "uart0_base"))))]
fn console_init() {}

unsafe fn puts(mut s: *const i8) -> i32 {
    while *s != 0 {
        putchar(*s as i32);
        s = s.add(1);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
