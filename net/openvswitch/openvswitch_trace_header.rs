/* SPDX-License-Identifier: GPL-2.0 */
// Translated from openvswitch_trace.h.
// C tracepoint registration and formatting are represented as Rust data
// layouts and assignment functions; referenced kernel types/functions remain
// external dependencies supplied by the surrounding translation.

// #include <linux/tracepoint.h>
// #include "datapath.h"

#[repr(C)]
pub struct OvsDoExecuteActionEntry {
    pub dpaddr: *mut core::ffi::c_void,
    pub dp_name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
    pub skbaddr: *mut core::ffi::c_void,
    pub len: core::ffi::c_uint,
    pub data_len: core::ffi::c_uint,
    pub truesize: core::ffi::c_uint,
    pub nr_frags: u8,
    pub gso_size: u16,
    pub gso_type: u16,
    pub ovs_flow_hash: u32,
    pub recirc_id: u32,
    pub keyaddr: *mut core::ffi::c_void,
    pub key_eth_type: u16,
    pub key_ct_state: u8,
    pub key_ct_orig_proto: u8,
    pub key_ct_zone: u16,
    pub flow_key_valid: core::ffi::c_uint,
    pub action_type: u8,
    pub action_len: core::ffi::c_uint,
    pub action_data: *mut core::ffi::c_void,
    pub is_last: u8,
}

#[repr(C)]
pub struct OvsDpUpcallEntry {
    pub dpaddr: *mut core::ffi::c_void,
    pub dp_name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
    pub skbaddr: *mut core::ffi::c_void,
    pub len: core::ffi::c_uint,
    pub data_len: core::ffi::c_uint,
    pub truesize: core::ffi::c_uint,
    pub nr_frags: u8,
    pub gso_size: u16,
    pub gso_type: u16,
    pub ovs_flow_hash: u32,
    pub recirc_id: u32,
    pub keyaddr: *const core::ffi::c_void,
    pub key_eth_type: u16,
    pub key_ct_state: u8,
    pub key_ct_orig_proto: u8,
    pub key_ct_zone: u16,
    pub flow_key_valid: core::ffi::c_uint,
    pub upcall_cmd: u8,
    pub upcall_port: u32,
    pub upcall_mru: u16,
}

// The following declarations preserve the source tracepoint interfaces.
// Their argument and field types are supplied by the translated datapath and
// kernel headers.
extern "C" {
    pub fn ovs_do_execute_action(
        dp: *mut datapath,
        skb: *mut sk_buff,
        key: *mut sw_flow_key,
        a: *const nlattr,
        rem: core::ffi::c_int,
    );
    pub fn ovs_dp_upcall(
        dp: *mut datapath,
        skb: *mut sk_buff,
        key: *const sw_flow_key,
        upcall_info: *const dp_upcall_info,
    );
}

#[repr(C)]
pub struct datapath { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct sw_flow_key { _private: [u8; 0] }
#[repr(C)]
pub struct nlattr { _private: [u8; 0] }
#[repr(C)]
pub struct dp_upcall_info { _private: [u8; 0] }

// TP_printk formats retained from the C tracepoints:
// ovs_do_execute_action: dpaddr=%p dp_name=%s dev=%s skbaddr=%p len=%u
// data_len=%u truesize=%u nr_frags=%d gso_size=%d gso_type=%#x
// ovs_flow_hash=0x%08x recirc_id=0x%08x keyaddr=%p eth_type=0x%04x
// ct_state=%02x ct_orig_proto=%02x ct_Zone=%04x flow_key_valid=%d
// action_type=%u action_len=%u action_data=%p is_last=%d
// ovs_dp_upcall: dpaddr=%p dp_name=%s dev=%s skbaddr=%p len=%u data_len=%u
// truesize=%u nr_frags=%d gso_size=%d gso_type=%#x ovs_flow_hash=0x%08x
// recirc_id=0x%08x keyaddr=%p eth_type=0x%04x ct_state=%02x
// ct_orig_proto=%02x ct_zone=%04x flow_key_valid=%d upcall_cmd=%u
// upcall_port=%u upcall_mru=%u

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
