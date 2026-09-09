// SPDX-License-Identifier: GPL-2.0-only
/*
 * User address space access functions.
 *
 * Copyright 1997 Andi Kleen <ak@muc.de>
 * Copyright 1997 Linus Torvalds
 * Copyright 2002 Andi Kleen <ak@suse.de>
 */

/* The declarations below are supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct BootCpuData {
    pub x86_clflush_size: u16,
}

extern "C" {
    pub static boot_cpu_data: BootCpuData;
    fn clwb(addr: *mut core::ffi::c_void);
    fn masked_user_access_begin(src: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn copy_to_nontemporal(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        size: usize,
    ) -> usize;
    fn user_access_end();
}

#[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
unsafe fn clean_cache_range(addr: *mut core::ffi::c_void, size: usize) {
    let x86_clflush_size: u16 = boot_cpu_data.x86_clflush_size;
    let clflush_mask = x86_clflush_size as usize - 1;
    let vend = (addr as usize).wrapping_add(size);
    let mut p = (addr as usize & !clflush_mask) as *mut core::ffi::c_void;

    while (p as usize) < vend {
        clwb(p);
        p = (p as usize).wrapping_add(x86_clflush_size as usize) as *mut core::ffi::c_void;
    }
}

#[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut core::ffi::c_void, size: usize) {
    clean_cache_range(addr, size);
}

#[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
#[no_mangle]
pub unsafe extern "C" fn copy_user_flushcache(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    size: usize,
) -> usize {
    let mut dest = dst as usize;
    let src = masked_user_access_begin(src);
    let rc = copy_to_nontemporal(dst, src, size);
    user_access_end();

    if size < 8 {
        if dest & 3 != 0 || size != 4 {
            clean_cache_range(dst, size);
        }
    } else {
        if dest & 7 != 0 {
            let alignment = boot_cpu_data.x86_clflush_size as usize;
            dest = (dest.wrapping_add(alignment - 1)) & !(alignment - 1);
            clean_cache_range(dst, 1);
        }

        let flushed = dest.wrapping_sub(dst as usize);
        if size > flushed && (size - flushed) & 7 != 0 {
            clean_cache_range((dst as usize + size - 1) as *mut core::ffi::c_void, 1);
        }
    }

    rc
}

#[cfg(CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE)]
#[no_mangle]
pub unsafe extern "C" fn __memcpy_flushcache(
    _dst: *mut core::ffi::c_void,
    _src: *const core::ffi::c_void,
    mut size: usize,
) {
    let mut dest = _dst as usize;
    let mut source = _src as usize;

    if dest & 7 != 0 {
        let len = core::cmp::min(size, (dest.wrapping_add(7) & !7).wrapping_sub(dest));
        core::ptr::copy_nonoverlapping(source as *const u8, dest as *mut u8, len);
        clean_cache_range(dest as *mut core::ffi::c_void, len);
        dest = dest.wrapping_add(len);
        source = source.wrapping_add(len);
        size -= len;
        if size == 0 {
            return;
        }
    }

    /* 4x8 movnti loop */
    while size >= 32 {
        let a = (source as *const u64).read_unaligned();
        let b = (source as *const u64).add(1).read_unaligned();
        let c = (source as *const u64).add(2).read_unaligned();
        let d = (source as *const u64).add(3).read_unaligned();
        (dest as *mut u64).write_volatile(a);
        (dest as *mut u64).add(1).write_volatile(b);
        (dest as *mut u64).add(2).write_volatile(c);
        (dest as *mut u64).add(3).write_volatile(d);
        dest += 32;
        source += 32;
        size -= 32;
    }

    /* 1x8 movnti loop */
    while size >= 8 {
        (dest as *mut u64).write_volatile((source as *const u64).read_unaligned());
        dest += 8;
        source += 8;
        size -= 8;
    }

    /* 1x4 movnti loop */
    while size >= 4 {
        (dest as *mut u32).write_volatile((source as *const u32).read_unaligned());
        dest += 4;
        source += 4;
        size -= 4;
    }

    /* cache copy for remaining bytes */
    if size != 0 {
        core::ptr::copy_nonoverlapping(source as *const u8, dest as *mut u8, size);
        clean_cache_range(dest as *mut core::ffi::c_void, size);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
