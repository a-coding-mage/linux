// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic MMIO clocksource support
 */

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

// Supplied by the Linux clocksource, errno, init, and slab dependencies.
#[repr(C)]
pub struct clocksource {
    pub name: *const c_char,
    pub rating: c_int,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: c_uint,
}

#[repr(C)]
pub struct clocksource_mmio {
    pub reg: *mut c_void,
    pub clksrc: clocksource,
}

unsafe fn to_mmio_clksrc(c: *mut clocksource) -> *mut clocksource_mmio {
    let offset = core::mem::offset_of!(clocksource_mmio, clksrc);
    (c as *mut u8).sub(offset) as *mut clocksource_mmio
}

unsafe extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn readw_relaxed(addr: *mut c_void) -> u16;
    fn clocksource_register_hz(c: *mut clocksource, hz: c_ulong) -> c_int;
}

pub unsafe extern "C" fn clocksource_mmio_readl_up(c: *mut clocksource) -> u64 {
    readl_relaxed((*to_mmio_clksrc(c)).reg) as u64
}

pub unsafe extern "C" fn clocksource_mmio_readl_down(c: *mut clocksource) -> u64 {
    (!((readl_relaxed((*to_mmio_clksrc(c)).reg)) as u64)) & (*c).mask
}

pub unsafe extern "C" fn clocksource_mmio_readw_up(c: *mut clocksource) -> u64 {
    readw_relaxed((*to_mmio_clksrc(c)).reg) as u64
}

pub unsafe extern "C" fn clocksource_mmio_readw_down(c: *mut clocksource) -> u64 {
    (!((readw_relaxed((*to_mmio_clksrc(c)).reg)) as u64)) & (*c).mask
}

/**
 * clocksource_mmio_init - Initialize a simple mmio based clocksource
 * @base: Virtual address of the clock readout register
 * @name: Name of the clocksource
 * @hz: Frequency of the clocksource in Hz
 * @rating: Rating of the clocksource
 * @bits: Number of valid bits
 * @read: One of clocksource_mmio_read*() above
 */
pub unsafe extern "C" fn clocksource_mmio_init(
    base: *mut c_void,
    name: *const c_char,
    hz: c_ulong,
    rating: c_int,
    bits: c_uint,
    read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
) -> c_int {
    let cs: *mut clocksource_mmio;

    if bits > 64 || bits < 16 {
        return -EINVAL;
    }

    cs = kzalloc_obj!(clocksource_mmio);
    if cs.is_null() {
        return -ENOMEM;
    }

    (*cs).reg = base;
    (*cs).clksrc.name = name;
    (*cs).clksrc.rating = rating;
    (*cs).clksrc.read = read;
    (*cs).clksrc.mask = CLOCKSOURCE_MASK(bits);
    (*cs).clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;

    clocksource_register_hz(&mut (*cs).clksrc, hz)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
