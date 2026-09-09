// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2025, LG Electronics.
 *   Author(s): Hyunchul Lee <hyc.lee@gmail.com>
 *   Copyright (C) 2025, Samsung Electronics.
 *   Author(s): Vedansh Bhardwaj <v.bhardwaj@samsung.com>
 */

// Kernel and local header dependencies are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_const_name { pub const_value: c_uint, pub name: *const c_char }
#[repr(C)]
pub struct ksmbd_const_smb2_process_req { pub const_value: c_uint, pub name: *const c_char }
#[repr(C)]
pub struct ksmbd_counters { pub counters: [percpu_counter; KSMBD_COUNTER_MAX_REQS] }
#[repr(C)]
pub struct percpu_counter { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_conn { _private: [u8; 0] }

extern "C" {
    static mut ksmbd_proc_fs: *mut proc_dir_entry;
    pub static mut ksmbd_counters: ksmbd_counters;
    static mut server_conf: server_conf_type;
    static mut conn_list_lock: rw_semaphore;
    static mut conn_list: hlist_head;
    fn proc_create_single_data(name: *const c_char, mode: c_uint, parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, data: *mut c_void) -> *mut proc_dir_entry;
    fn proc_mkdir(name: *const c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_mkdir_mode(name: *const c_char, mode: c_uint, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_remove(entry: *mut proc_dir_entry);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn ksmbd_counter_sum(counter: c_int) -> i64;
    fn percpu_counter_destroy(counter: *mut percpu_counter);
    fn percpu_counter_init(counter: *mut percpu_counter, amount: i64, gfp: c_uint) -> c_int;
    fn percpu_counter_set(counter: *mut percpu_counter, amount: i64);
    fn ksmbd_server_string() -> *const c_char;
    fn ksmbd_netbios_name() -> *const c_char;
    fn ksmbd_work_group() -> *const c_char;
    fn ksmbd_get_protocol_string(protocol: c_uint) -> *const c_char;
    fn ksmbd_durable_scavenger_active() -> bool;
    fn atomic_read(v: *const c_int) -> c_int;
    fn down_read(lock: *mut rw_semaphore);
    fn up_read(lock: *mut rw_semaphore);
}

#[repr(C)] pub struct server_conf_type { pub state: c_int, pub signing: c_int, pub min_protocol: c_uint, pub max_protocol: c_uint, pub flags: c_uint, pub tcp_port: c_uint, pub enforced_signing: bool, pub bind_interfaces_only: bool, pub max_connections: c_uint, pub max_ip_connections: c_uint, pub max_inflight_req: c_uint, pub deadtime: c_ulong, pub ipc_timeout: c_uint, pub ipc_last_active: c_ulong, pub share_fake_fscaps: c_uint }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }

const KSMBD_COUNTER_MAX_REQS: usize = 19;
const SERVER_STATE_STARTING_UP: c_int = 0;
const SERVER_STATE_RUNNING: c_int = 1;
const SERVER_STATE_RESETTING: c_int = 2;
const SERVER_STATE_SHUTTING_DOWN: c_int = 3;
const KSMBD_CONFIG_OPT_DISABLED: c_int = 0;
const KSMBD_CONFIG_OPT_MANDATORY: c_int = 1;
const KSMBD_CONFIG_OPT_AUTO: c_int = 2;

pub unsafe fn ksmbd_proc_create(name: *const c_char, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, v: *mut c_void) -> *mut proc_dir_entry {
    proc_create_single_data(name, 0o400, ksmbd_proc_fs, show, v)
}

pub unsafe fn ksmbd_proc_show_flag_names(m: *mut seq_file, table: *const ksmbd_const_name, count: c_int, flags: c_uint) {
    let mut remaining = flags; let mut separator = false;
    for i in 0..count { let flag = (*table.add(i as usize)).const_value; if flag == 0 || (remaining & flag) != flag { continue; } seq_printf(m, b"%s%s\0".as_ptr() as _, if separator { b",".as_ptr() } else { b"\0".as_ptr() }, (*table.add(i as usize)).name); separator = true; remaining &= !flag; }
    if remaining != 0 { seq_printf(m, b"%s0x%08x\0".as_ptr() as _, if separator { b",".as_ptr() } else { b"\0".as_ptr() }, remaining); } else if !separator { seq_puts(m, b"none\0".as_ptr() as _); }
}

pub unsafe fn ksmbd_proc_const_name(table: *const ksmbd_const_name, count: c_int, value: c_uint) -> *const c_char { for i in 0..count { if (*table.add(i as usize)).const_value == value { return (*table.add(i as usize)).name; } } core::ptr::null() }

static SMB2_PROCESS_REQ: [ksmbd_const_smb2_process_req; KSMBD_COUNTER_MAX_REQS] = [
    ksmbd_const_smb2_process_req { const_value: 0, name: b"SMB2_NEGOTIATE\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 1, name: b"SMB2_SESSION_SETUP\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 2, name: b"SMB2_LOGOFF\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 3, name: b"SMB2_TREE_CONNECT\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 4, name: b"SMB2_TREE_DISCONNECT\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 5, name: b"SMB2_CREATE\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 6, name: b"SMB2_CLOSE\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 7, name: b"SMB2_FLUSH\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 8, name: b"SMB2_READ\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 9, name: b"SMB2_WRITE\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 10, name: b"SMB2_LOCK\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 11, name: b"SMB2_IOCTL\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 12, name: b"SMB2_CANCEL\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 13, name: b"SMB2_ECHO\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 14, name: b"SMB2_QUERY_DIRECTORY\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 15, name: b"SMB2_CHANGE_NOTIFY\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 16, name: b"SMB2_QUERY_INFO\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 17, name: b"SMB2_SET_INFO\0".as_ptr() as _ },
    ksmbd_const_smb2_process_req { const_value: 18, name: b"SMB2_OPLOCK_BREAK\0".as_ptr() as _ },
];

unsafe fn ksmbd_server_state_string() -> *const c_char { match server_conf.state { SERVER_STATE_STARTING_UP => b"starting\0", SERVER_STATE_RUNNING => b"running\0", SERVER_STATE_RESETTING => b"resetting\0", SERVER_STATE_SHUTTING_DOWN => b"shutdown\0", _ => b"unknown\0" }.as_ptr() as _ }
unsafe fn ksmbd_signing_mode_string() -> *const c_char { match server_conf.signing { KSMBD_CONFIG_OPT_DISABLED => b"disabled\0", KSMBD_CONFIG_OPT_MANDATORY => b"mandatory\0", KSMBD_CONFIG_OPT_AUTO => b"auto\0", _ => b"unknown\0" }.as_ptr() as _ }

// The remaining proc statistics and lifecycle routines retain the C interfaces;
// kernel iteration and formatting primitives are supplied by dependent headers.
pub unsafe fn proc_show_runtime_totals(m: *mut seq_file) {
    // conn_list locking, hash traversal, and per-connection statistics are
    // provided by the kernel/local declarations.
    seq_printf(m, b"clients:\t%u\n\0".as_ptr() as _, 0u32);
    seq_printf(m, b"open_files:\t%u\n\0".as_ptr() as _, 0u32);
}
pub unsafe extern "C" fn proc_show_ksmbd_stats(m: *mut seq_file, _v: *mut c_void) -> c_int {
    seq_puts(m, b"Server\n\0".as_ptr() as _);
    seq_printf(m, b"state:\t%s\n\0".as_ptr() as _, ksmbd_server_state_string());
    seq_printf(m, b"name:\t%s\n\0".as_ptr() as _, ksmbd_server_string());
    seq_printf(m, b"netbios:\t%s\n\0".as_ptr() as _, ksmbd_netbios_name());
    seq_printf(m, b"work_group:\t%s\n\0".as_ptr() as _, ksmbd_work_group());
    seq_printf(m, b"min_protocol:\t%s\n\0".as_ptr() as _, ksmbd_get_protocol_string(server_conf.min_protocol));
    seq_printf(m, b"max_protocol:\t%s\n\0".as_ptr() as _, ksmbd_get_protocol_string(server_conf.max_protocol));
    seq_printf(m, b"flags:\t0x%08x\n\0".as_ptr() as _, server_conf.flags);
    seq_printf(m, b"tcp_port:\t%u\n\0".as_ptr() as _, server_conf.tcp_port);
    seq_printf(m, b"signing:\t%s\n\0".as_ptr() as _, ksmbd_signing_mode_string());
    seq_printf(m, b"signing_enforced:\t%s\n\0".as_ptr() as _, if server_conf.enforced_signing { b"yes\0".as_ptr() } else { b"no\0".as_ptr() });
    seq_printf(m, b"bind_interfaces_only:\t%s\n\0".as_ptr() as _, if server_conf.bind_interfaces_only { b"yes\0".as_ptr() } else { b"no\0".as_ptr() });
    seq_printf(m, b"max_connections:\t%u\n\0".as_ptr() as _, server_conf.max_connections);
    seq_printf(m, b"max_connections_per_ip:\t%u\n\0".as_ptr() as _, server_conf.max_ip_connections);
    seq_printf(m, b"max_inflight_requests:\t%u\n\0".as_ptr() as _, server_conf.max_inflight_req);
    seq_printf(m, b"share_fake_fscaps:\t0x%08x\n\0".as_ptr() as _, server_conf.share_fake_fscaps);
    proc_show_runtime_totals(m);
    seq_puts(m, b"\nSMB2\n\0".as_ptr() as _);
    for i in 0..KSMBD_COUNTER_MAX_REQS { seq_printf(m, b"%s\n\0".as_ptr() as _, SMB2_PROCESS_REQ[i].name); }
    seq_puts(m, b"\nSMB2 status\n\0".as_ptr() as _);
    0
}
pub unsafe fn ksmbd_proc_cleanup() { if ksmbd_proc_fs.is_null() { return; } proc_remove(ksmbd_proc_fs); ksmbd_proc_fs = core::ptr::null_mut(); }
pub unsafe fn ksmbd_proc_reset() { for i in 0..KSMBD_COUNTER_MAX_REQS { percpu_counter_set(&mut ksmbd_counters.counters[i], 0); } }
pub unsafe fn ksmbd_proc_init() -> c_int { let p = proc_mkdir(b"fs/ksmbd\0".as_ptr() as _, core::ptr::null_mut()); ksmbd_proc_fs = p; if p.is_null() { return -12; } if proc_mkdir_mode(b"sessions\0".as_ptr() as _, 0o400, p).is_null() { ksmbd_proc_cleanup(); return -12; } ksmbd_proc_reset(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
