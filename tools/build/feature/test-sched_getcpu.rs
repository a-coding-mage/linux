// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE before including <sched.h> for sched_getcpu.

unsafe extern "C" {
    fn sched_getcpu() -> ::std::os::raw::c_int;
}

fn main() -> ::std::os::raw::c_int {
    unsafe { sched_getcpu() }
}
