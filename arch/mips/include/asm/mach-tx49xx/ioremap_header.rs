/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/mach-tx49xx/ioremap.h
 */

// The original header includes <linux/types.h> for `phys_addr_t`.

// CONFIG_64BIT selects the direct-map base in the original C header.
#[cfg(target_pointer_width = "64")]
pub const TXX9_DIRECTMAP_BASE: u64 = 0xfff000000u64;
#[cfg(not(target_pointer_width = "64"))]
pub const TXX9_DIRECTMAP_BASE: u64 = 0xff000000u64;

pub unsafe fn plat_ioremap(
    offset: phys_addr_t,
    _size: core::ffi::c_ulong,
    _flags: core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    let offset = offset as u64;
    if offset >= TXX9_DIRECTMAP_BASE
        && offset < TXX9_DIRECTMAP_BASE.wrapping_add(0x400000)
    {
        return (offset as i32 as u32 as usize) as *mut core::ffi::c_void;
    }
    core::ptr::null_mut()
}

pub unsafe fn plat_iounmap(addr: *const core::ffi::c_void) -> core::ffi::c_int {
    if (addr as usize) as u64
        >= ((TXX9_DIRECTMAP_BASE & 0xffffffff) as u32 as usize) as u64
    {
        1
    } else {
        0
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
