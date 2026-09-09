/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __REBOOT_MODE_H__

// Supplied by the corresponding dependency headers.
pub enum device {}
pub enum list_head {}
pub enum notifier_block {}

#[repr(C)]
pub struct reboot_mode_driver {
    pub dev: *mut device,
    pub head: list_head,
    pub write: Option<unsafe extern "C" fn(reboot: *mut reboot_mode_driver, magic: u32) -> i32>,
    pub reboot_notifier: notifier_block,
}

unsafe extern "C" {
    pub fn reboot_mode_register(reboot: *mut reboot_mode_driver) -> i32;
    pub fn reboot_mode_unregister(reboot: *mut reboot_mode_driver) -> i32;
    pub fn devm_reboot_mode_register(
        dev: *mut device,
        reboot: *mut reboot_mode_driver,
    ) -> i32;
    pub fn devm_reboot_mode_unregister(
        dev: *mut device,
        reboot: *mut reboot_mode_driver,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
