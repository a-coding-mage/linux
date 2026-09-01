// SPDX-License-Identifier: GPL-2.0
/*
 * Test that loads/stores expand the stack segment, or trigger a SEGV, in
 * various conditions.
 *
 * Based on test code by Tom Lane.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

type pid_t = c_int;
type rlim_t = c_ulong;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

const RLIMIT_STACK: c_int = 3;

const _KB: c_ulong = 1024;
const _MB: c_ulong = 1024 * 1024;

static mut stack_top_ptr: *mut c_char = core::ptr::null_mut();
static mut stack_top_sp: c_ulong = 0;
static mut c: c_char = 0;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum access_type {
    LOAD,
    STORE,
}

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn getpagesize() -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;

    /*
     * In the original C source this declaration is provided by "utils.h" only
     * for __powerpc__ builds.
     */
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

/*
 * Consume stack until the stack pointer is below @target_sp, then do an access
 * (load or store) at offset @delta from either the base of the stack or the
 * current stack pointer.
 */
#[inline(never)]
unsafe extern "C" fn consume_stack(
    target_sp: c_ulong,
    stack_high: c_ulong,
    delta: c_int,
    type_: access_type,
) -> c_int {
    let target: c_ulong;
    let stack_cur: c_char = 0;

    if (&stack_cur as *const c_char as c_ulong) > target_sp {
        return unsafe { consume_stack(target_sp, stack_high, delta, type_) };
    } else {
        /*
         * We don't really need this, but without it GCC might not
         * generate a recursive call above.
         */
        unsafe {
            stack_top_ptr = &stack_cur as *const c_char as *mut c_char;
        }

        #[cfg(target_arch = "powerpc")]
        unsafe {
            asm!("mr {sp}, r1", sp = out(reg) stack_top_sp, options(nostack, preserves_flags));
        }
        #[cfg(target_arch = "powerpc64")]
        unsafe {
            asm!("mr {sp}, r1", sp = out(reg) stack_top_sp, options(nostack, preserves_flags));
        }
        #[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
        unsafe {
            asm!("mov {}, rsp", out(reg) stack_top_sp, options(nostack, preserves_flags));
        }

        target = stack_high.wrapping_sub(delta as c_ulong).wrapping_add(1);
        let p: *mut c_char = target as *mut c_char;

        if type_ == access_type::STORE {
            unsafe {
                core::ptr::write_volatile(p, c);
            }
        } else {
            unsafe {
                c = core::ptr::read_volatile(p);
            }
        }

        /*
         * Do something to prevent the stack frame being popped prior to
         * our access above.
         */
        unsafe {
            getpid();
        }
    }

    0
}

unsafe extern "C" fn search_proc_maps(
    needle: *mut c_char,
    low: *mut c_ulong,
    high: *mut c_ulong,
) -> c_int {
    let mut start: c_ulong = 0;
    let mut end: c_ulong = 0;
    static mut buf: [c_char; 4096] = [0; 4096];
    let mut name: [c_char; 128] = [0; 128];
    let f: *mut FILE;
    let mut rc: c_int;

    unsafe {
        f = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    }
    if f.is_null() {
        unsafe {
            perror(c"fopen".as_ptr());
        }
        return -1;
    }

    rc = 0;
    while unsafe { !fgets(core::ptr::addr_of_mut!(buf) as *mut c_char, buf.len() as c_int, f).is_null() } {
        rc = unsafe {
            sscanf(
                core::ptr::addr_of!(buf) as *const c_char,
                c"%lx-%lx %*c%*c%*c%*c %*x %*d:%*d %*d %127s\n".as_ptr(),
                &mut start as *mut c_ulong,
                &mut end as *mut c_ulong,
                name.as_mut_ptr(),
            )
        };
        if rc == 2 {
            continue;
        }

        if rc != 3 {
            unsafe {
                printf(c"sscanf errored\n".as_ptr());
            }
            rc = -1;
            break;
        }

        if unsafe { !strstr(name.as_ptr(), needle).is_null() } {
            unsafe {
                *low = start;
                *high = end.wrapping_sub(1);
            }
            rc = 0;
            break;
        }
    }

    unsafe {
        fclose(f);
    }

    rc
}

unsafe extern "C" fn child(stack_used: c_uint, delta: c_int, type_: access_type) -> c_int {
    let mut low: c_ulong = 0;
    let mut stack_high: c_ulong = 0;

    assert!(
        unsafe {
            search_proc_maps(
                c"[stack]".as_ptr() as *mut c_char,
                &mut low as *mut c_ulong,
                &mut stack_high as *mut c_ulong,
            )
        } == 0
    );

    assert!(
        unsafe {
            consume_stack(
                stack_high.wrapping_sub(stack_used as c_ulong),
                stack_high,
                delta,
                type_,
            )
        } == 0
    );

    unsafe {
        printf(
            c"Access OK: %s delta %-7d used size 0x%06x stack high 0x%lx top_ptr %p top sp 0x%lx actual used 0x%lx\n".as_ptr(),
            if type_ == access_type::LOAD {
                c"load".as_ptr()
            } else {
                c"store".as_ptr()
            },
            delta,
            stack_used,
            stack_high,
            stack_top_ptr as *mut c_void,
            stack_top_sp,
            stack_high.wrapping_sub(stack_top_sp).wrapping_add(1),
        );
    }

    0
}

unsafe extern "C" fn test_one(stack_used: c_uint, delta: c_int, type_: access_type) -> c_int {
    let pid: pid_t;
    let mut rc: c_int = 0;

    unsafe {
        pid = fork();
    }
    if pid == 0 {
        unsafe {
            exit(child(stack_used, delta, type_));
        }
    }

    assert!(unsafe { waitpid(pid, &mut rc as *mut c_int, 0) } != -1);

    if WIFEXITED(rc) && WEXITSTATUS(rc) == 0 {
        return 0;
    }

    /* We don't expect a non-zero exit that's not a signal */
    assert!(!WIFEXITED(rc));

    unsafe {
        printf(
            c"Faulted:   %s delta %-7d used size 0x%06x signal %d\n".as_ptr(),
            if type_ == access_type::LOAD {
                c"load".as_ptr()
            } else {
                c"store".as_ptr()
            },
            delta,
            stack_used,
            WTERMSIG(rc),
        );
    }

    1
}

/*
 * This is fairly arbitrary but is well below any of the targets below,
 * so that the delta between the stack pointer and the target is large.
 */
const DEFAULT_SIZE: c_uint = 32 * _KB as c_uint;

unsafe extern "C" fn test_one_type(type_: access_type, page_size: c_ulong, rlim_cur: c_ulong) {
    let mut delta: c_ulong;

    /* We should be able to access anywhere within the rlimit */
    delta = page_size;
    while delta <= rlim_cur {
        assert!(unsafe { test_one(DEFAULT_SIZE, delta as c_int, type_) } == 0);
        delta = delta.wrapping_add(page_size);
    }

    assert!(unsafe { test_one(DEFAULT_SIZE, rlim_cur as c_int, type_) } == 0);

    /* But if we go past the rlimit it should fail */
    assert!(unsafe { test_one(DEFAULT_SIZE, rlim_cur.wrapping_add(1) as c_int, type_) } != 0);
}

unsafe extern "C" fn test() -> c_int {
    let page_size: c_ulong;
    let mut rlimit: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    page_size = unsafe { getpagesize() as c_ulong };
    unsafe {
        getrlimit(RLIMIT_STACK, &mut rlimit as *mut rlimit);
        printf(
            c"Stack rlimit is 0x%llx\n".as_ptr(),
            rlimit.rlim_cur as c_ulong,
        );
    }

    unsafe {
        printf(c"Testing loads ...\n".as_ptr());
    }
    unsafe {
        test_one_type(access_type::LOAD, page_size, rlimit.rlim_cur);
    }
    unsafe {
        printf(c"Testing stores ...\n".as_ptr());
    }
    unsafe {
        test_one_type(access_type::STORE, page_size, rlimit.rlim_cur);
    }

    unsafe {
        printf(c"All OK\n".as_ptr());
    }

    0
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
fn main() {
    unsafe {
        test_harness(test, c"stack_expansion_ldst".as_ptr());
    }
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
fn main() {
    unsafe {
        test();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
