/* SPDX-License-Identifier: LGPL-2.1 */

use libc::{c_char, c_int, rlimit, RLIMIT_MEMLOCK, RLIMIT_NOFILE};

// From "util/rlimit.h".
pub type rlimit_action = c_int;

extern "C" {
    static NO_CHANGE: rlimit_action;
    static INCREASED_MAX: rlimit_action;

    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn __errno_location() -> *mut c_int;

    // From "util/debug.h".
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
}

/*
 * Bump the memlock so that we can get bpf maps of a reasonable size,
 * like the ones used with 'perf trace' and with 'perf test bpf',
 * improve this to some specific request if needed.
 */
#[no_mangle]
pub unsafe extern "C" fn rlimit__bump_memlock() {
    let mut rlim: rlimit = std::mem::zeroed();

    if getrlimit(RLIMIT_MEMLOCK as c_int, &mut rlim) == 0 {
        rlim.rlim_cur *= 4;
        rlim.rlim_max *= 4;

        if setrlimit(RLIMIT_MEMLOCK as c_int, &rlim) < 0 {
            rlim.rlim_cur /= 2;
            rlim.rlim_max /= 2;

            if setrlimit(RLIMIT_MEMLOCK as c_int, &rlim) < 0 {
                pr_debug(
                    b"Couldn't bump rlimit(MEMLOCK), failures may take place when creating BPF maps, etc\n\0"
                        .as_ptr() as *const c_char,
                );
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rlimit__increase_nofile(set_rlimit: *mut rlimit_action) -> bool {
    let old_errno: c_int;
    let mut l: rlimit = std::mem::zeroed();

    if *set_rlimit < INCREASED_MAX {
        old_errno = *__errno_location();

        if getrlimit(RLIMIT_NOFILE as c_int, &mut l) == 0 {
            if *set_rlimit == NO_CHANGE {
                l.rlim_cur = l.rlim_max;
            } else {
                l.rlim_cur = l.rlim_max + 1000;
                l.rlim_max = l.rlim_cur;
            }
            if setrlimit(RLIMIT_NOFILE as c_int, &l) == 0 {
                *set_rlimit += 1;
                *__errno_location() = old_errno;
                return true;
            }
        }
        *__errno_location() = old_errno;
    }

    false
}
