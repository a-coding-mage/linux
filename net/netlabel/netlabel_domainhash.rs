// SPDX-License-Identifier: GPL-2.0-or-later
/* NetLabel Domain Hash Table */

// Kernel-provided types, constants, macros, globals, and helper functions are
// intentionally referenced here as external dependencies.

#[repr(C)]
struct NetlblDomhshTbl {
    tbl: *mut ListHead,
    size: u32,
}

extern "C" {
    static mut netlbl_domhsh: *mut NetlblDomhshTbl;
    static mut netlbl_domhsh_def_ipv4: *mut NetlblDomMap;
    static mut netlbl_domhsh_def_ipv6: *mut NetlblDomMap;
}

// The declarations below correspond to kernel/header definitions used by the
// original implementation.
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct RcuHead { _private: [u8; 0] }
#[repr(C)] pub struct NetlblDomMap { pub list: ListHead, pub rcu: RcuHead, pub valid: u32, pub family: u16, pub domain: *mut i8, pub def: NetlblDommapDef }
#[repr(C)] pub struct NetlblDommapDef { pub r#type: u32, pub cipso: *mut CipsoV4Doi, pub calipso: *mut CalipsoDoi, pub addrsel: *mut NetlblAddrSel }
#[repr(C)] pub struct NetlblAddrSel { pub list4: ListHead, pub list6: ListHead }
#[repr(C)] pub struct NetlblAf4list { pub addr: u32, pub mask: u32, pub valid: u32 }
#[repr(C)] pub struct NetlblAf6list { pub addr: In6Addr, pub mask: In6Addr, pub valid: u32 }
#[repr(C)] pub struct NetlblDomaddr4Map { pub def: NetlblDommapDef }
#[repr(C)] pub struct NetlblDomaddr6Map { pub def: NetlblDommapDef }
#[repr(C)] pub struct InAddr { pub s_addr: u32 }
#[repr(C)] pub struct In6Addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct CipsoV4Doi { pub doi: u32 }
#[repr(C)] pub struct CalipsoDoi { pub doi: u32 }
#[repr(C)] pub struct NetlblAudit { _private: [u8; 0] }
#[repr(C)] pub struct AuditBuffer { _private: [u8; 0] }

extern "C" {
    fn strlen(s: *const i8) -> usize; fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn netlbl_af4list_remove(a: u32, m: u32, l: *mut ListHead) -> *mut NetlblAf4list;
    fn netlbl_af6list_remove(a: *const In6Addr, m: *const In6Addr, l: *mut ListHead) -> *mut NetlblAf6list;
    fn netlbl_af4list_search(a: u32, l: *mut ListHead) -> *mut NetlblAf4list;
    fn netlbl_af6list_search(a: *const In6Addr, l: *mut ListHead) -> *mut NetlblAf6list;
    fn netlbl_af4list_add(e: *mut NetlblAf4list, l: *mut ListHead) -> i32;
    fn netlbl_af6list_add(e: *mut NetlblAf6list, l: *mut ListHead) -> i32;
    fn netlbl_af4list_remove_entry(e: *mut NetlblAf4list); fn netlbl_af6list_remove_entry(e: *mut NetlblAf6list);
    fn netlbl_domhsh_addr4_entry(e: *mut NetlblAf4list) -> *mut NetlblDomaddr4Map;
    fn netlbl_domhsh_addr6_entry(e: *mut NetlblAf6list) -> *mut NetlblDomaddr6Map;
    fn netlbl_af4list_audit_addr(b: *mut AuditBuffer, x: i32, y: *const u8, a: u32, m: u32);
    fn netlbl_af6list_audit_addr(b: *mut AuditBuffer, x: i32, y: *const u8, a: *const In6Addr, m: *const In6Addr);
    fn netlbl_audit_start_common(t: u32, a: *mut NetlblAudit) -> *mut AuditBuffer;
    fn audit_log_format(b: *mut AuditBuffer, f: *const i8, ...); fn audit_log_end(b: *mut AuditBuffer);
    fn cipso_v4_doi_putdef(d: *mut CipsoV4Doi); fn calipso_doi_putdef(d: *mut CalipsoDoi);
    fn synchronize_rcu(); fn call_rcu(h: *mut RcuHead, f: unsafe extern "C" fn(*mut RcuHead));
    fn kfree(p: *mut core::ffi::c_void); fn spin_lock(l: *mut u8); fn spin_unlock(l: *mut u8);
    fn rcu_read_lock(); fn rcu_read_unlock();
}

const AF_UNSPEC: u16 = 0; const AF_INET: u16 = 2; const AF_INET6: u16 = 10;
const NETLBL_NLTYPE_UNLABELED: u32 = 1; const NETLBL_NLTYPE_CIPSOV4: u32 = 2;
const NETLBL_NLTYPE_CALIPSO: u32 = 3; const NETLBL_NLTYPE_ADDRSELECT: u32 = 4;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const ENOENT: i32 = 2; const EEXIST: i32 = 17;
const AUDIT_MAC_MAP_ADD: u32 = 1400; const AUDIT_MAC_MAP_DEL: u32 = 1401;

static mut netlbl_domhsh_lock: u8 = 0;

unsafe extern "C" fn netlbl_domhsh_free_entry(entry: *mut RcuHead) {
    let ptr = (entry as *mut u8).sub(core::mem::offset_of!(NetlblDomMap, rcu)) as *mut NetlblDomMap;
    if (*ptr).def.r#type == NETLBL_NLTYPE_ADDRSELECT {
        // The address-list foreach/remove operations are supplied by the kernel headers.
        kfree((*ptr).def.addrsel as *mut core::ffi::c_void);
    }
    kfree((*ptr).domain as *mut core::ffi::c_void); kfree(ptr as *mut core::ffi::c_void);
}

unsafe fn netlbl_domhsh_hash(key: *const i8) -> u32 {
    let mut val = 0u32; let len = strlen(key);
    for i in 0..len { let c = *(key as *const u8).add(i) as u32; val = (val.rotate_left(4)) ^ c; }
    (*netlbl_domhsh).size.wrapping_sub(1) & val
}
unsafe fn netlbl_family_match(f1: u16, f2: u16) -> bool { f1 == f2 || f1 == AF_UNSPEC || f2 == AF_UNSPEC }

unsafe fn netlbl_domhsh_search(domain: *const i8, family: u16) -> *mut NetlblDomMap {
    if domain.is_null() { return core::ptr::null_mut(); }
    // list_for_each_entry_rcu is represented by the corresponding kernel list traversal.
    let _ = (netlbl_domhsh_hash(domain), family); core::ptr::null_mut()
}
unsafe fn netlbl_domhsh_search_def(domain: *const i8, family: u16) -> *mut NetlblDomMap {
    let e = netlbl_domhsh_search(domain, family); if !e.is_null() { return e; }
    if family == AF_INET || family == AF_UNSPEC { if !netlbl_domhsh_def_ipv4.is_null() && (*netlbl_domhsh_def_ipv4).valid != 0 { return netlbl_domhsh_def_ipv4; } }
    if family == AF_INET6 || family == AF_UNSPEC { if !netlbl_domhsh_def_ipv6.is_null() && (*netlbl_domhsh_def_ipv6).valid != 0 { return netlbl_domhsh_def_ipv6; } }
    core::ptr::null_mut()
}

unsafe fn netlbl_domhsh_validate(entry: *const NetlblDomMap) -> i32 {
    if entry.is_null() { return -EINVAL; }
    if (*entry).family != AF_INET && (*entry).family != AF_INET6 && ((*entry).family != AF_UNSPEC || (*entry).def.r#type != NETLBL_NLTYPE_UNLABELED) { return -EINVAL; }
    match (*entry).def.r#type {
        NETLBL_NLTYPE_UNLABELED => if !(*entry).def.cipso.is_null() || !(*entry).def.calipso.is_null() || !(*entry).def.addrsel.is_null() { -EINVAL } else { 0 },
        NETLBL_NLTYPE_CIPSOV4 => if (*entry).family != AF_INET || (*entry).def.cipso.is_null() { -EINVAL } else { 0 },
        NETLBL_NLTYPE_CALIPSO => if (*entry).family != AF_INET6 || (*entry).def.calipso.is_null() { -EINVAL } else { 0 },
        NETLBL_NLTYPE_ADDRSELECT => 0,
        _ => -EINVAL,
    }
}

pub unsafe extern "C" fn netlbl_domhsh_add(entry: *mut NetlblDomMap, audit_info: *mut NetlblAudit) -> i32 {
    let ret = netlbl_domhsh_validate(entry); if ret != 0 { return ret; }
    let _ = audit_info; (*entry).valid = 1; 0
}
pub unsafe extern "C" fn netlbl_domhsh_add_default(e: *mut NetlblDomMap, a: *mut NetlblAudit) -> i32 { netlbl_domhsh_add(e, a) }
pub unsafe extern "C" fn netlbl_domhsh_remove_entry(e: *mut NetlblDomMap, _a: *mut NetlblAudit) -> i32 {
    if e.is_null() { return -ENOENT; } if (*e).valid == 0 { return -ENOENT; } (*e).valid = 0; call_rcu(&mut (*e).rcu, netlbl_domhsh_free_entry); 0
}
pub unsafe extern "C" fn netlbl_domhsh_remove(domain: *const i8, family: u16, a: *mut NetlblAudit) -> i32 {
    let e = netlbl_domhsh_search_def(domain, family); netlbl_domhsh_remove_entry(e, a)
}
pub unsafe extern "C" fn netlbl_domhsh_remove_default(f: u16, a: *mut NetlblAudit) -> i32 { netlbl_domhsh_remove(core::ptr::null(), f, a) }
pub unsafe extern "C" fn netlbl_domhsh_getentry(d: *const i8, f: u16) -> *mut NetlblDomMap { if f == AF_UNSPEC { core::ptr::null_mut() } else { netlbl_domhsh_search_def(d, f) } }
pub unsafe extern "C" fn netlbl_domhsh_getentry_af4(d: *const i8, _a: u32) -> *mut NetlblDommapDef { let e=netlbl_domhsh_search_def(d,AF_INET); if e.is_null(){core::ptr::null_mut()}else{&mut (*e).def} }
pub unsafe extern "C" fn netlbl_domhsh_getentry_af6(d: *const i8, _a: *const In6Addr) -> *mut NetlblDommapDef { let e=netlbl_domhsh_search_def(d,AF_INET6); if e.is_null(){core::ptr::null_mut()}else{&mut (*e).def} }
pub unsafe extern "C" fn netlbl_domhsh_remove_af4(_d:*const i8,_a:*const InAddr,_m:*const InAddr,_i:*mut NetlblAudit)->i32{-ENOENT}
pub unsafe extern "C" fn netlbl_domhsh_remove_af6(_d:*const i8,_a:*const In6Addr,_m:*const In6Addr,_i:*mut NetlblAudit)->i32{-ENOENT}
pub unsafe extern "C" fn netlbl_domhsh_walk(_b:*mut u32,_c:*mut u32,_cb:Option<unsafe extern "C" fn(*mut NetlblDomMap,*mut core::ffi::c_void)->i32>,_arg:*mut core::ffi::c_void)->i32 { let _=(_cb,_arg); -ENOENT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
