/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rust translation of linux/ipmi.h.
 * C-only includes and header guards are omitted; referenced kernel types are
 * supplied by the surrounding translation unit.
 */

use core::ffi::c_void;

/* Opaque types supplied by other kernel headers or translation units. */
pub enum module {}
pub enum device {}
pub enum ipmi_user {}
pub enum list_head {}
pub enum ipmi_addr {}
pub enum kernel_ipmi_msg {}
pub type acpi_handle = *mut c_void;

#[repr(C)]
pub struct ipmi_recv_msg {
    pub link: list_head,
    pub recv_type: ::core::ffi::c_int,
    pub user: *mut ipmi_user,
    pub addr: ipmi_addr,
    pub msgid: ::core::ffi::c_long,
    pub msg: kernel_ipmi_msg,
    pub user_msg_data: *mut c_void,
    pub done: Option<unsafe extern "C" fn(msg: *mut ipmi_recv_msg)>,
    pub msg_data: [u8; IPMI_MAX_MSG_LENGTH as usize],
}

/* IPMI_MAX_MSG_LENGTH is supplied by uapi/linux/ipmi.h. */

#[macro_export]
macro_rules! INIT_IPMI_RECV_MSG {
    ($done_handler:expr) => {{
        ipmi_recv_msg {
            done: Some($done_handler),
            ..unsafe { core::mem::zeroed() }
        }
    }};
}

extern "C" {
    pub fn ipmi_free_recv_msg(msg: *mut ipmi_recv_msg);
}

#[repr(C)]
pub struct ipmi_user_hndl {
    pub ipmi_recv_hndl: Option<unsafe extern "C" fn(*mut ipmi_recv_msg, *mut c_void)>,
    pub ipmi_watchdog_pretimeout: Option<unsafe extern "C" fn(*mut c_void)>,
    pub ipmi_panic_handler: Option<unsafe extern "C" fn(*mut c_void)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>,
}

extern "C" {
    pub fn ipmi_create_user(
        if_num: u32,
        handler: *const ipmi_user_hndl,
        handler_data: *mut c_void,
        user: *mut *mut ipmi_user,
    ) -> ::core::ffi::c_int;
    pub fn ipmi_destroy_user(user: *mut ipmi_user);
    pub fn ipmi_get_version(
        user: *mut ipmi_user,
        major: *mut u8,
        minor: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn ipmi_set_my_address(user: *mut ipmi_user, channel: u32, address: u8) -> ::core::ffi::c_int;
    pub fn ipmi_get_my_address(user: *mut ipmi_user, channel: u32, address: *mut u8) -> ::core::ffi::c_int;
    pub fn ipmi_set_my_LUN(user: *mut ipmi_user, channel: u32, lun: u8) -> ::core::ffi::c_int;
    pub fn ipmi_get_my_LUN(user: *mut ipmi_user, channel: u32, lun: *mut u8) -> ::core::ffi::c_int;
    pub fn ipmi_request_settime(
        user: *mut ipmi_user, addr: *mut ipmi_addr, msgid: ::core::ffi::c_long,
        msg: *mut kernel_ipmi_msg, user_msg_data: *mut c_void, priority: ::core::ffi::c_int,
        max_retries: ::core::ffi::c_int, retry_time_ms: u32,
    ) -> ::core::ffi::c_int;
    pub fn ipmi_request_supply_msgs(
        user: *mut ipmi_user, addr: *mut ipmi_addr, msgid: ::core::ffi::c_long,
        msg: *mut kernel_ipmi_msg, user_msg_data: *mut c_void, supplied_smi: *mut c_void,
        supplied_recv: *mut ipmi_recv_msg, priority: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn ipmi_poll_interface(user: *mut ipmi_user);
    pub fn ipmi_register_for_cmd(user: *mut ipmi_user, netfn: u8, cmd: u8, chans: u32) -> ::core::ffi::c_int;
    pub fn ipmi_unregister_for_cmd(user: *mut ipmi_user, netfn: u8, cmd: u8, chans: u32) -> ::core::ffi::c_int;
    pub fn ipmi_get_maintenance_mode(user: *mut ipmi_user) -> ::core::ffi::c_int;
    pub fn ipmi_set_maintenance_mode(user: *mut ipmi_user, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ipmi_set_gets_events(user: *mut ipmi_user, val: bool) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct ipmi_smi_watcher {
    pub link: list_head,
    pub owner: *mut module,
    pub new_smi: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut device)>,
    pub smi_gone: Option<unsafe extern "C" fn(::core::ffi::c_int)>,
}

extern "C" {
    pub fn ipmi_smi_watcher_register(watcher: *mut ipmi_smi_watcher) -> ::core::ffi::c_int;
    pub fn ipmi_smi_watcher_unregister(watcher: *mut ipmi_smi_watcher) -> ::core::ffi::c_int;
    pub fn ipmi_addr_length(addr_type: ::core::ffi::c_int) -> u32;
    pub fn ipmi_validate_addr(addr: *mut ipmi_addr, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ipmi_addr_src {
    SI_INVALID = 0,
    SI_HOTMOD,
    SI_HARDCODED,
    SI_SPMI,
    SI_ACPI,
    SI_SMBIOS,
    SI_PCI,
    SI_DEVICETREE,
    SI_PLATFORM,
    SI_LAST,
}

extern "C" {
    pub fn ipmi_addr_src_to_str(src: ipmi_addr_src) -> *const ::core::ffi::c_char;
}

#[repr(C)]
pub union ipmi_smi_info_union {
    /* CONFIG_ACPI: the acpi_info element is defined for SI_ACPI addresses. */
    pub acpi_info: ipmi_smi_acpi_info,
}

#[repr(C)]
pub struct ipmi_smi_acpi_info {
    pub acpi_handle: acpi_handle,
}

#[repr(C)]
pub struct ipmi_smi_info {
    pub addr_src: ipmi_addr_src,
    pub dev: *mut device,
    pub addr_info: ipmi_smi_info_union,
}

extern "C" {
    pub fn ipmi_get_smi_info(if_num: ::core::ffi::c_int, data: *mut ipmi_smi_info) -> ::core::ffi::c_int;
    pub fn ipmb_checksum(data: *mut u8, size: ::core::ffi::c_int) -> u8;
    pub fn ipmi_panic_request_and_wait(user: *mut ipmi_user, addr: *mut ipmi_addr, msg: *mut kernel_ipmi_msg);
}

pub const GET_DEVICE_ID_MAX_RETRY: ::core::ffi::c_int = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
