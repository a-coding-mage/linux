/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/reboot.h> supplies `reboot_mode`.

unsafe extern "C" {
    pub fn highbank_restart(mode: reboot_mode, command: *const kernel::ffi::c_char);
    pub static mut scu_base_addr: *mut kernel::ffi::c_void;
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" {
    pub fn highbank_pm_init();
}

#[cfg(not(CONFIG_PM_SLEEP))]
#[inline]
pub unsafe fn highbank_pm_init() {}

unsafe extern "C" {
    pub fn highbank_smc1(function: kernel::ffi::c_int, argument: kernel::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
