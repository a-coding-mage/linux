// SPDX-License-Identifier: GPL-2.0
/*
 * 16550 serial console support.
 *
 * Original copied from <file:arch/ppc/boot/common/ns16550.c>
 * (which had no copyright)
 * Modifications: 2006 (c) MontaVista Software, Inc.
 *
 * Modified by: Mark A. Greer <mgreer@mvista.com>
 */

// Dependencies supplied by the surrounding boot environment.
use core::ffi::c_void;

type U8 = u8;
type U32 = u32;

#[repr(C)]
pub struct serial_console_data {
    pub open: Option<unsafe extern "C" fn() -> i32>,
    pub putc: Option<unsafe extern "C" fn(U8)>,
    pub getc: Option<unsafe extern "C" fn() -> U8>,
    pub tstc: Option<unsafe extern "C" fn() -> U8>,
    pub close: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    fn out_8(addr: *mut U8, value: U8);
    fn in_8(addr: *mut U8) -> U8;
    fn dt_get_virtual_reg(devp: *mut c_void, reg: *mut *mut U8, count: i32) -> i32;
    fn getprop(devp: *mut c_void, name: *const u8, value: *mut c_void, size: usize) -> i32;
    fn be32_to_cpu(value: U32) -> U32;
    fn printf(format: *const u8, ...);
}

const UART_DLL: U32 = 0; // Out: Divisor Latch Low
const UART_DLM: U32 = 1; // Out: Divisor Latch High
const UART_FCR: U32 = 2; // Out: FIFO Control Register
const UART_LCR: U32 = 3; // Out: Line Control Register
const UART_MCR: U32 = 4; // Out: Modem Control Register
const UART_LSR: U32 = 5; // In:  Line Status Register
const UART_LSR_THRE: U8 = 0x20; // Transmit-hold-register empty
const UART_LSR_DR: U8 = 0x01; // Receiver data ready
const UART_MSR: U32 = 6; // In:  Modem Status Register
const UART_SCR: U32 = 7; // I/O: Scratch Register

static mut reg_base: *mut U8 = core::ptr::null_mut();
static mut reg_shift: U32 = 0;

unsafe extern "C" fn ns16550_open() -> i32 {
    out_8(reg_base.add((UART_FCR << reg_shift) as usize), 0x06);
    0
}

unsafe extern "C" fn ns16550_putc(c: U8) {
    while (in_8(reg_base.add((UART_LSR << reg_shift) as usize)) & UART_LSR_THRE) == 0 {}
    out_8(reg_base, c);
}

unsafe extern "C" fn ns16550_getc() -> U8 {
    while (in_8(reg_base.add((UART_LSR << reg_shift) as usize)) & UART_LSR_DR) == 0 {}
    in_8(reg_base)
}

unsafe extern "C" fn ns16550_tstc() -> U8 {
    (in_8(reg_base.add((UART_LSR << reg_shift) as usize)) & UART_LSR_DR != 0) as U8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns16550_console_init(
    devp: *mut c_void,
    scdp: *mut serial_console_data,
) -> i32 {
    let mut n: i32;
    let mut reg_offset: U32 = 0;

    if dt_get_virtual_reg(devp, &mut reg_base, 1) < 1 {
        printf(b"virt reg parse fail...\r\n\0".as_ptr());
        return -1;
    }

    n = getprop(
        devp,
        b"reg-offset\0".as_ptr(),
        &mut reg_offset as *mut U32 as *mut c_void,
        core::mem::size_of::<U32>(),
    );
    if n == core::mem::size_of::<U32>() as i32 {
        reg_base = reg_base.add(be32_to_cpu(reg_offset) as usize);
    }

    n = getprop(
        devp,
        b"reg-shift\0".as_ptr(),
        &mut reg_shift as *mut U32 as *mut c_void,
        core::mem::size_of::<U32>(),
    );
    if n != core::mem::size_of::<U32>() as i32 {
        reg_shift = 0;
    } else {
        reg_shift = be32_to_cpu(reg_shift);
    }

    (*scdp).open = Some(ns16550_open);
    (*scdp).putc = Some(ns16550_putc);
    (*scdp).getc = Some(ns16550_getc);
    (*scdp).tstc = Some(ns16550_tstc);
    (*scdp).close = None;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
