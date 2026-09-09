// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// C dependencies supplied by the surrounding kernel sources.
use core::ffi::{c_char, c_int, c_void};

const EARLY_CONSOLE_PORT: c_int = 1;
const EARLY_CONSOLE_BAUDRATE: c_int = 115200;

const UART_ENABLE: u32 = 1 << 15;
const UART_ENABLE_RX: u32 = 1 << 12;
const UART_ENABLE_TX: u32 = 1 << 10;
const UART_TX_FULL: u32 = 1 << 9;

const fn uart_base(x: c_int) -> usize { (x as usize) * 0x0200 }
const fn u_mode(x: c_int) -> usize { uart_base(x) }
const fn u_sta(x: c_int) -> usize { uart_base(x) + 0x10 }
const fn u_txr(x: c_int) -> usize { uart_base(x) + 0x20 }
const fn u_brg(x: c_int) -> usize { uart_base(x) + 0x40 }

static mut uart_base_ptr: *mut c_void = core::ptr::null_mut();
static mut console_port: c_int = -1;

extern "C" {
    fn pic32_pps_input(function: c_int, pin: c_int);
    fn pic32_pps_output(function: c_int, pin: c_int);
    fn pic32_get_pbclk(bus: c_int) -> u32;
    fn fw_getcmdline() -> *mut c_char;
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn __raw_writel(value: u32, addr: *mut c_void);
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

// Supplied by pic32mzda.h and early_pin.h.
const PIC32_BASE_UART: usize = 0;
const PIC32_SET_OFFSET: usize = 0;
const IN_FUNC_U2RX: c_int = 0;
const IN_RPB0: c_int = 0;
const OUT_FUNC_U2TX: c_int = 0;
const OUT_RPG9: c_int = 0;
const IN_FUNC_U6RX: c_int = 0;
const IN_RPD0: c_int = 0;
const OUT_FUNC_U6TX: c_int = 0;
const OUT_RPB8: c_int = 0;

const fn pic32_set(offset: usize) -> usize { offset + PIC32_SET_OFFSET }

unsafe fn configure_uart_pins(port: c_int) -> c_int {
    match port {
        1 => {
            pic32_pps_input(IN_FUNC_U2RX, IN_RPB0);
            pic32_pps_output(OUT_FUNC_U2TX, OUT_RPG9);
        }
        5 => {
            pic32_pps_input(IN_FUNC_U6RX, IN_RPD0);
            pic32_pps_output(OUT_FUNC_U6TX, OUT_RPB8);
        }
        _ => return -1,
    }
    0
}

unsafe fn configure_uart(port: c_int, baud: c_int) {
    let pbclk: u32 = pic32_get_pbclk(2);
    __raw_writel(0, uart_base_ptr.add(u_mode(port)));
    __raw_writel(((pbclk / baud as u32) / 16) - 1, uart_base_ptr.add(u_brg(port)));
    __raw_writel(UART_ENABLE, uart_base_ptr.add(u_mode(port)));
    __raw_writel(UART_ENABLE_TX | UART_ENABLE_RX,
                 uart_base_ptr.add(pic32_set(u_sta(port))));
}

unsafe fn setup_early_console(port: c_int, baud: c_int) {
    if configure_uart_pins(port) != 0 { return; }
    console_port = port;
    configure_uart(console_port, baud);
}

unsafe fn pic32_getcmdline() -> *mut c_char {
    // arch_mem_init() has not been called yet, so we don't have a real
    // command line setup if using CONFIG_CMDLINE_BOOL.
    fw_getcmdline()
}

unsafe fn get_port_from_cmdline(arch_cmdline: *mut c_char) -> c_int {
    if arch_cmdline.is_null() || *arch_cmdline == 0 { return -1; }
    let key = b"earlyprintk=\0".as_ptr() as *const c_char;
    let tty = b"ttyS\0".as_ptr() as *const c_char;
    let mut s = strstr(arch_cmdline, key);
    if !s.is_null() {
        s = strstr(s, tty);
        if !s.is_null() { return *s.add(4) as c_int - '0' as c_int; }
    }
    -1
}

unsafe fn get_baud_from_cmdline(arch_cmdline: *mut c_char) -> c_int {
    if arch_cmdline.is_null() || *arch_cmdline == 0 { return -1; }
    let key = b"earlyprintk=\0".as_ptr() as *const c_char;
    let tty = b"ttyS\0".as_ptr() as *const c_char;
    let mut s = strstr(arch_cmdline, key);
    if !s.is_null() {
        s = strstr(s, tty);
        if !s.is_null() {
            s = s.add(6);
            let mut baud = 0;
            while *s >= b'0' as c_char && *s <= b'9' as c_char {
                baud = baud * 10 + *s as c_int - '0' as c_int;
                s = s.add(1);
            }
            return baud;
        }
    }
    -1
}

pub unsafe fn fw_init_early_console() {
    let arch_cmdline = pic32_getcmdline();
    uart_base_ptr = ioremap(PIC32_BASE_UART, 0xc00);
    let baud = get_baud_from_cmdline(arch_cmdline);
    let port = get_port_from_cmdline(arch_cmdline);
    setup_early_console(if port == -1 { EARLY_CONSOLE_PORT } else { port },
                        if baud == -1 { EARLY_CONSOLE_BAUDRATE } else { baud });
}

pub unsafe fn prom_putchar(c: c_char) {
    if console_port >= 0 {
        while __raw_readl(uart_base_ptr.add(u_sta(console_port))) & UART_TX_FULL != 0 {}
        __raw_writel(c as u32, uart_base_ptr.add(u_txr(console_port)));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
