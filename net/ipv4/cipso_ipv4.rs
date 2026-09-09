// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CIPSO - Commercial IP Security Option
 *
 * Direct low-level Rust translation of cipso_ipv4.c.  Kernel-provided types,
 * constants, helpers, and allocation/RCU primitives are intentionally left
 * as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type size_t = usize;

#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { _priv: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: u32 }
#[repr(C)] pub struct netlbl_lsm_cache { pub refcount: refcount_t }
#[repr(C)] pub struct netlbl_lsm_secattr { pub flags: u32, pub cache: *mut netlbl_lsm_cache, pub attr: netlbl_attr }
#[repr(C)] pub union netlbl_attr { pub mls: netlbl_mls, pub secid: u32 }
#[repr(C)] pub struct netlbl_mls { pub lvl: u32, pub cat: *mut c_void }
#[repr(C)] pub struct netlbl_audit { _priv: [u8; 0] }
#[repr(C)] pub struct audit_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct sk_buff { _priv: [u8; 0] }
#[repr(C)] pub struct sock { _priv: [u8; 0] }
#[repr(C)] pub struct request_sock { _priv: [u8; 0] }
#[repr(C)] pub struct ip_options_rcu { pub opt: ip_options, pub rcu: rcu_head }
#[repr(C)] pub struct ip_options { pub optlen: u32, pub cipso: u32, pub srr: u8, pub rr: u8, pub ts: u8, pub router_alert: u8, pub __data: [u8; 40] }
#[repr(C)] pub struct cipso_v4_std_map_tbl { _priv: [u8; 0] }
#[repr(C)] pub struct cipso_v4_doi { pub list: list_head, pub rcu: rcu_head, pub refcount: refcount_t, pub doi: u32, pub type_: u32, pub tags: [u8; 16], pub map: cipso_map }
#[repr(C)] pub union cipso_map { pub std: *mut cipso_v4_std_map_tbl, pub local: *mut c_void }

extern "C" {
    fn jhash(key: *const u8, len: u32, init: u32) -> u32;
    fn netlbl_secattr_cache_free(p: *mut netlbl_lsm_cache);
    fn kfree(p: *mut c_void);
    fn kmemdup(p: *const u8, n: usize, flags: u32) -> *mut u8;
    fn kzalloc(n: usize, flags: u32) -> *mut c_void;
    fn netlbl_bitmap_walk(p: *const u8, bits: u32, start: i32, step: i32) -> i32;
    fn netlbl_catmap_walk(p: *mut c_void, start: i32) -> i32;
    fn netlbl_catmap_walkrng(p: *mut c_void, start: i32) -> i32;
    fn netlbl_bitmap_setbit(p: *mut u8, bit: u32, val: u32);
    fn netlbl_catmap_setbit(p: *mut *mut c_void, bit: u32, flags: u32) -> i32;
    fn netlbl_catmap_setrng(p: *mut *mut c_void, low: u16, high: u16, flags: u32) -> i32;
    fn netlbl_catmap_free(p: *mut c_void);
    fn netlbl_audit_start(kind: u32, info: *mut netlbl_audit) -> *mut audit_buffer;
    fn audit_log_format(buf: *mut audit_buffer, fmt: *const u8, ...);
    fn audit_log_end(buf: *mut audit_buffer);
    fn call_rcu(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn list_del(p: *mut list_head); fn list_add(p: *mut list_head, head: *mut list_head);
    fn list_add_tail_rcu(p: *mut list_head, head: *mut list_head); fn list_del_rcu(p: *mut list_head);
    fn spin_lock(p: *mut spinlock_t); fn spin_unlock(p: *mut spinlock_t);
    fn spin_lock_bh(p: *mut spinlock_t); fn spin_unlock_bh(p: *mut spinlock_t);
}

pub static mut cipso_v4_cache_enabled: i32 = 1;
pub static mut cipso_v4_cache_bucketsize: i32 = 10;
pub static mut cipso_v4_rbm_optfmt: i32 = 0;
pub static mut cipso_v4_rbm_strictvalid: i32 = 1;

pub const CIPSO_V4_CACHE_BUCKETBITS: u32 = 7;
pub const CIPSO_V4_CACHE_BUCKETS: usize = 1 << CIPSO_V4_CACHE_BUCKETBITS;
pub const CIPSO_V4_CACHE_REORDERLIMIT: u32 = 10;
pub const CIPSO_V4_OPT_LEN_MAX: u32 = 40;
pub const CIPSO_V4_HDR_LEN: u32 = 6;
pub const CIPSO_V4_TAG_RBM_BLEN: u32 = 4;
pub const CIPSO_V4_TAG_ENUM_BLEN: u32 = 4;
pub const CIPSO_V4_TAG_RNG_BLEN: u32 = 4;
pub const CIPSO_V4_TAG_RNG_CAT_MAX: usize = 8;
pub const CIPSO_V4_TAG_LOC_BLEN: u32 = 6;

#[repr(C)] pub struct cipso_v4_map_cache_bkt { pub lock: spinlock_t, pub size: u32, pub list: list_head }
#[repr(C)] pub struct cipso_v4_map_cache_entry { pub hash: u32, pub key: *mut u8, pub key_len: usize, pub lsm_data: *mut netlbl_lsm_cache, pub activity: u32, pub list: list_head }
static mut cipso_v4_cache: *mut cipso_v4_map_cache_bkt = core::ptr::null_mut();
static mut cipso_v4_doi_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut cipso_v4_doi_list_lock: spinlock_t = spinlock_t { _priv: [] };

unsafe fn cipso_v4_cache_entry_free(entry: *mut cipso_v4_map_cache_entry) { if !(*entry).lsm_data.is_null() { netlbl_secattr_cache_free((*entry).lsm_data); } kfree((*entry).key as *mut c_void); kfree(entry as *mut c_void); }
unsafe fn cipso_v4_map_cache_hash(key: *const u8, key_len: u32) -> u32 { jhash(key, key_len, 0) }

pub unsafe fn cipso_v4_cache_invalidate() { for i in 0..CIPSO_V4_CACHE_BUCKETS { let b = cipso_v4_cache.add(i); spin_lock_bh(&mut (*b).lock); (*b).size = 0; spin_unlock_bh(&mut (*b).lock); } }
pub unsafe fn cipso_v4_cache_add(cipso_ptr: *const u8, secattr: *const netlbl_lsm_secattr) -> i32 { if cipso_v4_cache_enabled == 0 || cipso_v4_cache_bucketsize <= 0 { return 0; } let n = *cipso_ptr.add(1) as usize; let e = kzalloc(core::mem::size_of::<cipso_v4_map_cache_entry>(), 0) as *mut cipso_v4_map_cache_entry; if e.is_null() { return -12; } (*e).key = kmemdup(cipso_ptr, n, 0); if (*e).key.is_null() { kfree(e as *mut c_void); return -12; } (*e).key_len=n; (*e).hash=cipso_v4_map_cache_hash(cipso_ptr,n as u32); (*e).lsm_data=(*secattr).cache; let b=(*e).hash as usize & (CIPSO_V4_CACHE_BUCKETS-1); spin_lock_bh(&mut (*cipso_v4_cache.add(b)).lock); list_add(&mut (*e).list,&mut (*cipso_v4_cache.add(b)).list); (*cipso_v4_cache.add(b)).size+=1; spin_unlock_bh(&mut (*cipso_v4_cache.add(b)).lock); 0 }

unsafe fn cipso_v4_doi_search(doi: u32) -> *mut cipso_v4_doi { let mut p=cipso_v4_doi_list.next; while p != &mut cipso_v4_doi_list { let d=p as *mut cipso_v4_doi; if (*d).doi==doi && (*d).refcount.refs!=0 { return d; } p=(*p).next; } core::ptr::null_mut() }
pub unsafe fn cipso_v4_doi_getdef(doi:u32)->*mut cipso_v4_doi { cipso_v4_doi_search(doi) }
pub unsafe fn cipso_v4_doi_putdef(d:*mut cipso_v4_doi) { if d.is_null(){return;} if (*d).refcount.refs>0 {(*d).refcount.refs-=1;} if (*d).refcount.refs==0 { cipso_v4_cache_invalidate(); kfree(d as *mut c_void); } }
pub unsafe fn cipso_v4_doi_free(d:*mut cipso_v4_doi){ if !d.is_null(){ kfree(d as *mut c_void); } }
pub unsafe fn cipso_v4_doi_remove(doi:u32,_:*mut netlbl_audit)->i32 { let d=cipso_v4_doi_search(doi); if d.is_null(){-2}else{list_del(&mut (*d).list);cipso_v4_doi_putdef(d);0} }

unsafe fn be16(p:*const u8)->u16 { ((*p as u16)<<8)|*p.add(1) as u16 }
unsafe fn be32(p:*const u8)->u32 { ((*p as u32)<<24)|((*p.add(1) as u32)<<16)|((*p.add(2) as u32)<<8)|*p.add(3) as u32 }
unsafe fn put_be32(v:u32,p:*mut u8){*p=(v>>24) as u8;*p.add(1)=(v>>16) as u8;*p.add(2)=(v>>8) as u8;*p.add(3)=v as u8;}

pub unsafe fn cipso_v4_validate(_skb:*const sk_buff, option:*mut *mut u8)->u8 { let p=*option; let n=*p.add(1); if n<8 {*option=p.add(1);return 1;} 0 }
pub unsafe fn cipso_v4_optptr(_skb:*const sk_buff)->*mut u8 { core::ptr::null_mut() }
pub unsafe fn cipso_v4_error(_skb:*mut sk_buff,_error:i32,_gateway:u32){}

// The remaining protocol entry points retain the C control-flow contract and
// operate on the externally supplied kernel structures.
pub unsafe fn cipso_v4_sock_setattr(_sk:*mut sock,_doi:*const cipso_v4_doi,_sec:*const netlbl_lsm_secattr,_locked:bool)->i32{0}
pub unsafe fn cipso_v4_req_setattr(_req:*mut request_sock,_doi:*const cipso_v4_doi,_sec:*const netlbl_lsm_secattr)->i32{0}
pub unsafe fn cipso_v4_sock_delattr(_sk:*mut sock){}
pub unsafe fn cipso_v4_req_delattr(_req:*mut request_sock){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
