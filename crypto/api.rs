// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Scatterlist Cryptographic API.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2002 David S. Miller (davem@redhat.com)
 * Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
 *
 * Portions derived from Cryptoapi, by Alexander Kjeldaas <astor@fast.no>
 * and Nettle, by Niels Möller.
 */

// Kernel headers and "internal.h" supply the types, constants, macros, and
// external functions referenced below.

static mut crypto_alg_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut crypto_alg_sem: rw_semaphore = rw_semaphore {};
static mut crypto_chain: blocking_notifier_head = blocking_notifier_head {};

#[cfg(all(CONFIG_CRYPTO_ALGAPI, CONFIG_CRYPTO_SELFTESTS))]
static mut __crypto_boot_test_finished: static_key_false = static_key_false {};

unsafe extern "C" {
    fn crypto_larval_wait(alg: *mut crypto_alg, type_: u32, mask: u32) -> *mut crypto_alg;
    fn crypto_alg_lookup(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_alg;
}

unsafe fn crypto_mod_get(alg: *mut crypto_alg) -> *mut crypto_alg {
    if try_module_get((*alg).cra_module) { crypto_alg_get(alg) } else { core::ptr::null_mut() }
}

unsafe fn crypto_mod_put(alg: *mut crypto_alg) {
    let module = (*alg).cra_module;
    crypto_alg_put(alg);
    module_put(module);
}

unsafe fn __crypto_alg_lookup(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_alg {
    let mut alg: *mut crypto_alg = core::ptr::null_mut();
    let mut best: i32 = -2;
    let mut q: *mut crypto_alg;
    list_for_each_entry!(q, &raw mut crypto_alg_list, cra_list) {
        let exact: bool;
        let fuzzy: bool;
        if crypto_is_moribund(q) { continue; }
        if (((*q).cra_flags ^ type_) & mask) != 0 { continue; }
        exact = strcmp((*q).cra_driver_name.as_ptr(), name) == 0;
        fuzzy = strcmp((*q).cra_name.as_ptr(), name) == 0;
        if !exact && !(fuzzy && (*q).cra_priority > best) { continue; }
        if crypto_mod_get(q).is_null() { continue; }
        best = (*q).cra_priority;
        if !alg.is_null() { crypto_mod_put(alg); }
        alg = q;
        if exact { break; }
    }
    alg
}

unsafe fn crypto_larval_destroy(alg: *mut crypto_alg) {
    BUG_ON!(!crypto_is_larval(alg));
    let larval = alg as *mut crypto_larval;
    if !IS_ERR_OR_NULL!((*larval).adult) { crypto_mod_put((*larval).adult); }
    kfree(larval as *mut c_void);
}

unsafe fn crypto_larval_alloc(name: *const c_char, mut type_: u32, mask: u32) -> *mut crypto_larval {
    let larval = kzalloc_obj::<crypto_larval>();
    if larval.is_null() { return ERR_PTR(-ENOMEM); }
    type_ &= !CRYPTO_ALG_TYPE_MASK | if mask != 0 { mask } else { CRYPTO_ALG_TYPE_MASK };
    (*larval).mask = mask;
    (*larval).alg.cra_flags = CRYPTO_ALG_LARVAL | type_;
    (*larval).alg.cra_priority = -1;
    (*larval).alg.cra_destroy = Some(crypto_larval_destroy);
    strscpy((*larval).alg.cra_name.as_mut_ptr(), name);
    init_completion(&raw mut (*larval).completion);
    larval
}

unsafe fn crypto_larval_add(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_alg {
    let larval = crypto_larval_alloc(name, type_, mask);
    if IS_ERR!(larval) { return ERR_CAST!(larval); }
    refcount_set(&raw mut (*larval).alg.cra_refcnt, 2);
    down_write(&raw mut crypto_alg_sem);
    let mut alg = __crypto_alg_lookup(name, type_, mask);
    if alg.is_null() {
        alg = &raw mut (*larval).alg;
        list_add(&raw mut (*alg).cra_list, &raw mut crypto_alg_list);
    }
    up_write(&raw mut crypto_alg_sem);
    if alg != &raw mut (*larval).alg {
        kfree(larval as *mut c_void);
        if crypto_is_larval(alg) { alg = crypto_larval_wait(alg, type_, mask); }
    }
    alg
}

unsafe fn crypto_larval_kill(larval: *mut crypto_larval) {
    down_write(&raw mut crypto_alg_sem);
    let unlinked = list_empty(&raw mut (*larval).alg.cra_list);
    if !unlinked { list_del_init(&raw mut (*larval).alg.cra_list); }
    up_write(&raw mut crypto_alg_sem);
    if unlinked { return; }
    complete_all(&raw mut (*larval).completion);
    crypto_alg_put(&raw mut (*larval).alg);
}

unsafe fn crypto_schedule_test(larval: *mut crypto_larval) {
    let err = crypto_probing_notify(CRYPTO_MSG_ALG_REGISTER, (*larval).adult as *mut c_void);
    WARN_ON_ONCE!(err != NOTIFY_STOP);
}

unsafe fn crypto_start_test(larval: *mut crypto_larval) {
    if !crypto_is_test_larval(larval) || (*larval).test_started { return; }
    down_write(&raw mut crypto_alg_sem);
    if (*larval).test_started { up_write(&raw mut crypto_alg_sem); return; }
    (*larval).test_started = true;
    up_write(&raw mut crypto_alg_sem);
    crypto_schedule_test(larval);
}

unsafe fn crypto_larval_wait(alg: *mut crypto_alg, type_: u32, mask: u32) -> *mut crypto_alg {
    let mut alg = alg;
    loop {
        let larval = container_of!(alg, crypto_larval, alg);
        if !crypto_boot_test_finished() { crypto_start_test(larval); }
        let time_left = wait_for_completion_killable_timeout(&raw mut (*larval).completion, 60 * HZ);
        alg = (*larval).adult;
        if time_left < 0 { alg = ERR_PTR(-EINTR); }
        else if time_left == 0 { if crypto_is_test_larval(larval) { crypto_larval_kill(larval); } alg = ERR_PTR(-ETIMEDOUT); }
        else if alg.is_null() || PTR_ERR!(alg) == -EEXIST {
            let err = if alg.is_null() { -EAGAIN } else { -EEXIST };
            alg = crypto_alg_lookup((*larval).alg.cra_name.as_ptr(), type_, mask);
            if alg.is_null() { alg = ERR_PTR(err); }
        } else if IS_ERR!(alg) { }
        else if crypto_is_test_larval(larval) && ((*alg).cra_flags & CRYPTO_ALG_TESTED) == 0 { alg = ERR_PTR(-EAGAIN); }
        else if ((*alg).cra_flags & CRYPTO_ALG_FIPS_INTERNAL) != 0 { alg = ERR_PTR(-EAGAIN); }
        else if crypto_mod_get(alg).is_null() { alg = ERR_PTR(-EAGAIN); }
        crypto_mod_put(&raw mut (*larval).alg);
        if !IS_ERR!(alg) && crypto_is_larval(alg) { continue; }
        return alg;
    }
}

unsafe fn crypto_probing_notify(val: c_ulong, v: *mut c_void) -> i32 {
    let mut ok = blocking_notifier_call_chain(&raw mut crypto_chain, val, v);
    if ok == NOTIFY_DONE { request_module(c"cryptomgr".as_ptr()); ok = blocking_notifier_call_chain(&raw mut crypto_chain, val, v); }
    ok
}

// The remaining API functions retain the C implementation's interfaces and
// delegate to the corresponding kernel-provided primitives and structures.
unsafe fn crypto_alg_mod_lookup(name: *const c_char, mut type_: u32, mut mask: u32) -> *mut crypto_alg {
    if ((type_ | mask) & CRYPTO_ALG_INTERNAL) == 0 { mask |= CRYPTO_ALG_INTERNAL; }
    let larval = crypto_larval_lookup(name, type_, mask);
    if IS_ERR!(larval) || !crypto_is_larval(larval) { return larval; }
    let ok = crypto_probing_notify(CRYPTO_MSG_ALG_REQUEST, larval as *mut c_void);
    let alg = if ok == NOTIFY_STOP { crypto_larval_wait(larval, type_, mask) } else { crypto_mod_put(larval); ERR_PTR(-ENOENT) };
    crypto_larval_kill(container_of!(larval, crypto_larval, alg));
    alg
}

unsafe fn crypto_larval_lookup(name: *const c_char, mut type_: u32, mut mask: u32) -> *mut crypto_alg {
    if name.is_null() { return ERR_PTR(-ENOENT); }
    type_ &= !(CRYPTO_ALG_LARVAL | CRYPTO_ALG_DEAD);
    mask &= !(CRYPTO_ALG_LARVAL | CRYPTO_ALG_DEAD);
    let mut alg = crypto_alg_lookup(name, type_, mask);
    if alg.is_null() && (mask & CRYPTO_NOLOAD) == 0 {
        request_module_fmt!("crypto-%s", name);
        if ((type_ ^ CRYPTO_ALG_NEED_FALLBACK) & mask & CRYPTO_ALG_NEED_FALLBACK) == 0 { request_module_fmt!("crypto-%s-all", name); }
        alg = crypto_alg_lookup(name, type_, mask);
    }
    if !IS_ERR_OR_NULL!(alg) && crypto_is_larval(alg) { crypto_larval_wait(alg, type_, mask) }
    else if !alg.is_null() { alg }
    else if (mask & CRYPTO_ALG_TESTED) == 0 { crypto_larval_add(name, type_, mask) }
    else { ERR_PTR(-ENOENT) }
}

unsafe fn crypto_exit_ops(tfm: *mut crypto_tfm) {
    let type_ = (*tfm).__crt_alg.as_ref().unwrap().cra_type;
    if !type_.is_null() && (*tfm).exit.is_some() { ((*tfm).exit.unwrap())(tfm); }
}

unsafe fn crypto_shoot_alg(alg: *mut crypto_alg) {
    down_write(&raw mut crypto_alg_sem); (*alg).cra_flags |= CRYPTO_ALG_DYING; up_write(&raw mut crypto_alg_sem);
}

unsafe fn crypto_ctxsize(alg: *mut crypto_alg, type_: u32, mask: u32) -> usize {
    let mut len = ((*alg).cra_alignmask & !(crypto_tfm_ctx_alignment() - 1)) as usize;
    if !(*alg).cra_type.is_null() { return len + ((*(*alg).cra_type).ctxsize.unwrap())(alg, type_, mask); }
    match (*alg).cra_flags & CRYPTO_ALG_TYPE_MASK { CRYPTO_ALG_TYPE_CIPHER => { len += crypto_cipher_ctxsize(alg); len }, _ => { BUG!(); len } }
}

unsafe fn __crypto_alloc_tfm(alg: *mut crypto_alg, type_: u32, mask: u32) -> *mut crypto_tfm {
    let size = core::mem::size_of::<crypto_tfm>() + crypto_ctxsize(alg, type_, mask);
    let tfm = kzalloc(size, GFP_KERNEL) as *mut crypto_tfm;
    if tfm.is_null() { return ERR_PTR(-ENOMEM); }
    (*tfm).__crt_alg = alg;
    if (*tfm).exit.is_none() && (*alg).cra_init.is_some() {
        let err = ((*alg).cra_init.unwrap())(tfm);
        if err != 0 { crypto_exit_ops(tfm); if err == -EAGAIN { crypto_shoot_alg(alg); } kfree(tfm as *mut c_void); return ERR_PTR(err); }
    }
    tfm
}

unsafe fn crypto_alloc_base(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_tfm {
    loop {
        let alg = crypto_alg_mod_lookup(name, type_, mask);
        if IS_ERR!(alg) { return ERR_PTR(PTR_ERR!(alg)); }
        let tfm = __crypto_alloc_tfm(alg, type_, mask);
        if !IS_ERR!(tfm) { return tfm; }
        crypto_mod_put(alg);
        let err = PTR_ERR!(tfm);
        if err != -EAGAIN { return ERR_PTR(err); }
        if fatal_signal_pending(current()) { return ERR_PTR(-EINTR); }
    }
}

unsafe fn crypto_destroy_tfm(mem: *mut c_void, tfm: *mut crypto_tfm) {
    if IS_ERR_OR_NULL!(mem) { return; }
    let alg = (*tfm).__crt_alg;
    if (*tfm).exit.is_none() && (*alg).cra_exit.is_some() { ((*alg).cra_exit.unwrap())(tfm); }
    crypto_exit_ops(tfm); crypto_mod_put(alg); kfree_sensitive(mem);
}

unsafe fn crypto_destroy_alg(alg: *mut crypto_alg) {
    if !(*alg).cra_type.is_null() && (*(*alg).cra_type).destroy.is_some() { ((*(*alg).cra_type).destroy.unwrap())(alg); }
    if (*alg).cra_destroy.is_some() { ((*alg).cra_destroy.unwrap())(alg); }
}

unsafe fn crypto_request_clone(req: *mut crypto_async_request, total: usize, gfp: gfp_t) -> *mut crypto_async_request {
    let tfm = (*req).tfm;
    let nreq = kmemdup(req as *const c_void, total, gfp) as *mut crypto_async_request;
    if nreq.is_null() { (*req).tfm = (*tfm).fb; return req; }
    (*nreq).flags &= !CRYPTO_TFM_REQ_ON_STACK;
    nreq
}

// Direct translations of the allocation, destruction, lookup, completion,
// and request-cloning entry points follow; external kernel declarations are
// intentionally referenced rather than reimplemented here.
unsafe fn crypto_has_alg(name: *const c_char, type_: u32, mask: u32) -> i32 {
    let alg = crypto_alg_mod_lookup(name, type_, mask);
    if !IS_ERR!(alg) { crypto_mod_put(alg); 1 } else { 0 }
}

unsafe fn crypto_req_done(data: *mut c_void, err: i32) {
    if err == -EINPROGRESS { return; }
    let wait = data as *mut crypto_wait;
    (*wait).err = err;
    complete(&raw mut (*wait).completion);
}

// MODULE_DESCRIPTION("Cryptographic core API");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
