/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux/types.h and linux/list.h. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void, VaList};

/* Opaque declarations supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct pt_regs;

pub type kmmio_pre_handler_t = unsafe extern "C" fn(
    probe: *mut kmmio_probe,
    regs: *mut pt_regs,
    addr: c_ulong,
);
pub type kmmio_post_handler_t = unsafe extern "C" fn(
    probe: *mut kmmio_probe,
    condition: c_ulong,
    regs: *mut pt_regs,
);

#[repr(C)]
pub struct kmmio_probe {
    /* kmmio internal list: */
    pub list: list_head,
    /* start location of the probe point: */
    pub addr: c_ulong,
    /* length of the probe region: */
    pub len: c_ulong,
    /* Called before addr is executed: */
    pub pre_handler: kmmio_pre_handler_t,
    /* Called after addr is executed: */
    pub post_handler: kmmio_post_handler_t,
    pub private: *mut c_void,
}

extern "C" {
    pub static mut kmmio_count: c_uint;

    pub fn register_kmmio_probe(p: *mut kmmio_probe) -> c_int;
    pub fn unregister_kmmio_probe(p: *mut kmmio_probe);
    pub fn kmmio_init() -> c_int;
    pub fn kmmio_cleanup();
}

/* CONFIG_MMIOTRACE selects the external implementations below. */
pub fn is_kmmio_active() -> c_int {
    unsafe { kmmio_count as c_int }
}

extern "C" {
    pub fn kmmio_handler(regs: *mut pt_regs, addr: c_ulong) -> c_int;
    pub fn mmiotrace_ioremap(offset: resource_size_t, size: c_ulong, addr: *mut c_void);
    pub fn mmiotrace_iounmap(addr: *mut c_void);
    pub fn mmiotrace_printk(fmt: *const c_char, ...) -> c_int;
}

/* !CONFIG_MMIOTRACE: use these inline definitions instead. */
#[inline]
pub fn kmmio_handler_disabled(_regs: *mut pt_regs, _addr: c_ulong) -> c_int {
    0
}

#[inline]
pub fn mmiotrace_ioremap_disabled(
    _offset: resource_size_t,
    _size: c_ulong,
    _addr: *mut c_void,
) {
}

#[inline]
pub fn mmiotrace_iounmap_disabled(_addr: *mut c_void) {
}

#[inline]
pub unsafe extern "C" fn mmiotrace_printk_disabled(_fmt: *const c_char, ...) -> c_int {
    0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mm_io_opcode {
    MMIO_READ = 0x1,   /* struct mmiotrace_rw */
    MMIO_WRITE = 0x2,  /* struct mmiotrace_rw */
    MMIO_PROBE = 0x3,  /* struct mmiotrace_map */
    MMIO_UNPROBE = 0x4, /* struct mmiotrace_map */
    MMIO_UNKNOWN_OP = 0x5, /* struct mmiotrace_rw */
}

#[repr(C)]
pub struct mmiotrace_rw {
    pub phys: resource_size_t, /* PCI address of register */
    pub value: c_ulong,
    pub pc: c_ulong, /* optional program counter */
    pub map_id: c_int,
    pub opcode: c_uchar, /* one of MMIO_{READ,WRITE,UNKNOWN_OP} */
    pub width: c_uchar, /* size of register access in bytes */
}

#[repr(C)]
pub struct mmiotrace_map {
    pub phys: resource_size_t, /* base address in PCI space */
    pub virt: c_ulong, /* base virtual address */
    pub len: c_ulong, /* mapping size */
    pub map_id: c_int,
    pub opcode: c_uchar, /* MMIO_PROBE or MMIO_UNPROBE */
}

/* in kernel/trace/trace_mmiotrace.c */
extern "C" {
    pub fn enable_mmiotrace();
    pub fn disable_mmiotrace();
    pub fn mmio_trace_rw(rw: *mut mmiotrace_rw);
    pub fn mmio_trace_mapping(map: *mut mmiotrace_map);
    pub fn mmio_trace_printk(fmt: *const c_char, args: VaList<'_>) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
