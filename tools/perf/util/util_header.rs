/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency intent: dirent.h, fcntl.h, stdbool.h, stddef.h,
 * linux/compiler.h, linux/bitmap.h, sys/types.h, and, outside C++,
 * internal/cpumap.h.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

pub type mode_t = c_uint;

#[repr(C)]
pub struct dirent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static perf_usage_string: c_char;
    pub static perf_more_info_string: c_char;

    pub static mut input_name: *const c_char;

    /* This will control if perf_{host,guest} will set attr.exclude_{host,guest}. */
    pub static mut exclude_GH_default: bool;

    pub static mut perf_host: bool;
    pub static mut perf_guest: bool;

    /* General helper functions */
    pub fn usage(err: *const c_char) -> !;

    pub fn mkdir_p(path: *mut c_char, mode: mode_t) -> c_int;
    pub fn rm_rf(path: *const c_char) -> c_int;
    pub fn rm_rf_perf_data(path: *const c_char) -> c_int;
    pub fn lsdir(
        name: *const c_char,
        filter: Option<unsafe extern "C" fn(*const c_char, *mut dirent) -> bool>,
    ) -> *mut strlist;
    pub fn lsdir_no_dot_filter(name: *const c_char, d: *mut dirent) -> bool;

    pub fn hex_width(v: u64) -> usize;

    pub fn sysctl__max_stack() -> c_int;

    pub fn sysctl__nmi_watchdog_enabled() -> bool;

    pub fn perf_tip(strp: *mut *mut c_char, dirpath: *const c_char) -> c_int;

    pub fn cpumask_to_cpulist(cpumask: *mut c_char, cpulist: *mut c_char);

    pub fn print_separator2(pre_dash_cnt: c_int, s: *const c_char, post_dash_cnt: c_int);

    /* If HAVE_SCHED_GETCPU_SUPPORT is not provided by the build, C declares this. */
    pub fn sched_getcpu() -> c_int;

    /* If HAVE_SCANDIRAT_SUPPORT is not provided by the build, C declares this. */
    pub fn scandirat(
        dirfd: c_int,
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;

    pub static mut perf_singlethreaded: bool;

    pub fn perf_set_singlethreaded();
    pub fn perf_set_multithreaded();

    pub fn perf_exe(buf: *mut c_char, len: c_int) -> *mut c_char;

    pub fn perf_debuginfod_setup(di: *mut perf_debuginfod);

    pub fn perf_basename(path: *const c_char) -> *const c_char;

    pub fn filename_with_chroot(pid: c_int, filename: *const c_char) -> *mut c_char;

    pub fn do_realloc_array_as_needed(
        arr: *mut *mut c_void,
        arr_sz: *mut usize,
        x: usize,
        msz: usize,
        init_val: *const c_void,
    ) -> c_int;
}

/* Fallback O_CLOEXEC values used when the platform headers do not define it.
 * C selects 0x400000 on sparc, 010000000 on alpha/hppa, and 02000000 otherwise.
 */
#[cfg(target_arch = "sparc")]
pub const O_CLOEXEC: c_int = 0x400000;
#[cfg(any(target_arch = "alpha", target_arch = "hppa"))]
pub const O_CLOEXEC: c_int = 0o10000000;
#[cfg(not(any(target_arch = "sparc", target_arch = "alpha", target_arch = "hppa")))]
pub const O_CLOEXEC: c_int = 0o2000000;

#[repr(C)]
pub struct perf_debuginfod {
    pub urls: *const c_char,
    pub set: bool,
}

#[macro_export]
macro_rules! realloc_array_as_needed {
    ($a:expr, $n:expr, $x:expr, $v:expr) => {{
        let __x = $x;
        if __x >= $n {
            do_realloc_array_as_needed(
                (&mut $a as *mut _ as *mut *mut ::std::ffi::c_void),
                (&mut $n as *mut _),
                __x as usize,
                ::std::mem::size_of_val(&*$a),
                ($v as *const ::std::ffi::c_void),
            )
        } else {
            0
        }
    }};
}

#[inline]
pub fn host_is_bigendian() -> bool {
    cfg!(target_endian = "big")
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
