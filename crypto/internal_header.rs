/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Cryptographic API. */

// C dependencies supplied by other translation units/headers:
// crypto/algapi.h, linux/completion.h, linux/jump_label.h, linux/list.h,
// linux/module.h, linux/notifier.h, linux/numa.h, linux/refcount.h,
// linux/rwsem.h, linux/scatterlist.h, linux/sched.h, linux/types.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub struct crypto_instance;
pub struct crypto_template;

#[repr(C)]
pub struct crypto_larval {
    pub alg: crypto_alg,
    pub adult: *mut crypto_alg,
    pub completion: completion,
    pub mask: u32,
    pub test_started: bool,
}

#[repr(C)]
pub struct crypto_type {
    pub ctxsize: Option<unsafe extern "C" fn(*mut crypto_alg, u32, u32) -> c_uint>,
    pub extsize: Option<unsafe extern "C" fn(*mut crypto_alg) -> c_uint>,
    pub init_tfm: Option<unsafe extern "C" fn(*mut crypto_tfm) -> c_int>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut crypto_alg)>,
    pub report: Option<unsafe extern "C" fn(*mut sk_buff, *mut crypto_alg) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut crypto_instance)>,
    pub destroy: Option<unsafe extern "C" fn(*mut crypto_alg)>,
    pub type_: c_uint,
    pub maskclear: c_uint,
    pub maskset: c_uint,
    pub tfmsize: c_uint,
    pub algsize: c_uint,
}

pub const CRYPTOA_UNSPEC: c_uint = 0;
pub const CRYPTOA_ALG: c_uint = 1;
pub const CRYPTOA_TYPE: c_uint = 2;
pub const __CRYPTOA_MAX: c_uint = 3;
pub const CRYPTOA_MAX: c_uint = __CRYPTOA_MAX - 1;
pub const CRYPTO_MAX_ATTRS: c_uint = 32;

extern "C" {
    pub static mut crypto_alg_sem: rw_semaphore;
    pub static mut crypto_alg_list: list_head;
    pub static mut crypto_chain: blocking_notifier_head;

    pub fn alg_test(driver: *const c_char, alg: *const c_char, type_: u32, mask: u32) -> c_int;

    pub fn crypto_init_proc();
    pub fn crypto_exit_proc();
    pub fn crypto_mod_get(alg: *mut crypto_alg) -> *mut crypto_alg;
    pub fn crypto_alg_mod_lookup(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_alg;
    pub fn crypto_larval_alloc(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_larval;
    pub fn crypto_schedule_test(larval: *mut crypto_larval);
    pub fn crypto_alg_tested(name: *const c_char, err: c_int);
    pub fn crypto_remove_spawns(alg: *mut crypto_alg, list: *mut list_head, nalg: *mut crypto_alg);
    pub fn crypto_remove_final(list: *mut list_head);
    pub fn crypto_shoot_alg(alg: *mut crypto_alg);
    pub fn __crypto_alloc_tfm(alg: *mut crypto_alg, type_: u32, mask: u32) -> *mut crypto_tfm;
    pub fn crypto_create_tfm_node(alg: *mut crypto_alg, frontend: *const crypto_type, node: c_int) -> *mut c_void;
    pub fn crypto_find_alg(alg_name: *const c_char, frontend: *const crypto_type, type_: u32, mask: u32) -> *mut crypto_alg;
    pub fn crypto_alloc_tfm_node(alg_name: *const c_char, frontend: *const crypto_type, type_: u32, mask: u32, node: c_int) -> *mut c_void;
    pub fn crypto_probing_notify(val: c_ulong, v: *mut c_void) -> c_int;
    pub fn crypto_alg_extsize(alg: *mut crypto_alg) -> c_uint;
    pub fn crypto_type_has_alg(name: *const c_char, frontend: *const crypto_type, type_: u32, mask: u32) -> c_int;
    pub fn crypto_destroy_alg(alg: *mut crypto_alg);
}

// Build-time configuration in the C header selects either these constant
// implementations or the static-key implementations.
#[inline]
pub fn crypto_boot_test_finished() -> bool { true }
#[inline]
pub fn set_crypto_boot_test_finished() {}

#[inline]
pub unsafe fn crypto_init_proc_inline() {}
#[inline]
pub unsafe fn crypto_exit_proc_inline() {}

#[inline]
pub unsafe fn crypto_cipher_ctxsize(alg: *mut crypto_alg) -> c_uint { (*alg).cra_ctxsize }
#[inline]
pub unsafe fn crypto_compress_ctxsize(alg: *mut crypto_alg) -> c_uint { (*alg).cra_ctxsize }

#[inline]
pub unsafe fn crypto_create_tfm(alg: *mut crypto_alg, frontend: *const crypto_type) -> *mut c_void {
    crypto_create_tfm_node(alg, frontend, NUMA_NO_NODE)
}
#[inline]
pub unsafe fn crypto_alloc_tfm(alg_name: *const c_char, frontend: *const crypto_type, type_: u32, mask: u32) -> *mut c_void {
    crypto_alloc_tfm_node(alg_name, frontend, type_, mask, NUMA_NO_NODE)
}
#[inline]
pub unsafe fn crypto_alg_get(alg: *mut crypto_alg) -> *mut crypto_alg {
    refcount_inc(&mut (*alg).cra_refcnt);
    alg
}
#[inline]
pub unsafe fn crypto_alg_put(alg: *mut crypto_alg) {
    if refcount_dec_and_test(&mut (*alg).cra_refcnt) { crypto_destroy_alg(alg); }
}
#[inline]
pub unsafe fn crypto_tmpl_get(tmpl: *mut crypto_template) -> c_int { try_module_get((*tmpl).module) }
#[inline]
pub unsafe fn crypto_tmpl_put(tmpl: *mut crypto_template) { module_put((*tmpl).module); }
#[inline]
pub unsafe fn crypto_is_larval(alg: *mut crypto_alg) -> c_int { ((*alg).cra_flags & CRYPTO_ALG_LARVAL) as c_int }
#[inline]
pub unsafe fn crypto_is_dead(alg: *mut crypto_alg) -> c_int { ((*alg).cra_flags & CRYPTO_ALG_DEAD) as c_int }
#[inline]
pub unsafe fn crypto_is_moribund(alg: *mut crypto_alg) -> c_int { ((*alg).cra_flags & (CRYPTO_ALG_DEAD | CRYPTO_ALG_DYING)) as c_int }
#[inline]
pub unsafe fn crypto_notify(val: c_ulong, v: *mut c_void) { blocking_notifier_call_chain(&mut crypto_chain, val, v); }
#[inline]
pub unsafe fn crypto_yield(flags: u32) { if flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { cond_resched(); } }
#[inline]
pub unsafe fn crypto_is_test_larval(larval: *mut crypto_larval) -> c_int { (*larval).alg.cra_driver_name[0] as c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
