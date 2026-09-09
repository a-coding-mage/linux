/*
 * Copyright (C) 2004 PathScale, Inc
 * Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// C dependencies supplied by the surrounding build are intentionally not
// reimplemented here.

use core::ffi::c_void;
use libc::{c_int, c_long, size_t};

static mut ptrace_regset: libc::c_ulong = 0;
pub static mut host_fp_size: libc::c_ulong = 0;

extern "C" {
    fn ptrace(request: c_long, pid: c_int, addr: libc::c_ulong, data: *mut c_void) -> c_long;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int,
            fd: c_int, offset: libc::off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn printk(fmt: *const libc::c_char, ...);
}

extern "C" {
    pub static NT_X86_XSTATE: libc::c_ulong;
    pub static NT_PRFPREG: libc::c_ulong;
    #[cfg(CONFIG_X86_32)]
    pub static NT_PRXFPREG: libc::c_ulong;
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

pub unsafe fn get_fp_registers(pid: c_int, regs: *mut libc::c_ulong) -> c_int {
    let mut iov = iovec {
        iov_base: regs as *mut c_void,
        iov_len: host_fp_size as size_t,
    };

    if ptrace(libc::PTRACE_GETREGSET as c_long, pid, ptrace_regset,
              &mut iov as *mut iovec as *mut c_void) < 0 {
        return -(*__errno_location());
    }
    0
}

pub unsafe fn put_fp_registers(pid: c_int, regs: *mut libc::c_ulong) -> c_int {
    let mut iov = iovec {
        iov_base: regs as *mut c_void,
        iov_len: host_fp_size as size_t,
    };

    if ptrace(libc::PTRACE_SETREGSET as c_long, pid, ptrace_regset,
              &mut iov as *mut iovec as *mut c_void) < 0 {
        return -(*__errno_location());
    }
    0
}

pub unsafe fn arch_init_registers(pid: c_int) -> c_int {
    let mut iov = iovec {
        // Just use plenty of space, it does not cost us anything
        iov_base: core::ptr::null_mut(),
        iov_len: 2 * 1024 * 1024,
    };
    let mut ret: c_int;

    iov.iov_base = mmap(core::ptr::null_mut(), iov.iov_len,
                        libc::PROT_WRITE | libc::PROT_READ,
                        libc::MAP_ANONYMOUS | libc::MAP_PRIVATE, -1, 0);
    if iov.iov_base == libc::MAP_FAILED {
        return -libc::ENOMEM;
    }

    // GDB has x86_xsave_length, which uses x86_cpuid_count
    ptrace_regset = NT_X86_XSTATE;
    ret = ptrace(libc::PTRACE_GETREGSET as c_long, pid, ptrace_regset,
                 &mut iov as *mut iovec as *mut c_void) as c_int;
    if ret != 0 {
        ret = -(*__errno_location());
    }

    if ret == -libc::ENODEV {
        #[cfg(CONFIG_X86_32)]
        {
            ptrace_regset = NT_PRXFPREG;
        }
        #[cfg(not(CONFIG_X86_32))]
        {
            ptrace_regset = NT_PRFPREG;
        }
        iov.iov_len = 2 * 1024 * 1024;
        ret = ptrace(libc::PTRACE_GETREGSET as c_long, pid, ptrace_regset,
                     &mut iov as *mut iovec as *mut c_void) as c_int;
        if ret != 0 {
            ret = -(*__errno_location());
        }
    }

    munmap(iov.iov_base, 2 * 1024 * 1024);
    host_fp_size = iov.iov_len as libc::c_ulong;
    ret
}

#[repr(C)]
pub struct jmp_buf_x86 {
    pub __eip: libc::c_ulong,
    pub __esp: libc::c_ulong,
    pub __ebp: libc::c_ulong,
    pub __rip: libc::c_ulong,
    pub __rsp: libc::c_ulong,
    pub __rbp: libc::c_ulong,
}

pub unsafe fn get_thread_reg(reg: c_int, buf: *mut jmp_buf_x86) -> libc::c_ulong {
    #[cfg(__i386__)]
    {
        match reg {
            HOST_IP => (*buf).__eip,
            HOST_SP => (*buf).__esp,
            HOST_BP => (*buf).__ebp,
            _ => {
                printk(UM_KERN_ERR as *const libc::c_char, reg);
                0
            }
        }
    }
    #[cfg(not(__i386__))]
    {
        match reg {
            HOST_IP => (*buf).__rip,
            HOST_SP => (*buf).__rsp,
            HOST_BP => (*buf).__rbp,
            _ => {
                printk(UM_KERN_ERR as *const libc::c_char, reg);
                0
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
