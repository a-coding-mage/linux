/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: `phys_addr_t` is supplied by the Linux type definitions.

#[inline]
pub unsafe fn is_bmips_internal_registers(offset: phys_addr_t) -> i32 {
    if offset >= 0xfff8_0000 {
        return 1;
    }

    0
}

#[inline]
pub unsafe fn plat_ioremap(
    offset: phys_addr_t,
    _size: core::ffi::c_ulong,
    _flags: core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    if is_bmips_internal_registers(offset) != 0 {
        return offset as usize as *mut core::ffi::c_void;
    }

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn plat_iounmap(addr: *const core::ffi::c_void) -> i32 {
    is_bmips_internal_registers(addr as usize as phys_addr_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
