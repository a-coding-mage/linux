// SPDX-License-Identifier: GPL-2.0-only
/* X86 ACPI Utility Functions */

#[repr(C)]
pub struct OverrideStatusId {
    pub hid: [AcpiDeviceId; 2],
    pub cpu_ids: [X86CpuId; 2],
    pub dmi_ids: [DmiSystemId; 2],
    pub uid: *const core::ffi::c_char,
    pub path: *const core::ffi::c_char,
    pub status: u64,
}

// The following types, constants, macros, and functions are supplied by the
// surrounding kernel translation unit.
extern "C" {
    pub static mut acpi_gbl_FADT: AcpiFadt;
    pub static mut boot_option_idle_override: i32;
    fn x86_match_cpu(ids: *const X86CpuId) -> bool;
    fn dmi_check_system(ids: *const DmiSystemId) -> i32;
    fn acpi_get_name(handle: *mut core::ffi::c_void, typ: u32, path: *mut AcpiBuffer) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn acpi_match_device_ids(adev: *mut AcpiDevice, ids: *const AcpiDeviceId) -> i32;
    fn acpi_dev_uid_match(adev: *mut AcpiDevice, uid: *const core::ffi::c_char) -> bool;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn dmi_first_match(ids: *const DmiSystemId) -> *const DmiSystemId;
    fn acpi_dev_uid_to_integer(adev: *mut AcpiDevice, uid: *mut u64) -> i32;
    fn dev_is_pci(dev: *mut Device) -> bool;
    fn dev_is_platform(dev: *mut Device) -> bool;
    fn to_pci_dev(dev: *mut Device) -> *mut PciDev;
    fn acpi_dev_hid_match(adev: *mut AcpiDevice, hid: *const core::ffi::c_char) -> bool;
    fn platform_device_register_simple(name: *const core::ffi::c_char, id: i32, data: *mut core::ffi::c_void, size: u32) -> *mut Device;
    fn acpi_dev_present(hid: *const core::ffi::c_char, uid: *const core::ffi::c_char, hrv: i32) -> bool;
    fn pr_info_once(fmt: *const core::ffi::c_char, ...);
    fn pr_notice(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)] pub struct AcpiDeviceId { pub hid: *const core::ffi::c_char, pub driver_data: usize }
#[repr(C)] pub struct X86CpuId { pub family: u16, pub model: u16, pub feature: u16, pub driver_data: usize }
#[repr(C)] pub struct DmiSystemId { pub matches: [DmiMatch; 8], pub callback: Option<unsafe extern "C" fn(*const DmiSystemId) -> i32>, pub ident: *const core::ffi::c_char, pub driver_data: *mut core::ffi::c_void }
#[repr(C)] pub struct DmiMatch { pub slot: i32, pub substr: *const core::ffi::c_char }
#[repr(C)] pub struct AcpiBuffer { pub length: usize, pub pointer: *mut core::ffi::c_void }
#[repr(C)] pub struct AcpiFadt { pub flags: u32 }
#[repr(C)] pub struct AcpiDevice { pub handle: *mut core::ffi::c_void }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PciDev { pub devfn: u8 }

static mut OVERRIDE_STATUS_IDS: [OverrideStatusId; 0] = [];

pub unsafe extern "C" fn acpi_device_override_status(adev: *mut AcpiDevice, status: *mut u64) -> bool {
    let mut ret = false;
    let mut i = 0usize;
    while i < OVERRIDE_STATUS_IDS.len() {
        let entry = &OVERRIDE_STATUS_IDS[i];
        if !x86_match_cpu(entry.cpu_ids.as_ptr()) { i += 1; continue; }
        if entry.dmi_ids[0].matches[0].slot != 0 && dmi_check_system(entry.dmi_ids.as_ptr()) == 0 { i += 1; continue; }
        if !entry.path.is_null() {
            let mut path = AcpiBuffer { length: usize::MAX, pointer: core::ptr::null_mut() };
            if acpi_get_name((*adev).handle, 0, &mut path) != 0 { i += 1; continue; }
            let matched = strcmp(path.pointer as *const _, entry.path) == 0;
            kfree(path.pointer);
            if !matched { i += 1; continue; }
        } else {
            if acpi_match_device_ids(adev, entry.hid.as_ptr()) != 0 { i += 1; continue; }
            if !acpi_dev_uid_match(adev, entry.uid) { i += 1; continue; }
        }
        *status = entry.status;
        ret = true;
        break;
    }
    ret
}

pub unsafe extern "C" fn force_storage_d3() -> bool {
    if !cpu_feature_enabled(0) { return false; }
    (acpi_gbl_FADT.flags & 0x8000) != 0
}

const ACPI_QUIRK_SKIP_I2C_CLIENTS: u64 = 1 << 0;
const ACPI_QUIRK_UART1_SKIP: u64 = 1 << 1;
const ACPI_QUIRK_UART1_TTY_UART2_SKIP: u64 = 1 << 2;
const ACPI_QUIRK_PNP_UART1_SKIP: u64 = 1 << 3;
const ACPI_QUIRK_SKIP_ACPI_AC_AND_BATTERY: u64 = 1 << 4;
const ACPI_QUIRK_USE_ACPI_AC_AND_BATTERY: u64 = 1 << 5;
const ACPI_QUIRK_SKIP_GPIO_EVENT_HANDLERS: u64 = 1 << 6;

static ACPI_QUIRK_SKIP_DMI_IDS: [DmiSystemId; 1] = [DmiSystemId { matches: [DmiMatch { slot: 0, substr: core::ptr::null() }; 8], callback: None, ident: core::ptr::null(), driver_data: core::ptr::null_mut() }];

// CONFIG_X86_ANDROID_TABLETS controls the complete DMI table in the C source.
// The table's external matching records are represented here by the same
// sentinel form; dependency-provided DMI initializers populate it in builds
// that enable the option.
static I2C_ACPI_KNOWN_GOOD_IDS: [AcpiDeviceId; 9] = [
    AcpiDeviceId { hid: b"10EC5640\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"10EC5651\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"INT33F4\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"INT33F5\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"INT33FD\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"INT34D3\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: b"NPCE69A\0".as_ptr() as *const _, driver_data: 0 },
    AcpiDeviceId { hid: core::ptr::null(), driver_data: 0 },
    AcpiDeviceId { hid: core::ptr::null(), driver_data: 0 },
];

pub unsafe extern "C" fn acpi_quirk_skip_i2c_client_enumeration(adev: *mut AcpiDevice) -> bool {
    let dmi_id = dmi_first_match(ACPI_QUIRK_SKIP_DMI_IDS.as_ptr());
    if dmi_id.is_null() { return false; }
    let quirks = (*dmi_id).driver_data as usize as u64;
    if quirks & ACPI_QUIRK_SKIP_I2C_CLIENTS == 0 { return false; }
    acpi_match_device_ids(adev, core::ptr::null()) != 0
}

unsafe extern "C" fn acpi_dmi_skip_serdev_enumeration(controller_parent: *mut Device, skip: *mut bool) -> i32 {
    let dmi_id = dmi_first_match(ACPI_QUIRK_SKIP_DMI_IDS.as_ptr());
    if dmi_id.is_null() { return 0; }
    let quirks = (*dmi_id).driver_data as usize as u64;
    let mut uid = 0u64;
    let adev = controller_parent as *mut AcpiDevice;
    acpi_dev_uid_to_integer(adev, &mut uid);
    if uid == 0 && dev_is_pci(controller_parent) {
        let pdev = to_pci_dev(controller_parent);
        if (*pdev).devfn == ((0x1e << 3) | 3) { uid = 1; }
        else if (*pdev).devfn == ((0x1e << 3) | 4) { uid = 2; }
    }
    if uid == 0 { return 0; }
    if !dev_is_platform(controller_parent) && !dev_is_pci(controller_parent) {
        if quirks & ACPI_QUIRK_PNP_UART1_SKIP != 0 && uid == 1 { *skip = true; }
        return 0;
    }
    if quirks & ACPI_QUIRK_UART1_SKIP != 0 && uid == 1 { *skip = true; }
    if quirks & ACPI_QUIRK_UART1_TTY_UART2_SKIP != 0 {
        if uid == 1 { return -19; }
        if uid == 2 { *skip = true; }
    }
    0
}

pub unsafe extern "C" fn acpi_quirk_skip_gpio_event_handlers() -> bool {
    let dmi_id = dmi_first_match(ACPI_QUIRK_SKIP_DMI_IDS.as_ptr());
    if dmi_id.is_null() { return false; }
    ((*dmi_id).driver_data as usize as u64 & ACPI_QUIRK_SKIP_GPIO_EVENT_HANDLERS) != 0
}

pub unsafe extern "C" fn acpi_quirk_skip_serdev_enumeration(controller_parent: *mut Device, skip: *mut bool) -> i32 {
    let adev = controller_parent as *mut AcpiDevice;
    *skip = false;
    if !adev.is_null() && acpi_dev_hid_match(adev, b"DELL0501\0".as_ptr() as *const _) {
        *skip = true;
        platform_device_register_simple(b"dell-uart-backlight\0".as_ptr() as *const _, -1, core::ptr::null_mut(), 0);
        return 0;
    }
    acpi_dmi_skip_serdev_enumeration(controller_parent, skip)
}

pub unsafe extern "C" fn acpi_quirk_skip_acpi_ac_and_battery() -> bool {
    let dmi_id = dmi_first_match(ACPI_QUIRK_SKIP_DMI_IDS.as_ptr());
    let quirks = if dmi_id.is_null() { 0 } else { (*dmi_id).driver_data as usize as u64 };
    if quirks & ACPI_QUIRK_SKIP_ACPI_AC_AND_BATTERY != 0 { return true; }
    if quirks & ACPI_QUIRK_USE_ACPI_AC_AND_BATTERY != 0 { return false; }
    if acpi_dev_present(b"INT33F4\0".as_ptr() as *const _, b"1\0".as_ptr() as *const _, -1) { return true; }
    if acpi_dev_present(b"INT34D3\0".as_ptr() as *const _, b"1\0".as_ptr() as *const _, 3) { return true; }
    false
}

unsafe extern "C" fn acpi_proc_quirk_set_no_mwait(id: *const DmiSystemId) -> i32 {
    boot_option_idle_override = 1;
    let _ = id;
    0
}

pub unsafe extern "C" fn acpi_proc_quirk_mwait_check() {
    let _ = dmi_check_system(core::ptr::null());
    let _ = acpi_proc_quirk_set_no_mwait;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
