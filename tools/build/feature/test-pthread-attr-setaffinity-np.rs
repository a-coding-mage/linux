// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdint.h>, <pthread.h>, <sched.h>

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::{size_of, zeroed};

#[repr(C)]
struct pthread_attr_t {
    __size: [u8; 56],
    __align: c_long,
}

type c_long = isize;

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 16],
}

unsafe extern "C" {
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(
        attr: *const pthread_attr_t,
        cpusetsize: usize,
        cpuset: *const cpu_set_t,
    ) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

unsafe fn CPU_ZERO(cpuset: *mut cpu_set_t) {
    unsafe {
        memset(
            cpuset as *mut c_void,
            0,
            size_of::<cpu_set_t>(),
        );
    }
}

fn main() -> c_int {
    let mut ret: c_int = 0;
    let mut thread_attr: pthread_attr_t = unsafe { zeroed() };
    let mut cs: cpu_set_t = unsafe { zeroed() };

    unsafe {
        pthread_attr_init(&mut thread_attr);
        CPU_ZERO(&mut cs);

        ret = pthread_attr_setaffinity_np(&thread_attr, size_of::<cpu_set_t>(), &cs);
    }

    ret
}
