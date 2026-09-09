// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependency supplied by <linux/syscalls.h>.

extern "C" {
    static mut current: *mut task_struct;

    fn current_pt_regs() -> *mut pt_regs;
    fn ksys_mmap_pgoff(
        addr: c_ulong,
        len: c_ulong,
        prot: c_ulong,
        flags: c_ulong,
        fd: c_ulong,
        pgoff: c_ulong,
    ) -> c_long;
    fn ksys_fadvise64_64(
        fd: c_int,
        offset: loff_t,
        len: loff_t,
        advice: c_int,
    ) -> c_long;
}

// Types, constants, and helpers supplied by the kernel headers.
#[allow(non_camel_case_types)]
type c_ulong = ::core::ffi::c_ulong;
#[allow(non_camel_case_types)]
type c_long = ::core::ffi::c_long;
#[allow(non_camel_case_types)]
type c_int = ::core::ffi::c_int;
#[allow(non_camel_case_types)]
type loff_t = i64;

#[repr(C)]
struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct thread_info {
    tp_value: c_ulong,
}

#[repr(C)]
struct pt_regs {
    tls: c_ulong,
}

unsafe fn task_thread_info(task: *mut task_struct) -> *mut thread_info {
    task as *mut thread_info
}

const PAGE_MASK: c_ulong = !0;
const PAGE_SHIFT: c_ulong = 12;
const EINVAL: c_long = 22;

#[no_mangle]
pub unsafe extern "C" fn set_thread_area(addr: c_ulong) -> c_long {
    let ti: *mut thread_info = task_thread_info(current);
    let reg: *mut pt_regs = current_pt_regs();

    (*reg).tls = addr;
    (*ti).tp_value = addr;

    0
}

#[no_mangle]
pub unsafe extern "C" fn mmap2(
    addr: c_ulong,
    len: c_ulong,
    prot: c_ulong,
    flags: c_ulong,
    fd: c_ulong,
    offset: c_ulong,
) -> c_long {
    if (offset & (!(PAGE_MASK) >> 12)) != 0 {
        return -EINVAL;
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> (PAGE_SHIFT - 12))
}

/*
 * for abiv1 the 64bits args should be even th, So we need mov the advice
 * forward.
 */
#[no_mangle]
pub unsafe extern "C" fn csky_fadvise64_64(
    fd: c_int,
    advice: c_int,
    offset: loff_t,
    len: loff_t,
) -> c_long {
    ksys_fadvise64_64(fd, offset, len, advice)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
