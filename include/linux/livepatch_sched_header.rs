/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding kernel headers:
// linux/jump_label.h and linux/sched.h

#[cfg(CONFIG_LIVEPATCH)]
extern "C" {
    pub fn __klp_sched_try_switch();
}

// Equivalent of DECLARE_STATIC_KEY_FALSE(klp_sched_try_switch_key).
// The static-key type and static_branch_unlikely operation are supplied by
// linux/jump_label.h.
#[cfg(CONFIG_LIVEPATCH)]
extern "C" {
    pub static klp_sched_try_switch_key: core::ffi::c_void;
}

// Equivalent of struct task_struct from linux/sched.h.
#[repr(C)]
pub struct task_struct {
    pub __state: core::ffi::c_uint,
}

// TASK_FREEZABLE and the READ_ONCE/static_branch_unlikely operations are
// supplied by the corresponding kernel headers.
#[cfg(CONFIG_LIVEPATCH)]
#[inline(always)]
pub unsafe fn klp_sched_try_switch(curr: *mut task_struct) {
    if static_branch_unlikely(&klp_sched_try_switch_key)
        && (read_once(core::ptr::addr_of!((*curr).__state)) & TASK_FREEZABLE) != 0
    {
        __klp_sched_try_switch();
    }
}

#[cfg(not(CONFIG_LIVEPATCH))]
#[inline]
pub unsafe fn klp_sched_try_switch(_curr: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
