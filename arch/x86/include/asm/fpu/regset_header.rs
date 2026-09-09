/* SPDX-License-Identifier: GPL-2.0 */
/*
 * FPU regset handling methods:
 */

// Dependency supplied by the Linux regset translation.

extern "C" {
    pub static mut regset_fpregs_active: user_regset_active_fn;
    pub static mut regset_xregset_fpregs_active: user_regset_active_fn;
    pub static mut ssp_active: user_regset_active_fn;

    pub static mut fpregs_get: user_regset_get2_fn;
    pub static mut xfpregs_get: user_regset_get2_fn;
    pub static mut fpregs_soft_get: user_regset_get2_fn;
    pub static mut xstateregs_get: user_regset_get2_fn;
    pub static mut ssp_get: user_regset_get2_fn;

    pub static mut fpregs_set: user_regset_set_fn;
    pub static mut xfpregs_set: user_regset_set_fn;
    pub static mut fpregs_soft_set: user_regset_set_fn;
    pub static mut xstateregs_set: user_regset_set_fn;
    pub static mut ssp_set: user_regset_set_fn;
}

/*
 * xstateregs_active == regset_fpregs_active. Please refer to the comment
 * at the definition of regset_fpregs_active.
 */
pub use regset_fpregs_active as xstateregs_active;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
