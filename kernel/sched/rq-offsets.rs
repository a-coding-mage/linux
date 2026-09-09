// SPDX-License-Identifier: GPL-2.0
// COMPILE_OFFSETS
// Dependencies: linux/kbuild.h, linux/types.h, and sched.h provide DEFINE,
// struct rq, and its nr_pinned field.

pub fn main() -> i32 {
    unsafe {
        DEFINE!(RQ_nr_pinned, core::mem::offset_of!(rq, nr_pinned));
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
