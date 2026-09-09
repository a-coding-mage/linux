/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding scheduler, current-task, and
// lowcore modules are intentionally referenced here rather than implemented.

#[repr(C)]
pub struct TaskStruct {
    pub stack_canary: usize,
}

#[repr(C)]
pub struct Lowcore {
    pub stack_canary: usize,
}

extern "C" {
    pub fn current() -> *mut TaskStruct;
    pub fn get_lowcore() -> *mut Lowcore;
    pub fn get_random_canary() -> usize;
}

#[inline(always)]
pub unsafe fn boot_init_stack_canary() {
    (*current()).stack_canary = get_random_canary();
    (*get_lowcore()).stack_canary = (*current()).stack_canary;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
