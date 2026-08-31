// SPDX-License-Identifier: GPL-2.0
/*
 * Test that we can take signals with and without the VDSO mapped, which trigger
 * different paths in the signal handling code.
 *
 * See handle_rt_signal64() and setup_trampoline() in signal_64.c
 */

// Original C dependencies:
// errno.h, stdio.h, signal.h, stdlib.h, string.h, sys/mman.h, sys/types.h,
// unistd.h, assert.h, and "utils.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn mremap(
        old_address: *mut c_void,
        old_size: size_t,
        new_size: size_t,
        flags: c_int,
        new_address: *mut c_void,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;

    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

type size_t = usize;
type off_t = i64;
type pid_t = c_int;
type sig_atomic_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_flags: c_ulong,
    sa_restorer: *mut c_void,
    sa_mask: sigset_t,
}

const SIGUSR1: c_int = 10;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;

const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const MREMAP_MAYMOVE: c_int = 1;
const MREMAP_FIXED: c_int = 2;

unsafe fn search_proc_maps(
    needle: *mut c_char,
    low: *mut c_ulong,
    high: *mut c_ulong,
) -> c_int {
    let mut start: c_ulong = 0;
    let mut end: c_ulong = 0;
    static mut BUF: [c_char; 4096] = [0; 4096];
    let mut name: [c_char; 128] = [0; 128];
    let mut f: *mut FILE;
    let mut rc: c_int = -1;

    f = fopen(b"/proc/self/maps\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(b"fopen\0".as_ptr() as *const c_char);
        return -1;
    }

    while !fgets(BUF.as_mut_ptr(), BUF.len() as c_int, f).is_null() {
        rc = sscanf(
            BUF.as_ptr(),
            b"%lx-%lx %*c%*c%*c%*c %*x %*d:%*d %*d %127s\n\0".as_ptr() as *const c_char,
            &mut start as *mut c_ulong,
            &mut end as *mut c_ulong,
            name.as_mut_ptr(),
        );
        if rc == 2 {
            continue;
        }

        if rc != 3 {
            printf(b"sscanf errored\n\0".as_ptr() as *const c_char);
            rc = -1;
            break;
        }

        if !strstr(name.as_ptr(), needle as *const c_char).is_null() {
            *low = start;
            *high = end - 1;
            rc = 0;
            break;
        }
    }

    fclose(f);

    rc
}

static mut took_signal: sig_atomic_t = 0;

extern "C" fn sigusr1_handler(_sig: c_int) {
    unsafe {
        let current = core::ptr::read_volatile(core::ptr::addr_of!(took_signal));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(took_signal), current + 1);
    }
}

extern "C" fn test_sigreturn_vdso() -> c_int {
    unsafe {
        let mut low: c_ulong = 0;
        let mut high: c_ulong = 0;
        let mut size: c_ulong;
        let mut act: sigaction = core::mem::zeroed();
        let mut p: *mut c_char;

        act.sa_handler = sigusr1_handler;
        act.sa_flags = 0;
        sigemptyset(&mut act.sa_mask);

        assert!(sigaction(SIGUSR1, &act, core::ptr::null_mut()) == 0);

        // Confirm the VDSO is mapped, and work out where it is
        assert!(search_proc_maps(b"[vdso]\0".as_ptr() as *mut c_char, &mut low, &mut high) == 0);
        size = high - low + 1;
        printf(
            b"VDSO is at 0x%lx-0x%lx (%lu bytes)\n\0".as_ptr() as *const c_char,
            low,
            high,
            size,
        );

        kill(getpid(), SIGUSR1);
        assert!(core::ptr::read_volatile(core::ptr::addr_of!(took_signal)) == 1);
        printf(b"Signal delivered OK with VDSO mapped\n\0".as_ptr() as *const c_char);

        // Remap the VDSO somewhere else
        p = mmap(
            core::ptr::null_mut(),
            size as size_t,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        ) as *mut c_char;
        assert!(p as *mut c_void != MAP_FAILED);
        assert!(
            mremap(
                low as *mut c_void,
                size as size_t,
                size as size_t,
                MREMAP_MAYMOVE | MREMAP_FIXED,
                p as *mut c_void,
            ) != MAP_FAILED
        );
        assert!(search_proc_maps(b"[vdso]\0".as_ptr() as *mut c_char, &mut low, &mut high) == 0);
        size = high - low + 1;
        printf(
            b"VDSO moved to 0x%lx-0x%lx (%lu bytes)\n\0".as_ptr() as *const c_char,
            low,
            high,
            size,
        );

        kill(getpid(), SIGUSR1);
        assert!(core::ptr::read_volatile(core::ptr::addr_of!(took_signal)) == 2);
        printf(b"Signal delivered OK with VDSO moved\n\0".as_ptr() as *const c_char);

        assert!(munmap(low as *mut c_void, size as size_t) == 0);
        printf(b"Unmapped VDSO\n\0".as_ptr() as *const c_char);

        // Confirm the VDSO is not mapped anymore
        assert!(search_proc_maps(b"[vdso]\0".as_ptr() as *mut c_char, &mut low, &mut high) != 0);

        // Make the stack executable
        assert!(search_proc_maps(b"[stack]\0".as_ptr() as *mut c_char, &mut low, &mut high) == 0);
        size = high - low + 1;
        mprotect(
            low as *mut c_void,
            size as size_t,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );
        printf(b"Remapped the stack executable\n\0".as_ptr() as *const c_char);

        kill(getpid(), SIGUSR1);
        assert!(core::ptr::read_volatile(core::ptr::addr_of!(took_signal)) == 3);
        printf(b"Signal delivered OK with VDSO unmapped\n\0".as_ptr() as *const c_char);

        0
    }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test_sigreturn_vdso,
            b"sigreturn_vdso\0".as_ptr() as *const c_char,
        ));
    }
}
