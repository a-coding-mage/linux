// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/kernel/sys_sh.c
 *
 * This file contains various random system calls that
 * have a non-standard calling sequence on the Linux/SuperH
 * platform.
 *
 * Taken from i386 version.
 */

// Linux and SuperH dependencies are supplied by the surrounding kernel.

pub unsafe fn old_mmap(
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    prot: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    fd: ::core::ffi::c_int,
    off: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    if off & !PAGE_MASK != 0 {
        return -EINVAL;
    }
    ksys_mmap_pgoff(addr, len, prot, flags, fd as ::core::ffi::c_ulong, off >> PAGE_SHIFT)
}

pub unsafe fn sys_mmap2(
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    prot: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    fd: ::core::ffi::c_ulong,
    mut pgoff: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    /*
     * The shift for mmap2 is constant, regardless of PAGE_SIZE
     * setting.
     */
    if pgoff & ((1 << (PAGE_SHIFT - 12)) - 1) != 0 {
        return -EINVAL as ::core::ffi::c_long;
    }

    pgoff >>= PAGE_SHIFT - 12;

    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff)
}

/* sys_cacheflush -- flush (part of) the processor cache.  */
pub unsafe fn sys_cacheflush(
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut vma: *mut vm_area_struct;

    if (op <= 0) || (op > (CACHEFLUSH_D_PURGE | CACHEFLUSH_I)) {
        return -EINVAL;
    }

    /*
     * Verify that the specified address region actually belongs
     * to this process.
     */
    if addr.wrapping_add(len) < addr {
        return -EFAULT;
    }

    mmap_read_lock((*current).mm);
    vma = find_vma((*current).mm, addr);
    if vma.is_null()
        || addr < (*vma).vm_start
        || addr.wrapping_add(len) > (*vma).vm_end
    {
        mmap_read_unlock((*current).mm);
        return -EFAULT;
    }

    match op & CACHEFLUSH_D_PURGE {
        CACHEFLUSH_D_INVAL => {
            __flush_invalidate_region(addr as *mut ::core::ffi::c_void, len);
        }
        CACHEFLUSH_D_WB => {
            __flush_wback_region(addr as *mut ::core::ffi::c_void, len);
        }
        CACHEFLUSH_D_PURGE => {
            __flush_purge_region(addr as *mut ::core::ffi::c_void, len);
        }
        _ => {}
    }

    if op & CACHEFLUSH_I != 0 {
        flush_icache_range(addr, addr.wrapping_add(len));
    }

    mmap_read_unlock((*current).mm);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
