/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Bluetooth support for Realtek devices
 *
 * Copyright (C) 2015 Endless Mobile, Inc.
 */

pub const RTL_FRAG_LEN: usize = 252;

/* C logging macros: the bt_dev_* symbols are supplied by the surrounding code. */
macro_rules! rtl_dev_err { ($dev:expr, $($arg:tt)*) => { bt_dev_err!($dev, "RTL: {}", format_args!($($arg)*)) }; }
macro_rules! rtl_dev_warn { ($dev:expr, $($arg:tt)*) => { bt_dev_warn!($dev, "RTL: {}", format_args!($($arg)*)) }; }
macro_rules! rtl_dev_info { ($dev:expr, $($arg:tt)*) => { bt_dev_info!($dev, "RTL: {}", format_args!($($arg)*)) }; }
macro_rules! rtl_dev_dbg { ($dev:expr, $($arg:tt)*) => { bt_dev_dbg!($dev, "RTL: {}", format_args!($($arg)*)) }; }

#[repr(C)]
pub struct btrtl_device_info;

#[repr(C, packed)]
pub struct rtl_chip_type_evt { pub status: u8, pub r#type: u8 }

#[repr(C, packed)]
pub struct rtl_download_cmd { pub index: u8, pub data: [u8; RTL_FRAG_LEN] }

#[repr(C, packed)]
pub struct rtl_download_response { pub status: u8, pub index: u8 }

#[repr(C, packed)]
pub struct rtl_rom_version_evt { pub status: u8, pub version: u8 }

#[repr(C, packed)]
pub struct rtl_epatch_header {
    pub signature: [u8; 8], pub fw_version: __le32, pub num_patches: __le16,
}

#[repr(C, packed)]
pub struct rtl_vendor_config_entry { pub offset: __le16, pub len: u8, pub data: [u8; 0] }

#[repr(C, packed)]
pub struct rtl_vendor_config { pub signature: __le32, pub total_len: __le16, pub entry: [u8; 0] }

#[repr(C, packed)]
pub struct rtl_epatch_header_v2 { pub signature: [u8; 8], pub fw_version: [u8; 8], pub num_sections: __le32 }

#[repr(C, packed)]
pub struct rtl_section { pub opcode: __le32, pub len: __le32, pub data: [u8; 0] }

#[repr(C, packed)]
pub struct rtl_section_hdr { pub num: __le16, pub reserved: __le16 }

#[repr(C)]
pub struct rtl_common_subsec { pub eco: u8, pub prio: u8, pub cb: [u8; 2], pub len: __le32, pub data: [u8; 0] }

#[repr(C, packed)]
pub struct rtl_sec_hdr { pub eco: u8, pub prio: u8, pub key_id: u8, pub reserved: u8, pub len: __le32, pub data: [u8; 0] }

#[repr(C)]
pub struct rtl_subsection { pub list: list_head, pub opcode: u32, pub len: u32, pub prio: u8, pub data: *mut u8 }

#[repr(C)]
pub struct rtl_iovec { pub data: *mut u8, pub len: u32 }

#[repr(C, packed)]
pub struct rtl_vendor_cmd { pub param: [u8; 5] }

pub const REALTEK_ALT6_CONTINUOUS_TX_CHIP: u32 = 0;
pub const __REALTEK_NUM_FLAGS: u32 = 1;

#[repr(C)]
pub struct rtl_dump_info { pub driver_name: *const c_char, pub controller: *mut c_char, pub fw_version: u32 }

#[repr(C)]
pub struct btrealtek_data { pub flags: [c_ulong; 1], pub rtl_dump: rtl_dump_info }

/* The hci_get_priv, set_bit, and test_bit operations are supplied externally. */
macro_rules! btrealtek_set_flag {
    ($hdev:expr, $nr:expr) => {{ let realtek: *mut btrealtek_data = hci_get_priv($hdev); set_bit($nr, unsafe { (*realtek).flags.as_mut_ptr() }); }};
}
macro_rules! btrealtek_get_flag {
    ($hdev:expr) => {{ let realtek: *mut btrealtek_data = hci_get_priv($hdev); unsafe { (*realtek).flags.as_ptr() } }};
}
macro_rules! btrealtek_test_flag { ($hdev:expr, $nr:expr) => { test_bit($nr, btrealtek_get_flag!($hdev)) }; }

/* CONFIG_BT_RTL controls whether these declarations or the inline stubs are used. */
#[cfg(feature = "CONFIG_BT_RTL")]
extern "C" {
    pub fn btrtl_initialize(hdev: *mut hci_dev, postfix: *const c_char) -> *mut btrtl_device_info;
    pub fn btrtl_free(btrtl_dev: *mut btrtl_device_info);
    pub fn btrtl_download_firmware(hdev: *mut hci_dev, btrtl_dev: *mut btrtl_device_info) -> c_int;
    pub fn btrtl_set_quirks(hdev: *mut hci_dev, btrtl_dev: *mut btrtl_device_info);
    pub fn btrtl_setup_realtek(hdev: *mut hci_dev) -> c_int;
    pub fn btrtl_shutdown_realtek(hdev: *mut hci_dev) -> c_int;
    pub fn btrtl_get_uart_settings(hdev: *mut hci_dev, btrtl_dev: *mut btrtl_device_info, controller_baudrate: *mut c_uint, device_baudrate: *mut u32, flow_control: *mut bool) -> c_int;
    pub fn btrtl_set_driver_name(hdev: *mut hci_dev, driver_name: *const c_char);
}

#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_initialize(_: *mut hci_dev, _: *const c_char) -> *mut btrtl_device_info { ERR_PTR(-EOPNOTSUPP) }
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_free(_: *mut btrtl_device_info) {}
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_download_firmware(_: *mut hci_dev, _: *mut btrtl_device_info) -> c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_set_quirks(_: *mut hci_dev, _: *mut btrtl_device_info) {}
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_setup_realtek(_: *mut hci_dev) -> c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_shutdown_realtek(_: *mut hci_dev) -> c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_get_uart_settings(_: *mut hci_dev, _: *mut btrtl_device_info, _: *mut c_uint, _: *mut u32, _: *mut bool) -> c_int { -ENOENT }
#[cfg(not(feature = "CONFIG_BT_RTL"))]
pub unsafe fn btrtl_set_driver_name(_: *mut hci_dev, _: *const c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
