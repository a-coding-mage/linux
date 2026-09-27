// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2014 Darius Rad <darius@bluespec.com>
 * Copyright (C) 2017 SiFive
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn ksys_mmap_pgoff(
        addr: ::kernel::ffi::c_ulong,
        len: ::kernel::ffi::c_ulong,
        prot: ::kernel::ffi::c_ulong,
        flags: ::kernel::ffi::c_ulong,
        fd: ::kernel::ffi::c_ulong,
        pgoff: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_long;
    fn flush_icache_mm(mm: *mut ::kernel::ffi::c_void, local: ::kernel::ffi::c_ulong);
    static mut current: *mut ::kernel::ffi::c_void;
}

const EINVAL: ::kernel::ffi::c_long = 22;
const ENOSYS: ::kernel::ffi::c_long = 38;

// These values are supplied by the kernel headers in the original source.
const PAGE_MASK: ::kernel::ffi::c_ulong = !0;
const PAGE_SHIFT: ::kernel::ffi::c_ulong = 12;
const PROT_READ: ::kernel::ffi::c_ulong = 0x1;
const PROT_WRITE: ::kernel::ffi::c_ulong = 0x2;
const SYS_RISCV_FLUSH_ICACHE_ALL: ::kernel::ffi::c_ulong = 0x1;
const SYS_RISCV_FLUSH_ICACHE_LOCAL: ::kernel::ffi::c_ulong = 0x2;

unsafe fn riscv_sys_mmap(
    addr: ::kernel::ffi::c_ulong,
    len: ::kernel::ffi::c_ulong,
    mut prot: ::kernel::ffi::c_ulong,
    flags: ::kernel::ffi::c_ulong,
    fd: ::kernel::ffi::c_ulong,
    offset: ::kernel::ffi::c_ulong,
    page_shift_offset: ::kernel::ffi::c_ulong,
) -> ::kernel::ffi::c_long {
    if (offset & (!PAGE_MASK >> page_shift_offset)) != 0 {
        return -EINVAL;
    }

    /*
     * If PROT_WRITE is specified then extend that to PROT_READ
     * protection_map[VM_WRITE] is now going to select shadow stack encodings.
     * So specifying PROT_WRITE actually should select protection_map [VM_WRITE | VM_READ]
     * If user wants to create shadow stack then they should use `map_shadow_stack` syscall.
     */
    if (prot & PROT_WRITE) != 0 && (prot & PROT_READ) == 0 {
        prot |= PROT_READ;
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> (PAGE_SHIFT - page_shift_offset))
}

#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mmap(
    addr: ::kernel::ffi::c_ulong,
    len: ::kernel::ffi::c_ulong,
    prot: ::kernel::ffi::c_ulong,
    flags: ::kernel::ffi::c_ulong,
    fd: ::kernel::ffi::c_ulong,
    offset: ::kernel::ffi::c_ulong,
) -> ::kernel::ffi::c_long {
    riscv_sys_mmap(addr, len, prot, flags, fd, offset, 0)
}

#[cfg(any(target_pointer_width = "32", feature = "compat"))]
pub unsafe extern "C" fn mmap2(
    addr: ::kernel::ffi::c_ulong,
    len: ::kernel::ffi::c_ulong,
    prot: ::kernel::ffi::c_ulong,
    flags: ::kernel::ffi::c_ulong,
    fd: ::kernel::ffi::c_ulong,
    offset: ::kernel::ffi::c_ulong,
) -> ::kernel::ffi::c_long {
    /* Note that the shift for mmap2 is constant (12), regardless of PAGE_SIZE. */
    riscv_sys_mmap(addr, len, prot, flags, fd, offset, 12)
}

/*
 * Allows the instruction cache to be flushed from userspace.  Despite RISC-V
 * having a direct 'fence.i' instruction available to userspace (which we
 * can't trap!), that's not actually viable when running on Linux because the
 * kernel might schedule a process on another hart.  There is no way for
 * userspace to handle this without invoking the kernel (as it doesn't know the
 * thread->hart mappings), so we've defined a RISC-V specific system call to
 * flush the instruction cache.
 *
 * sys_riscv_flush_icache() is defined to flush the instruction cache over an
 * address range, with the flush applying to either all threads or just the
 * caller.  We don't currently do anything with the address range, that's just
 * in there for forwards compatibility.
 */
pub unsafe extern "C" fn riscv_flush_icache(
    _start: usize,
    _end: usize,
    flags: usize,
) -> ::kernel::ffi::c_long {
    /* Check the reserved flags. */
    if (flags as ::kernel::ffi::c_ulong & !SYS_RISCV_FLUSH_ICACHE_ALL) != 0 {
        return -EINVAL;
    }

    flush_icache_mm(
        (*(core::ptr::addr_of_mut!(current))).cast::<::kernel::ffi::c_void>(),
        flags as ::kernel::ffi::c_ulong & SYS_RISCV_FLUSH_ICACHE_LOCAL,
    );

    0
}

/* Not defined using SYSCALL_DEFINE0 to avoid error injection */
pub unsafe extern "C" fn __riscv_sys_ni_syscall(
    _unused: *const ::kernel::ffi::c_void,
) -> ::kernel::ffi::c_long {
    -ENOSYS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
