/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by the surrounding kernel translation are referenced
// here but intentionally not defined in this header translation.

pub const IDX_INVALID: i32 = -1;

#[repr(C)]
pub struct cpudl_item {
    pub dl: u64,
    pub cpu: i32,
    pub idx: i32,
}

#[repr(C)]
pub struct cpudl {
    pub lock: raw_spinlock_t,
    pub size: i32,
    pub free_cpus: cpumask_var_t,
    pub elements: *mut cpudl_item,
}

unsafe extern "C" {
    pub fn cpudl_find(
        cp: *mut cpudl,
        p: *mut task_struct,
        later_mask: *mut cpumask,
    ) -> i32;
    pub fn cpudl_set(cp: *mut cpudl, cpu: i32, dl: u64);
    pub fn cpudl_clear(cp: *mut cpudl, cpu: i32, online: bool);
    pub fn cpudl_init(cp: *mut cpudl) -> i32;
    pub fn cpudl_cleanup(cp: *mut cpudl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
