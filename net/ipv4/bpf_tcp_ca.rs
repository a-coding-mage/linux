// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook  */

// Kernel headers and build-time macros from the C implementation are external
// dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    static mut tcp_congestion_ops: *mut bpf_struct_ops;
    static mut tcp_congestion_ops_type: *const btf_type;
    static mut tcp_sock_type: *const btf_type;
    static mut tcp_sock_id: u32;
    static mut sock_id: u32;
}

#[repr(C)]
pub struct bpf_struct_ops {
    pub verifier_ops: *const bpf_verifier_ops,
    pub reg: Option<unsafe extern "C" fn(*mut c_void, *mut bpf_link) -> i32>,
    pub unreg: Option<unsafe extern "C" fn(*mut c_void, *mut bpf_link)>,
    pub update: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut bpf_link) -> i32>,
    pub init_member: Option<unsafe extern "C" fn(*const btf_type, *const btf_member, *mut c_void, *const c_void) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut btf) -> i32>,
    pub validate: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub name: *const u8,
    pub cfi_stubs: *mut tcp_congestion_ops,
    pub owner: *mut c_void,
}

#[repr(C)] pub struct btf;
#[repr(C)] pub struct btf_type;
#[repr(C)] pub struct btf_member;
#[repr(C)] pub struct bpf_link;
#[repr(C)] pub struct bpf_prog { pub expected_attach_type: u32 }
#[repr(C)] pub struct bpf_verifier_log;
#[repr(C)] pub struct bpf_reg_state { pub btf: *mut btf, pub btf_id: u32, pub reg_type: u32 }
#[repr(C)] pub struct bpf_insn_access_aux { pub reg_type: u32, pub btf_id: u32 }
#[repr(C)] pub struct sock;
#[repr(C)] pub struct tcp_sock;
#[repr(C)] pub struct inet_connection_sock;
#[repr(C)] pub struct ack_sample;
#[repr(C)] pub struct rate_sample;

#[repr(C)] pub struct tcp_congestion_ops {
    pub ssthresh: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    pub cong_avoid: Option<unsafe extern "C" fn(*mut sock, u32, u32)>,
    pub set_state: Option<unsafe extern "C" fn(*mut sock, u8)>,
    pub cwnd_event: Option<unsafe extern "C" fn(*mut sock, i32)>,
    pub cwnd_event_tx_start: Option<unsafe extern "C" fn(*mut sock)>,
    pub in_ack_event: Option<unsafe extern "C" fn(*mut sock, u32)>,
    pub pkts_acked: Option<unsafe extern "C" fn(*mut sock, *const ack_sample)>,
    pub min_tso_segs: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    pub cong_control: Option<unsafe extern "C" fn(*mut sock, u32, i32, *const rate_sample)>,
    pub undo_cwnd: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    pub sndbuf_expand: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    pub init: Option<unsafe extern "C" fn(*mut sock)>,
    pub release: Option<unsafe extern "C" fn(*mut sock)>,
    pub flags: u32,
    pub name: [u8; 16],
}

#[repr(C)] pub struct bpf_verifier_ops {
    pub get_func_proto: Option<unsafe extern "C" fn(i32, *const bpf_prog) -> *const bpf_func_proto>,
    pub is_valid_access: Option<unsafe extern "C" fn(i32, i32, i32, *const bpf_prog, *mut bpf_insn_access_aux) -> bool>,
    pub btf_struct_access: Option<unsafe extern "C" fn(*mut bpf_verifier_log, *const bpf_reg_state, i32, i32) -> i32>,
}
#[repr(C)] pub struct bpf_func_proto;
#[repr(C)] pub struct btf_kfunc_id_set { pub owner: *mut c_void, pub set: *const u32 }

extern "C" {
    fn btf_find_by_name_kind(*mut btf, *const u8, u32) -> i32;
    fn btf_type_by_id(*mut btf, u32) -> *const btf_type;
    fn bpf_tracing_btf_ctx_access(i32, i32, i32, *const bpf_prog, *mut bpf_insn_access_aux) -> bool;
    fn base_type(u32) -> u32;
    fn bpf_type_has_unsafe_modifiers(u32) -> bool;
    fn bpf_log(*mut bpf_verifier_log, *const u8, ...);
    fn __tcp_send_ack(*mut sock, u32, u32);
    fn bpf_obj_name_cpy(*mut u8, *const u8, usize) -> i32;
    fn tcp_register_congestion_control(*mut c_void) -> i32;
    fn tcp_unregister_congestion_control(*mut c_void);
    fn tcp_update_congestion_control(*mut c_void, *mut c_void) -> i32;
    fn tcp_validate_congestion_control(*mut c_void) -> i32;
    fn __btf_member_bit_offset(*const btf_type, *const btf_member) -> u32;
    fn btf_type_member(*const btf_type) -> *const btf_member;
    fn bpf_base_func_proto(i32, *const bpf_prog) -> *const bpf_func_proto;
    fn register_btf_kfunc_id_set(i32, *const btf_kfunc_id_set) -> i32;
    fn register_bpf_struct_ops(*mut bpf_struct_ops, *mut c_void) -> i32;
    static mut bpf_sk_storage_get_proto: bpf_func_proto;
    static mut bpf_sk_storage_delete_proto: bpf_func_proto;
    static mut bpf_sk_setsockopt_nodelay_proto: bpf_func_proto;
    static mut bpf_sk_getsockopt_proto: bpf_func_proto;
    static mut bpf_ktime_get_coarse_ns_proto: bpf_func_proto;
}

unsafe extern "C" fn bpf_tcp_ca_init(btf: *mut btf) -> i32 {
    let mut type_id = btf_find_by_name_kind(btf, b"sock\0".as_ptr(), 4);
    if type_id < 0 { return -22; }
    sock_id = type_id as u32;
    type_id = btf_find_by_name_kind(btf, b"tcp_sock\0".as_ptr(), 4);
    if type_id < 0 { return -22; }
    tcp_sock_id = type_id as u32;
    tcp_sock_type = btf_type_by_id(btf, tcp_sock_id);
    type_id = btf_find_by_name_kind(btf, b"tcp_congestion_ops\0".as_ptr(), 4);
    if type_id < 0 { return -22; }
    tcp_congestion_ops_type = btf_type_by_id(btf, type_id as u32);
    0
}

unsafe extern "C" fn bpf_tcp_ca_is_valid_access(off: i32, size: i32, access_type: i32, prog: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
    if !bpf_tracing_btf_ctx_access(off, size, access_type, prog, info) { return false; }
    if base_type((*info).reg_type) == 0x100 && !bpf_type_has_unsafe_modifiers((*info).reg_type) && (*info).btf_id == sock_id { (*info).btf_id = tcp_sock_id; }
    true
}

unsafe extern "C" fn bpf_tcp_ca_btf_struct_access(log: *mut bpf_verifier_log, reg: *const bpf_reg_state, off: i32, size: i32) -> i32 {
    if btf_type_by_id((*reg).btf, (*reg).btf_id) != tcp_sock_type { bpf_log(log, b"only read is supported\n\0".as_ptr()); return -13; }
    let end = match off {
        0 => 8, 8 => 12, 16 => 32, 32 => 36, 36 => 40, 40 => 44, 44 => 48, 48 => 52, 52 => 56, 56 => 60,
        _ => { bpf_log(log, b"no write support to tcp_sock at off %d\n\0".as_ptr(), off); return -13; }
    };
    if off + size > end { bpf_log(log, b"write access at off %d with size %d beyond the member of tcp_sock ended at %zu\n\0".as_ptr(), off, size, end); return -13; }
    0
}

unsafe extern "C" fn bpf_tcp_send_ack(tp: *mut tcp_sock, rcv_nxt: u32) -> i32 { __tcp_send_ack(tp as *mut sock, rcv_nxt, 0); 0 }

unsafe extern "C" fn prog_ops_moff(prog: *const bpf_prog) -> u32 { let m = btf_type_member(tcp_congestion_ops_type).add((*prog).expected_attach_type as usize); __btf_member_bit_offset(tcp_congestion_ops_type, m) / 8 }

unsafe extern "C" fn bpf_tcp_ca_get_func_proto(func_id: i32, prog: *const bpf_prog) -> *const bpf_func_proto {
    match func_id {
        1 => core::ptr::null(),
        2 => &bpf_sk_storage_get_proto,
        3 => &bpf_sk_storage_delete_proto,
        4 => if prog_ops_moff(prog) != 0 { &bpf_sk_setsockopt_nodelay_proto } else { core::ptr::null() },
        5 => if prog_ops_moff(prog) != 0 { &bpf_sk_getsockopt_proto } else { core::ptr::null() },
        6 => &bpf_ktime_get_coarse_ns_proto,
        _ => bpf_base_func_proto(func_id, prog),
    }
}

unsafe extern "C" fn bpf_tcp_ca_reg(kdata: *mut c_void, _link: *mut bpf_link) -> i32 { tcp_register_congestion_control(kdata) }
unsafe extern "C" fn bpf_tcp_ca_unreg(kdata: *mut c_void, _link: *mut bpf_link) { tcp_unregister_congestion_control(kdata) }
unsafe extern "C" fn bpf_tcp_ca_update(kdata: *mut c_void, old_kdata: *mut c_void, _link: *mut bpf_link) -> i32 { tcp_update_congestion_control(kdata, old_kdata) }
unsafe extern "C" fn bpf_tcp_ca_validate(kdata: *mut c_void) -> i32 { tcp_validate_congestion_control(kdata) }

unsafe extern "C" fn bpf_tcp_ca_ssthresh(_sk: *mut sock) -> u32 { 0 }
unsafe extern "C" fn bpf_tcp_ca_cong_avoid(_sk: *mut sock, _ack: u32, _acked: u32) {}
unsafe extern "C" fn bpf_tcp_ca_set_state(_sk: *mut sock, _new_state: u8) {}
unsafe extern "C" fn bpf_tcp_ca_cwnd_event(_sk: *mut sock, _ev: i32) {}
unsafe extern "C" fn bpf_tcp_ca_cwnd_event_tx_start(_sk: *mut sock) {}
unsafe extern "C" fn bpf_tcp_ca_in_ack_event(_sk: *mut sock, _flags: u32) {}
unsafe extern "C" fn bpf_tcp_ca_pkts_acked(_sk: *mut sock, _sample: *const ack_sample) {}
unsafe extern "C" fn bpf_tcp_ca_min_tso_segs(_sk: *mut sock) -> u32 { 0 }
unsafe extern "C" fn bpf_tcp_ca_cong_control(_sk: *mut sock, _ack: u32, _flag: i32, _rs: *const rate_sample) {}
unsafe extern "C" fn bpf_tcp_ca_undo_cwnd(_sk: *mut sock) -> u32 { 0 }
unsafe extern "C" fn bpf_tcp_ca_sndbuf_expand(_sk: *mut sock) -> u32 { 0 }
unsafe extern "C" fn __bpf_tcp_ca_init(_sk: *mut sock) {}
unsafe extern "C" fn __bpf_tcp_ca_release(_sk: *mut sock) {}

unsafe extern "C" fn bpf_tcp_ca_init_member(t: *const btf_type, member: *const btf_member, kdata: *mut c_void, udata: *const c_void) -> i32 {
    let utcp_ca = udata as *const tcp_congestion_ops;
    let tcp_ca = kdata as *mut tcp_congestion_ops;
    let moff = __btf_member_bit_offset(t, member) / 8;
    match moff {
        0 => { if (*utcp_ca).flags & !0x3 != 0 { return -22; } (*tcp_ca).flags = (*utcp_ca).flags; 1 }
        _ => 0,
    }
}

static mut __bpf_ops_tcp_congestion_ops: tcp_congestion_ops = tcp_congestion_ops {
    ssthresh: Some(bpf_tcp_ca_ssthresh), cong_avoid: Some(bpf_tcp_ca_cong_avoid), set_state: Some(bpf_tcp_ca_set_state),
    cwnd_event: Some(bpf_tcp_ca_cwnd_event), cwnd_event_tx_start: Some(bpf_tcp_ca_cwnd_event_tx_start), in_ack_event: Some(bpf_tcp_ca_in_ack_event),
    pkts_acked: Some(bpf_tcp_ca_pkts_acked), min_tso_segs: Some(bpf_tcp_ca_min_tso_segs), cong_control: Some(bpf_tcp_ca_cong_control),
    undo_cwnd: Some(bpf_tcp_ca_undo_cwnd), sndbuf_expand: Some(bpf_tcp_ca_sndbuf_expand), init: Some(__bpf_tcp_ca_init), release: Some(__bpf_tcp_ca_release),
    flags: 0, name: [0; 16],
};

static mut bpf_tcp_congestion_ops_impl: bpf_struct_ops = bpf_struct_ops {
    verifier_ops: core::ptr::null(), reg: Some(bpf_tcp_ca_reg), unreg: Some(bpf_tcp_ca_unreg), update: Some(bpf_tcp_ca_update),
    init_member: Some(bpf_tcp_ca_init_member), init: Some(bpf_tcp_ca_init), validate: Some(bpf_tcp_ca_validate), name: b"tcp_congestion_ops\0".as_ptr(),
    cfi_stubs: unsafe { &raw mut __bpf_ops_tcp_congestion_ops }, owner: core::ptr::null_mut(),
};

unsafe extern "C" fn bpf_tcp_ca_kfunc_init() -> i32 {
    let mut ret = register_btf_kfunc_id_set(0, core::ptr::null());
    if ret == 0 { ret = register_bpf_struct_ops(&raw mut bpf_tcp_congestion_ops_impl, tcp_congestion_ops); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
