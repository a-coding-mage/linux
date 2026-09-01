/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint};

type sig_atomic_t = c_int;

const SIGINT: c_int = 2;
const SIGALRM: c_int = 14;

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>) -> usize;
    fn alarm(seconds: c_uint) -> c_uint;
}

/* We want to check these symbols in perf script */
#[cfg(not(target_arch = "aarch64"))]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn leaf() {
    while core::ptr::read_volatile(&raw const done) == 0 {}
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn parent() {
    unsafe {
        leaf();
    }
}

#[export_name = "leafloop_done"]
static mut done: sig_atomic_t = 0;

unsafe extern "C" fn sighandler(_sig: c_int) {
    unsafe {
        core::ptr::write_volatile(&raw mut done, 1);
    }
}

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    r#"
    .pushsection .text,"ax",%progbits
    .global leaf
    .type leaf, %function
leaf:
    adrp    x1, leafloop_done
    ldr     w2, [x1, #:lo12:leafloop_done]
    cbz     w2, leaf
    ret
    .size leaf, .-leaf
    .popsection
"#
);

unsafe fn leafloop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut sec: c_int = 1;

    if argc > 0 {
        unsafe {
            sec = atoi(*argv);
        }
    }

    unsafe {
        signal(SIGINT, Some(sighandler));
        signal(SIGALRM, Some(sighandler));
        alarm(sec as c_uint);

        parent();
    }
    0
}

/* C dependency intent: DEFINE_WORKLOAD(leafloop); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
