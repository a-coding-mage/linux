// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2017 IBM Corporation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/string.h, linux/export.h, linux/uaccess.h, linux/libnvdimm.h,
// asm/cacheflush.h

unsafe fn __clean_pmem_range(start: usize, stop: usize) {
    let shift = l1_dcache_shift();
    let bytes = l1_dcache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut core::ffi::c_void;
    let size = stop.wrapping_sub(addr as usize).wrapping_add(bytes - 1);
    let mut i: usize = 0;

    while i < (size >> shift) {
        // C inline assembly: PPC_DCBSTPS(0, addr), with a memory clobber.
        core::arch::asm!("dcbstps 0, 0({addr})", addr = in(reg) addr, options(nostack));
        i = i.wrapping_add(1);
        addr = (addr as usize).wrapping_add(bytes) as *mut core::ffi::c_void;
    }
}

unsafe fn __flush_pmem_range(start: usize, stop: usize) {
    let shift = l1_dcache_shift();
    let bytes = l1_dcache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut core::ffi::c_void;
    let size = stop.wrapping_sub(addr as usize).wrapping_add(bytes - 1);
    let mut i: usize = 0;

    while i < (size >> shift) {
        // C inline assembly: PPC_DCBFPS(0, addr), with a memory clobber.
        core::arch::asm!("dcbfps 0, 0({addr})", addr = in(reg) addr, options(nostack));
        i = i.wrapping_add(1);
        addr = (addr as usize).wrapping_add(bytes) as *mut core::ffi::c_void;
    }
}

unsafe fn clean_pmem_range(start: usize, stop: usize) {
    if cpu_has_feature(CPU_FTR_ARCH_207S) {
        return __clean_pmem_range(start, stop);
    }
}

unsafe fn flush_pmem_range(start: usize, stop: usize) {
    if cpu_has_feature(CPU_FTR_ARCH_207S) {
        return __flush_pmem_range(start, stop);
    }
}

/*
 * CONFIG_ARCH_HAS_PMEM_API symbols
 */
#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut core::ffi::c_void, size: usize) {
    let start = addr as usize;
    clean_pmem_range(start, start.wrapping_add(size));
}

#[no_mangle]
pub unsafe extern "C" fn arch_invalidate_pmem(addr: *mut core::ffi::c_void, size: usize) {
    let start = addr as usize;
    flush_pmem_range(start, start.wrapping_add(size));
}

/*
 * CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE symbols
 */
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_flushcache(
    dest: *mut core::ffi::c_void,
    mut src: *const core::ffi::c_void,
    size: usize,
) -> usize {
    let start = dest as usize;

    src = mask_user_address(src);
    let not_copied = __copy_from_user(dest, src, size);
    clean_pmem_range(start, start.wrapping_add(size));

    not_copied
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_flushcache(
    dest: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    size: usize,
) {
    let start = dest as usize;

    memcpy(dest, src, size);
    clean_pmem_range(start, start.wrapping_add(size));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
