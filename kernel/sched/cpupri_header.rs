/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by the corresponding Linux headers:
// linux/atomic.h, linux/cpumask.h, linux/sched/rt.h

pub const CPUPRI_NR_PRIORITIES: usize = (MAX_RT_PRIO + 1) as usize;

pub const CPUPRI_INVALID: i32 = -1;
pub const CPUPRI_NORMAL: i32 = 0;
// values 1-99 are for RT1-RT99 priorities
pub const CPUPRI_HIGHER: i32 = 100;

#[repr(C)]
pub struct cpupri_vec {
    pub count: atomic_t,
    pub mask: cpumask_var_t,
}

#[repr(C)]
pub struct cpupri {
    pub pri_to_cpu: [cpupri_vec; CPUPRI_NR_PRIORITIES],
    pub cpu_to_pri: *mut i32,
}

unsafe extern "C" {
    pub fn cpupri_find(
        cp: *mut cpupri,
        p: *mut task_struct,
        lowest_mask: *mut cpumask,
    ) -> i32;
    pub fn cpupri_find_fitness(
        cp: *mut cpupri,
        p: *mut task_struct,
        lowest_mask: *mut cpumask,
        fitness_fn: Option<unsafe extern "C" fn(p: *mut task_struct, cpu: i32) -> bool>,
    ) -> i32;
    pub fn cpupri_set(cp: *mut cpupri, cpu: i32, pri: i32);
    pub fn cpupri_init(cp: *mut cpupri) -> i32;
    pub fn cpupri_cleanup(cp: *mut cpupri);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
