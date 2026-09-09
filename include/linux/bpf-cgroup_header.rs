/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_void;

/* Kernel-provided scalar/enumeration values and helpers are intentionally
 * referenced symbolically; their definitions belong to the included headers. */
#[macro_export] macro_rules! cgroup_bpf_enabled { ($atype:expr) => { unsafe { enabled::cgroup_bpf_enabled_key.get_unchecked($atype as usize) } }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET_INGRESS { ($sk:expr, $skb:expr) => {{ let mut __ret = 0; if cgroup_bpf_enabled!(CGROUP_INET_INGRESS) != core::ptr::null() { __ret = unsafe { enabled::__cgroup_bpf_run_filter_skb($sk, $skb, CGROUP_INET_INGRESS) }; } __ret }}; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET_EGRESS { ($sk:expr, $skb:expr) => {{ let mut __ret = 0; if cgroup_bpf_enabled!(CGROUP_INET_EGRESS) != core::ptr::null() { __ret = unsafe { enabled::__cgroup_bpf_run_filter_skb($sk, $skb, CGROUP_INET_EGRESS) }; } __ret }}; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_SK_PROG { ($sk:expr, $atype:expr) => {{ if cgroup_bpf_enabled!($atype) != core::ptr::null() { unsafe { enabled::__cgroup_bpf_run_filter_sk($sk, $atype) } } else { 0 } }}; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET_SOCK { ($sk:expr) => { BPF_CGROUP_RUN_SK_PROG!($sk, CGROUP_INET_SOCK_CREATE) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET_SOCK_RELEASE { ($sk:expr) => { BPF_CGROUP_RUN_SK_PROG!($sk, CGROUP_INET_SOCK_RELEASE) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET4_POST_BIND { ($sk:expr) => { BPF_CGROUP_RUN_SK_PROG!($sk, CGROUP_INET4_POST_BIND) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET6_POST_BIND { ($sk:expr) => { BPF_CGROUP_RUN_SK_PROG!($sk, CGROUP_INET6_POST_BIND) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_SA_PROG { ($sk:expr,$uaddr:expr,$uaddrlen:expr,$atype:expr) => {{ if cgroup_bpf_enabled!($atype) != core::ptr::null() { unsafe { enabled::__cgroup_bpf_run_filter_sock_addr($sk,$uaddr as *mut _, $uaddrlen,$atype,core::ptr::null_mut(),core::ptr::null_mut()) } } else { 0 } }}; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_SA_PROG_LOCK { ($sk:expr,$uaddr:expr,$uaddrlen:expr,$atype:expr,$t_ctx:expr) => { BPF_CGROUP_RUN_SA_PROG!($sk,$uaddr,$uaddrlen,$atype) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET4_CONNECT { ($sk:expr,$uaddr:expr,$len:expr) => { BPF_CGROUP_RUN_SA_PROG!($sk,$uaddr,$len,CGROUP_INET4_CONNECT) }; }
#[macro_export] macro_rules! BPF_CGROUP_RUN_PROG_INET6_CONNECT { ($sk:expr,$uaddr:expr,$len:expr) => { BPF_CGROUP_RUN_SA_PROG!($sk,$uaddr,$len,CGROUP_INET6_CONNECT) }; }
#[macro_export] macro_rules! BPF_CGROUP_PRE_CONNECT_ENABLED { ($sk:expr) => { (cgroup_bpf_enabled!(CGROUP_INET4_CONNECT) || cgroup_bpf_enabled!(CGROUP_INET6_CONNECT)) }; }

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct bpf_map { pub map_type: u32 }
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub struct bpf_sock_ops_kern { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table_header { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog_aux { _private: [u8; 0] }
#[repr(C)] pub struct bpf_func_proto { _private: [u8; 0] }
#[repr(C)] pub struct bpf_insn { _private: [u8; 0] }
#[repr(C)] pub struct bpf_link { _private: [u8; 0] }
#[repr(C)] pub struct bpf_storage_buffer { pub rcu: rcu_head, pub data: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct bpf_cgroup_storage_key { _private: [u8; 0] }
#[repr(C)] pub struct bpf_cgroup_storage_map { _private: [u8; 0] }
pub type sockaddr_unsized = sockaddr;
pub type sockptr_t = *mut c_void;
pub type loff_t = i64;
pub type size_t = usize;
pub type u32 = u32;
pub type u64 = u64;

extern "C" {
    pub fn __cgroup_bpf_run_lsm_sock(ctx: *const c_void, insn: *const bpf_insn) -> u32;
    pub fn __cgroup_bpf_run_lsm_socket(ctx: *const c_void, insn: *const bpf_insn) -> u32;
    pub fn __cgroup_bpf_run_lsm_current(ctx: *const c_void, insn: *const bpf_insn) -> u32;
}

#[cfg(CONFIG_CGROUP_BPF)]
pub mod enabled {
    use super::*;

    #[repr(C)] pub struct static_key_false { _private: [u8; 0] }
    #[repr(C)] pub struct bpf_prog_array { _private: [u8; 0] }
    #[repr(C)] pub struct bpf_cgroup_link { pub link: bpf_link, pub cgroup: *mut cgroup }
    #[repr(C)] pub union bpf_storage_buffer_union { pub buf: *mut bpf_storage_buffer, pub percpu_buf: *mut c_void }
    #[repr(C)] pub struct bpf_cgroup_storage {
        pub storage: bpf_storage_buffer_union,
        pub map: *mut bpf_cgroup_storage_map,
        pub key: bpf_cgroup_storage_key,
        pub list_map: list_head,
        pub list_cg: list_head,
        pub node: rb_node,
        pub rcu: rcu_head,
    }
    #[repr(C)] pub struct bpf_prog_list {
        pub node: hlist_node,
        pub prog: *mut bpf_prog,
        pub link: *mut bpf_cgroup_link,
        pub storage: [*mut bpf_cgroup_storage; 0],
        pub flags: u32,
    }

    pub const MAX_BPF_CGROUP_ATTACH_TYPE: usize = 0; // supplied by linux/bpf-cgroup-defs.h
    pub const MAX_BPF_CGROUP_STORAGE_TYPE: usize = 0; // supplied by linux/bpf-cgroup-defs.h
    pub static mut cgroup_bpf_enabled_key: [static_key_false; MAX_BPF_CGROUP_ATTACH_TYPE] = [];

    pub unsafe fn to_cgroup_bpf_attach_type(attach_type: i32) -> i32 {
        match attach_type {
            BPF_CGROUP_ATTACH_TYPE_VALUES => attach_type,
            _ => CGROUP_BPF_ATTACH_TYPE_INVALID,
        }
    }
    pub const BPF_CGROUP_ATTACH_TYPE_VALUES: i32 = 0; // enum values supplied by linux/bpf.h
    pub const CGROUP_BPF_ATTACH_TYPE_INVALID: i32 = -1;

    extern "C" {
        pub fn cgroup_bpf_lifetime_notifier_init();
        pub fn __cgroup_bpf_run_filter_skb(sk: *mut sock, skb: *mut sk_buff, atype: i32) -> i32;
        pub fn __cgroup_bpf_run_filter_sk(sk: *mut sock, atype: i32) -> i32;
        pub fn __cgroup_bpf_run_filter_sock_addr(sk: *mut sock, uaddr: *mut sockaddr_unsized, uaddrlen: *mut i32, atype: i32, t_ctx: *mut c_void, flags: *mut u32) -> i32;
        pub fn __cgroup_bpf_run_filter_sock_ops(sk: *mut sock, sock_ops: *mut bpf_sock_ops_kern, atype: i32) -> i32;
        pub fn __cgroup_bpf_check_dev_permission(dev_type: i16, major: u32, minor: u32, access: i16, atype: i32) -> i32;
        pub fn __cgroup_bpf_run_filter_sysctl(head: *mut ctl_table_header, table: *const ctl_table, write: i32, buf: *mut *mut u8, pcount: *mut size_t, ppos: *mut loff_t, atype: i32) -> i32;
        pub fn __cgroup_bpf_run_filter_setsockopt(sock: *mut sock, level: *mut i32, optname: *mut i32, optval: sockptr_t, optlen: *mut i32, kernel_optval: *mut *mut u8) -> i32;
        pub fn __cgroup_bpf_run_filter_getsockopt(sk: *mut sock, level: i32, optname: i32, optval: sockptr_t, optlen: sockptr_t, max_optlen: i32, retval: i32) -> i32;
        pub fn __cgroup_bpf_run_filter_getsockopt_kern(sk: *mut sock, level: i32, optname: i32, optval: *mut c_void, optlen: *mut i32, retval: i32) -> i32;
        pub fn cgroup_bpf_prog_attach(attr: *const c_void, ptype: i32, prog: *mut bpf_prog) -> i32;
        pub fn cgroup_bpf_prog_detach(attr: *const c_void, ptype: i32) -> i32;
        pub fn cgroup_bpf_link_attach(attr: *const c_void, prog: *mut bpf_prog) -> i32;
        pub fn cgroup_bpf_prog_query(attr: *const c_void, uattr: *mut c_void, uattr_size: u32) -> i32;
        pub fn cgroup_common_func_proto(func_id: i32, prog: *const bpf_prog) -> *const bpf_func_proto;
        pub fn cgroup_storage_lookup(map: *mut bpf_cgroup_storage_map, key: *mut c_void, locked: bool) -> *mut bpf_cgroup_storage;
        pub fn bpf_cgroup_storage_alloc(prog: *mut bpf_prog, stype: i32) -> *mut bpf_cgroup_storage;
        pub fn bpf_cgroup_storage_free(storage: *mut bpf_cgroup_storage);
        pub fn bpf_cgroup_storage_link(storage: *mut bpf_cgroup_storage, cgroup: *mut cgroup, ty: i32);
        pub fn bpf_cgroup_storage_unlink(storage: *mut bpf_cgroup_storage);
        pub fn bpf_cgroup_storage_assign(aux: *mut bpf_prog_aux, map: *mut bpf_map) -> i32;
        pub fn bpf_percpu_cgroup_storage_copy(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i32;
        pub fn bpf_percpu_cgroup_storage_update(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i32;
    }

    #[inline] pub unsafe fn cgroup_storage_type(map: *mut bpf_map) -> i32 {
        if (*map).map_type == BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE { BPF_CGROUP_STORAGE_PERCPU } else { BPF_CGROUP_STORAGE_SHARED }
    }
    pub const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: u32 = 0;
    pub const BPF_CGROUP_STORAGE_PERCPU: i32 = 0;
    pub const BPF_CGROUP_STORAGE_SHARED: i32 = 1;
}

#[cfg(not(CONFIG_CGROUP_BPF))]
pub mod disabled {
    pub unsafe fn cgroup_bpf_lifetime_notifier_init() {}
    pub unsafe fn cgroup_bpf_prog_attach(_: *const core::ffi::c_void, _: i32, _: *mut super::bpf_prog) -> i32 { -22 }
    pub unsafe fn cgroup_bpf_prog_detach(_: *const core::ffi::c_void, _: i32) -> i32 { -22 }
    pub unsafe fn cgroup_bpf_link_attach(_: *const core::ffi::c_void, _: *mut super::bpf_prog) -> i32 { -22 }
    pub unsafe fn cgroup_bpf_prog_query(_: *const core::ffi::c_void, _: *mut core::ffi::c_void, _: u32) -> i32 { -22 }
    pub unsafe fn cgroup_common_func_proto(_: i32, _: *const super::bpf_prog) -> *const super::bpf_func_proto { core::ptr::null() }
    pub unsafe fn bpf_cgroup_storage_assign(_: *mut super::bpf_prog_aux, _: *mut super::bpf_map) -> i32 { 0 }
    pub unsafe fn bpf_cgroup_storage_alloc(_: *mut super::bpf_prog, _: i32) -> *mut super::bpf_cgroup_storage { core::ptr::null_mut() }
    pub unsafe fn bpf_cgroup_storage_free(_: *mut super::bpf_cgroup_storage) {}
    pub unsafe fn bpf_percpu_cgroup_storage_copy(_: *mut super::bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: u64) -> i32 { 0 }
    pub unsafe fn bpf_percpu_cgroup_storage_update(_: *mut super::bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: u64) -> i32 { 0 }
    pub const EINVAL: i32 = -22;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
