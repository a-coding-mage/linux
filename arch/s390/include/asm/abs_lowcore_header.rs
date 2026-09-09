/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux and s390 headers:
// linux/smp.h, asm/lowcore.h

use core::ffi::{c_int, c_ulong};

extern "C" {
    static mut __abs_lowcore: c_ulong;

    fn get_cpu() -> c_int;
    fn put_cpu();
}

pub const ABS_LOWCORE_MAP_SIZE: usize =
    NR_CPUS * core::mem::size_of::<lowcore>();

extern "C" {
    pub fn abs_lowcore_map(cpu: c_int, lc: *mut lowcore, alloc: bool) -> c_int;
    pub fn abs_lowcore_unmap(cpu: c_int);
}

#[inline]
pub unsafe fn get_abs_lowcore() -> *mut lowcore {
    let cpu: usize;

    cpu = get_cpu() as usize;
    (__abs_lowcore as *mut lowcore).add(cpu)
}

#[inline]
pub unsafe fn put_abs_lowcore(_lc: *mut lowcore) {
    put_cpu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
