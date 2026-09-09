// SPDX-License-Identifier: GPL-2.0-only
// External Linux/Ceph declarations supplied by the surrounding kernel tree are
// intentionally referenced but not implemented here.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    pub fn ceph_fsid_compare(a: *const ceph_fsid, b: *const ceph_fsid) -> c_int;
    pub fn ceph_compare_crush_locs(a: *const c_void, b: *const c_void) -> c_int;
    pub fn ceph_monmap_contains(m: *mut c_void, a: *const ceph_entity_addr) -> bool;
    pub fn ceph_parse_ips(a: *const c_char, b: *const c_char, c: *mut ceph_entity_addr, n: c_int, used: *mut c_int, delim: c_char) -> c_int;
    pub fn ceph_parse_crush_location(s: *const c_char, r: *mut c_void) -> c_int;
    pub fn ceph_clear_crush_locs(r: *mut c_void);
    pub fn ceph_crypto_key_destroy(k: *mut ceph_crypto_key);
    pub fn ceph_crypto_key_clone(a: *mut ceph_crypto_key, b: *const ceph_crypto_key) -> c_int;
    pub fn ceph_crypto_key_unarmor(k: *mut ceph_crypto_key, s: *const c_char) -> c_int;
    pub fn ceph_messenger_init(m: *mut c_void, a: *mut ceph_entity_addr);
    pub fn ceph_messenger_fini(m: *mut c_void);
    pub fn ceph_messenger_reset_nonce(m: *mut c_void);
    pub fn ceph_monc_init(m: *mut c_void, c: *mut ceph_client) -> c_int;
    pub fn ceph_monc_stop(m: *mut c_void);
    pub fn ceph_monc_open_session(m: *mut c_void) -> c_int;
    pub fn ceph_monc_reopen_session(m: *mut c_void);
    pub fn ceph_monc_get_version(m: *mut c_void, s: *const c_char, e: *mut u64) -> c_int;
    pub fn ceph_monc_wait_osdmap(m: *mut c_void, e: u64, t: c_ulong) -> c_int;
    pub fn ceph_osdc_init(o: *mut c_void, c: *mut ceph_client) -> c_int;
    pub fn ceph_osdc_stop(o: *mut c_void);
    pub fn ceph_osdc_reopen_osds(o: *mut c_void);
    pub fn ceph_osdc_maybe_request_map(o: *mut c_void);
    pub fn ceph_osdc_setup() -> c_int;
    pub fn ceph_osdc_cleanup();
    pub fn ceph_msgr_init() -> c_int;
    pub fn ceph_msgr_exit();
    pub fn ceph_crypto_init() -> c_int;
    pub fn ceph_crypto_shutdown();
    pub fn ceph_debugfs_init();
    pub fn ceph_debugfs_cleanup();
    pub fn ceph_debugfs_client_init(c: *mut ceph_client);
    pub fn ceph_debugfs_client_cleanup(c: *mut ceph_client);
    pub fn ceph_strings_empty() -> bool;
}

#[repr(C)] pub struct ceph_fsid { pub fsid: [u8; 16] }
#[repr(C)] pub struct ceph_entity_addr { _p: [u8; 0] }
#[repr(C)] pub struct ceph_crypto_key { _p: [u8; 0] }
#[repr(C)] pub struct ceph_options { _p: [u8; 0] }
#[repr(C)] pub struct ceph_client { _p: [u8; 0] }
#[repr(C)] pub struct ceph_param { _p: [u8; 0] }
#[repr(C)] pub struct fc_log { _p: [u8; 0] }
#[repr(C)] pub struct seq_file { pub count: usize, _p: [u8; 0] }

pub const OPT_FSID: u64 = 1 << 0;

#[no_mangle]
pub unsafe extern "C" fn libceph_compatible(_data: *mut c_void) -> bool { true }

#[no_mangle]
pub unsafe extern "C" fn ceph_msg_type_name(ty: c_int) -> *const c_char {
    static NAMES: [&[u8]; 32] = [b"shutdown\0",b"ping\0",b"auth\0",b"auth_reply\0",b"mon_map\0",b"mon_get_map\0",b"mon_subscribe\0",b"mon_subscribe_ack\0",b"statfs\0",b"statfs_reply\0",b"mon_get_version\0",b"mon_get_version_reply\0",b"mds_map\0",b"fs_map_user\0",b"client_session\0",b"client_reconnect\0",b"client_request\0",b"client_request_forward\0",b"client_reply\0",b"client_caps\0",b"client_cap_release\0",b"client_quota\0",b"client_snap\0",b"client_lease\0",b"poolop_reply\0",b"poolop\0",b"mon_command\0",b"mon_command_ack\0",b"osd_map\0",b"osd_op\0",b"osd_opreply\0",b"watch_notify\0"];
    if ty >= 0 && (ty as usize) < NAMES.len() { NAMES[ty as usize].as_ptr() as *const c_char } else { b"unknown\0".as_ptr() as *const c_char }
}

#[no_mangle]
pub unsafe extern "C" fn ceph_check_fsid(_client: *mut ceph_client, _fsid: *mut ceph_fsid) -> c_int { 0 }

// The remaining declarations preserve the implementation interface; their
// bodies are supplied by the translated Ceph support units.
extern "C" {
    pub fn ceph_alloc_options() -> *mut ceph_options;
    pub fn ceph_destroy_options(o: *mut ceph_options);
    pub fn ceph_parse_mon_ips(b: *const c_char, l: usize, o: *mut ceph_options, log: *mut fc_log, d: c_char) -> c_int;
    pub fn ceph_parse_param(p: *mut ceph_param, o: *mut ceph_options, l: *mut fc_log) -> c_int;
    pub fn ceph_print_client_options(m: *mut seq_file, c: *mut ceph_client, all: bool) -> c_int;
    pub fn ceph_client_addr(c: *mut ceph_client) -> *mut ceph_entity_addr;
    pub fn ceph_client_gid(c: *mut ceph_client) -> u64;
    pub fn ceph_create_client(o: *mut ceph_options, private: *mut c_void) -> *mut ceph_client;
    pub fn ceph_destroy_client(c: *mut ceph_client);
    pub fn ceph_reset_client_addr(c: *mut ceph_client);
    pub fn __ceph_open_session(c: *mut ceph_client) -> c_int;
    pub fn ceph_open_session(c: *mut ceph_client) -> c_int;
    pub fn ceph_wait_for_latest_osdmap(c: *mut ceph_client, timeout: c_ulong) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
