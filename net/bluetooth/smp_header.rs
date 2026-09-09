/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2011 Nokia Corporation and/or its subsidiary(-ies).

   Translated from smp.h.
*/

#[repr(C, packed)]
pub struct smp_command_hdr {
    pub code: u8,
}

pub const SMP_CMD_PAIRING_REQ: u8 = 0x01;
pub const SMP_CMD_PAIRING_RSP: u8 = 0x02;

#[repr(C, packed)]
pub struct smp_cmd_pairing {
    pub io_capability: u8,
    pub oob_flag: u8,
    pub auth_req: u8,
    pub max_key_size: u8,
    pub init_key_dist: u8,
    pub resp_key_dist: u8,
}

pub const SMP_IO_DISPLAY_ONLY: u8 = 0x00;
pub const SMP_IO_DISPLAY_YESNO: u8 = 0x01;
pub const SMP_IO_KEYBOARD_ONLY: u8 = 0x02;
pub const SMP_IO_NO_INPUT_OUTPUT: u8 = 0x03;
pub const SMP_IO_KEYBOARD_DISPLAY: u8 = 0x04;
pub const SMP_OOB_NOT_PRESENT: u8 = 0x00;
pub const SMP_OOB_PRESENT: u8 = 0x01;
pub const SMP_DIST_ENC_KEY: u8 = 0x01;
pub const SMP_DIST_ID_KEY: u8 = 0x02;
pub const SMP_DIST_SIGN: u8 = 0x04;
pub const SMP_DIST_LINK_KEY: u8 = 0x08;
pub const SMP_AUTH_NONE: u8 = 0x00;
pub const SMP_AUTH_BONDING: u8 = 0x01;
pub const SMP_AUTH_MITM: u8 = 0x04;
pub const SMP_AUTH_SC: u8 = 0x08;
pub const SMP_AUTH_KEYPRESS: u8 = 0x10;
pub const SMP_AUTH_CT2: u8 = 0x20;

pub const SMP_CMD_PAIRING_CONFIRM: u8 = 0x03;
#[repr(C, packed)] pub struct smp_cmd_pairing_confirm { pub confirm_val: [u8; 16], }
pub const SMP_CMD_PAIRING_RANDOM: u8 = 0x04;
#[repr(C, packed)] pub struct smp_cmd_pairing_random { pub rand_val: [u8; 16], }
pub const SMP_CMD_PAIRING_FAIL: u8 = 0x05;
#[repr(C, packed)] pub struct smp_cmd_pairing_fail { pub reason: u8, }
pub const SMP_CMD_ENCRYPT_INFO: u8 = 0x06;
#[repr(C, packed)] pub struct smp_cmd_encrypt_info { pub ltk: [u8; 16], }
pub const SMP_CMD_INITIATOR_IDENT: u8 = 0x07;
#[repr(C, packed)] pub struct smp_cmd_initiator_ident { pub ediv: u16, pub rand: u64, }
pub const SMP_CMD_IDENT_INFO: u8 = 0x08;
#[repr(C, packed)] pub struct smp_cmd_ident_info { pub irk: [u8; 16], }
pub const SMP_CMD_IDENT_ADDR_INFO: u8 = 0x09;
#[repr(C, packed)] pub struct smp_cmd_ident_addr_info { pub addr_type: u8, pub bdaddr: bdaddr_t, }
pub const SMP_CMD_SIGN_INFO: u8 = 0x0a;
#[repr(C, packed)] pub struct smp_cmd_sign_info { pub csrk: [u8; 16], }
pub const SMP_CMD_SECURITY_REQ: u8 = 0x0b;
#[repr(C, packed)] pub struct smp_cmd_security_req { pub auth_req: u8, }
pub const SMP_CMD_PUBLIC_KEY: u8 = 0x0c;
#[repr(C, packed)] pub struct smp_cmd_public_key { pub x: [u8; 32], pub y: [u8; 32], }
pub const SMP_CMD_DHKEY_CHECK: u8 = 0x0d;
#[repr(C, packed)] pub struct smp_cmd_dhkey_check { pub e: [u8; 16], }
pub const SMP_CMD_KEYPRESS_NOTIFY: u8 = 0x0e;
#[repr(C, packed)] pub struct smp_cmd_keypress_notify { pub value: u8, }
pub const SMP_CMD_MAX: u8 = 0x0e;

pub const SMP_PASSKEY_ENTRY_FAILED: u8 = 0x01;
pub const SMP_OOB_NOT_AVAIL: u8 = 0x02;
pub const SMP_AUTH_REQUIREMENTS: u8 = 0x03;
pub const SMP_CONFIRM_FAILED: u8 = 0x04;
pub const SMP_PAIRING_NOTSUPP: u8 = 0x05;
pub const SMP_ENC_KEY_SIZE: u8 = 0x06;
pub const SMP_CMD_NOTSUPP: u8 = 0x07;
pub const SMP_UNSPECIFIED: u8 = 0x08;
pub const SMP_REPEATED_ATTEMPTS: u8 = 0x09;
pub const SMP_INVALID_PARAMS: u8 = 0x0a;
pub const SMP_DHKEY_CHECK_FAILED: u8 = 0x0b;
pub const SMP_NUMERIC_COMP_FAILED: u8 = 0x0c;
pub const SMP_BREDR_PAIRING_IN_PROGRESS: u8 = 0x0d;
pub const SMP_CROSS_TRANSP_NOT_ALLOWED: u8 = 0x0e;
pub const SMP_KEY_REJECTED: u8 = 0x0f;
pub const SMP_MIN_ENC_KEY_SIZE: u8 = 7;
pub const SMP_MAX_ENC_KEY_SIZE: u8 = 16;

#[repr(i32)]
pub enum smp_ltk_type { SMP_STK, SMP_LTK, SMP_LTK_RESPONDER, SMP_LTK_P256, SMP_LTK_P256_DEBUG }

pub unsafe fn smp_ltk_is_sc(key: *mut smp_ltk) -> bool {
    match (*key).r#type {
        SMP_LTK_P256 | SMP_LTK_P256_DEBUG => true,
        _ => false,
    }
}

pub unsafe fn smp_ltk_sec_level(key: *mut smp_ltk) -> u8 {
    if (*key).authenticated {
        if smp_ltk_is_sc(key) { BT_SECURITY_FIPS } else { BT_SECURITY_HIGH }
    } else { BT_SECURITY_MEDIUM }
}

#[repr(C)]
pub enum smp_key_pref { SMP_ALLOW_STK, SMP_USE_LTK }

extern "C" {
    pub fn smp_cancel_and_remove_pairing(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, addr_type: u8) -> i32;
    pub fn smp_sufficient_security(hcon: *mut hci_conn, sec_level: u8, key_pref: smp_key_pref) -> bool;
    pub fn smp_conn_security(hcon: *mut hci_conn, sec_level: u8) -> i32;
    pub fn smp_user_confirm_reply(conn: *mut hci_conn, mgmt_op: u16, passkey: u32) -> i32;
    pub fn smp_irk_matches(hdev: *mut hci_dev, irk: *const u8, bdaddr: *const bdaddr_t) -> bool;
    pub fn smp_generate_rpa(hdev: *mut hci_dev, irk: *const u8, rpa: *mut bdaddr_t) -> i32;
    pub fn smp_generate_oob(hdev: *mut hci_dev, hash: *mut u8, rand: *mut u8) -> i32;
    pub fn smp_force_bredr(hdev: *mut hci_dev, enable: bool) -> i32;
    pub fn smp_register(hdev: *mut hci_dev) -> i32;
    pub fn smp_unregister(hdev: *mut hci_dev);
    pub fn bt_selftest_smp() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
