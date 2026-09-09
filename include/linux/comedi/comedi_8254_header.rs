/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * comedi_8254.h
 * Generic 8254 timer/counter support
 * Copyright (C) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/* Dependencies supplied by the surrounding translation unit. */
use core::ffi::c_void;

/* Common oscillator base values in nanoseconds */
pub const I8254_OSC_BASE_10MHZ: u32 = 100;
pub const I8254_OSC_BASE_5MHZ: u32 = 200;
pub const I8254_OSC_BASE_4MHZ: u32 = 250;
pub const I8254_OSC_BASE_2MHZ: u32 = 500;
pub const I8254_OSC_BASE_1MHZ: u32 = 1000;
pub const I8254_OSC_BASE_100KHZ: u32 = 10000;
pub const I8254_OSC_BASE_10KHZ: u32 = 100000;
pub const I8254_OSC_BASE_1KHZ: u32 = 1000000;

/* I/O access size used to read/write registers */
pub const I8254_IO8: u32 = 1;
pub const I8254_IO16: u32 = 2;
pub const I8254_IO32: u32 = 4;

/* Register map for generic 8254 timer (I8254_IO8 with 0 regshift) */
pub const I8254_COUNTER0_REG: u32 = 0x00;
pub const I8254_COUNTER1_REG: u32 = 0x01;
pub const I8254_COUNTER2_REG: u32 = 0x02;
pub const I8254_CTRL_REG: u32 = 0x03;
#[inline]
pub const fn I8254_CTRL_SEL_CTR(x: u32) -> u32 { x << 6 }
#[inline]
pub const fn I8254_CTRL_READBACK(x: u32) -> u32 { I8254_CTRL_SEL_CTR(3) | (1u32 << x) }
pub const I8254_CTRL_READBACK_COUNT: u32 = I8254_CTRL_READBACK(4);
pub const I8254_CTRL_READBACK_STATUS: u32 = I8254_CTRL_READBACK(5);
#[inline]
pub const fn I8254_CTRL_READBACK_SEL_CTR(x: u32) -> u32 { 2 << x }
#[inline]
pub const fn I8254_CTRL_RW(x: u32) -> u32 { (x & 0x3) << 4 }
pub const I8254_CTRL_LATCH: u32 = I8254_CTRL_RW(0);
pub const I8254_CTRL_LSB_ONLY: u32 = I8254_CTRL_RW(1);
pub const I8254_CTRL_MSB_ONLY: u32 = I8254_CTRL_RW(2);
pub const I8254_CTRL_LSB_MSB: u32 = I8254_CTRL_RW(3);

/* counter maps zero to 0x10000 */
pub const I8254_MAX_COUNT: u32 = 0x10000;

#[repr(C)]
pub struct comedi_device;
#[repr(C)]
pub struct comedi_insn;
#[repr(C)]
pub struct comedi_subdevice;

pub type comedi_8254_iocb_fn = unsafe extern "C" fn(
    i8254: *mut comedi_8254,
    dir: core::ffi::c_int,
    reg: u32,
    val: u32,
) -> u32;

#[repr(C)]
pub struct comedi_8254 {
    pub iocb: Option<comedi_8254_iocb_fn>,
    pub context: core::ffi::c_ulong,
    pub iosize: u32,
    pub regshift: u32,
    pub osc_base: u32,
    pub divisor: u32,
    pub divisor1: u32,
    pub divisor2: u32,
    pub next_div: u32,
    pub next_div1: u32,
    pub next_div2: u32,
    pub clock_src: [u32; 3],
    pub gate_src: [u32; 3],
    pub busy: [bool; 3],
    pub insn_config: Option<unsafe extern "C" fn(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        insn: *mut comedi_insn,
        data: *mut u32,
    ) -> core::ffi::c_int>,
}

extern "C" {
    pub fn comedi_8254_status(i8254: *mut comedi_8254, counter: u32) -> u32;
    pub fn comedi_8254_read(i8254: *mut comedi_8254, counter: u32) -> u32;
    pub fn comedi_8254_write(i8254: *mut comedi_8254, counter: u32, val: u32);
    pub fn comedi_8254_set_mode(i8254: *mut comedi_8254, counter: u32, mode: u32) -> core::ffi::c_int;
    pub fn comedi_8254_load(i8254: *mut comedi_8254, counter: u32, val: u32, mode: u32) -> core::ffi::c_int;
    pub fn comedi_8254_pacer_enable(i8254: *mut comedi_8254, counter1: u32, counter2: u32, enable: bool);
    pub fn comedi_8254_update_divisors(i8254: *mut comedi_8254);
    pub fn comedi_8254_cascade_ns_to_timer(i8254: *mut comedi_8254, nanosec: *mut u32, flags: u32);
    pub fn comedi_8254_ns_to_timer(i8254: *mut comedi_8254, nanosec: *mut u32, flags: u32);
    pub fn comedi_8254_set_busy(i8254: *mut comedi_8254, counter: u32, busy: bool);
    pub fn comedi_8254_subdevice_init(s: *mut comedi_subdevice, i8254: *mut comedi_8254);
    pub fn comedi_8254_mm_alloc(mmio: *mut c_void, osc_base: u32, iosize: u32, regshift: u32) -> *mut comedi_8254;
}

/* CONFIG_HAS_IOPORT selects the real declaration; otherwise the C inline returns ERR_PTR(-ENXIO). */
#[cfg(has_ioport)]
extern "C" {
    pub fn comedi_8254_io_alloc(iobase: core::ffi::c_ulong, osc_base: u32, iosize: u32, regshift: u32) -> *mut comedi_8254;
}

#[cfg(not(has_ioport))]
#[inline]
pub unsafe fn comedi_8254_io_alloc(
    _iobase: core::ffi::c_ulong,
    _osc_base: u32,
    _iosize: u32,
    _regshift: u32,
) -> *mut comedi_8254 {
    /* ERR_PTR(-ENXIO), supplied by the surrounding Linux translation. */
    (-6isize) as *mut comedi_8254
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
