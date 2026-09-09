// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Intel Corporation; author Matt Fleming
 * Copyright (c) 2014 Red Hat, Inc., Mark Salter <msalter@redhat.com>
 */

// Dependencies supplied by the Linux EFI and reboot interfaces.

#[repr(C)]
pub struct SysOffHandler {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SysOffData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Efi {
    pub reset_system: unsafe extern "C" fn(
        reset_type: i32,
        status: usize,
        data_size: usize,
        data: *mut core::ffi::c_void,
    ),
}

extern "C" {
    static mut efi: Efi;

    fn efi_rt_services_supported(feature: i32) -> bool;
    fn efi_capsule_pending(reset_mode: *mut i32) -> bool;
    fn printk(format: *const core::ffi::c_char, ...) -> i32;
    fn register_sys_off_handler(
        mode: i32,
        priority: i32,
        callback: unsafe extern "C" fn(*mut SysOffData) -> i32,
        data: *mut core::ffi::c_void,
    ) -> *mut SysOffHandler;
    fn ptr_err(ptr: *mut SysOffHandler) -> i32;
}

static mut efi_sys_off_handler: *mut SysOffHandler = core::ptr::null_mut();

pub static mut efi_reboot_quirk_mode: i32 = -1;

pub unsafe extern "C" fn efi_reboot(reboot_mode: i32, _unused: *const core::ffi::c_char) {
    let str_: [*const core::ffi::c_char; 4] = [
        b"cold\0".as_ptr() as *const core::ffi::c_char,
        b"warm\0".as_ptr() as *const core::ffi::c_char,
        b"shutdown\0".as_ptr() as *const core::ffi::c_char,
        b"platform\0".as_ptr() as *const core::ffi::c_char,
    ];
    let mut efi_mode: i32;
    let mut cap_reset_mode: i32 = 0;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_RESET_SYSTEM) {
        return;
    }

    efi_mode = match reboot_mode {
        REBOOT_WARM | REBOOT_SOFT => EFI_RESET_WARM,
        _ => EFI_RESET_COLD,
    };

    /*
     * If a quirk forced an EFI reset mode, always use that.
     */
    if efi_reboot_quirk_mode != -1 {
        efi_mode = efi_reboot_quirk_mode;
    }

    if efi_capsule_pending(&mut cap_reset_mode) {
        if efi_mode != cap_reset_mode {
            printk(
                b"efi: %s reset requested but pending capsule update requires %s reset... Performing %s reset.\n\0"
                    .as_ptr() as *const core::ffi::c_char,
                str_[efi_mode as usize],
                str_[cap_reset_mode as usize],
                str_[cap_reset_mode as usize],
            );
        }
        efi_mode = cap_reset_mode;
    }

    (efi.reset_system)(efi_mode, EFI_SUCCESS as usize, 0, core::ptr::null_mut());
}

pub unsafe extern "C" fn efi_poweroff_required() -> bool {
    false
}

unsafe extern "C" fn efi_power_off(_data: *mut SysOffData) -> i32 {
    (efi.reset_system)(
        EFI_RESET_SHUTDOWN,
        EFI_SUCCESS as usize,
        0,
        core::ptr::null_mut(),
    );

    NOTIFY_DONE
}

unsafe extern "C" fn efi_shutdown_init() -> i32 {
    if !efi_rt_services_supported(EFI_RT_SUPPORTED_RESET_SYSTEM) {
        return -ENODEV;
    }

    if efi_poweroff_required() {
        /* SYS_OFF_PRIO_FIRMWARE + 1 so that it runs before acpi_power_off */
        efi_sys_off_handler = register_sys_off_handler(
            SYS_OFF_MODE_POWER_OFF,
            SYS_OFF_PRIO_FIRMWARE + 1,
            efi_power_off,
            core::ptr::null_mut(),
        );
        if (efi_sys_off_handler as isize) < 0 {
            return ptr_err(efi_sys_off_handler);
        }
    }

    0
}

// late_initcall(efi_shutdown_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
