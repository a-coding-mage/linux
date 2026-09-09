// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation. Kernel declarations and
// macros referenced here are supplied by other translation units.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

type u32_kernel = u32;
type u64_kernel = u64;

const NSEC_PER_SEC: u64_kernel = 1_000_000_000;
const CPUTIME_IDLE: c_int = 1;

extern "C" {
    static nr_cpu_ids: c_int;

    fn cpu_possible(cpu: c_int) -> bool;
    fn kcpustat_field(field: c_int, cpu: c_int) -> u64_kernel;
    fn ktime_get_boottime_ts64(ts: *mut timespec64);
    fn timens_add_boottime(ts: *mut timespec64);
    fn div_u64_rem(dividend: u64_kernel, divisor: u32_kernel, remainder: *mut u32_kernel) -> u64_kernel;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn proc_create_single(
        name: *const c_char,
        mode: c_int,
        parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut proc_dir_entry;
    fn pde_make_permanent(pde: *mut proc_dir_entry);
}

unsafe extern "C" fn uptime_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut uptime = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut idle = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut idle_nsec: u64_kernel = 0;
    let mut rem: u32_kernel = 0;
    let mut i: c_int = 0;

    // Equivalent to the kernel's for_each_possible_cpu(i) macro.
    while i < nr_cpu_ids {
        if cpu_possible(i) {
            idle_nsec = idle_nsec.wrapping_add(kcpustat_field(CPUTIME_IDLE, i));
        }
        i += 1;
    }

    ktime_get_boottime_ts64(&mut uptime);
    timens_add_boottime(&mut uptime);

    idle.tv_sec = div_u64_rem(idle_nsec, NSEC_PER_SEC as u32_kernel, &mut rem) as i64;
    idle.tv_nsec = rem as i32;
    seq_printf(
        m,
        c"%lu.%02lu %lu.%02lu\n".as_ptr(),
        uptime.tv_sec as c_ulong,
        (uptime.tv_nsec as u64_kernel / (NSEC_PER_SEC / 100)) as c_ulong,
        idle.tv_sec as c_ulong,
        (idle.tv_nsec as u64_kernel / (NSEC_PER_SEC / 100)) as c_ulong,
    );
    0
}

unsafe extern "C" fn proc_uptime_init() -> c_int {
    let pde = proc_create_single(c"uptime".as_ptr(), 0, core::ptr::null_mut(), uptime_proc_show);
    pde_make_permanent(pde);
    0
}

// Equivalent to fs_initcall(proc_uptime_init).
#[used]
static PROC_UPTIME_INIT: unsafe extern "C" fn() -> c_int = proc_uptime_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
