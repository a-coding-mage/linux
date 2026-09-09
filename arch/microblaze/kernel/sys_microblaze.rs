/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <john.williams@petalogix.com>
 *
 * Copyright (C) 2006 Atmark Techno, Inc.
 *\tYasushi SHOJI <yashi@atmark-techno.com>
 *\tTetsuya OHKAWA <tetsuya@atmark-techno.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the Linux kernel headers:
// linux/errno.h, linux/export.h, linux/mm.h, linux/smp.h, linux/syscalls.h,
// linux/sem.h, linux/msg.h, linux/shm.h, linux/stat.h, linux/mman.h,
// linux/sys.h, linux/ipc.h, linux/file.h, linux/err.h, linux/fs.h,
// linux/semaphore.h, linux/uaccess.h, linux/unistd.h, linux/slab.h,
// asm/syscalls.h

extern "C" {
    fn ksys_mmap_pgoff(
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        prot: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_ulong,
        fd: ::core::ffi::c_ulong,
        pgoff: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
}

// SYSCALL_DEFINE6(mmap, ...)
pub unsafe fn mmap(
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    prot: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    fd: ::core::ffi::c_ulong,
    pgoff: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    if pgoff & !PAGE_MASK != 0 {
        return -(EINVAL as ::core::ffi::c_long);
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff >> PAGE_SHIFT)
}

// SYSCALL_DEFINE6(mmap2, ...)
pub unsafe fn mmap2(
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    prot: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    fd: ::core::ffi::c_ulong,
    pgoff: ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    if pgoff & (!PAGE_MASK >> 12) != 0 {
        return -(EINVAL as ::core::ffi::c_long);
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff >> (PAGE_SHIFT - 12))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
