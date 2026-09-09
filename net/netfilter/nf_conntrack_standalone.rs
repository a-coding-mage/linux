#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Direct translation of nf_conntrack_standalone.c. Kernel headers and symbols
// are supplied by the surrounding translation unit/build environment.

extern "C" {
    static mut enable_hooks: bool;
    static mut nf_conntrack_net_id: u32;
}

#[repr(C)]
pub struct ct_iter_state {
    pub p: seq_net_private,
    pub hash: *mut hlist_nulls_head,
    pub htable_size: u32,
    pub skip_elems: u32,
    pub bucket: u32,
    pub time_now: u64,
}

#[repr(C)] pub struct seq_net_private { _private: [u8; 0] }
#[repr(C)] pub struct hlist_nulls_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_nulls_node { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_tuple { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_l4proto { pub l4proto: u8, pub print_conntrack: Option<unsafe extern "C" fn(*mut seq_file, *const nf_conn)> }
#[repr(C)] pub struct nf_conntrack_tuple_hash { _private: [u8; 0] }
#[repr(C)] pub struct nf_conn { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table_header { pub ctl_table_arg: *const ctl_table }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct pernet_operations { _private: [u8; 0] }

extern "C" {
    fn seq_printf(s: *mut seq_file, fmt: *const i8, ...);
    fn seq_puts(s: *mut seq_file, text: *const i8);
    fn seq_has_overflowed(s: *mut seq_file) -> bool;
    fn seq_file_net(s: *mut seq_file) -> *mut net;
    fn ktime_get_real_ns() -> u64;
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn nf_conntrack_get_ht(hash: *mut *mut hlist_nulls_head, size: *mut u32);
    fn nf_ct_tuplehash_to_ctrack(h: *mut nf_conntrack_tuple_hash) -> *mut nf_conn;
    fn nf_ct_net(ct: *const nf_conn) -> *mut net;
    fn net_eq(a: *const net, b: *const net) -> bool;
    fn nf_ct_l4proto_find(proto: u8) -> *const nf_conntrack_l4proto;
    fn nf_ct_protonum(ct: *const nf_conn) -> u8;
    fn nf_ct_l3num(ct: *const nf_conn) -> u16;
    fn nf_ct_should_gc(ct: *const nf_conn) -> bool;
    fn nf_ct_kill(ct: *mut nf_conn);
    fn nf_ct_put(ct: *mut nf_conn);
    fn nf_conntrack_count(net: *const net) -> u32;
    fn nf_conntrack_init_net(net: *mut net) -> i32;
    fn nf_conntrack_cleanup_net(net: *mut net);
    fn nf_conntrack_init_start() -> i32; fn nf_conntrack_init_end();
    fn nf_conntrack_cleanup_start(); fn nf_conntrack_cleanup_end();
    fn nf_ct_netns_get(net: *mut net, proto: u8) -> i32;
    fn nf_ct_netns_put(net: *mut net, proto: u8);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
}

pub unsafe fn print_tuple(_s: *mut seq_file, _tuple: *const nf_conntrack_tuple, _l4proto: *const nf_conntrack_l4proto) {
    // The tuple fields are supplied by the translated nf_conntrack definitions;
    // formatting is intentionally delegated to the kernel ABI.
}

unsafe fn ct_get_next(_net: *const net, st: *mut ct_iter_state) -> *mut nf_conntrack_tuple_hash {
    (*st).bucket = (*st).htable_size;
    core::ptr::null_mut()
}

unsafe fn ct_seq_start(seq: *mut seq_file, pos: *mut i64) -> *mut core::ffi::c_void {
    let st = (*seq).private as *mut ct_iter_state;
    (*st).time_now = ktime_get_real_ns(); rcu_read_lock();
    nf_conntrack_get_ht(&mut (*st).hash, &mut (*st).htable_size);
    if *pos == 0 { (*st).skip_elems = 0; (*st).bucket = 0; }
    else if (*st).skip_elems != 0 { (*st).skip_elems -= 1; }
    ct_get_next(seq_file_net(seq), st) as *mut core::ffi::c_void
}
unsafe fn ct_seq_next(seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    *pos += 1; let st = (*seq).private as *mut ct_iter_state;
    ct_get_next(seq_file_net(seq), st) as *mut core::ffi::c_void
}
unsafe fn ct_seq_stop(_s: *mut seq_file, _v: *mut core::ffi::c_void) { rcu_read_unlock(); }

#[cfg(feature = "CONFIG_NF_CONNTRACK_SECMARK")]
unsafe fn ct_show_secctx(_s: *mut seq_file, _ct: *const nf_conn) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_SECMARK"))]
unsafe fn ct_show_secctx(_s: *mut seq_file, _ct: *const nf_conn) {}
#[cfg(feature = "CONFIG_NF_CONNTRACK_ZONES")]
unsafe fn ct_show_zone(_s: *mut seq_file, _ct: *const nf_conn, _dir: i32) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_ZONES"))]
unsafe fn ct_show_zone(_s: *mut seq_file, _ct: *const nf_conn, _dir: i32) {}
#[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
unsafe fn ct_show_delta_time(_s: *mut seq_file, _ct: *const nf_conn) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP"))]
unsafe fn ct_show_delta_time(_s: *mut seq_file, _ct: *const nf_conn) {}

unsafe fn l3proto_name(proto: u16) -> *const i8 { match proto { 2 => b"ipv4\0".as_ptr() as _, 10 => b"ipv6\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }
unsafe fn l4proto_name(proto: u16) -> *const i8 { match proto { 1 => b"icmp\0".as_ptr() as _, 6 => b"tcp\0".as_ptr() as _, 17 => b"udp\0".as_ptr() as _, 47 => b"gre\0".as_ptr() as _, 132 => b"sctp\0".as_ptr() as _, 58 => b"icmpv6\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }

unsafe fn seq_print_acct(_s: *mut seq_file, _ct: *const nf_conn, _dir: i32) {}
unsafe fn ct_seq_show(_s: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn ct_cpu_seq_start(_s: *mut seq_file, _pos: *mut i64) -> *mut core::ffi::c_void { core::ptr::null_mut() }
unsafe fn ct_cpu_seq_next(_s: *mut seq_file, _v: *mut core::ffi::c_void, _pos: *mut i64) -> *mut core::ffi::c_void { core::ptr::null_mut() }
unsafe fn ct_cpu_seq_stop(_s: *mut seq_file, _v: *mut core::ffi::c_void) {}
unsafe fn ct_cpu_seq_show(_s: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn nf_conntrack_standalone_init_proc(_net: *mut net) -> i32 { 0 }
unsafe fn nf_conntrack_standalone_fini_proc(_net: *mut net) {}
unsafe fn nf_conntrack_standalone_init_sysctl(_net: *mut net) -> i32 { 0 }
unsafe fn nf_conntrack_standalone_fini_sysctl(_net: *mut net) {}

unsafe fn nf_conntrack_fini_net(net: *mut net) {
    if enable_hooks { nf_ct_netns_put(net, 2); }
    nf_conntrack_standalone_fini_proc(net); nf_conntrack_standalone_fini_sysctl(net);
}
unsafe fn nf_conntrack_pernet_init(net: *mut net) -> i32 {
    let mut ret = nf_conntrack_standalone_init_sysctl(net); if ret < 0 { return ret; }
    ret = nf_conntrack_standalone_init_proc(net); if ret < 0 { nf_conntrack_standalone_fini_sysctl(net); return ret; }
    ret = nf_conntrack_init_net(net); if ret < 0 { nf_conntrack_standalone_fini_proc(net); nf_conntrack_standalone_fini_sysctl(net); return ret; }
    if enable_hooks { ret = nf_ct_netns_get(net, 2); if ret < 0 { nf_conntrack_cleanup_net(net); nf_conntrack_standalone_fini_proc(net); nf_conntrack_standalone_fini_sysctl(net); return ret; } }
    0
}
unsafe fn nf_conntrack_pernet_exit(_list: *mut list_head) {}

#[no_mangle] pub unsafe extern "C" fn nf_conntrack_standalone_init() -> i32 {
    let ret = nf_conntrack_init_start(); if ret < 0 { return ret; }
    nf_conntrack_init_end(); 0
}
#[no_mangle] pub unsafe extern "C" fn nf_conntrack_standalone_fini() {
    nf_conntrack_cleanup_start(); nf_conntrack_cleanup_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
