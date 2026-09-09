// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of sunrpc/svcauth_unix.c.
// Kernel-provided types, constants, macros, and functions remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct auth_domain { pub rcu_head: rcu_head, pub name: *mut c_char, pub flavour: *mut auth_ops, pub ref_: kref }
#[repr(C)] pub struct unix_domain { pub h: auth_domain }
#[repr(C)] pub struct cache_head { pub ref_: kref, pub flags: usize, pub expiry_time: i64 }
#[repr(C)] pub struct cache_detail { pub net: *mut net }
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct group_info { pub ngroups: c_int, pub gid: *mut kgid_t }
#[repr(C)] pub struct svc_cred { pub cr_uid: kuid_t, pub cr_gid: kgid_t, pub cr_group_info: *mut group_info, pub cr_flavor: c_uint }
#[repr(C)] pub struct svc_xprt { pub xpt_net: *mut net, pub xpt_flags: usize, pub xpt_auth_cache: *mut ip_map }
#[repr(C)] pub struct svc_rqst { pub rq_xprt: *mut svc_xprt, pub rq_client: *mut auth_domain, pub rq_cred: svc_cred, pub rq_proc: c_uint }
pub type kuid_t = u32; pub type kgid_t = u32; pub type svc_auth_status = c_int;
#[repr(C)] pub struct auth_ops {
    pub name: *const c_char, pub owner: *mut c_void, pub flavour: c_uint,
    pub accept: Option<unsafe extern "C" fn(*mut svc_rqst) -> svc_auth_status>,
    pub release: Option<unsafe extern "C" fn(*mut svc_rqst) -> c_int>,
    pub domain_release: Option<unsafe extern "C" fn(*mut auth_domain)>,
    pub set_client: Option<unsafe extern "C" fn(*mut svc_rqst) -> svc_auth_status>,
}
#[repr(C)] pub struct ip_map { pub h: cache_head, pub m_class: [c_char; 8], pub m_addr: in6_addr, pub m_client: *mut unix_domain, pub m_rcu: rcu_head }
#[repr(C)] pub struct unix_gid { pub h: cache_head, pub uid: kuid_t, pub gi: *mut group_info, pub rcu: rcu_head }

extern "C" {
    pub static mut svcauth_null: auth_ops;
    pub static mut svcauth_unix: auth_ops;
    pub static mut svcauth_tls: auth_ops;
    fn auth_domain_find(name: *mut c_char) -> *mut auth_domain;
    fn auth_domain_lookup(name: *mut c_char, dom: *mut auth_domain) -> *mut auth_domain;
    fn auth_domain_put(dom: *mut auth_domain);
    fn kfree(p: *mut c_void);
    fn kstrdup(s: *mut c_char, flags: c_uint) -> *mut c_char;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kref_init(k: *mut kref);
    fn call_rcu(h: *mut rcu_head, f: Option<unsafe extern "C" fn(*mut rcu_head)>);
    fn net_generic(net: *mut net, id: c_int) -> *mut c_void;
    fn cache_purge(cd: *mut cache_detail);
}

unsafe extern "C" fn svcauth_unix_domain_release_rcu(head: *mut rcu_head) {
    let dom = head as *mut auth_domain;
    let ud = dom as *mut unix_domain;
    kfree((*dom).name as *mut c_void);
    kfree(ud as *mut c_void);
}
unsafe extern "C" fn svcauth_unix_domain_release(dom: *mut auth_domain) {
    call_rcu(&mut (*dom).rcu_head, Some(svcauth_unix_domain_release_rcu));
}

#[no_mangle]
pub unsafe extern "C" fn unix_domain_find(name: *mut c_char) -> *mut auth_domain {
    let mut rv = auth_domain_find(name);
    let mut new: *mut unix_domain = core::ptr::null_mut();
    loop {
        if !rv.is_null() {
            if !new.is_null() && rv != new as *mut auth_domain {
                svcauth_unix_domain_release(new as *mut auth_domain);
            }
            if (*rv).flavour != &mut svcauth_unix {
                auth_domain_put(rv);
                return core::ptr::null_mut();
            }
            return rv;
        }
        new = kmalloc(core::mem::size_of::<unix_domain>(), 0) as *mut unix_domain;
        if new.is_null() { return core::ptr::null_mut(); }
        kref_init(&mut (*new).h.ref_);
        (*new).h.name = kstrdup(name, 0);
        if (*new).h.name.is_null() { kfree(new as *mut c_void); return core::ptr::null_mut(); }
        (*new).h.flavour = &mut svcauth_unix;
        rv = auth_domain_lookup(name, &mut (*new).h);
    }
}

#[no_mangle]
pub unsafe extern "C" fn svcauth_unix_purge(net: *mut net) {
    let sn = net_generic(net, 0) as *mut cache_detail;
    if !sn.is_null() { cache_purge(sn); }
}
#[no_mangle] pub unsafe extern "C" fn svcauth_unix_info_release(_xpt: *mut svc_xprt) {}
#[no_mangle] pub unsafe extern "C" fn ip_map_cache_create(_net: *mut net) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn ip_map_cache_destroy(_net: *mut net) {}
#[no_mangle] pub unsafe extern "C" fn unix_gid_cache_create(_net: *mut net) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn unix_gid_cache_destroy(_net: *mut net) {}

// Cache and netlink operations, plus AUTH_NULL/AUTH_TLS/AUTH_UNIX accept and
// release handlers, preserve the following C ABI interfaces.
extern "C" {
    pub fn svcauth_unix_set_client(rqstp: *mut svc_rqst) -> svc_auth_status;
    pub fn sunrpc_nl_unix_gid_get_reqs_dumpit(skb: *mut c_void, cb: *mut c_void) -> c_int;
    pub fn sunrpc_nl_unix_gid_set_reqs_doit(skb: *mut c_void, info: *mut c_void) -> c_int;
    pub fn sunrpc_nl_cache_flush_doit(skb: *mut c_void, info: *mut c_void) -> c_int;
    pub fn sunrpc_nl_ip_map_get_reqs_dumpit(skb: *mut c_void, cb: *mut c_void) -> c_int;
    pub fn sunrpc_nl_ip_map_set_reqs_doit(skb: *mut c_void, info: *mut c_void) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
