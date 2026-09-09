/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/static_key.h>.

extern "C" {
    pub static mut sched_smt_present: static_key_false;

    fn static_branch_likely(key: *const static_key_false) -> bool;

    pub fn arch_smt_update();
}

#[inline(always)]
pub unsafe fn sched_smt_active() -> bool {
    static_branch_likely(&sched_smt_present)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
