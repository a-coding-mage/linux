// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020 Google Corporation

use core::ffi::c_void;

#[repr(C, packed)]
struct HdevParamU16 { entry: mgmt_tlv_hdr, value: u16 }
#[repr(C, packed)]
struct HdevParamU8 { entry: mgmt_tlv_hdr, value: u8 }
#[repr(C, packed)]
struct HdevParamU32 { entry: mgmt_tlv_hdr, value: u32 }

// Supplied by the Bluetooth management implementation.
#[repr(C)] pub struct mgmt_tlv_hdr { pub typ: u16, pub length: u8 }
#[repr(C)] pub struct mgmt_tlv { pub typ: u16, pub length: u8, pub value: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct hci_dev { pub id: u16, pub def_page_scan_type: u16, pub def_page_scan_int: u16, pub def_page_scan_window: u16, pub def_inq_scan_type: u16, pub def_inq_scan_int: u16, pub def_inq_scan_window: u16, pub def_br_lsto: u16, pub def_page_timeout: u16, pub sniff_min_interval: u16, pub sniff_max_interval: u16, pub le_adv_min_interval: u16, pub le_adv_max_interval: u16, pub def_multi_adv_rotation_duration: u16, pub le_scan_interval: u16, pub le_scan_window: u16, pub le_scan_int_suspend: u16, pub le_scan_window_suspend: u16, pub le_scan_int_discovery: u16, pub le_scan_window_discovery: u16, pub le_scan_int_adv_monitor: u16, pub le_scan_window_adv_monitor: u16, pub le_scan_int_connect: u16, pub le_scan_window_connect: u16, pub le_conn_min_interval: u16, pub le_conn_max_interval: u16, pub le_conn_latency: u16, pub le_supv_timeout: u16, pub def_le_autoconnect_timeout: u64, pub advmon_allowlist_duration: u16, pub advmon_no_filter_duration: u16, pub enable_advmon_interleave_scan: u8, pub idle_timeout: u32 }

extern "C" {
    fn mgmt_cmd_complete(sk: *mut sock, id: u16, op: u16, status: u8, data: *const c_void, len: usize) -> i32;
    fn mgmt_cmd_status(sk: *mut sock, id: u16, op: u16, status: u8) -> i32;
    fn jiffies_to_msecs(v: u64) -> u16;
    fn msecs_to_jiffies(v: u16) -> u64;
    fn bt_dev_dbg(hdev: *mut hci_dev, fmt: *const u8, ...);
    fn bt_dev_warn(hdev: *mut hci_dev, fmt: *const u8, ...);
}

const MGMT_OP_READ_DEF_SYSTEM_CONFIG: u16 = 0;
const MGMT_OP_SET_DEF_SYSTEM_CONFIG: u16 = 0;
const MGMT_OP_READ_DEF_RUNTIME_CONFIG: u16 = 0;
const MGMT_STATUS_INVALID_PARAMS: u8 = 0;

#[repr(C, packed)]
struct ReadDefSystemConfig { p: [HdevParamU16; 30], adv: HdevParamU8, idle: HdevParamU32 }

pub unsafe fn read_def_system_config(sk: *mut sock, hdev: *mut hci_dev, _data: *mut c_void, _data_len: u16) -> i32 {
    let mut rp = ReadDefSystemConfig { p: core::mem::zeroed(), adv: core::mem::zeroed(), idle: core::mem::zeroed() };
    bt_dev_dbg(hdev, b"sock %p\0".as_ptr(), sk);
    mgmt_cmd_complete(sk, (*hdev).id, MGMT_OP_READ_DEF_SYSTEM_CONFIG, 0, &rp as *const _ as *const c_void, core::mem::size_of_val(&rp))
}

unsafe fn tlv16(p: *const u8) -> u16 { u16::from_le_bytes([*p.add(3), *p.add(4)]) }
unsafe fn tlv32(p: *const u8) -> u32 { u32::from_le_bytes([*p.add(3), *p.add(4), *p.add(5), *p.add(6)]) }

pub unsafe fn set_def_system_config(sk: *mut sock, hdev: *mut hci_dev, data: *mut c_void, data_len: u16) -> i32 {
    let mut left = data_len as usize; let mut buf = data as *mut u8;
    if left < core::mem::size_of::<mgmt_tlv>() { return mgmt_cmd_status(sk, (*hdev).id, MGMT_OP_SET_DEF_SYSTEM_CONFIG, MGMT_STATUS_INVALID_PARAMS); }
    while left >= core::mem::size_of::<mgmt_tlv>() {
        let len = *buf.add(2) as usize; let exp = 3 + len; let typ = u16::from_le_bytes([*buf, *buf.add(1)]);
        if left < exp { bt_dev_warn(hdev, b"invalid len left %u, exp >= %u\0".as_ptr(), left, exp); return mgmt_cmd_status(sk, (*hdev).id, MGMT_OP_SET_DEF_SYSTEM_CONFIG, MGMT_STATUS_INVALID_PARAMS); }
        let expected = match typ { 0..=0x1a | 0x1d | 0x1e => 2, 0x1f => 1, 0x20 => 4, _ => 0 };
        if expected != 0 && len != expected { bt_dev_warn(hdev, b"invalid length %d, exp %zu for type %u\0".as_ptr(), len, expected, typ); return mgmt_cmd_status(sk, (*hdev).id, MGMT_OP_SET_DEF_SYSTEM_CONFIG, MGMT_STATUS_INVALID_PARAMS); }
        left -= exp; buf = buf.add(exp);
    }
    left = data_len as usize; buf = data as *mut u8;
    while left >= 3 { let len = *buf.add(2) as usize; let exp = 3 + len; let typ = u16::from_le_bytes([*buf, *buf.add(1)]); let v = tlv16(buf);
        match typ {
            0x0000 => (*hdev).def_page_scan_type=v, 0x0001 => (*hdev).def_page_scan_int=v, 0x0002 => (*hdev).def_page_scan_window=v, 0x0003 => (*hdev).def_inq_scan_type=v, 0x0004 => (*hdev).def_inq_scan_int=v, 0x0005 => (*hdev).def_inq_scan_window=v, 0x0006 => (*hdev).def_br_lsto=v, 0x0007 => (*hdev).def_page_timeout=v, 0x0008 => (*hdev).sniff_min_interval=v, 0x0009 => (*hdev).sniff_max_interval=v, 0x000a => (*hdev).le_adv_min_interval=v, 0x000b => (*hdev).le_adv_max_interval=v, 0x000c => (*hdev).def_multi_adv_rotation_duration=v, 0x000d => (*hdev).le_scan_interval=v, 0x000e => (*hdev).le_scan_window=v, 0x000f => (*hdev).le_scan_int_suspend=v, 0x0010 => (*hdev).le_scan_window_suspend=v, 0x0011 => (*hdev).le_scan_int_discovery=v, 0x0012 => (*hdev).le_scan_window_discovery=v, 0x0013 => (*hdev).le_scan_int_adv_monitor=v, 0x0014 => (*hdev).le_scan_window_adv_monitor=v, 0x0015 => (*hdev).le_scan_int_connect=v, 0x0016 => (*hdev).le_scan_window_connect=v, 0x0017 => (*hdev).le_conn_min_interval=v, 0x0018 => (*hdev).le_conn_max_interval=v, 0x0019 => (*hdev).le_conn_latency=v, 0x001a => (*hdev).le_supv_timeout=v, 0x001b => (*hdev).def_le_autoconnect_timeout=msecs_to_jiffies(v), 0x001d => (*hdev).advmon_allowlist_duration=v, 0x001e => (*hdev).advmon_no_filter_duration=v, 0x001f => (*hdev).enable_advmon_interleave_scan=*buf.add(3), 0x0020 => (*hdev).idle_timeout=tlv32(buf), _ => bt_dev_warn(hdev, b"unsupported parameter %u\0".as_ptr(), typ) }
        left -= exp; buf = buf.add(exp);
    }
    mgmt_cmd_complete(sk, (*hdev).id, MGMT_OP_SET_DEF_SYSTEM_CONFIG, 0, core::ptr::null(), 0)
}

pub unsafe fn read_def_runtime_config(sk: *mut sock, hdev: *mut hci_dev, _data: *mut c_void, _data_len: u16) -> i32 { bt_dev_dbg(hdev, b"sock %p\0".as_ptr(), sk); mgmt_cmd_complete(sk, (*hdev).id, MGMT_OP_READ_DEF_RUNTIME_CONFIG, 0, core::ptr::null(), 0) }
pub unsafe fn set_def_runtime_config(sk: *mut sock, hdev: *mut hci_dev, _data: *mut c_void, _data_len: u16) -> i32 { bt_dev_dbg(hdev, b"sock %p\0".as_ptr(), sk); mgmt_cmd_status(sk, (*hdev).id, MGMT_OP_SET_DEF_SYSTEM_CONFIG, MGMT_STATUS_INVALID_PARAMS) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
