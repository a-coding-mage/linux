/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

// Dependencies supplied by linux/notifier.h and uapi/linux/reboot.h remain external.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sys_off_handler {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

pub const SYS_DOWN: i32 = 0x0001;
pub const SYS_RESTART: i32 = SYS_DOWN;
pub const SYS_HALT: i32 = 0x0002;
pub const SYS_POWER_OFF: i32 = 0x0003;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reboot_mode {
    REBOOT_UNDEFINED = -1,
    REBOOT_COLD = 0,
    REBOOT_WARM,
    REBOOT_HARD,
    REBOOT_SOFT,
    REBOOT_GPIO,
}

extern "C" {
    pub static mut reboot_mode: reboot_mode;
    pub static mut panic_reboot_mode: reboot_mode;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reboot_type {
    BOOT_TRIPLE = b't' as isize,
    BOOT_KBD = b'k' as isize,
    BOOT_BIOS = b'b' as isize,
    BOOT_ACPI = b'a' as isize,
    BOOT_EFI = b'e' as isize,
    BOOT_CF9_FORCE = b'p' as isize,
    BOOT_CF9_SAFE = b'q' as isize,
}

extern "C" {
    pub static mut reboot_type: reboot_type;
    pub static mut reboot_default: i32;
    pub static mut reboot_cpu: i32;
    pub static mut reboot_force: i32;

    pub fn register_reboot_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_reboot_notifier(nb: *mut notifier_block) -> i32;
    pub fn devm_register_reboot_notifier(dev: *mut device, nb: *mut notifier_block) -> i32;
    pub fn register_restart_handler(nb: *mut notifier_block) -> i32;
    pub fn unregister_restart_handler(nb: *mut notifier_block) -> i32;
    pub fn do_kernel_restart(cmd: *mut c_char);

    pub fn migrate_to_reboot_cpu();
    pub fn machine_restart(cmd: *mut c_char);
    pub fn machine_halt();
    pub fn machine_power_off();
    pub fn machine_shutdown();
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn machine_crash_shutdown(regs: *mut pt_regs);
    pub fn do_kernel_power_off();
}

pub const SYS_OFF_PRIO_PLATFORM: i32 = -256;
pub const SYS_OFF_PRIO_LOW: i32 = -128;
pub const SYS_OFF_PRIO_DEFAULT: i32 = 0;
pub const SYS_OFF_PRIO_HIGH: i32 = 192;
pub const SYS_OFF_PRIO_FIRMWARE: i32 = 224;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sys_off_mode {
    SYS_OFF_MODE_POWER_OFF_PREPARE,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_MODE_RESTART_PREPARE,
    SYS_OFF_MODE_RESTART,
}

#[repr(C)]
pub struct sys_off_data {
    pub mode: i32,
    pub cb_data: *mut c_void,
    pub cmd: *const c_char,
    pub dev: *mut device,
}

extern "C" {
    pub fn register_sys_off_handler(
        mode: sys_off_mode,
        priority: i32,
        callback: extern "C" fn(*mut sys_off_data) -> i32,
        cb_data: *mut c_void,
    ) -> *mut sys_off_handler;
    pub fn unregister_sys_off_handler(handler: *mut sys_off_handler);
    pub fn devm_register_sys_off_handler(
        dev: *mut device,
        mode: sys_off_mode,
        priority: i32,
        callback: extern "C" fn(*mut sys_off_data) -> i32,
        cb_data: *mut c_void,
    ) -> i32;
    pub fn devm_register_power_off_handler(
        dev: *mut device,
        callback: extern "C" fn(*mut sys_off_data) -> i32,
        cb_data: *mut c_void,
    ) -> i32;
    pub fn devm_register_restart_handler(
        dev: *mut device,
        callback: extern "C" fn(*mut sys_off_data) -> i32,
        cb_data: *mut c_void,
    ) -> i32;
    pub fn register_platform_power_off(power_off: extern "C" fn());
    pub fn unregister_platform_power_off(power_off: extern "C" fn());

    pub fn kernel_restart_prepare(cmd: *mut c_char);
    pub fn kernel_restart(cmd: *mut c_char);
    pub fn kernel_halt();
    pub fn kernel_power_off();
    pub fn kernel_can_power_off() -> bool;
    pub fn ctrl_alt_del();
    pub fn orderly_poweroff(force: bool);
    pub fn orderly_reboot();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_protection_action {
    HWPROT_ACT_DEFAULT,
    HWPROT_ACT_SHUTDOWN,
    HWPROT_ACT_REBOOT,
}

extern "C" {
    pub fn __hw_protection_trigger(
        reason: *const c_char,
        ms_until_forced: i32,
        action: hw_protection_action,
    );
}

#[inline]
pub unsafe fn hw_protection_trigger(reason: *const c_char, ms_until_forced: i32) {
    __hw_protection_trigger(reason, ms_until_forced, hw_protection_action::HWPROT_ACT_DEFAULT);
}

extern "C" {
    pub fn emergency_restart();
}

// Architecture-specific emergency-restart declarations are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
