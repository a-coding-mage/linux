/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/list.h, linux/kobject.h, and linux/device-id/dmi.h.
// enum dmi_field is in mod_devicetable.h.

#[repr(i32)]
pub enum dmi_device_type {
    DMI_DEV_TYPE_ANY = 0,
    DMI_DEV_TYPE_OTHER,
    DMI_DEV_TYPE_UNKNOWN,
    DMI_DEV_TYPE_VIDEO,
    DMI_DEV_TYPE_SCSI,
    DMI_DEV_TYPE_ETHERNET,
    DMI_DEV_TYPE_TOKENRING,
    DMI_DEV_TYPE_SOUND,
    DMI_DEV_TYPE_PATA,
    DMI_DEV_TYPE_SATA,
    DMI_DEV_TYPE_SAS,
    DMI_DEV_TYPE_IPMI = -1,
    DMI_DEV_TYPE_OEM_STRING = -2,
    DMI_DEV_TYPE_DEV_ONBOARD = -3,
    DMI_DEV_TYPE_DEV_SLOT = -4,
}

#[repr(i32)]
pub enum dmi_entry_type {
    DMI_ENTRY_BIOS = 0,
    DMI_ENTRY_SYSTEM,
    DMI_ENTRY_BASEBOARD,
    DMI_ENTRY_CHASSIS,
    DMI_ENTRY_PROCESSOR,
    DMI_ENTRY_MEM_CONTROLLER,
    DMI_ENTRY_MEM_MODULE,
    DMI_ENTRY_CACHE,
    DMI_ENTRY_PORT_CONNECTOR,
    DMI_ENTRY_SYSTEM_SLOT,
    DMI_ENTRY_ONBOARD_DEVICE,
    DMI_ENTRY_OEMSTRINGS,
    DMI_ENTRY_SYSCONF,
    DMI_ENTRY_BIOS_LANG,
    DMI_ENTRY_GROUP_ASSOC,
    DMI_ENTRY_SYSTEM_EVENT_LOG,
    DMI_ENTRY_PHYS_MEM_ARRAY,
    DMI_ENTRY_MEM_DEVICE,
    DMI_ENTRY_32_MEM_ERROR,
    DMI_ENTRY_MEM_ARRAY_MAPPED_ADDR,
    DMI_ENTRY_MEM_DEV_MAPPED_ADDR,
    DMI_ENTRY_BUILTIN_POINTING_DEV,
    DMI_ENTRY_PORTABLE_BATTERY,
    DMI_ENTRY_SYSTEM_RESET,
    DMI_ENTRY_HW_SECURITY,
    DMI_ENTRY_SYSTEM_POWER_CONTROLS,
    DMI_ENTRY_VOLTAGE_PROBE,
    DMI_ENTRY_COOLING_DEV,
    DMI_ENTRY_TEMP_PROBE,
    DMI_ENTRY_ELECTRICAL_CURRENT_PROBE,
    DMI_ENTRY_OOB_REMOTE_ACCESS,
    DMI_ENTRY_BIS_ENTRY,
    DMI_ENTRY_SYSTEM_BOOT,
    DMI_ENTRY_64_MEM_ERROR,
    DMI_ENTRY_MGMT_DEV,
    DMI_ENTRY_MGMT_DEV_COMPONENT,
    DMI_ENTRY_MGMT_DEV_THRES,
    DMI_ENTRY_MEM_CHANNEL,
    DMI_ENTRY_IPMI_DEV,
    DMI_ENTRY_SYS_POWER_SUPPLY,
    DMI_ENTRY_ADDITIONAL,
    DMI_ENTRY_ONBOARD_DEV_EXT,
    DMI_ENTRY_MGMT_CONTROLLER_HOST,
    DMI_ENTRY_TPM_DEVICE,
    DMI_ENTRY_PROCESSOR_ADDITIONAL,
    DMI_ENTRY_FIRMWARE_INVENTORY,
    DMI_ENTRY_STRING_PROPERTY,
    DMI_ENTRY_INACTIVE = 126,
    DMI_ENTRY_END_OF_TABLE = 127,
}

#[repr(C, packed)]
pub struct dmi_header {
    pub type_: u8,
    pub length: u8,
    pub handle: u16,
}

#[repr(C)]
pub struct dmi_device {
    pub list: list_head,
    pub type_: i32,
    pub name: *const core::ffi::c_char,
    pub device_data: *mut core::ffi::c_void, /* Type specific data */
}

pub const DMI_A_INFO_ENT_MIN_SIZE: usize = 0x6;
#[repr(C, packed)]
pub struct dmi_a_info_entry {
    pub length: u8,
    pub handle: u16,
    pub offset: u8,
    pub str_num: u8,
    pub value: [u8; 0],
}

pub const DMI_A_INFO_MIN_SIZE: usize = 0xB;
#[repr(C, packed)]
pub struct dmi_a_info {
    pub header: dmi_header,
    pub count: u8,
}

#[cfg(feature = "CONFIG_DMI")]
#[repr(C)]
pub struct dmi_dev_onboard {
    pub dev: dmi_device,
    pub instance: i32,
    pub segment: i32,
    pub bus: i32,
    pub devfn: i32,
}

#[cfg(feature = "CONFIG_DMI")]
extern "C" {
    pub static mut dmi_kobj: *mut kobject;
    pub fn dmi_check_system(list: *const dmi_system_id) -> i32;
    pub fn dmi_first_match(list: *const dmi_system_id) -> *const dmi_system_id;
    pub fn dmi_get_system_info(field: i32) -> *const core::ffi::c_char;
    pub fn dmi_find_device(type_: i32, name: *const core::ffi::c_char,
        from: *const dmi_device) -> *const dmi_device;
    pub fn dmi_setup();
    pub fn dmi_get_date(field: i32, yearp: *mut i32, monthp: *mut i32, dayp: *mut i32) -> bool;
    pub fn dmi_get_bios_year() -> i32;
    pub fn dmi_name_in_vendors(string: *const core::ffi::c_char) -> i32;
    pub fn dmi_name_in_serial(string: *const core::ffi::c_char) -> i32;
    pub static mut dmi_available: i32;
    pub fn dmi_walk(decode: Option<unsafe extern "C" fn(*const dmi_header, *mut core::ffi::c_void)>,
        private_data: *mut core::ffi::c_void) -> i32;
    pub fn dmi_match(f: dmi_field, string: *const core::ffi::c_char) -> bool;
    pub fn dmi_memdev_name(handle: u16, bank: *mut *const core::ffi::c_char,
        device: *mut *const core::ffi::c_char);
    pub fn dmi_memdev_size(handle: u16) -> u64;
    pub fn dmi_memdev_type(handle: u16) -> u8;
    pub fn dmi_memdev_handle(slot: i32) -> u16;
    pub fn dmi_string_nosave(dm: *const dmi_header, s: u8) -> *const core::ffi::c_char;
}

#[cfg(not(feature = "CONFIG_DMI"))]
pub const dmi_available: i32 = 0;

#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_check_system(_list: *const dmi_system_id) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_get_system_info(_field: i32) -> *const core::ffi::c_char { core::ptr::null() }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_find_device(_type_: i32, _name: *const core::ffi::c_char, _from: *const dmi_device) -> *const dmi_device { core::ptr::null() }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_setup() {}
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_get_date(_field: i32, yearp: *mut i32, monthp: *mut i32, dayp: *mut i32) -> bool {
    if !yearp.is_null() { *yearp = 0; }
    if !monthp.is_null() { *monthp = 0; }
    if !dayp.is_null() { *dayp = 0; }
    false
}
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_get_bios_year() -> i32 { -6 /* -ENXIO */ }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_name_in_vendors(_s: *const core::ffi::c_char) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_name_in_serial(_s: *const core::ffi::c_char) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_walk(_decode: Option<unsafe extern "C" fn(*const dmi_header, *mut core::ffi::c_void)>, _private_data: *mut core::ffi::c_void) -> i32 { -6 /* -ENXIO */ }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_match(_f: dmi_field, _str_: *const core::ffi::c_char) -> bool { false }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_memdev_name(_handle: u16, _bank: *mut *const core::ffi::c_char, _device: *mut *const core::ffi::c_char) {}
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_memdev_size(_handle: u16) -> u64 { u64::MAX }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_memdev_type(_handle: u16) -> u8 { 0x0 }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_memdev_handle(_slot: i32) -> u16 { 0xffff }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_first_match(_list: *const dmi_system_id) -> *const dmi_system_id { core::ptr::null() }
#[cfg(not(feature = "CONFIG_DMI"))]
pub unsafe fn dmi_string_nosave(_dm: *const dmi_header, _s: u8) -> *const core::ffi::c_char { b"\0".as_ptr() as *const core::ffi::c_char }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
