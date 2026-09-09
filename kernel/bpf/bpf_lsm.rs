// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Google LLC.
 *
 * Rust translation of bpf_lsm.c. Kernel-provided types, constants, macros,
 * functions, and generated BTF sets are intentionally left as dependencies.
 */

/* The C LSM_HOOK/BTF_SET definitions expand from linux/lsm_hook_defs.h.
 * Their generated declarations and set membership are supplied by the kernel
 * translation environment.
 */

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe extern "C" fn bpf_lsm_find_cgroup_shim(
    prog: *const bpf_prog,
    bpf_func: *mut bpf_func_t,
) {
    let mut args: *const btf_param = core::ptr::null();

    if btf_type_vlen((*(*prog).aux).attach_func_proto) < 1
        || btf_id_set_contains(&bpf_lsm_current_hooks, (*(*prog).aux).attach_btf_id)
    {
        *bpf_func = __cgroup_bpf_run_lsm_current;
        return;
    }

    #[cfg(CONFIG_NET)]
    {
        args = btf_params((*(*prog).aux).attach_func_proto);
        if (*args).type_ == btf_sock_ids[BTF_SOCK_TYPE_SOCKET as usize] {
            *bpf_func = __cgroup_bpf_run_lsm_socket;
        } else if (*args).type_ == btf_sock_ids[BTF_SOCK_TYPE_SOCK as usize] {
            *bpf_func = __cgroup_bpf_run_lsm_sock;
        } else {
            *bpf_func = __cgroup_bpf_run_lsm_current;
        }
    }
    #[cfg(not(CONFIG_NET))]
    {
        *bpf_func = __cgroup_bpf_run_lsm_current;
    }
}

pub unsafe extern "C" fn bpf_lsm_verify_prog(
    vlog: *mut bpf_verifier_log,
    prog: *const bpf_prog,
) -> i32 {
    let btf_id: u32 = (*(*prog).aux).attach_btf_id;
    let func_name: *const core::ffi::c_char = (*(*prog).aux).attach_func_name;

    if !(*prog).gpl_compatible {
        bpf_log(vlog, c"LSM programs must have a GPL compatible license\n".as_ptr());
        return -EINVAL;
    }
    if btf_id_set_contains(&bpf_lsm_disabled_hooks, btf_id) {
        bpf_log(vlog, c"attach_btf_id %u points to disabled hook %s\n".as_ptr(), btf_id, func_name);
        return -EINVAL;
    }
    if !btf_id_set_contains(&bpf_lsm_hooks, btf_id) {
        bpf_log(vlog, c"attach_btf_id %u points to wrong type name %s\n".as_ptr(), btf_id, func_name);
        return -EINVAL;
    }
    0
}

pub const BPF_F_BRPM_OPTS_MASK: u64 = BPF_F_BPRM_SECUREEXEC as u64;

pub unsafe extern "C" fn bpf_bprm_opts_set(bprm: *mut linux_binprm, flags: u64) -> i64 {
    if flags & !BPF_F_BRPM_OPTS_MASK != 0 {
        return -EINVAL as i64;
    }
    (*bprm).secureexec = flags & BPF_F_BPRM_SECUREEXEC as u64;
    0
}

pub unsafe extern "C" fn bpf_ima_inode_hash(
    inode: *mut inode, dst: *mut core::ffi::c_void, size: u32,
) -> i64 { ima_inode_hash(inode, dst, size) }

pub unsafe extern "C" fn bpf_ima_inode_hash_allowed(prog: *const bpf_prog) -> bool {
    bpf_lsm_is_sleepable_hook((*(*prog).aux).attach_btf_id)
}

pub unsafe extern "C" fn bpf_ima_file_hash(
    file: *mut file, dst: *mut core::ffi::c_void, size: u32,
) -> i64 { ima_file_hash(file, dst, size) }

pub unsafe extern "C" fn bpf_get_attach_cookie(ctx: *mut core::ffi::c_void) -> u64 {
    let run_ctx = container_of((*current).bpf_ctx, bpf_trace_run_ctx, run_ctx);
    (*run_ctx).bpf_cookie
}

pub unsafe extern "C" fn bpf_lsm_func_proto(
    func_id: bpf_func_id, prog: *const bpf_prog,
) -> *const bpf_func_proto {
    let mut func_proto: *const bpf_func_proto;
    if (*prog).expected_attach_type == BPF_LSM_CGROUP {
        func_proto = cgroup_common_func_proto(func_id, prog);
        if !func_proto.is_null() { return func_proto; }
    }
    match func_id {
        BPF_FUNC_inode_storage_get => &bpf_inode_storage_get_proto,
        BPF_FUNC_inode_storage_delete => &bpf_inode_storage_delete_proto,
        #[cfg(CONFIG_NET)]
        BPF_FUNC_sk_storage_get => &bpf_sk_storage_get_proto,
        #[cfg(CONFIG_NET)]
        BPF_FUNC_sk_storage_delete => &bpf_sk_storage_delete_proto,
        BPF_FUNC_spin_lock => &bpf_spin_lock_proto,
        BPF_FUNC_spin_unlock => &bpf_spin_unlock_proto,
        BPF_FUNC_bprm_opts_set => &bpf_bprm_opts_set_proto,
        BPF_FUNC_ima_inode_hash => &bpf_ima_inode_hash_proto,
        BPF_FUNC_ima_file_hash => &bpf_ima_file_hash_proto,
        BPF_FUNC_get_attach_cookie => if bpf_prog_has_trampoline(prog) { &bpf_get_attach_cookie_proto } else { core::ptr::null() },
        #[cfg(CONFIG_NET)]
        BPF_FUNC_setsockopt => {
            if (*prog).expected_attach_type != BPF_LSM_CGROUP { return core::ptr::null(); }
            if btf_id_set_contains(&bpf_lsm_locked_sockopt_hooks, (*(*prog).aux).attach_btf_id) { &bpf_sk_setsockopt_proto }
            else if btf_id_set_contains(&bpf_lsm_unlocked_sockopt_hooks, (*(*prog).aux).attach_btf_id) { &bpf_unlocked_sk_setsockopt_proto }
            else { core::ptr::null() }
        },
        #[cfg(CONFIG_NET)]
        BPF_FUNC_getsockopt => {
            if (*prog).expected_attach_type != BPF_LSM_CGROUP { return core::ptr::null(); }
            if btf_id_set_contains(&bpf_lsm_locked_sockopt_hooks, (*(*prog).aux).attach_btf_id) { &bpf_sk_getsockopt_proto }
            else if btf_id_set_contains(&bpf_lsm_unlocked_sockopt_hooks, (*(*prog).aux).attach_btf_id) { &bpf_unlocked_sk_getsockopt_proto }
            else { core::ptr::null() }
        },
        _ => tracing_prog_func_proto(func_id, prog),
    }
}

/* BTF sets generated by the corresponding C macro expansions. */
pub unsafe extern "C" fn bpf_lsm_is_sleepable_hook(btf_id: u32) -> bool {
    btf_id_set_contains(&sleepable_lsm_hooks, btf_id)
}
pub unsafe extern "C" fn bpf_lsm_is_trusted(prog: *const bpf_prog) -> bool {
    !btf_id_set_contains(&untrusted_lsm_hooks, (*(*prog).aux).attach_btf_id)
}

pub static lsm_prog_ops: bpf_prog_ops = bpf_prog_ops {};
pub static lsm_verifier_ops: bpf_verifier_ops = bpf_verifier_ops {
    get_func_proto: Some(bpf_lsm_func_proto),
    is_valid_access: Some(btf_ctx_access),
};

pub unsafe extern "C" fn bpf_lsm_hook_returns_errno(btf_id: u32) -> bool {
    if btf_id_set_contains(&bool_lsm_hooks, btf_id) { return false; }
    if btf_id_set_contains(&void_lsm_hooks, btf_id) { return false; }
    true
}

pub unsafe extern "C" fn bpf_lsm_get_retval_range(
    prog: *const bpf_prog, retval_range: *mut bpf_retval_range,
) -> i32 {
    if (*(*prog).aux).attach_func_proto.type_ == 0 { return -EINVAL; }
    if btf_id_set_contains(&bool_lsm_hooks, (*(*prog).aux).attach_btf_id) {
        (*retval_range).minval = 0;
        (*retval_range).maxval = 1;
    } else {
        (*retval_range).minval = -MAX_ERRNO;
        (*retval_range).maxval = 0;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
