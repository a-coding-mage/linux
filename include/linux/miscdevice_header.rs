/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Linux miscdevice header.
// Dependencies supplied by the original includes remain external.

/*
 * These allocations are managed by device@lanana.org. If you need an entry
 * that is not assigned here, it can be moved and reassigned or dynamically set
 * if a fixed value is not justified.
 */

pub const PSMOUSE_MINOR: i32 = 1;
pub const MS_BUSMOUSE_MINOR: i32 = 2; // unused
pub const ATIXL_BUSMOUSE_MINOR: i32 = 3; // unused
// AMIGAMOUSE_MINOR = 4: FIXME OBSOLETE
pub const ATARIMOUSE_MINOR: i32 = 5; // unused
pub const SUN_MOUSE_MINOR: i32 = 6; // unused
pub const APOLLO_MOUSE_MINOR: i32 = 7; // unused
pub const PC110PAD_MINOR: i32 = 9; // unused
// ADB_MOUSE_MINOR = 10: FIXME OBSOLETE
pub const WATCHDOG_MINOR: i32 = 130; // Watchdog timer
pub const TEMP_MINOR: i32 = 131; // Temperature Sensor
pub const APM_MINOR_DEV: i32 = 134;
pub const RTC_MINOR: i32 = 135;
// EFI_RTC_MINOR = 136: was EFI Time services
pub const VHCI_MINOR: i32 = 137;
pub const SUN_OPENPROM_MINOR: i32 = 139;
pub const DMAPI_MINOR: i32 = 140; // unused
pub const NVRAM_MINOR: i32 = 144;
pub const SBUS_FLASH_MINOR: i32 = 152;
pub const SGI_MMTIMER: i32 = 153;
pub const PMU_MINOR: i32 = 154;
pub const STORE_QUEUE_MINOR: i32 = 155; // unused
pub const LCD_MINOR: i32 = 156;
pub const AC_MINOR: i32 = 157;
pub const BUTTON_MINOR: i32 = 158; // Major 10, Minor 158, /dev/nwbutton
pub const NWFLASH_MINOR: i32 = 160; // MAJOR is 10 - miscdevice
pub const ENVCTRL_MINOR: i32 = 162;
pub const I2O_MINOR: i32 = 166;
pub const UCTRL_MINOR: i32 = 174;
pub const AGPGART_MINOR: i32 = 175;
pub const TOSH_MINOR_DEV: i32 = 181;
pub const HWRNG_MINOR: i32 = 183;
// MICROCODE_MINOR = 184: unused
pub const KEYPAD_MINOR: i32 = 185;
pub const IRNET_MINOR: i32 = 187;
pub const D7S_MINOR: i32 = 193;
pub const VFIO_MINOR: i32 = 196;
pub const PXA3XX_GCU_MINOR: i32 = 197;
pub const TUN_MINOR: i32 = 200;
pub const CUSE_MINOR: i32 = 203;
pub const MPT_MINOR: i32 = 220;
pub const MPT2SAS_MINOR: i32 = 221;
pub const MPT3SAS_MINOR: i32 = 222;
pub const UINPUT_MINOR: i32 = 223;
pub const MISC_MCELOG_MINOR: i32 = 227;
pub const HPET_MINOR: i32 = 228;
pub const FUSE_MINOR: i32 = 229;
pub const SNAPSHOT_MINOR: i32 = 231;
pub const KVM_MINOR: i32 = 232;
pub const BTRFS_MINOR: i32 = 234;
pub const AUTOFS_MINOR: i32 = 235;
pub const MAPPER_CTRL_MINOR: i32 = 236;
pub const LOOP_CTRL_MINOR: i32 = 237;
pub const VHOST_NET_MINOR: i32 = 238;
pub const UHID_MINOR: i32 = 239;
pub const USERIO_MINOR: i32 = 240;
pub const VHOST_VSOCK_MINOR: i32 = 241;
pub const EISA_EEPROM_MINOR: i32 = 241;
pub const RFKILL_MINOR: i32 = 242;

/* < 255: fixed; == 255: dynamic; > 255: dynamic minor requested. */
pub const MISC_DYNAMIC_MINOR: i32 = 255;

#[repr(C)]
pub struct miscdevice {
    pub minor: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub fops: *const file_operations,
    pub list: list_head,
    pub parent: *mut device,
    pub this_device: *mut device,
    pub groups: *const *const attribute_group,
    pub nodename: *const ::core::ffi::c_char,
    pub mode: umode_t,
}

unsafe extern "C" {
    pub fn misc_register(misc: *mut miscdevice) -> ::core::ffi::c_int;
    pub fn misc_deregister(misc: *mut miscdevice);
}

/* Helper macros are retained as Rust macro forms; their driver dependencies
 * are supplied by the surrounding kernel translation. */
#[macro_export]
macro_rules! builtin_misc_device {
    ($misc_device:expr) => {
        builtin_driver!($misc_device, misc_register)
    };
}

#[macro_export]
macro_rules! module_misc_device {
    ($misc_device:expr) => {
        module_driver!($misc_device, misc_register, misc_deregister)
    };
}

#[macro_export]
macro_rules! MODULE_ALIAS_MISCDEV {
    ($minor:expr) => {
        MODULE_ALIAS!(concat!("char-major-", stringify!(MISC_MAJOR), "-", stringify!($minor)))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
