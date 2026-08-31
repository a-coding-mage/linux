// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included <sched.h> for setns(2).

unsafe extern "C" {
    fn setns(fd: i32, nstype: i32) -> i32;
}

fn main() -> i32 {
    unsafe { setns(0, 0) }
}
