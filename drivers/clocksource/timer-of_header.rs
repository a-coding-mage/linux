/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: declarations supplied by the Linux clockchips interfaces. */

pub const TIMER_OF_BASE: u32 = 0x1;
pub const TIMER_OF_CLOCK: u32 = 0x2;
pub const TIMER_OF_IRQ: u32 = 0x4;

#[repr(C)]
pub struct of_timer_irq {
    pub irq: i32,
    pub index: i32,
    pub name: *const core::ffi::c_char,
    pub flags: libc::c_ulong,
    pub handler: irq_handler_t,
}

#[repr(C)]
pub struct of_timer_base {
    pub base: *mut core::ffi::c_void,
    pub name: *const core::ffi::c_char,
    pub index: i32,
}

#[repr(C)]
pub struct of_timer_clk {
    pub clk: *mut clk,
    pub name: *const core::ffi::c_char,
    pub index: i32,
    pub rate: libc::c_ulong,
    pub period: libc::c_ulong,
}

#[repr(C)]
pub struct timer_of {
    pub flags: u32,
    pub np: *mut device_node,
    pub clkevt: clock_event_device,
    pub of_base: of_timer_base,
    pub of_irq: of_timer_irq,
    pub of_clk: of_timer_clk,
    pub private_data: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn to_timer_of(clkevt: *mut clock_event_device) -> *mut timer_of {
    (clkevt as *mut u8).sub(core::mem::offset_of!(timer_of, clkevt)) as *mut timer_of
}

#[inline]
pub unsafe fn timer_of_base(to: *mut timer_of) -> *mut core::ffi::c_void {
    (*to).of_base.base
}

#[inline]
pub unsafe fn timer_of_irq(to: *mut timer_of) -> i32 {
    (*to).of_irq.irq
}

#[inline]
pub unsafe fn timer_of_rate(to: *mut timer_of) -> libc::c_ulong {
    (*to).of_clk.rate
}

#[inline]
pub unsafe fn timer_of_period(to: *mut timer_of) -> libc::c_ulong {
    (*to).of_clk.period
}

extern "C" {
    pub fn timer_of_init(np: *mut device_node, to: *mut timer_of) -> i32;
    pub fn timer_of_cleanup(to: *mut timer_of);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
