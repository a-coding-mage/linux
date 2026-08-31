// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 *
 * Original: Michael Neuling 4/12/2013
 * Edited: Rashmica Gupta 4/12/2015
 *
 * See if the altivec state is leaked out of an aborted transaction due to
 * kernel vmx copy loops.
 *
 * When the transaction aborts, VSR values should rollback to the values
 * they held before the transaction commenced. Using VSRs while transaction
 * is suspended should not affect the checkpointed values.
 *
 * (1) write A to a VSR
 * (2) start transaction
 * (3) suspend transaction
 * (4) change the VSR to B
 * (5) trigger kernel vmx copy loop
 * (6) abort transaction
 * (7) check that the VSR value is A
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type uint64_t = u64;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    fn getpagesize() -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn unlink(pathname: *const c_char) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn is_ppc64le() -> c_int;
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

extern "C" fn test_vmxcopy() -> c_int {
    let vecin: f64 = 1.3;
    let mut vecout: f64 = 0.0;
    let pgsize: c_ulong = unsafe { getpagesize() as c_ulong };
    let mut i: c_int;
    let fd: c_int;
    let size: c_int = (pgsize * 16) as c_int;
    let mut tmpfile = *b"/tmp/page_faultXXXXXX\0";
    let mut buf = vec![0 as c_char; pgsize as usize];
    let a: *mut c_char;
    let aborted: uint64_t;

    unsafe {
        SKIP_IF!(have_htm() == 0);
        SKIP_IF!(htm_is_synthetic() != 0);
        SKIP_IF!(is_ppc64le() == 0);
    }

    fd = unsafe { mkstemp(tmpfile.as_mut_ptr() as *mut c_char) };
    assert!(fd >= 0);

    unsafe {
        memset(buf.as_mut_ptr() as *mut c_void, 0, pgsize as usize);
    }
    i = 0;
    while i < size {
        assert!(
            unsafe { write(fd, buf.as_ptr() as *const c_void, pgsize as usize) }
                == pgsize as isize
        );
        i += pgsize as c_int;
    }

    unsafe {
        unlink(tmpfile.as_ptr() as *const c_char);
    }

    a = unsafe {
        mmap(
            core::ptr::null_mut(),
            size as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE,
            fd,
            0,
        ) as *mut c_char
    };
    assert!(a as *mut c_void != MAP_FAILED);

    unsafe {
        asm!(
            "lxvd2x 40,0,{vecinptr};",
            "tbegin.;",
            "beq 3f;",
            "tsuspend.;",
            "xxlxor 40,40,40;",
            "std 5, 0({map});",
            "tabort. 0;",
            "tresume.;",
            "tend.;",
            "li {res}, 0;",
            "b 5f;",
            "3:",
            "li {res}, 1;",
            "5:",
            "stxvd2x 40,0,{vecoutptr};",
            res = lateout(reg) aborted,
            vecinptr = in(reg) &vecin,
            vecoutptr = in(reg) &mut vecout,
            map = in(reg) a,
            out("r0") _,
            out("r3") _,
            out("r4") _,
            out("r5") _,
            out("r6") _,
            out("r7") _,
        );
    }

    if aborted != 0 && vecin != vecout {
        unsafe {
            printf(
                b"FAILED: vector state leaked on abort %f != %f\n\0".as_ptr()
                    as *const c_char,
                vecin,
                vecout,
            );
        }
        return 1;
    }

    unsafe {
        munmap(a as *mut c_void, size as usize);
        close(fd);
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test_vmxcopy,
            b"tm_vmxcopy\0".as_ptr() as *const c_char,
        ));
    }
}
