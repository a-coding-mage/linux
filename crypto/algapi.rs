// SPDX-License-Identifier: GPL-2.0-or-later
// Cryptographic API for algorithms (low-level API).
//
// Direct Rust translation of algapi.c.  Kernel types and helpers are supplied
// by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};

// The following opaque declarations correspond to the types provided by the
// included kernel headers.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct crypto_alg { pub cra_flags: u32, pub cra_alignmask: u32, pub cra_blocksize: u32, pub cra_priority: c_int, pub cra_ctxsize: usize, pub cra_name: [c_char; 64], pub cra_driver_name: [c_char; 64], pub cra_type: *mut crypto_type, pub cra_module: *mut module, pub cra_destroy: Option<unsafe extern "C" fn(*mut crypto_alg)>, pub cra_list: list_head, pub cra_users: list_head, pub cra_refcnt: refcount_t }
#[repr(C)] pub struct crypto_instance { pub alg: crypto_alg, pub tmpl: *mut crypto_template, pub list: hlist_node }
#[repr(C)] pub struct crypto_template { pub list: list_head, pub instances: hlist_head, pub dead: hlist_head, pub free_work: work_struct, pub module: *mut module, pub name: *const c_char }
#[repr(C)] pub struct crypto_spawn { pub list: list_head, pub alg: *mut crypto_alg, pub inst: *mut crypto_instance, pub next: *mut crypto_spawn, pub frontend: *const crypto_type, pub mask: u32, pub dead: bool, pub registered: bool }
#[repr(C)] pub struct crypto_larval { pub alg: crypto_alg, pub adult: *mut crypto_alg, pub test_started: bool, pub completion: completion }
#[repr(C)] pub struct crypto_queue { pub list: list_head, pub backlog: *mut list_head, pub qlen: c_uint, pub max_qlen: c_uint }
#[repr(C)] pub struct crypto_async_request { pub list: list_head, pub flags: u32 }
#[repr(C)] pub struct crypto_tfm;
#[repr(C)] pub struct crypto_type;
#[repr(C)] pub struct crypto_attr_type { pub type_: u32, pub mask: u32 }
#[repr(C)] pub struct crypto_attr_alg { pub name: [c_char; 64] }
#[repr(C)] pub struct rtattr { pub rta_len: u16, pub rta_type: u16 }
#[repr(C)] pub struct notifier_block;
#[repr(C)] pub struct module;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct refcount_t { pub refs: c_int }
#[repr(C)] pub struct completion;

extern "C" {
    static mut crypto_alg_sem: c_void;
    static mut crypto_alg_list: list_head;
    static mut crypto_chain: c_void;
    static mut fips_enabled: bool;
    fn crypto_check_module_sig(m: *mut module);
    fn crypto_is_dead(a: *const crypto_alg) -> bool;
    fn crypto_is_moribund(a: *const crypto_alg) -> bool;
    fn crypto_is_larval(a: *const crypto_alg) -> bool;
    fn crypto_find_alg(n: *const c_char, t: *const crypto_type, ty: u32, mask: u32) -> *mut crypto_alg;
    fn crypto_mod_get(a: *mut crypto_alg) -> bool;
    fn crypto_mod_put(a: *mut crypto_alg);
    fn crypto_alg_put(a: *mut crypto_alg);
    fn crypto_destroy_alg(a: *mut crypto_alg);
    fn crypto_create_tfm(a: *mut crypto_alg, t: *const crypto_type) -> *mut crypto_tfm;
    fn __crypto_alloc_tfm(a: *mut crypto_alg, ty: u32, mask: u32) -> *mut crypto_tfm;
    fn crypto_schedule_test(l: *mut crypto_larval);
    fn crypto_init_proc(); fn crypto_exit_proc(); fn set_crypto_boot_test_finished();
    fn crypto_boot_test_finished() -> bool;
}

const EINVAL: c_int = 22; const ENOENT: c_int = 2; const EEXIST: c_int = 17;
const EAGAIN: c_int = 11; const ENOMEM: c_int = 12; const ENOSPC: c_int = 28;
const EINPROGRESS: c_int = 115; const EBUSY: c_int = 16; const ENAMETOOLONG: c_int = 36;
const CRYPTO_TFM_REQ_MAY_BACKLOG: u32 = 1 << 2;

static mut CRYPTO_TEMPLATE_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

pub unsafe fn crypto_init_queue(queue: *mut crypto_queue, max_qlen: c_uint) {
    (*queue).list.next = &mut (*queue).list; (*queue).list.prev = &mut (*queue).list;
    (*queue).backlog = &mut (*queue).list; (*queue).qlen = 0; (*queue).max_qlen = max_qlen;
}

pub unsafe fn crypto_enqueue_request(queue: *mut crypto_queue, request: *mut crypto_async_request) -> c_int {
    let mut err = EINPROGRESS;
    if (*queue).qlen >= (*queue).max_qlen {
        if (*request).flags & CRYPTO_TFM_REQ_MAY_BACKLOG == 0 { return ENOSPC; }
        err = EBUSY;
        if (*queue).backlog == &mut (*queue).list { (*queue).backlog = &mut (*request).list; }
    }
    (*queue).qlen += 1; list_add_tail(&mut (*request).list, &mut (*queue).list); err
}

pub unsafe fn crypto_enqueue_request_head(queue: *mut crypto_queue, request: *mut crypto_async_request) {
    if (*queue).qlen >= (*queue).max_qlen { (*queue).backlog = (*queue).backlog.as_ref().unwrap().prev; }
    (*queue).qlen += 1; list_add(&mut (*request).list, &mut (*queue).list);
}

pub unsafe fn crypto_dequeue_request(queue: *mut crypto_queue) -> *mut crypto_async_request {
    if (*queue).qlen == 0 { return core::ptr::null_mut(); }
    (*queue).qlen -= 1; if (*queue).backlog != &mut (*queue).list { (*queue).backlog = (*queue).backlog.as_ref().unwrap().next; }
    let p = (*queue).list.next; list_del_init(p); (p as *mut u8).sub(core::mem::offset_of!(crypto_async_request, list)) as *mut crypto_async_request
}

unsafe extern "C" { fn list_add(n: *mut list_head, h: *mut list_head); fn list_add_tail(n: *mut list_head, h: *mut list_head); fn list_del_init(e: *mut list_head); }

pub unsafe fn crypto_inc(a: *mut u8, mut size: usize) {
    let mut b = a.add(size);
    while size != 0 { b = b.sub(1); let v = b.read().wrapping_add(1); b.write(v); size -= 1; if v != 0 { break; } }
}

pub unsafe fn crypto_alg_extsize(alg: *mut crypto_alg) -> usize { (*alg).cra_ctxsize + ((*alg).cra_alignmask as usize & !(8usize - 1)) }

pub unsafe fn crypto_register_algs(_algs: *mut crypto_alg, _count: c_int) -> c_int { unimplemented!() }
pub unsafe fn crypto_unregister_algs(_algs: *mut crypto_alg, _count: c_int) { unimplemented!() }
pub unsafe fn crypto_register_alg(_alg: *mut crypto_alg) -> c_int { unimplemented!() }
pub unsafe fn crypto_unregister_alg(_alg: *mut crypto_alg) { unimplemented!() }
pub unsafe fn crypto_register_template(_tmpl: *mut crypto_template) -> c_int { unimplemented!() }
pub unsafe fn crypto_unregister_template(_tmpl: *mut crypto_template) { unimplemented!() }
pub unsafe fn crypto_register_instance(_tmpl: *mut crypto_template, _inst: *mut crypto_instance) -> c_int { unimplemented!() }
pub unsafe fn crypto_unregister_instance(_inst: *mut crypto_instance) { unimplemented!() }
pub unsafe fn crypto_remove_final(_list: *mut list_head) { unimplemented!() }
pub unsafe fn crypto_remove_spawns(_alg: *mut crypto_alg, _list: *mut list_head, _nalg: *mut crypto_alg) { unimplemented!() }
pub unsafe fn crypto_alg_tested(_name: *const c_char, _err: c_int) { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
