/*
 *  BRIEF MODULE DESCRIPTION
 *     Serial port initialisation.
 *
 *  Copyright 2004 IDT Inc. (rischelp@idt.com)
 *
 *  This program is free software; you can redistribute it and/or modify it
 *  under the terms of the GNU General Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
 *  AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 */

// Linux and platform headers supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    static mut idt_cpu_freq: c_uint;
    fn early_serial_setup(port: *mut uart_port) -> c_int;
}

#[repr(C)]
pub struct uart_port {
    pub flags: c_ulong,
    pub line: c_uint,
    pub irq: c_uint,
    pub iotype: c_uint,
    pub membase: *mut c_char,
    pub regshift: c_uint,
    pub uartclk: c_uint,
}

// These values and address macros are provided by the platform headers.
extern "C" {
    static UART0_IRQ: c_uint;
    static UPIO_MEM: c_uint;
    static UPF_BOOT_AUTOCONF: c_ulong;
    static REGBASE: c_ulong;
    static UART0BASE: c_ulong;
}

#[allow(non_upper_case_globals)]
static mut rb532_uart: uart_port = uart_port {
    flags: 0,
    line: 0,
    irq: 0,
    iotype: 0,
    membase: core::ptr::null_mut(),
    regshift: 2,
    uartclk: 0,
};

#[allow(non_snake_case)]
unsafe fn setup_serial_port() -> c_int {
    rb532_uart.flags = UPF_BOOT_AUTOCONF;
    rb532_uart.irq = UART0_IRQ;
    rb532_uart.iotype = UPIO_MEM;
    rb532_uart.membase = (REGBASE.wrapping_add(UART0BASE)) as *mut c_char;
    rb532_uart.uartclk = idt_cpu_freq;

    early_serial_setup(&mut rb532_uart)
}

// Equivalent of arch_initcall(setup_serial_port).
#[used]
#[cfg_attr(target_os = "linux", link_section = ".initcall6.init")]
static ARCH_INITCALL_SETUP_SERIAL_PORT: unsafe fn() -> c_int = setup_serial_port;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
