// SPDX-License-Identifier: GPL-2.0-only
/*
 * poweroff.c - sysrq handler to gracefully power down machine.
 */

use core::ffi::c_char;

// Kernel dependencies supplied by the surrounding tree.
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sysrq_key_op {
    pub handler: Option<unsafe extern "C" fn(u8)>,
    pub help_msg: *const c_char,
    pub action_msg: *const c_char,
    pub enable_mask: u32,
}

extern "C" {
    fn kernel_power_off();
    fn schedule_work_on(cpu: usize, work: *mut work_struct);
    fn register_sysrq_key(key: u8, op: *const sysrq_key_op) -> i32;
    fn cpumask_first(mask: *const core::ffi::c_void) -> usize;
    static cpu_online_mask: core::ffi::c_void;
}

// The value is supplied by the kernel sysrq interface.
const SYSRQ_ENABLE_BOOT: u32 = 0;

/*
 * When the user hits Sys-Rq o to power down the machine this is the
 * callback we use.
 */

unsafe extern "C" fn do_poweroff(_dummy: *mut work_struct) {
    kernel_power_off();
}

// Equivalent to DECLARE_WORK(poweroff_work, do_poweroff).
static mut poweroff_work: work_struct = work_struct { _private: [] };

unsafe extern "C" fn handle_poweroff(_key: u8) {
    /* run sysrq poweroff on boot cpu */
    schedule_work_on(
        cpumask_first(&cpu_online_mask as *const _ as *const core::ffi::c_void),
        &raw mut poweroff_work,
    );
}

static sysrq_poweroff_op: sysrq_key_op = sysrq_key_op {
    handler: Some(handle_poweroff),
    help_msg: b"poweroff(o)\0".as_ptr() as *const c_char,
    action_msg: b"Power Off\0".as_ptr() as *const c_char,
    enable_mask: SYSRQ_ENABLE_BOOT,
};

unsafe extern "C" fn pm_sysrq_init() -> i32 {
    register_sysrq_key(b'o', &raw const sysrq_poweroff_op);
    0
}

// Equivalent to subsys_initcall(pm_sysrq_init).
// The kernel's initcall registration is supplied by the surrounding build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
