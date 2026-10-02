// SPDX-License-Identifier: GPL-2.0-or-later
// Cryptographic algorithm API, translated from crypto/algapi.c.
// C headers own every shared layout. Registration, dependency traversal,
// self-test state, reference ownership and request queues are implemented here.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/crypto_algapi_generated.rs"
    ));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_uint, c_void};

#[path = "../rust/ffi_export.rs"]
mod ffi_export;

static mut crypto_template_list: list_head = list_head {
    next: addr_of_mut!(crypto_template_list),
    prev: addr_of_mut!(crypto_template_list),
};

#[inline]
fn neg(e: u32) -> c_int {
    -(e as c_int)
}
#[inline]
fn err_ptr<T>(e: c_int) -> *mut T {
    e as isize as *mut T
}
#[inline]
fn is_err<T>(p: *const T) -> bool {
    p as usize >= usize::MAX - 4094
}
#[inline]
fn ptr_err<T>(p: *const T) -> c_int {
    p as isize as c_int
}
#[inline]
unsafe fn empty(h: *const list_head) -> bool {
    rust_algapi_list_empty(h)
}
#[inline]
unsafe fn dead(a: *const crypto_alg) -> bool {
    (*a).cra_flags & CRYPTO_ALG_DEAD != 0
}
#[inline]
unsafe fn moribund(a: *const crypto_alg) -> bool {
    (*a).cra_flags & (CRYPTO_ALG_DEAD | CRYPTO_ALG_DYING) != 0
}
#[inline]
unsafe fn larval(a: *const crypto_alg) -> bool {
    (*a).cra_flags & CRYPTO_ALG_LARVAL != 0
}
#[inline]
unsafe fn list_alg(n: *mut list_head) -> *mut crypto_alg {
    n.cast::<u8>().sub(offset_of!(crypto_alg, cra_list)).cast()
}
#[inline]
unsafe fn list_spawn(n: *mut list_head) -> *mut crypto_spawn {
    n.cast::<u8>().sub(offset_of!(crypto_spawn, list)).cast()
}
#[inline]
unsafe fn hlist_instance(n: *mut hlist_node) -> *mut crypto_instance {
    n.cast::<u8>()
        .sub(offset_of!(crypto_instance, __bindgen_anon_1))
        .cast()
}
#[inline]
unsafe fn inst_list(i: *mut crypto_instance) -> *mut hlist_node {
    addr_of_mut!((*i).__bindgen_anon_1.list)
}

unsafe fn crypto_check_module_sig(m: *mut module) {
    if rust_algapi_fips_enabled() != 0 && !m.is_null() && !rust_algapi_module_sig_ok(m) {
        bindings::panic(
            c"Module %s signature verification failed in FIPS mode\n"
                .as_ptr()
                .cast(),
            rust_algapi_module_name(m),
        );
    }
}

unsafe fn crypto_check_alg(alg: *mut crypto_alg) -> c_int {
    crypto_check_module_sig((*alg).cra_module);
    if (*alg).cra_name[0] == 0 || (*alg).cra_driver_name[0] == 0 {
        return neg(EINVAL);
    }
    if (*alg).cra_alignmask & (*alg).cra_alignmask.wrapping_add(1) != 0 {
        return neg(EINVAL);
    }
    if (*alg).cra_alignmask > MAX_ALGAPI_ALIGNMASK || (*alg).cra_blocksize > MAX_ALGAPI_BLOCKSIZE {
        return neg(EINVAL);
    }
    if (*alg).cra_type.is_null()
        && (*alg).cra_flags & CRYPTO_ALG_TYPE_MASK == CRYPTO_ALG_TYPE_CIPHER
    {
        if (*alg).cra_alignmask > MAX_CIPHER_ALIGNMASK
            || (*alg).cra_blocksize > MAX_CIPHER_BLOCKSIZE
        {
            return neg(EINVAL);
        }
    }
    if (*alg).cra_priority < 0 {
        return neg(EINVAL);
    }
    rust_algapi_ref_set(addr_of_mut!((*alg).cra_refcnt), 1);
    0
}

unsafe fn crypto_free_instance(inst: *mut crypto_instance) {
    ((*(*inst).alg.cra_type).free.unwrap_unchecked())(inst);
}

unsafe extern "C" fn crypto_destroy_instance_workfn(w: *mut work_struct) {
    let tmpl = w
        .cast::<u8>()
        .sub(offset_of!(crypto_template, free_work))
        .cast::<crypto_template>();
    let mut list: hlist_head = zeroed();
    down_write(addr_of_mut!(crypto_alg_sem));
    let mut node = (*tmpl).dead.first;
    while !node.is_null() {
        let next = (*node).next;
        let inst = hlist_instance(node);
        if rust_algapi_ref_read(addr_of_mut!((*inst).alg.cra_refcnt)) == u32::MAX {
            rust_algapi_hlist_del(node);
            rust_algapi_hlist_add(node, addr_of_mut!(list));
        }
        node = next;
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    let mut node = list.first;
    while !node.is_null() {
        let next = (*node).next;
        crypto_free_instance(hlist_instance(node));
        node = next;
    }
}

unsafe extern "C" fn crypto_destroy_instance(alg: *mut crypto_alg) {
    let inst = alg
        .cast::<u8>()
        .sub(offset_of!(crypto_instance, alg))
        .cast::<crypto_instance>();
    let tmpl = (*inst).tmpl;
    rust_algapi_ref_set(addr_of_mut!((*alg).cra_refcnt), -1);
    rust_algapi_schedule_work(addr_of_mut!((*tmpl).free_work));
}

// Pop one DFS path entry and propagate resurrection to its predecessor.
unsafe fn crypto_more_spawns(
    stack: *mut list_head,
    top: *mut list_head,
    secondary: *mut list_head,
) -> *mut list_head {
    if empty(stack) {
        return null_mut();
    }
    let spawn = list_spawn((*stack).next);
    rust_algapi_list_move(addr_of_mut!((*spawn).list), secondary);
    if empty(stack) {
        return top;
    }
    let n = list_spawn((*stack).next);
    if !(*spawn).dead {
        (*n).dead = false;
    }
    addr_of_mut!((*(*n).__bindgen_anon_1.inst).alg.cra_users)
}

unsafe fn crypto_remove_instance(inst: *mut crypto_instance, _list: *mut list_head) {
    let tmpl = (*inst).tmpl;
    if dead(addr_of_mut!((*inst).alg)) {
        return;
    }
    (*inst).alg.cra_flags |= CRYPTO_ALG_DEAD;
    if tmpl.is_null() {
        return;
    }
    rust_algapi_list_del_init(addr_of_mut!((*inst).alg.cra_list));
    rust_algapi_hlist_del(inst_list(inst));
    rust_algapi_hlist_add(inst_list(inst), addr_of_mut!((*tmpl).dead));
    if !empty(addr_of_mut!((*inst).alg.cra_users)) {
        kernel::bindings::BUG();
    }
    rust_algapi_put(addr_of_mut!((*inst).alg));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_remove_spawns(
    alg: *mut crypto_alg,
    list: *mut list_head,
    nalg: *mut crypto_alg,
) {
    let new_type = (*if nalg.is_null() { alg } else { nalg }).cra_flags;
    let mut secondary: list_head = zeroed();
    let mut stack: list_head = zeroed();
    let mut top: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(secondary));
    rust_algapi_list_init(addr_of_mut!(stack));
    rust_algapi_list_init(addr_of_mut!(top));
    let mut spawns = addr_of_mut!((*alg).cra_users);
    let mut node = (*spawns).next;
    while node != spawns {
        let next = (*node).next;
        let spawn = list_spawn(node);
        if ((*(*spawn).alg).cra_flags ^ new_type) & (*spawn).mask == 0 {
            rust_algapi_list_move(node, addr_of_mut!(top));
        }
        node = next;
    }
    spawns = addr_of_mut!(top);
    loop {
        while !empty(spawns) {
            let spawn = list_spawn((*spawns).next);
            let inst = (*spawn).__bindgen_anon_1.inst;
            rust_algapi_list_move(addr_of_mut!((*spawn).list), addr_of_mut!(stack));
            (*spawn).dead = !(*spawn).registered || addr_of_mut!((*inst).alg) != nalg;
            if !(*spawn).registered {
                break;
            }
            if addr_of_mut!((*inst).alg) == alg {
                kernel::bindings::BUG();
            }
            if addr_of_mut!((*inst).alg) == nalg {
                break;
            }
            spawns = addr_of_mut!((*inst).alg.cra_users);
            // A failed, not-yet-registered instance has no initialized users head.
            if (*spawns).next.is_null() {
                break;
            }
        }
        spawns = crypto_more_spawns(
            addr_of_mut!(stack),
            addr_of_mut!(top),
            addr_of_mut!(secondary),
        );
        if spawns.is_null() {
            break;
        }
    }
    let mut node = secondary.next;
    while node != addr_of_mut!(secondary) {
        let next = (*node).next;
        let spawn = list_spawn(node);
        if !(*spawn).dead {
            rust_algapi_list_move(node, addr_of_mut!((*(*spawn).alg).cra_users));
        } else if (*spawn).registered {
            crypto_remove_instance((*spawn).__bindgen_anon_1.inst, list);
        }
        node = next;
    }
}

// Caller holds crypto_alg_sem for writing, as in the C provider.
unsafe fn crypto_alg_finish_registration(alg: *mut crypto_alg, algs_to_put: *mut list_head) {
    let head = addr_of_mut!(crypto_alg_list);
    let mut node = (*head).next;
    while node != head {
        let q = list_alg(node);
        if q != alg
            && !moribund(q)
            && !larval(q)
            && strcmp((*alg).cra_name.as_ptr(), (*q).cra_name.as_ptr()) == 0
            && (strcmp(
                (*alg).cra_driver_name.as_ptr(),
                (*q).cra_driver_name.as_ptr(),
            ) == 0
                || (*q).cra_priority <= (*alg).cra_priority)
        {
            crypto_remove_spawns(q, algs_to_put, alg);
        }
        node = (*node).next;
    }
    blocking_notifier_call_chain(
        addr_of_mut!(crypto_chain),
        CRYPTO_MSG_ALG_LOADED as _,
        alg.cast(),
    );
}

unsafe fn crypto_alloc_test_larval(alg: *mut crypto_alg) -> *mut crypto_larval {
    if RUST_ALGAPI_SELFTESTS == 0 || (*alg).cra_flags & CRYPTO_ALG_INTERNAL != 0 {
        return null_mut();
    }
    let test = crypto_larval_alloc(
        (*alg).cra_name.as_ptr(),
        (*alg).cra_flags | CRYPTO_ALG_TESTED,
        0,
    );
    if is_err(test) {
        return test;
    }
    (*test).adult = crypto_mod_get(alg);
    if (*test).adult.is_null() {
        kfree(test.cast());
        return err_ptr(neg(ENOENT));
    }
    rust_algapi_ref_set(addr_of_mut!((*test).alg.cra_refcnt), 1);
    core::ptr::copy_nonoverlapping(
        (*alg).cra_driver_name.as_ptr(),
        (*test).alg.cra_driver_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME as usize,
    );
    (*test).alg.cra_priority = (*alg).cra_priority;
    test
}

unsafe fn __crypto_register_alg(
    alg: *mut crypto_alg,
    algs_to_put: *mut list_head,
) -> *mut crypto_larval {
    if dead(alg) {
        return err_ptr(neg(EAGAIN));
    }
    rust_algapi_list_init(addr_of_mut!((*alg).cra_users));
    let head = addr_of_mut!(crypto_alg_list);
    let mut node = (*head).next;
    while node != head {
        let q = list_alg(node);
        if q == alg {
            return err_ptr(neg(EEXIST));
        }
        if !moribund(q) {
            if larval(q) {
                if strcmp(
                    (*alg).cra_driver_name.as_ptr(),
                    (*q).cra_driver_name.as_ptr(),
                ) == 0
                {
                    return err_ptr(neg(EEXIST));
                }
            } else if strcmp((*q).cra_driver_name.as_ptr(), (*alg).cra_name.as_ptr()) == 0
                || strcmp(
                    (*q).cra_driver_name.as_ptr(),
                    (*alg).cra_driver_name.as_ptr(),
                ) == 0
                || strcmp((*q).cra_name.as_ptr(), (*alg).cra_driver_name.as_ptr()) == 0
            {
                return err_ptr(neg(EEXIST));
            }
        }
        node = (*node).next;
    }
    let test = crypto_alloc_test_larval(alg);
    if is_err(test) {
        return test;
    }
    rust_algapi_list_add(addr_of_mut!((*alg).cra_list), head);
    if !test.is_null() {
        (*alg).cra_flags &= !CRYPTO_ALG_TESTED;
        rust_algapi_list_add(addr_of_mut!((*test).alg.cra_list), head);
    } else {
        (*alg).cra_flags |= CRYPTO_ALG_TESTED;
        crypto_alg_finish_registration(alg, algs_to_put);
    }
    test
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_alg_tested(name: *const c_char, err: c_int) {
    let mut list: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(list));
    down_write(addr_of_mut!(crypto_alg_sem));
    let head = addr_of_mut!(crypto_alg_list);
    let mut node = (*head).next;
    let mut test: *mut crypto_larval = null_mut();
    while node != head {
        let q = list_alg(node);
        if !moribund(q) && larval(q) && strcmp((*q).cra_driver_name.as_ptr(), name) == 0 {
            test = q.cast();
            break;
        }
        node = (*node).next;
    }
    if test.is_null() {
        rust_algapi_unexpected_test(name, err);
        up_write(addr_of_mut!(crypto_alg_sem));
        return;
    }
    (*test).alg.cra_flags |= CRYPTO_ALG_DEAD;
    let alg = (*test).adult;
    if !dead(alg) && (err == 0 || err == neg(ECANCELED)) {
        if err == neg(ECANCELED) {
            (*alg).cra_flags |= CRYPTO_ALG_FIPS_INTERNAL;
        } else {
            (*alg).cra_flags &= !CRYPTO_ALG_FIPS_INTERNAL;
        }
        (*alg).cra_flags |= CRYPTO_ALG_TESTED;
        crypto_alg_finish_registration(alg, addr_of_mut!(list));
    }
    rust_algapi_list_del_init(addr_of_mut!((*test).alg.cra_list));
    complete_all(addr_of_mut!((*test).completion));
    up_write(addr_of_mut!(crypto_alg_sem));
    rust_algapi_put(addr_of_mut!((*test).alg));
    crypto_remove_final(addr_of_mut!(list));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_remove_final(list: *mut list_head) {
    let mut node = (*list).next;
    while node != list {
        let next = (*node).next;
        let alg = list_alg(node);
        rust_algapi_list_del_init(node);
        rust_algapi_put(alg);
        node = next;
    }
}

unsafe extern "C" fn crypto_free_alg(alg: *mut crypto_alg) {
    let algsize = (*(*alg).cra_type).algsize;
    let p = alg.cast::<u8>().sub(algsize as usize);
    crypto_destroy_alg(alg);
    kfree(p.cast());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_alg(mut alg: *mut crypto_alg) -> c_int {
    let mut algs_to_put: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(algs_to_put));
    (*alg).cra_flags &= !CRYPTO_ALG_DEAD;
    let err = crypto_check_alg(alg);
    if err != 0 {
        return err;
    }
    if (*alg).cra_flags & CRYPTO_ALG_DUP_FIRST != 0
        && !rust_algapi_warn_dup((*alg).cra_destroy.is_some())
    {
        let algsize = (*(*alg).cra_type).algsize as usize;
        let p = alg.cast::<u8>().sub(algsize);
        let p = rust_algapi_kmemdup(p.cast(), algsize + size_of::<crypto_alg>()).cast::<u8>();
        if p.is_null() {
            return neg(ENOMEM);
        }
        alg = p.add(algsize).cast();
        (*alg).cra_destroy = Some(crypto_free_alg);
    }
    down_write(addr_of_mut!(crypto_alg_sem));
    let test = __crypto_register_alg(alg, addr_of_mut!(algs_to_put));
    let mut test_started = false;
    if !is_err(test) && !test.is_null() {
        test_started = rust_algapi_boot_test_finished();
        (*test).test_started = test_started;
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    if is_err(test) {
        rust_algapi_put(alg);
        return ptr_err(test);
    }
    if test_started {
        crypto_schedule_test(test);
    } else {
        crypto_remove_final(addr_of_mut!(algs_to_put));
    }
    0
}

unsafe fn crypto_remove_alg(alg: *mut crypto_alg, list: *mut list_head) -> c_int {
    if empty(addr_of_mut!((*alg).cra_list)) {
        return neg(ENOENT);
    }
    (*alg).cra_flags |= CRYPTO_ALG_DEAD;
    rust_algapi_list_del_init(addr_of_mut!((*alg).cra_list));
    crypto_remove_spawns(alg, list, null_mut());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_alg(alg: *mut crypto_alg) {
    let mut list: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(list));
    down_write(addr_of_mut!(crypto_alg_sem));
    let ret = crypto_remove_alg(alg, addr_of_mut!(list));
    up_write(addr_of_mut!(crypto_alg_sem));
    if rust_algapi_warn_unregister(ret, (*alg).cra_driver_name.as_ptr()) {
        return;
    }
    rust_algapi_warn_ref(
        (*alg).cra_destroy.is_none() && rust_algapi_ref_read(addr_of_mut!((*alg).cra_refcnt)) != 1,
    );
    rust_algapi_list_add(addr_of_mut!((*alg).cra_list), addr_of_mut!(list));
    crypto_remove_final(addr_of_mut!(list));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_algs(algs: *mut crypto_alg, count: c_int) -> c_int {
    for i in 0..count {
        let ret = crypto_register_alg(algs.add(i as usize));
        if ret != 0 {
            crypto_unregister_algs(algs, i);
            return ret;
        }
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_algs(algs: *mut crypto_alg, count: c_int) {
    for i in (0..count).rev() {
        crypto_unregister_alg(algs.add(i as usize));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_template(tmpl: *mut crypto_template) -> c_int {
    rust_algapi_init_work(
        addr_of_mut!((*tmpl).free_work),
        Some(crypto_destroy_instance_workfn),
    );
    down_write(addr_of_mut!(crypto_alg_sem));
    crypto_check_module_sig((*tmpl).module);
    let head = addr_of_mut!(crypto_template_list);
    let mut node = (*head).next;
    while node != head {
        let q = node
            .cast::<u8>()
            .sub(offset_of!(crypto_template, list))
            .cast::<crypto_template>();
        if q == tmpl {
            up_write(addr_of_mut!(crypto_alg_sem));
            return neg(EEXIST);
        }
        node = (*node).next;
    }
    rust_algapi_list_add(addr_of_mut!((*tmpl).list), head);
    up_write(addr_of_mut!(crypto_alg_sem));
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_templates(
    tmpls: *mut crypto_template,
    count: c_int,
) -> c_int {
    for i in 0..count {
        let ret = crypto_register_template(tmpls.add(i as usize));
        if ret != 0 {
            crypto_unregister_templates(tmpls, i);
            return ret;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_template(tmpl: *mut crypto_template) {
    let mut users: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(users));
    down_write(addr_of_mut!(crypto_alg_sem));
    if empty(addr_of_mut!((*tmpl).list)) {
        kernel::bindings::BUG();
    }
    rust_algapi_list_del_init(addr_of_mut!((*tmpl).list));
    let list = addr_of_mut!((*tmpl).instances);
    let mut node = (*list).first;
    while !node.is_null() {
        let inst = hlist_instance(node);
        if crypto_remove_alg(addr_of_mut!((*inst).alg), addr_of_mut!(users)) != 0 {
            kernel::bindings::BUG();
        }
        node = (*node).next;
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    let mut node = (*list).first;
    while !node.is_null() {
        let next = (*node).next;
        let inst = hlist_instance(node);
        if rust_algapi_ref_read(addr_of_mut!((*inst).alg.cra_refcnt)) != 1 {
            kernel::bindings::BUG();
        }
        crypto_free_instance(inst);
        node = next;
    }
    crypto_remove_final(addr_of_mut!(users));
    flush_work(addr_of_mut!((*tmpl).free_work));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_templates(tmpls: *mut crypto_template, count: c_int) {
    for i in (0..count).rev() {
        crypto_unregister_template(tmpls.add(i as usize));
    }
}

unsafe fn __crypto_lookup_template(name: *const c_char) -> *mut crypto_template {
    let mut tmpl = null_mut();
    down_read(addr_of_mut!(crypto_alg_sem));
    let head = addr_of_mut!(crypto_template_list);
    let mut node = (*head).next;
    while node != head {
        let q = node
            .cast::<u8>()
            .sub(offset_of!(crypto_template, list))
            .cast::<crypto_template>();
        if strcmp((*q).name.as_ptr(), name) == 0 && rust_algapi_tmpl_get(q) {
            tmpl = q;
            break;
        }
        node = (*node).next;
    }
    up_read(addr_of_mut!(crypto_alg_sem));
    tmpl
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_lookup_template(name: *const c_char) -> *mut crypto_template {
    let tmpl = __crypto_lookup_template(name);
    if !tmpl.is_null() || RUST_ALGAPI_MODULES == 0 {
        return tmpl;
    }
    rust_algapi_request_module(name);
    __crypto_lookup_template(name)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_instance(
    tmpl: *mut crypto_template,
    inst: *mut crypto_instance,
) -> c_int {
    let mut algs_to_put: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(algs_to_put));
    let err = crypto_check_alg(addr_of_mut!((*inst).alg));
    if err != 0 {
        return err;
    }
    (*inst).alg.cra_module = (*tmpl).module;
    (*inst).alg.cra_flags |= CRYPTO_ALG_INSTANCE;
    (*inst).alg.cra_destroy = Some(crypto_destroy_instance);
    down_write(addr_of_mut!(crypto_alg_sem));
    let mut fips_internal = 0;
    let mut spawn = (*inst).__bindgen_anon_1.spawns;
    while !spawn.is_null() {
        if (*spawn).dead {
            up_write(addr_of_mut!(crypto_alg_sem));
            return neg(EAGAIN);
        }
        let next = (*spawn).__bindgen_anon_1.next;
        (*spawn).__bindgen_anon_1.inst = inst;
        (*spawn).registered = true;
        fips_internal |= (*(*spawn).alg).cra_flags;
        crypto_mod_put((*spawn).alg);
        spawn = next;
    }
    (*inst).alg.cra_flags |= fips_internal & CRYPTO_ALG_FIPS_INTERNAL;
    let test = __crypto_register_alg(addr_of_mut!((*inst).alg), addr_of_mut!(algs_to_put));
    if !is_err(test) {
        if !test.is_null() {
            (*test).test_started = true;
        }
        rust_algapi_hlist_add(inst_list(inst), addr_of_mut!((*tmpl).instances));
        (*inst).tmpl = tmpl;
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    if is_err(test) {
        return ptr_err(test);
    }
    if !test.is_null() {
        crypto_schedule_test(test);
    } else {
        crypto_remove_final(addr_of_mut!(algs_to_put));
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_instance(inst: *mut crypto_instance) {
    let mut list: list_head = zeroed();
    rust_algapi_list_init(addr_of_mut!(list));
    down_write(addr_of_mut!(crypto_alg_sem));
    crypto_remove_spawns(addr_of_mut!((*inst).alg), addr_of_mut!(list), null_mut());
    crypto_remove_instance(inst, addr_of_mut!(list));
    up_write(addr_of_mut!(crypto_alg_sem));
    crypto_remove_final(addr_of_mut!(list));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_grab_spawn(
    spawn: *mut crypto_spawn,
    inst: *mut crypto_instance,
    name: *const c_char,
    ty: u32,
    mask: u32,
) -> c_int {
    if rust_algapi_warn_inst(inst.is_null()) {
        return neg(EINVAL);
    }
    if is_err(name) {
        return ptr_err(name);
    }
    let alg = crypto_find_alg(name, (*spawn).frontend, ty | CRYPTO_ALG_FIPS_INTERNAL, mask);
    if is_err(alg) {
        return ptr_err(alg);
    }
    let mut err = neg(EAGAIN);
    down_write(addr_of_mut!(crypto_alg_sem));
    if !moribund(alg) {
        rust_algapi_list_add(addr_of_mut!((*spawn).list), addr_of_mut!((*alg).cra_users));
        (*spawn).alg = alg;
        (*spawn).mask = mask;
        (*spawn).__bindgen_anon_1.next = (*inst).__bindgen_anon_1.spawns;
        (*inst).__bindgen_anon_1.spawns = spawn;
        (*inst).alg.cra_flags |= (*alg).cra_flags & CRYPTO_ALG_INHERITED_FLAGS;
        err = 0;
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    if err != 0 {
        crypto_mod_put(alg);
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_drop_spawn(spawn: *mut crypto_spawn) {
    if (*spawn).alg.is_null() {
        return;
    }
    down_write(addr_of_mut!(crypto_alg_sem));
    if !(*spawn).dead {
        rust_algapi_list_del(addr_of_mut!((*spawn).list));
    }
    up_write(addr_of_mut!(crypto_alg_sem));
    if !(*spawn).registered {
        crypto_mod_put((*spawn).alg);
    }
}

unsafe fn crypto_spawn_alg(spawn: *mut crypto_spawn) -> *mut crypto_alg {
    let mut alg = err_ptr(neg(EAGAIN));
    let mut target = null_mut();
    down_read(addr_of_mut!(crypto_alg_sem));
    if !(*spawn).dead {
        alg = (*spawn).alg;
        if crypto_mod_get(alg).is_null() {
            target = rust_algapi_get(alg);
            alg = err_ptr(neg(EAGAIN));
        }
    }
    up_read(addr_of_mut!(crypto_alg_sem));
    if !target.is_null() {
        crypto_shoot_alg(target);
        rust_algapi_put(target);
    }
    alg
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_spawn_tfm(
    spawn: *mut crypto_spawn,
    ty: u32,
    mask: u32,
) -> *mut crypto_tfm {
    let alg = crypto_spawn_alg(spawn);
    if is_err(alg) {
        return alg.cast();
    }
    let tfm = if ((*alg).cra_flags ^ ty) & mask != 0 {
        err_ptr(neg(EINVAL))
    } else {
        __crypto_alloc_tfm(alg, ty, mask)
    };
    if is_err(tfm) {
        crypto_mod_put(alg);
    }
    tfm
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_spawn_tfm2(spawn: *mut crypto_spawn) -> *mut c_void {
    let alg = crypto_spawn_alg(spawn);
    if is_err(alg) {
        return alg.cast();
    }
    let tfm = rust_algapi_create_tfm(alg, (*spawn).frontend);
    if is_err(tfm) {
        crypto_mod_put(alg);
    }
    tfm
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_register_notifier(nb: *mut notifier_block) -> c_int {
    blocking_notifier_chain_register(addr_of_mut!(crypto_chain), nb)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_unregister_notifier(nb: *mut notifier_block) -> c_int {
    blocking_notifier_chain_unregister(addr_of_mut!(crypto_chain), nb)
}

// RTA_LENGTH(0) is the aligned rtattr header size. Preserve the C macro's
// unsigned size_t subtraction, including its behavior on a short header.
const RTA_HEADER: usize = (size_of::<rtattr>() + 3) & !3;
#[inline]
unsafe fn rta_payload(rta: *mut rtattr) -> usize {
    ((*rta).rta_len as usize).wrapping_sub(RTA_HEADER)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_get_attr_type(tb: *mut *mut rtattr) -> *mut crypto_attr_type {
    let rta = *tb;
    if rta.is_null() {
        return err_ptr(neg(ENOENT));
    }
    if rta_payload(rta) < size_of::<crypto_attr_type>() || (*rta).rta_type as u32 != CRYPTOA_TYPE {
        return err_ptr(neg(EINVAL));
    }
    rta.cast::<u8>().add(RTA_HEADER).cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_check_attr_type(
    tb: *mut *mut rtattr,
    ty: u32,
    mask_ret: *mut u32,
) -> c_int {
    let algt = crypto_get_attr_type(tb);
    if is_err(algt) {
        return ptr_err(algt);
    }
    if ((*algt).type_ ^ ty) & (*algt).mask != 0 {
        return neg(EINVAL);
    }
    *mask_ret =
        ((*algt).type_ ^ CRYPTO_ALG_INHERITED_FLAGS) & (*algt).mask & CRYPTO_ALG_INHERITED_FLAGS;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_attr_alg_name(rta: *mut rtattr) -> *const c_char {
    if rta.is_null() {
        return err_ptr(neg(ENOENT));
    }
    if rta_payload(rta) < size_of::<crypto_attr_alg>() || (*rta).rta_type as u32 != CRYPTOA_ALG {
        return err_ptr(neg(EINVAL));
    }
    let alga = rta.cast::<u8>().add(RTA_HEADER).cast::<crypto_attr_alg>();
    (*alga).name[CRYPTO_MAX_ALG_NAME as usize - 1] = 0;
    (*alga).name.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __crypto_inst_setname(
    inst: *mut crypto_instance,
    name: *const c_char,
    driver: *const c_char,
    alg: *mut crypto_alg,
) -> c_int {
    if snprintf(
        (*inst).alg.cra_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME as usize,
        c"%s(%s)".as_ptr().cast(),
        name,
        (*alg).cra_name.as_ptr(),
    ) >= CRYPTO_MAX_ALG_NAME as c_int
    {
        return neg(ENAMETOOLONG);
    }
    if snprintf(
        (*inst).alg.cra_driver_name.as_mut_ptr(),
        CRYPTO_MAX_ALG_NAME as usize,
        c"%s(%s)".as_ptr().cast(),
        driver,
        (*alg).cra_driver_name.as_ptr(),
    ) >= CRYPTO_MAX_ALG_NAME as c_int
    {
        return neg(ENAMETOOLONG);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_init_queue(queue: *mut crypto_queue, max_qlen: c_uint) {
    rust_algapi_list_init(addr_of_mut!((*queue).list));
    (*queue).backlog = addr_of_mut!((*queue).list);
    (*queue).qlen = 0;
    (*queue).max_qlen = max_qlen;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_enqueue_request(
    queue: *mut crypto_queue,
    request: *mut crypto_async_request,
) -> c_int {
    let mut err = neg(EINPROGRESS);
    if (*queue).qlen >= (*queue).max_qlen {
        if (*request).flags & CRYPTO_TFM_REQ_MAY_BACKLOG == 0 {
            return neg(ENOSPC);
        }
        err = neg(EBUSY);
        if (*queue).backlog == addr_of_mut!((*queue).list) {
            (*queue).backlog = addr_of_mut!((*request).list);
        }
    }
    (*queue).qlen = (*queue).qlen.wrapping_add(1);
    rust_algapi_list_add_tail(addr_of_mut!((*request).list), addr_of_mut!((*queue).list));
    err
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_enqueue_request_head(
    queue: *mut crypto_queue,
    request: *mut crypto_async_request,
) {
    if (*queue).qlen >= (*queue).max_qlen {
        (*queue).backlog = (*(*queue).backlog).prev;
    }
    (*queue).qlen = (*queue).qlen.wrapping_add(1);
    rust_algapi_list_add(addr_of_mut!((*request).list), addr_of_mut!((*queue).list));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_dequeue_request(
    queue: *mut crypto_queue,
) -> *mut crypto_async_request {
    if (*queue).qlen == 0 {
        return null_mut();
    }
    (*queue).qlen -= 1;
    if (*queue).backlog != addr_of_mut!((*queue).list) {
        (*queue).backlog = (*(*queue).backlog).next;
    }
    let request = (*queue).list.next;
    rust_algapi_list_del_init(request);
    request
        .cast::<u8>()
        .sub(offset_of!(crypto_async_request, list))
        .cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_inc(a: *mut u8, mut size: c_uint) {
    let mut end = a.add(size as usize);
    if RUST_ALGAPI_UNALIGNED != 0 || (end as usize) & (core::mem::align_of::<u32>() - 1) == 0 {
        while size >= 4 {
            end = end.sub(4);
            let c = u32::from_be(end.cast::<u32>().read_unaligned()).wrapping_add(1);
            end.cast::<u32>().write_unaligned(c.to_be());
            if c != 0 {
                return;
            }
            size -= 4;
        }
    }
    let mut end = a.add(size as usize);
    while size != 0 {
        end = end.sub(1);
        let c = end.read().wrapping_add(1);
        end.write(c);
        if c != 0 {
            break;
        }
        size -= 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_alg_extsize(alg: *mut crypto_alg) -> c_uint {
    (*alg)
        .cra_ctxsize
        .wrapping_add((*alg).cra_alignmask & !(rust_algapi_ctx_alignment() - 1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crypto_type_has_alg(
    name: *const c_char,
    frontend: *const crypto_type,
    ty: u32,
    mask: u32,
) -> c_int {
    let alg = crypto_find_alg(name, frontend, ty, mask);
    if is_err(alg) {
        return 0;
    }
    crypto_mod_put(alg);
    1
}

#[unsafe(link_section = ".init.text")]
unsafe fn crypto_start_tests() {
    if RUST_ALGAPI_BUILTIN == 0 || RUST_ALGAPI_SELFTESTS == 0 {
        return;
    }
    rust_algapi_set_boot_test_finished();
    loop {
        let mut test: *mut crypto_larval = null_mut();
        down_write(addr_of_mut!(crypto_alg_sem));
        let head = addr_of_mut!(crypto_alg_list);
        let mut node = (*head).next;
        while node != head {
            let q = list_alg(node);
            if larval(q) {
                let l = q.cast::<crypto_larval>();
                if (*l).alg.cra_driver_name[0] != 0 && !(*l).test_started {
                    (*l).test_started = true;
                    test = l;
                    break;
                }
            }
            node = (*node).next;
        }
        up_write(addr_of_mut!(crypto_alg_sem));
        if test.is_null() {
            break;
        }
        crypto_schedule_test(test);
    }
}
#[unsafe(no_mangle)]
#[unsafe(link_section = ".init.text")]
pub unsafe extern "C" fn rust_crypto_algapi_init() -> c_int {
    rust_algapi_init_proc();
    crypto_start_tests();
    0
}
#[unsafe(no_mangle)]
#[unsafe(link_section = ".exit.text")]
pub unsafe extern "C" fn rust_crypto_algapi_exit() {
    rust_algapi_exit_proc();
}

// The defining Rust object owns the original GPL-only, empty-namespace exports.
ffi_export::export_symbol!(crypto_remove_spawns, crypto_remove_spawns, "GPL", "");
ffi_export::export_symbol!(crypto_alg_tested, crypto_alg_tested, "GPL", "");
ffi_export::export_symbol!(crypto_remove_final, crypto_remove_final, "GPL", "");
ffi_export::export_symbol!(crypto_register_alg, crypto_register_alg, "GPL", "");
ffi_export::export_symbol!(crypto_unregister_alg, crypto_unregister_alg, "GPL", "");
ffi_export::export_symbol!(crypto_register_algs, crypto_register_algs, "GPL", "");
ffi_export::export_symbol!(crypto_unregister_algs, crypto_unregister_algs, "GPL", "");
ffi_export::export_symbol!(
    crypto_register_template,
    crypto_register_template,
    "GPL",
    ""
);
ffi_export::export_symbol!(
    crypto_register_templates,
    crypto_register_templates,
    "GPL",
    ""
);
ffi_export::export_symbol!(
    crypto_unregister_template,
    crypto_unregister_template,
    "GPL",
    ""
);
ffi_export::export_symbol!(
    crypto_unregister_templates,
    crypto_unregister_templates,
    "GPL",
    ""
);
ffi_export::export_symbol!(crypto_lookup_template, crypto_lookup_template, "GPL", "");
ffi_export::export_symbol!(
    crypto_register_instance,
    crypto_register_instance,
    "GPL",
    ""
);
ffi_export::export_symbol!(
    crypto_unregister_instance,
    crypto_unregister_instance,
    "GPL",
    ""
);
ffi_export::export_symbol!(crypto_grab_spawn, crypto_grab_spawn, "GPL", "");
ffi_export::export_symbol!(crypto_drop_spawn, crypto_drop_spawn, "GPL", "");
ffi_export::export_symbol!(crypto_spawn_tfm, crypto_spawn_tfm, "GPL", "");
ffi_export::export_symbol!(crypto_spawn_tfm2, crypto_spawn_tfm2, "GPL", "");
ffi_export::export_symbol!(
    crypto_register_notifier,
    crypto_register_notifier,
    "GPL",
    ""
);
ffi_export::export_symbol!(
    crypto_unregister_notifier,
    crypto_unregister_notifier,
    "GPL",
    ""
);
ffi_export::export_symbol!(crypto_get_attr_type, crypto_get_attr_type, "GPL", "");
ffi_export::export_symbol!(crypto_check_attr_type, crypto_check_attr_type, "GPL", "");
ffi_export::export_symbol!(crypto_attr_alg_name, crypto_attr_alg_name, "GPL", "");
ffi_export::export_symbol!(__crypto_inst_setname, __crypto_inst_setname, "GPL", "");
ffi_export::export_symbol!(crypto_init_queue, crypto_init_queue, "GPL", "");
ffi_export::export_symbol!(crypto_enqueue_request, crypto_enqueue_request, "GPL", "");
ffi_export::export_symbol!(
    crypto_enqueue_request_head,
    crypto_enqueue_request_head,
    "GPL",
    ""
);
ffi_export::export_symbol!(crypto_dequeue_request, crypto_dequeue_request, "GPL", "");
ffi_export::export_symbol!(crypto_inc, crypto_inc, "GPL", "");
ffi_export::export_symbol!(crypto_alg_extsize, crypto_alg_extsize, "GPL", "");
ffi_export::export_symbol!(crypto_type_has_alg, crypto_type_has_alg, "GPL", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
