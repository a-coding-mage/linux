// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Declarations supplied by the kernel EFI, reboot, and power-management code.
extern "C" {
    fn wait_for_interrupt() -> !;
    fn efi_enabled(feature: u32) -> bool;
    fn efi_reboot(mode: i32, cmd: *mut core::ffi::c_void);
    fn do_kernel_restart(cmd: *mut core::ffi::c_char);
    fn do_kernel_power_off();

    static mut reboot_mode: i32;
}

// EFI_RUNTIME_SERVICES is supplied by the EFI interface.
extern "C" {
    static EFI_RUNTIME_SERVICES: u32;
}

unsafe fn default_power_off() -> ! {
    loop {
        wait_for_interrupt();
    }
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;
// EXPORT_SYMBOL(pm_power_off);

pub unsafe extern "C" fn machine_restart(cmd: *mut core::ffi::c_char) {
    /*
     * UpdateCapsule() depends on the system being reset via ResetSystem().
     */
    if efi_enabled(EFI_RUNTIME_SERVICES) {
        efi_reboot(reboot_mode, core::ptr::null_mut());
    }

    do_kernel_restart(cmd);
    loop {}
}

pub unsafe extern "C" fn machine_halt() {
    do_kernel_power_off();
    default_power_off();
}

pub unsafe extern "C" fn machine_power_off() {
    do_kernel_power_off();
    default_power_off();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
