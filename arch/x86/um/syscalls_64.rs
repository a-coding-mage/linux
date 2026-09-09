/*
 * Copyright (C) 2003 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Copyright 2003 PathScale, Inc.
 *
 * Licensed under the GPL
 */

// Dependencies supplied by the kernel and architecture-specific headers.
use core::ffi::c_int;

extern "C" {
    static mut current: *mut task_struct;
    fn put_user(value: c_ulong, ptr: *mut c_ulong) -> c_long;
    fn ksys_mmap_pgoff(
        addr: c_ulong,
        len: c_ulong,
        prot: c_ulong,
        flags: c_ulong,
        fd: c_ulong,
        pgoff: c_ulong,
    ) -> c_long;
}

type c_long = isize;
type c_ulong = usize;

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub regs: uml_pt_regs,
}

#[repr(C)]
pub struct uml_pt_regs {
    pub regs: Registers,
}

#[repr(C)]
pub struct Registers {
    pub gp: [c_ulong; 32],
}

// These constants are provided by the corresponding kernel architecture headers.
extern "C" {
    static ARCH_SET_FS: c_int;
    static ARCH_SET_GS: c_int;
    static ARCH_GET_FS: c_int;
    static ARCH_GET_GS: c_int;
    static FS_BASE: c_ulong;
    static GS_BASE: c_ulong;
    static EINVAL: c_long;
    static PAGE_MASK: c_ulong;
    static PAGE_SHIFT: c_ulong;
}

pub unsafe fn arch_prctl(
    task: *mut task_struct,
    option: c_int,
    arg2: *mut c_ulong,
) -> c_long {
    let mut ret: c_long = -EINVAL;

    match option {
        ARCH_SET_FS => {
            (*task).thread.regs.regs.gp[FS_BASE / core::mem::size_of::<c_ulong>()] = arg2 as c_ulong;
            ret = 0;
        }
        ARCH_SET_GS => {
            (*task).thread.regs.regs.gp[GS_BASE / core::mem::size_of::<c_ulong>()] = arg2 as c_ulong;
            ret = 0;
        }
        ARCH_GET_FS => {
            ret = put_user(
                (*task).thread.regs.regs.gp[FS_BASE / core::mem::size_of::<c_ulong>()],
                arg2,
            );
        }
        ARCH_GET_GS => {
            ret = put_user(
                (*task).thread.regs.regs.gp[GS_BASE / core::mem::size_of::<c_ulong>()],
                arg2,
            );
        }
        _ => {}
    }

    ret
}

// Translation of SYSCALL_DEFINE2(arch_prctl, int, option, unsigned long, arg2).
pub unsafe fn syscall_arch_prctl(option: c_int, arg2: c_ulong) -> c_long {
    arch_prctl(current, option, arg2 as *mut c_ulong)
}

pub unsafe fn arch_switch_to(to: *mut task_struct) {
    let _ = to;
    /*
     * Nothing needs to be done on x86_64.
     * The FS_BASE/GS_BASE registers are saved in the ptrace register set.
     */
}

// Translation of SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len,
// unsigned long, prot, unsigned long, flags, unsigned long, fd, unsigned long, off).
pub unsafe fn syscall_mmap(
    addr: c_ulong,
    len: c_ulong,
    prot: c_ulong,
    flags: c_ulong,
    fd: c_ulong,
    off: c_ulong,
) -> c_long {
    if off & !PAGE_MASK != 0 {
        return -EINVAL;
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
