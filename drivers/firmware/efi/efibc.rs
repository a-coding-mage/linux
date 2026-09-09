// SPDX-License-Identifier: GPL-2.0
/*
 * efibc: control EFI bootloaders which obey LoaderEntryOneShot var
 * Copyright (c) 2013-2016, Intel Corporation.
 */

// C dependency: #define pr_fmt(fmt) "efibc: " fmt
// C dependencies supplied by the kernel: linux/efi.h, linux/module.h,
// linux/reboot.h, linux/slab.h, linux/ucs2_string.h

const MAX_DATA_LEN: usize = 512;

// External kernel/EFI declarations supplied by other files.
type EfiChar16 = u16;
type EfiStatus = usize;

#[repr(C)]
pub struct NotifierBlock {
    pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut core::ffi::c_void) -> i32>,
}

extern "C" {
    static mut efi: EfiSystemTable;
    static LINUX_EFI_LOADER_ENTRY_GUID: EfiGuid;
    fn efi_rt_services_supported(feature: usize) -> bool;
    fn register_reboot_notifier(notifier: *mut NotifierBlock) -> i32;
    fn unregister_reboot_notifier(notifier: *mut NotifierBlock);
    fn ucs2_strlen(s: *const EfiChar16) -> usize;
    fn kmalloc_objs<T>(count: usize) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const u8, ...);
}

#[repr(C)]
struct EfiSystemTable {
    set_variable: unsafe extern "C" fn(
        *mut EfiChar16,
        *const EfiGuid,
        u32,
        usize,
        *mut EfiChar16,
    ) -> EfiStatus,
}

#[repr(C)]
struct EfiGuid {
    _opaque: [u8; 16],
}

const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;
const EFI_VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002;
const EFI_VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004;
const EFI_SUCCESS: EfiStatus = 0;
const EIO: i32 = 5;
const ENODEV: i32 = 19;
const SYS_RESTART: usize = 0x01234567; // Supplied by linux/reboot.h.
const NOTIFY_DONE: i32 = 0x0000;
const EFI_RT_SUPPORTED_SET_VARIABLE: usize = 0; // Supplied by linux/efi.h.

unsafe fn efibc_set_variable(
    name: *mut EfiChar16,
    value: *mut EfiChar16,
    len: usize,
) -> i32 {
    let status = (efi.set_variable)(
        name,
        &LINUX_EFI_LOADER_ENTRY_GUID,
        EFI_VARIABLE_NON_VOLATILE
            | EFI_VARIABLE_BOOTSERVICE_ACCESS
            | EFI_VARIABLE_RUNTIME_ACCESS,
        len * core::mem::size_of::<EfiChar16>(),
        value,
    );

    if status != EFI_SUCCESS {
        // C: pr_err("failed to set EFI variable: 0x%lx\n", status);
        return -EIO;
    }
    0
}

unsafe extern "C" fn efibc_reboot_notifier_call(
    _notifier: *mut NotifierBlock,
    event: usize,
    data: *mut core::ffi::c_void,
) -> i32 {
    let reason: *mut EfiChar16 = if event == SYS_RESTART {
        b"reboot\0" as *const u8 as *mut EfiChar16
    } else {
        b"shutdown\0" as *const u8 as *mut EfiChar16
    };
    let str_ptr = data as *const u8;
    let wdata: *mut EfiChar16;
    let mut l: usize;

    let ret = efibc_set_variable(
        b"LoaderEntryRebootReason\0" as *const u8 as *mut EfiChar16,
        reason,
        ucs2_strlen(reason),
    );
    if ret != 0 || data.is_null() {
        return NOTIFY_DONE;
    }

    wdata = kmalloc_objs::<EfiChar16>(MAX_DATA_LEN);
    if wdata.is_null() {
        return NOTIFY_DONE;
    }

    l = 0;
    while l < MAX_DATA_LEN - 1 && *str_ptr.add(l) != b'\0' {
        *wdata.add(l) = *str_ptr.add(l) as EfiChar16;
        l += 1;
    }
    *wdata.add(l) = 0;

    efibc_set_variable(
        b"LoaderEntryOneShot\0" as *const u8 as *mut EfiChar16,
        wdata,
        l,
    );

    kfree(wdata as *mut core::ffi::c_void);
    NOTIFY_DONE
}

static mut efibc_reboot_notifier: NotifierBlock = NotifierBlock {
    notifier_call: Some(efibc_reboot_notifier_call),
};

unsafe fn efibc_init() -> i32 {
    let ret: i32;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_SET_VARIABLE) {
        return -ENODEV;
    }

    ret = register_reboot_notifier(&mut efibc_reboot_notifier);
    if ret != 0 {
        // C: pr_err("unable to register reboot notifier\n");
    }

    ret
}

// C: module_init(efibc_init);

unsafe fn efibc_exit() {
    unregister_reboot_notifier(&mut efibc_reboot_notifier);
}

// C: module_exit(efibc_exit);
// C: MODULE_AUTHOR("Jeremy Compostella <jeremy.compostella@intel.com>");
// C: MODULE_AUTHOR("Matt Gumbel <matthew.k.gumbel@intel.com");
// C: MODULE_DESCRIPTION("EFI Bootloader Control");
// C: MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
