/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of ipmi_smi.h. */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    pub static mut THIS_MODULE: *mut module;
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct ipmi_smi;
#[repr(C)]
pub struct ipmi_recv_msg;
#[repr(C)]
pub struct ipmi_smi_info;
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub const IPMI_WATCH_MASK_CHECK_MESSAGES: u32 = 1 << 0;
pub const IPMI_WATCH_MASK_CHECK_WATCHDOG: u32 = 1 << 1;
pub const IPMI_WATCH_MASK_CHECK_COMMANDS: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ipmi_smi_msg_type {
    IPMI_SMI_MSG_TYPE_NORMAL = 0,
    IPMI_SMI_MSG_TYPE_IPMB_DIRECT,
}

#[repr(C)]
pub struct ipmi_smi_msg {
    pub link: list_head,
    pub type_: ipmi_smi_msg_type,
    pub msgid: libc::c_long,
    pub recv_msg: *mut ipmi_recv_msg,
    pub data_size: libc::c_int,
    pub data: [u8; IPMI_MAX_MSG_LENGTH as usize],
    pub rsp_size: libc::c_int,
    pub rsp: [u8; IPMI_MAX_MSG_LENGTH as usize],
    pub done: Option<unsafe extern "C" fn(msg: *mut ipmi_smi_msg)>,
}

pub const IPMI_SMI_CAN_HANDLE_IPMB_DIRECT: u32 = 1 << 0;

#[repr(C)]
pub struct ipmi_smi_handlers {
    pub owner: *mut module,
    pub flags: u32,
    pub start_processing: Option<unsafe extern "C" fn(*mut c_void, *mut ipmi_smi) -> libc::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>,
    pub get_smi_info: Option<unsafe extern "C" fn(*mut c_void, *mut ipmi_smi_info) -> libc::c_int>,
    pub sender: Option<unsafe extern "C" fn(*mut c_void, *mut ipmi_smi_msg) -> libc::c_int>,
    pub request_events: Option<unsafe extern "C" fn(*mut c_void)>,
    pub set_need_watch: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    pub flush_messages: Option<unsafe extern "C" fn(*mut c_void)>,
    pub set_run_to_completion: Option<unsafe extern "C" fn(*mut c_void, bool)>,
    pub poll: Option<unsafe extern "C" fn(*mut c_void)>,
    pub set_maintenance_mode: Option<unsafe extern "C" fn(*mut c_void, bool)>,
}

#[repr(C)]
pub struct ipmi_device_id {
    pub device_id: u8,
    pub device_revision: u8,
    pub firmware_revision_1: u8,
    pub firmware_revision_2: u8,
    pub ipmi_version: u8,
    pub additional_device_support: u8,
    pub manufacturer_id: u32,
    pub product_id: u32,
    pub aux_firmware_revision: [u8; 4],
    pub aux_firmware_revision_set: u32,
}

#[inline]
pub unsafe fn ipmi_version_major(v: *const ipmi_device_id) -> u8 { (*v).ipmi_version & 0xf }
#[inline]
pub unsafe fn ipmi_version_minor(v: *const ipmi_device_id) -> u8 { (*v).ipmi_version >> 4 }

#[inline]
pub unsafe fn ipmi_demangle_device_id(
    netfn: u8, cmd: u8, data: *const u8, data_len: u32, id: *mut ipmi_device_id,
) -> libc::c_int {
    if data_len < 7 || netfn != IPMI_NETFN_APP_RESPONSE || cmd != IPMI_GET_DEVICE_ID_CMD || *data != 0 {
        return -EINVAL;
    }
    let data = data.add(1);
    let data_len = data_len - 1;
    (*id).device_id = *data;
    (*id).device_revision = *data.add(1);
    (*id).firmware_revision_1 = *data.add(2);
    (*id).firmware_revision_2 = *data.add(3);
    (*id).ipmi_version = *data.add(4);
    (*id).additional_device_support = *data.add(5);
    if data_len >= 11 {
        (*id).manufacturer_id = (*data as u32) | ((*data.add(7) as u32) << 8) | ((*data.add(8) as u32) << 16);
        (*id).product_id = (*data.add(9) as u32) | ((*data.add(10) as u32) << 8);
    } else {
        (*id).manufacturer_id = 0;
        (*id).product_id = 0;
    }
    if data_len >= 15 {
        core::ptr::copy_nonoverlapping(data.add(11), (*id).aux_firmware_revision.as_mut_ptr(), 4);
        (*id).aux_firmware_revision_set = 1;
    } else {
        (*id).aux_firmware_revision_set = 0;
    }
    0
}

extern "C" {
    pub fn ipmi_add_smi(owner: *mut module, handlers: *const ipmi_smi_handlers, send_info: *mut c_void, dev: *mut device, slave_addr: u8) -> libc::c_int;
    pub fn ipmi_unregister_smi(intf: *mut ipmi_smi);
    pub fn ipmi_smi_msg_received(intf: *mut ipmi_smi, msg: *mut ipmi_smi_msg);
    pub fn ipmi_smi_watchdog_pretimeout(intf: *mut ipmi_smi);
    pub fn ipmi_alloc_smi_msg() -> *mut ipmi_smi_msg;
}

#[inline]
pub unsafe fn ipmi_free_smi_msg(msg: *mut ipmi_smi_msg) { ((*msg).done.unwrap())(msg); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
