// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * Copyright 2020 IBM Corp.
 *
 * Author: Bulent Abali <abali@us.ibm.com>
 *
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

macro_rules! barrier {
    () => {};
}

macro_rules! hwsync {
    () => {
        unsafe {
            asm!("sync", options(nostack, preserves_flags));
        }
    };
}

#[cfg(not(NX_NO_CPU_PRI))]
macro_rules! cpu_pri_default {
    () => {
        unsafe {
            asm!("or 2, 2, 2", options(nostack, preserves_flags));
        }
    };
}

#[cfg(not(NX_NO_CPU_PRI))]
macro_rules! cpu_pri_low {
    () => {
        unsafe {
            asm!("or 31, 31, 31", options(nostack, preserves_flags));
        }
    };
}

#[cfg(NX_NO_CPU_PRI)]
macro_rules! cpu_pri_default {
    () => {};
}

#[cfg(NX_NO_CPU_PRI)]
macro_rules! cpu_pri_low {
    () => {};
}

pub static mut nx_fault_storage_address: *mut c_void = ptr::null_mut();

#[repr(C)]
struct nx_handle {
    fd: c_int,
    function: c_int,
    paste_addr: *mut c_void,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: u64,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> c_int;
    fn __ppc_get_timebase() -> u64;

    fn vas_copy(crb: *mut nx_gzip_crb_t, offset: c_int);
    fn vas_paste(paste_addr: *mut c_void, offset: c_int) -> c_int;
    fn prt_err(format: *const c_char, ...);
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type c_uint = u32;

const O_RDWR: c_int = 0o2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const ETIMEDOUT: c_int = 110;

const CSB_MAX_POLL: c_ulong = 200000000;
const USLEEP_TH: u64 = 300000;
const SPIN_TH: u64 = 500;

/*
 * External constants, macros, and types are supplied by vas-api.h, nx.h,
 * copy-paste.h, nxu.h, and nx_dbg.h in the original repository.
 */

unsafe fn open_device_nodes(devname: *mut c_char, pri: c_int, handle: *mut nx_handle) -> c_int {
    let mut rc: c_int;
    let fd: c_int;
    let addr: *mut c_void;
    let mut txattr: vas_tx_win_open_attr = mem::zeroed();

    fd = open(devname, O_RDWR);
    if fd < 0 {
        fprintf(stderr, c" open device name %s\n".as_ptr(), devname);
        return -errno;
    }

    memset(
        (&mut txattr as *mut vas_tx_win_open_attr).cast::<c_void>(),
        0,
        mem::size_of::<vas_tx_win_open_attr>(),
    );
    txattr.version = 1;
    txattr.vas_id = pri;
    rc = ioctl(
        fd,
        VAS_TX_WIN_OPEN as c_ulong,
        (&mut txattr as *mut vas_tx_win_open_attr) as c_ulong,
    );
    if rc < 0 {
        fprintf(stderr, c"ioctl() n %d, error %d\n".as_ptr(), rc, errno);
        rc = -errno;
        close(fd);
        return rc;
    }

    addr = mmap(
        ptr::null_mut(),
        4096,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0u64,
    );
    if addr == MAP_FAILED {
        fprintf(stderr, c"mmap() failed, errno %d\n".as_ptr(), errno);
        rc = -errno;
        close(fd);
        return rc;
    }
    (*handle).fd = fd;
    (*handle).paste_addr = (addr as *mut c_char).add(0x400).cast::<c_void>();

    rc = 0;
    close(fd);
    rc
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nx_function_begin(function: c_int, pri: c_int) -> *mut c_void {
    let rc: c_int;
    let devname = c"/dev/crypto/nx-gzip".as_ptr() as *mut c_char;
    let nxhandle: *mut nx_handle;

    if function != NX_FUNC_COMP_GZIP {
        errno = EINVAL;
        fprintf(stderr, c" NX_FUNC_COMP_GZIP not found\n".as_ptr());
        return ptr::null_mut();
    }

    nxhandle = malloc(mem::size_of::<nx_handle>()).cast::<nx_handle>();
    if nxhandle.is_null() {
        errno = ENOMEM;
        fprintf(stderr, c" No memory\n".as_ptr());
        return ptr::null_mut();
    }

    (*nxhandle).function = function;
    rc = open_device_nodes(devname, pri, nxhandle);
    if rc < 0 {
        errno = -rc;
        fprintf(stderr, c" open_device_nodes failed\n".as_ptr());
        return ptr::null_mut();
    }

    nxhandle.cast::<c_void>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nx_function_end(handle: *mut c_void) -> c_int {
    let mut rc: c_int = 0;
    let nxhandle = handle.cast::<nx_handle>();

    rc = munmap(((*nxhandle).paste_addr as *mut c_char).sub(0x400).cast::<c_void>(), 4096);
    if rc < 0 {
        fprintf(stderr, c"munmap() failed, errno %d\n".as_ptr(), errno);
        return rc;
    }
    close((*nxhandle).fd);
    free(nxhandle.cast::<c_void>());

    rc
}

unsafe fn nx_wait_for_csb(cmdp: *mut nx_gzip_crb_cpb_t) -> c_int {
    let mut poll: c_long = 0;
    let t: u64;

    /*
     * Save power and let other threads use the h/w. top may show
     * 100% but only because OS doesn't know we slowed the this
     * h/w thread while polling. We're letting other threads have
     * higher throughput on the core.
     */
    cpu_pri_low!();

    t = __ppc_get_timebase();

    while getnn!((*cmdp).crb.csb, csb_v) == 0 {
        poll += 1;
        hwsync!();

        cpu_pri_low!();

        /*
         * usleep(0) takes around 29000 ticks ~60 us.
         * 300000 is spinning for about 600 us then
         * start sleeping.
         */
        if (__ppc_get_timebase().wrapping_sub(t)) > USLEEP_TH {
            cpu_pri_default!();
            usleep(1);
        }

        if (poll as c_ulong) > CSB_MAX_POLL {
            break;
        }

        /* Fault address from signal handler */
        if !nx_fault_storage_address.is_null() {
            cpu_pri_default!();
            return -EAGAIN;
        }
    }

    cpu_pri_default!();

    /* hw has updated csb and output buffer */
    hwsync!();

    /* Check CSB flags. */
    if getnn!((*cmdp).crb.csb, csb_v) == 0 {
        fprintf(
            stderr,
            c"CSB still not valid after %d polls.\n".as_ptr(),
            poll as c_int,
        );
        prt_err(
            c"CSB still not valid after %d polls, giving up.\n".as_ptr(),
            poll as c_int,
        );
        return -ETIMEDOUT;
    }

    0
}

unsafe fn nxu_run_job(cmdp: *mut nx_gzip_crb_cpb_t, handle: *mut c_void) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;
    let retries: c_int;
    let nxhandle = handle.cast::<nx_handle>();

    assert!(!handle.is_null());
    i = 0;
    retries = 5000;
    while {
        i += 1;
        i < retries + 1
    } {
        hwsync!();
        vas_copy(&mut (*cmdp).crb as *mut nx_gzip_crb_t, 0);
        ret = vas_paste((*nxhandle).paste_addr, 0);
        hwsync!();

        NXPRT!(fprintf(
            stderr,
            c"Paste attempt %d/%d returns 0x%x\n".as_ptr(),
            i,
            retries,
            ret,
        ));

        if (ret == 2) || (ret == 3) {
            ret = nx_wait_for_csb(cmdp);
            if ret == 0 {
                break;
            } else if ret == -EAGAIN {
                let x: c_long;

                prt_err(
                    c"Touching address %p, 0x%lx\n".as_ptr(),
                    nx_fault_storage_address,
                    *(nx_fault_storage_address.cast::<c_long>()),
                );
                x = *(nx_fault_storage_address.cast::<c_long>());
                *(nx_fault_storage_address.cast::<c_long>()) = x;
                nx_fault_storage_address = ptr::null_mut();
                continue;
            } else {
                prt_err(c"wait_for_csb() returns %d\n".as_ptr(), ret);
                break;
            }
        } else {
            if i < 10 {
                /* spin for few ticks */
                let fail_spin: u64;

                fail_spin = __ppc_get_timebase();
                while (__ppc_get_timebase().wrapping_sub(fail_spin)) < SPIN_TH {}
            } else {
                /* sleep */
                let mut pr: c_uint = 0;

                if {
                    let old = pr;
                    pr = pr.wrapping_add(1);
                    old % 100 == 0
                } {
                    prt_err(c"Paste attempt %d/".as_ptr(), i);
                    prt_err(c"%d, failed pid= %d\n".as_ptr(), retries, getpid());
                }
                usleep(1);
            }
            continue;
        }
    }

    cpu_pri_default!();

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
