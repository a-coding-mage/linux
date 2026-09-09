// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*-
 * sysctl_net.c: sysctl interface to net subsystem.
 */

use core::ffi::{c_char, c_int, c_void};

// These types and functions are supplied by the surrounding kernel sources.
#[repr(C)] pub struct ctl_table_set { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table_header { pub set: *mut ctl_table_set }
#[repr(C)] pub struct ctl_table_root {
    pub lookup: Option<unsafe extern "C" fn(*mut ctl_table_root) -> *mut ctl_table_set>,
    pub permissions: Option<unsafe extern "C" fn(*mut ctl_table_header, *const ctl_table) -> c_int>,
    pub set_ownership: Option<unsafe extern "C" fn(*mut ctl_table_header, *mut kuid_t, *mut kgid_t)>,
}
#[repr(C)] pub struct ctl_table {
    pub procname: *const c_char,
    pub mode: u16,
    pub proc_handler: *mut c_void,
    pub data: *mut c_void,
}
#[repr(C)] pub struct net { pub sysctls: ctl_table_set, pub user_ns: *mut user_namespace }
#[repr(C)] pub struct net_ns { pub sysctls: ctl_table_set }
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct nsproxy { pub net_ns: *mut net }
#[repr(C)] pub struct task_struct { pub nsproxy: *mut nsproxy }
#[repr(C)] pub struct pernet_operations {
    pub init: Option<unsafe extern "C" fn(*mut net) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut net)>,
}
#[repr(C)] pub struct kuid_t { pub val: u32 }
#[repr(C)] pub struct kgid_t { pub val: u32 }

extern "C" {
    static mut current: *mut task_struct;
    static mut init_net: net;
    fn setup_sysctl_set(set: *mut ctl_table_set, root: *mut ctl_table_root,
                        seen: Option<unsafe extern "C" fn(*mut ctl_table_set) -> c_int>);
    fn retire_sysctl_set(set: *mut ctl_table_set);
    fn register_sysctl_sz(path: *const c_char, table: *mut ctl_table, size: usize) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn __register_sysctl_table(set: *mut ctl_table_set, path: *const c_char,
                               table: *const ctl_table, size: usize) -> *mut ctl_table_header;
    fn ns_capable_noaudit(ns: *mut user_namespace, cap: c_int) -> bool;
    fn make_kuid(ns: *mut user_namespace, uid: u32) -> kuid_t;
    fn make_kgid(ns: *mut user_namespace, gid: u32) -> kgid_t;
    fn uid_valid(uid: kuid_t) -> bool;
    fn gid_valid(gid: kgid_t) -> bool;
    fn net_eq(a: *mut net, b: *mut net) -> bool;
    fn is_module_address(addr: usize) -> bool;
    fn is_kernel_core_data(addr: usize) -> bool;
}

const CAP_NET_ADMIN: c_int = 12;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;

unsafe extern "C" fn net_ctl_header_lookup(_root: *mut ctl_table_root) -> *mut ctl_table_set {
    &mut (*(*current).nsproxy).net_ns.as_mut().unwrap().sysctls
}

unsafe extern "C" fn is_seen(set: *mut ctl_table_set) -> c_int {
    (set == &mut (*(*current).nsproxy).net_ns.as_mut().unwrap().sysctls) as c_int
}

unsafe extern "C" fn net_ctl_permissions(head: *mut ctl_table_header,
                                           table: *const ctl_table) -> c_int {
    let net = (head as *mut net); // container_of(head->set, struct net, sysctls)
    if ns_capable_noaudit((*net).user_ns, CAP_NET_ADMIN) {
        let mode = ((*table).mode >> 6) & 7;
        return ((mode << 6) | (mode << 3) | mode) as c_int;
    }
    (*table).mode as c_int
}

unsafe extern "C" fn net_ctl_set_ownership(head: *mut ctl_table_header,
                                           uid: *mut kuid_t, gid: *mut kgid_t) {
    let net = head as *mut net; // container_of(head->set, struct net, sysctls)
    let ns_root_uid = make_kuid((*net).user_ns, 0);
    if uid_valid(ns_root_uid) { *uid = ns_root_uid; }
    let ns_root_gid = make_kgid((*net).user_ns, 0);
    if gid_valid(ns_root_gid) { *gid = ns_root_gid; }
}

static mut net_sysctl_root: ctl_table_root = ctl_table_root {
    lookup: Some(net_ctl_header_lookup),
    permissions: Some(net_ctl_permissions),
    set_ownership: Some(net_ctl_set_ownership),
};

unsafe extern "C" fn sysctl_net_init(net: *mut net) -> c_int {
    setup_sysctl_set(&mut (*net).sysctls, &mut net_sysctl_root, Some(is_seen));
    0
}
unsafe extern "C" fn sysctl_net_exit(net: *mut net) {
    retire_sysctl_set(&mut (*net).sysctls);
}
static mut sysctl_pernet_ops: pernet_operations = pernet_operations {
    init: Some(sysctl_net_init), exit: Some(sysctl_net_exit),
};
static mut net_header: *mut ctl_table_header = core::ptr::null_mut();

pub unsafe extern "C" fn net_sysctl_init() -> c_int {
    let mut empty: [ctl_table; 1] = unsafe { core::mem::zeroed() };
    let mut ret = -ENOMEM;
    net_header = register_sysctl_sz(c"net".as_ptr(), empty.as_mut_ptr(), 0);
    if net_header.is_null() { return ret; }
    ret = register_pernet_subsys(&mut sysctl_pernet_ops);
    if ret != 0 {
        unregister_sysctl_table(net_header);
        net_header = core::ptr::null_mut();
    }
    ret
}

pub unsafe extern "C" fn register_net_sysctl_sz(net: *mut net, path: *const c_char,
                                                  table: *const ctl_table, table_size: usize)
                                                  -> *mut ctl_table_header {
    if !net_eq(net, &mut init_net) {
        if ensure_safe_net_sysctl(net, path, table, table_size) != 0 {
            return core::ptr::null_mut();
        }
    }
    __register_sysctl_table(&mut (*net).sysctls, path, table, table_size)
}

unsafe fn ensure_safe_net_sysctl(_net: *mut net, _path: *const c_char,
                                 table: *const ctl_table, table_size: usize) -> c_int {
    for i in 0..table_size {
        let ent = &*table.add(i);
        if (ent.mode & 0o222) == 0 { continue; }
        let addr = ent.data as usize;
        if is_module_address(addr) || is_kernel_core_data(addr) {
            // C source emits a WARN identifying the global data leak.
            return -EACCES;
        }
    }
    0
}

pub unsafe extern "C" fn unregister_net_sysctl_table(header: *mut ctl_table_header) {
    unregister_sysctl_table(header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
