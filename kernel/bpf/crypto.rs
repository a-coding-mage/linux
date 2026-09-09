// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Meta, Inc */
// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct bpf_crypto_type_list {
    pub r#type: *const bpf_crypto_type,
    pub list: list_head,
}

/// BPF crypto initialization parameters struct.
#[repr(C)]
pub struct bpf_crypto_params {
    pub r#type: [c_char; 14],
    pub reserved: [u8; 2],
    pub algo: [c_char; 128],
    pub key: [u8; 256],
    pub key_len: u32,
    pub authsize: u32,
}

#[repr(C)]
pub struct bpf_crypto_ctx {
    pub r#type: *const bpf_crypto_type,
    pub tfm: *mut c_void,
    pub siv_len: u32,
    pub rcu: rcu_head,
    pub usage: refcount_t,
}

static mut bpf_crypto_types: list_head = LIST_HEAD_INIT;
static mut bpf_crypto_types_sem: rw_semaphore = DECLARE_RWSEM_INIT;

pub unsafe fn bpf_crypto_register_type(r#type: *const bpf_crypto_type) -> c_int {
    let mut err: c_int = -EBUSY;
    down_write(&raw mut bpf_crypto_types_sem);
    let mut node: *mut bpf_crypto_type_list;
    list_for_each_entry!(node, &raw mut bpf_crypto_types, list, {
        if strcmp((*(*node).r#type).name, (*r#type).name) == 0 { goto!(unlock); }
    });
    node = kmalloc_obj::<bpf_crypto_type_list>();
    err = -ENOMEM;
    if node.is_null() { goto!(unlock); }
    (*node).r#type = r#type;
    list_add(&raw mut (*node).list, &raw mut bpf_crypto_types);
    err = 0;
unlock:
    up_write(&raw mut bpf_crypto_types_sem);
    err
}

pub unsafe fn bpf_crypto_unregister_type(r#type: *const bpf_crypto_type) -> c_int {
    let mut err: c_int = -ENOENT;
    down_write(&raw mut bpf_crypto_types_sem);
    let mut node: *mut bpf_crypto_type_list;
    list_for_each_entry!(node, &raw mut bpf_crypto_types, list, {
        if strcmp((*(*node).r#type).name, (*r#type).name) != 0 { continue; }
        list_del(&raw mut (*node).list);
        kfree(node as *mut c_void);
        err = 0;
        break;
    });
    up_write(&raw mut bpf_crypto_types_sem);
    err
}

unsafe fn bpf_crypto_get_type(name: *const c_char) -> *const bpf_crypto_type {
    let mut r#type = ERR_PTR(-ENOENT);
    let mut node: *mut bpf_crypto_type_list;
    down_read(&raw mut bpf_crypto_types_sem);
    list_for_each_entry!(node, &raw mut bpf_crypto_types, list, {
        if strcmp((*(*node).r#type).name, name) != 0 { continue; }
        if try_module_get((*(*node).r#type).owner) { r#type = (*node).r#type; }
        break;
    });
    up_read(&raw mut bpf_crypto_types_sem);
    r#type
}

pub unsafe fn bpf_crypto_ctx_create(params: *const bpf_crypto_params, params__sz: u32, err: *mut c_int) -> *mut bpf_crypto_ctx {
    let mut r#type: *const bpf_crypto_type;
    let ctx: *mut bpf_crypto_ctx;
    if params.is_null() || (*params).reserved[0] != 0 || (*params).reserved[1] != 0 || params__sz as usize != core::mem::size_of::<bpf_crypto_params>() { *err = -EINVAL; return core::ptr::null_mut(); }
    r#type = bpf_crypto_get_type((*params).r#type.as_ptr());
    if IS_ERR(r#type) { *err = PTR_ERR(r#type); return core::ptr::null_mut(); }
    if !((*r#type).has_algo)((*params).algo.as_ptr()) { *err = -EOPNOTSUPP; goto!(err_module_put); }
    if ((!!(*params).authsize) ^ (!!(*r#type).setauthsize)) { *err = -EOPNOTSUPP; goto!(err_module_put); }
    if (*params).key_len == 0 || (*params).key_len as usize > (*params).key.len() { *err = -EINVAL; goto!(err_module_put); }
    ctx = kzalloc_obj::<bpf_crypto_ctx>();
    if ctx.is_null() { *err = -ENOMEM; goto!(err_module_put); }
    (*ctx).r#type = r#type;
    (*ctx).tfm = ((*r#type).alloc_tfm)((*params).algo.as_ptr());
    if IS_ERR((*ctx).tfm) { *err = PTR_ERR((*ctx).tfm); goto!(err_free_ctx); }
    if (*params).authsize != 0 { *err = ((*r#type).setauthsize)((*ctx).tfm, (*params).authsize); if *err != 0 { goto!(err_free_tfm); } }
    *err = ((*r#type).setkey)((*ctx).tfm, (*params).key.as_ptr(), (*params).key_len);
    if *err != 0 { goto!(err_free_tfm); }
    if ((*r#type).get_flags)((*ctx).tfm) & CRYPTO_TFM_NEED_KEY != 0 { *err = -EINVAL; goto!(err_free_tfm); }
    (*ctx).siv_len = ((*r#type).ivsize)((*ctx).tfm) + ((*r#type).statesize)((*ctx).tfm);
    refcount_set(&raw mut (*ctx).usage, 1);
    return ctx;
err_free_tfm:
    ((*r#type).free_tfm)((*ctx).tfm);
err_free_ctx:
    kfree(ctx as *mut c_void);
err_module_put:
    module_put((*r#type).owner);
    core::ptr::null_mut()
}

unsafe fn crypto_free_cb(head: *mut rcu_head) {
    let ctx = container_of!(head, bpf_crypto_ctx, rcu);
    ((*(*ctx).r#type).free_tfm)((*ctx).tfm);
    module_put((*(*ctx).r#type).owner);
    kfree(ctx as *mut c_void);
}

pub unsafe fn bpf_crypto_ctx_acquire(ctx: *mut bpf_crypto_ctx) -> *mut bpf_crypto_ctx {
    if !refcount_inc_not_zero(&raw mut (*ctx).usage) { return core::ptr::null_mut(); }
    ctx
}

pub unsafe fn bpf_crypto_ctx_release(ctx: *mut bpf_crypto_ctx) {
    if refcount_dec_and_test(&raw mut (*ctx).usage) { call_rcu(&raw mut (*ctx).rcu, crypto_free_cb); }
}

pub unsafe fn bpf_crypto_ctx_release_dtor(ctx: *mut c_void) { bpf_crypto_ctx_release(ctx as *mut bpf_crypto_ctx); }

unsafe fn bpf_crypto_crypt(ctx: *const bpf_crypto_ctx, src: *const bpf_dynptr_kern, dst: *const bpf_dynptr_kern, siv: *const bpf_dynptr_kern, decrypt: bool) -> c_int {
    if __bpf_dynptr_is_rdonly(dst) { return -EINVAL; }
    let siv_len = if !siv.is_null() { __bpf_dynptr_size(siv) } else { 0 };
    let src_len = __bpf_dynptr_size(src);
    let dst_len = __bpf_dynptr_size(dst);
    if src_len == 0 || dst_len == 0 || src_len > dst_len || siv_len != (*ctx).siv_len { return -EINVAL; }
    let psrc = __bpf_dynptr_data(src, src_len); if psrc.is_null() { return -EINVAL; }
    let pdst = __bpf_dynptr_data_rw(dst, dst_len); if pdst.is_null() { return -EINVAL; }
    let piv = if siv_len != 0 { __bpf_dynptr_data_rw(siv, siv_len) } else { core::ptr::null_mut() };
    if siv_len != 0 && piv.is_null() { return -EINVAL; }
    if decrypt { ((*(*ctx).r#type).decrypt)((*ctx).tfm, psrc, pdst, src_len, piv) } else { ((*(*ctx).r#type).encrypt)((*ctx).tfm, psrc, pdst, src_len, piv) }
}

pub unsafe fn bpf_crypto_decrypt(ctx: *mut bpf_crypto_ctx, src: *const bpf_dynptr, dst: *const bpf_dynptr, siv__nullable: *const bpf_dynptr) -> c_int { bpf_crypto_crypt(ctx, src as *const bpf_dynptr_kern, dst as *const bpf_dynptr_kern, siv__nullable as *const bpf_dynptr_kern, true) }
pub unsafe fn bpf_crypto_encrypt(ctx: *mut bpf_crypto_ctx, src: *const bpf_dynptr, dst: *const bpf_dynptr, siv__nullable: *const bpf_dynptr) -> c_int { bpf_crypto_crypt(ctx, src as *const bpf_dynptr_kern, dst as *const bpf_dynptr_kern, siv__nullable as *const bpf_dynptr_kern, false) }

// BTF_KFUNCS/BTF_ID registration metadata and late_initcall are provided by the kernel translation environment.
static crypt_init_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: THIS_MODULE, set: &crypt_init_kfunc_btf_ids };
static crypt_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: THIS_MODULE, set: &crypt_kfunc_btf_ids };

unsafe fn crypto_kfunc_init() -> c_int {
    let bpf_crypto_dtors = [btf_id_dtor_kfunc { btf_id: bpf_crypto_dtor_ids[0], kfunc_btf_id: bpf_crypto_dtor_ids[1] }];
    let mut ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &crypt_kfunc_set);
    ret = if ret != 0 { ret } else { register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_ACT, &crypt_kfunc_set) };
    ret = if ret != 0 { ret } else { register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, &crypt_kfunc_set) };
    ret = if ret != 0 { ret } else { register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &crypt_init_kfunc_set) };
    if ret != 0 { ret } else { register_btf_id_dtor_kfuncs(bpf_crypto_dtors.as_ptr(), bpf_crypto_dtors.len(), THIS_MODULE) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
