// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <numa.h>

unsafe extern "C" {
    fn numa_num_possible_cpus() -> ::std::os::raw::c_int;
}

fn main() -> ::std::os::raw::c_int {
    unsafe { numa_num_possible_cpus() }
}
