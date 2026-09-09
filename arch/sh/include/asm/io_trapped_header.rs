/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux::list, linux::ioport, asm::page, and pt_regs. */

pub const IO_TRAPPED_MAGIC: ::core::ffi::c_uint = 0xfeedbeef;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trapped_io {
    pub magic: ::core::ffi::c_uint,
    pub resource: *mut resource,
    pub num_resources: ::core::ffi::c_uint,
    pub minimum_bus_width: ::core::ffi::c_uint,
    pub list: list_head,
    pub virt_base: *mut ::core::ffi::c_void,
}

/* C declaration: __aligned(PAGE_SIZE). The required PAGE_SIZE-dependent
 * alignment is retained as a source-level requirement for the target build. */

#[cfg(CONFIG_IO_TRAPPED)]
extern "C" {
    pub fn register_trapped_io(tiop: *mut trapped_io) -> ::core::ffi::c_int;
    pub fn handle_trapped_io(
        regs: *mut pt_regs,
        address: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn match_trapped_io_handler(
        list: *mut list_head,
        offset: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
}

#[cfg(all(CONFIG_IO_TRAPPED, CONFIG_HAS_IOMEM))]
extern "C" {
    pub static mut trapped_mem: list_head;
}

#[cfg(all(CONFIG_IO_TRAPPED, CONFIG_HAS_IOMEM))]
#[inline]
pub unsafe fn __ioremap_trapped(
    offset: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    match_trapped_io_handler(&raw mut trapped_mem, offset, size)
}

#[cfg(any(not(CONFIG_IO_TRAPPED), all(CONFIG_IO_TRAPPED, not(CONFIG_HAS_IOMEM))))]
#[inline]
pub const fn __ioremap_trapped(
    _offset: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::null_mut()
}

#[cfg(all(CONFIG_IO_TRAPPED, CONFIG_HAS_IOPORT_MAP))]
extern "C" {
    pub static mut trapped_io: list_head;
}

#[cfg(all(CONFIG_IO_TRAPPED, CONFIG_HAS_IOPORT_MAP))]
#[inline]
pub unsafe fn __ioport_map_trapped(
    offset: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    match_trapped_io_handler(&raw mut trapped_io, offset, size)
}

#[cfg(any(not(CONFIG_IO_TRAPPED), all(CONFIG_IO_TRAPPED, not(CONFIG_HAS_IOPORT_MAP))))]
#[inline]
pub const fn __ioport_map_trapped(
    _offset: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_IO_TRAPPED))]
#[inline]
pub const fn register_trapped_io(_tiop: *mut trapped_io) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_IO_TRAPPED))]
#[inline]
pub const fn handle_trapped_io(
    _tiop: *mut pt_regs,
    _address: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
