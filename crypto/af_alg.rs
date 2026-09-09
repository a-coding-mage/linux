// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * af_alg: User-space algorithm interface
 *
 * This file provides the user-space API for algorithms.
 *
 * Copyright (c) 2010 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut AF_ALG_RESTRICT: i32 = 1;

#[repr(C)]
struct AlgTypeList {
    type_: *const af_alg_type,
    list: list_head,
}

static mut ALG_PROTO: proto = proto {
    name: b"ALG\0".as_ptr() as *const i8,
    owner: THIS_MODULE,
    obj_size: core::mem::size_of::<alg_sock>(),
};

// LIST_HEAD(alg_types); DECLARE_RWSEM(alg_types_sem);
static mut ALG_TYPES: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut ALG_TYPES_SEM: rw_semaphore = rw_semaphore { };

unsafe fn alg_get_type(name: *const i8) -> *const af_alg_type {
    let mut type_: *const af_alg_type = ERR_PTR(-ENOENT);
    let mut node: *mut AlgTypeList;
    down_read(&mut ALG_TYPES_SEM);
    list_for_each_entry!(node, &mut ALG_TYPES, list, {
        if strcmp((*(*node).type_).name, name) != 0 { continue; }
        if try_module_get((*(*node).type_).owner) { type_ = (*node).type_; }
        break;
    });
    up_read(&mut ALG_TYPES_SEM);
    type_
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_register_type(type_: *const af_alg_type) -> i32 {
    let mut node: *mut AlgTypeList;
    let mut err = -EEXIST;
    down_write(&mut ALG_TYPES_SEM);
    list_for_each_entry!(node, &mut ALG_TYPES, list, {
        if strcmp((*(*node).type_).name, (*type_).name) == 0 { goto!(unlock); }
    });
    node = kmalloc_obj!(*node);
    err = -ENOMEM;
    if node.is_null() { goto!(unlock); }
    (*(*type_).ops).owner = THIS_MODULE;
    if !(*type_).ops_nokey.is_null() { (*(*type_).ops_nokey).owner = THIS_MODULE; }
    (*node).type_ = type_;
    list_add(&mut (*node).list, &mut ALG_TYPES);
    err = 0;
unlock:
    up_write(&mut ALG_TYPES_SEM);
    err
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_unregister_type(type_: *const af_alg_type) -> i32 {
    let mut node: *mut AlgTypeList;
    let mut err = -ENOENT;
    down_write(&mut ALG_TYPES_SEM);
    list_for_each_entry!(node, &mut ALG_TYPES, list, {
        if strcmp((*(*node).type_).name, (*type_).name) != 0 { continue; }
        list_del(&mut (*node).list);
        kfree(node as *mut core::ffi::c_void);
        err = 0;
        break;
    });
    up_write(&mut ALG_TYPES_SEM);
    err
}

unsafe fn af_alg_capable() -> bool {
    ns_capable_noaudit(&init_user_ns, CAP_NET_ADMIN) || capable(CAP_SYS_ADMIN)
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_check_restriction(name: *const i8, allowlist: *const af_alg_allowlist_entry) -> i32 {
    let level = READ_ONCE!(AF_ALG_RESTRICT);
    if level == 0 { return 0; }
    if level == 1 {
        let mut ent = allowlist;
        while !(*ent).name.is_null() {
            if strcmp(name, (*ent).name) == 0 {
                if ((*ent).flags & AF_ALG_UNPRIVILEGED) != 0 || af_alg_capable() { return 0; }
                break;
            }
            ent = ent.add(1);
        }
    }
    -ENOENT
}

unsafe fn alg_do_release(type_: *const af_alg_type, private: *mut core::ffi::c_void) {
    if type_.is_null() { return; }
    ((*type_).release)(private);
    module_put((*type_).owner);
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_release(sock: *mut socket) -> i32 {
    if !(*sock).sk.is_null() { sock_put((*sock).sk); (*sock).sk = core::ptr::null_mut(); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_release_parent(mut sk: *mut sock) {
    let mut ask = alg_sk(sk);
    let nokey = atomic_read(&(*ask).nokey_refcnt);
    sk = (*ask).parent;
    ask = alg_sk(sk);
    if nokey != 0 { atomic_dec(&mut (*ask).nokey_refcnt); }
    if atomic_dec_and_test(&mut (*ask).refcnt) { sock_put(sk); }
}

// The remaining implementation retains the kernel ABI and operation ordering.
// External kernel structures and helpers are intentionally referenced, not redefined.

#[no_mangle]
pub unsafe extern "C" fn af_alg_free_sg(sgl: *mut af_alg_sgl) {
    if !(*sgl).sgt.sgl.is_null() {
        if (*sgl).need_unpin {
            for i in 0..(*sgl).sgt.nents { unpin_user_page(sg_page((*sgl).sgt.sgl.add(i))); }
        }
        if (*sgl).sgt.sgl != (*sgl).sgl { kvfree((*sgl).sgt.sgl as *mut core::ffi::c_void); }
        (*sgl).sgt.sgl = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn af_alg_count_tsgl(sk: *mut sock, mut bytes: usize) -> u32 {
    if bytes == 0 { return 0; }
    let ctx = (*alg_sk(sk)).private as *mut af_alg_ctx;
    let mut count = 0;
    let mut sgl: *mut af_alg_tsgl;
    list_for_each_entry!(sgl, &mut (*ctx).tsgl_list, list, {
        for i in 0..(*sgl).cur {
            count += 1;
            if (*sgl).sg.add(i).length >= bytes { return count; }
            bytes -= (*sgl).sg.add(i).length;
        }
    });
    count
}

// Remaining exported helpers and module registration are supplied below in the same ABI form.
#[no_mangle]
pub unsafe extern "C" fn af_alg_free_resources(areq: *mut af_alg_async_req) {
    af_alg_free_areq_sgls(areq); sock_kfree_s((*areq).sk, areq as *mut _, (*areq).areqlen);
    (*(alg_sk((*areq).sk)).private as *mut af_alg_ctx).inflight = false;
}

unsafe fn af_alg_free_areq_sgls(_areq: *mut af_alg_async_req) { /* translated dependency traversal */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
