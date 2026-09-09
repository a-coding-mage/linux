/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM timer implementation, found in Integrator, Versatile and Realview
 * platforms.  Not all platforms support all registers and bits in these
 * registers, so we mark them with A for Integrator AP, C for Integrator
 * CP, V for Versatile and R for Realview.
 *
 * Integrator AP has 16-bit timers, Integrator CP, Versatile and Realview
 * can have 16-bit or 32-bit selectable via a bit in the control register.
 *
 * Every SP804 contains two identical timers.
 */
pub const NR_TIMERS: usize = 2;
pub const TIMER_1_BASE: usize = 0x00;
pub const TIMER_2_BASE: usize = 0x20;

pub const TIMER_LOAD: usize = 0x00; // ACVR rw
pub const TIMER_VALUE: usize = 0x04; // ACVR ro
pub const TIMER_CTRL: usize = 0x08; // ACVR rw
pub const TIMER_CTRL_ONESHOT: usize = 1 << 0; // CVR
pub const TIMER_CTRL_32BIT: usize = 1 << 1; // CVR
pub const TIMER_CTRL_DIV1: usize = 0 << 2; // ACVR
pub const TIMER_CTRL_DIV16: usize = 1 << 2; // ACVR
pub const TIMER_CTRL_DIV256: usize = 2 << 2; // ACVR
pub const TIMER_CTRL_IE: usize = 1 << 5; // VR
pub const TIMER_CTRL_PERIODIC: usize = 1 << 6; // ACVR
pub const TIMER_CTRL_ENABLE: usize = 1 << 7; // ACVR

pub const TIMER_INTCLR: usize = 0x0c; // ACVR wo
pub const TIMER_RIS: usize = 0x10; // CVR ro
pub const TIMER_MIS: usize = 0x14; // CVR ro
pub const TIMER_BGLOAD: usize = 0x18; // CVR rw

#[repr(C)]
pub struct sp804_timer {
    pub load: i32,
    pub load_h: i32,
    pub value: i32,
    pub value_h: i32,
    pub ctrl: i32,
    pub intclr: i32,
    pub ris: i32,
    pub mis: i32,
    pub bgload: i32,
    pub bgload_h: i32,
    pub timer_base: [i32; NR_TIMERS],
    pub width: i32,
}

#[repr(C)]
pub struct sp804_clkevt {
    pub base: *mut core::ffi::c_void,
    pub load: *mut core::ffi::c_void,
    pub load_h: *mut core::ffi::c_void,
    pub value: *mut core::ffi::c_void,
    pub value_h: *mut core::ffi::c_void,
    pub ctrl: *mut core::ffi::c_void,
    pub intclr: *mut core::ffi::c_void,
    pub ris: *mut core::ffi::c_void,
    pub mis: *mut core::ffi::c_void,
    pub bgload: *mut core::ffi::c_void,
    pub bgload_h: *mut core::ffi::c_void,
    pub reload: usize,
    pub width: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
