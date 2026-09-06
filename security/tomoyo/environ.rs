// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/environ.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

// C source included "common.h"; translated items below expect the common
// TOMOYO types, constants, helpers, and container_of-style support to be
// supplied by the surrounding crate.
use crate::common::*;

/**
 * tomoyo_check_env_acl - Check permission for environment variable's name.
 *
 * @r:   Pointer to "struct tomoyo_request_info".
 * @ptr: Pointer to "struct tomoyo_acl_info".
 *
 * Returns true if granted, false otherwise.
 */
unsafe fn tomoyo_check_env_acl(
    r: *mut tomoyo_request_info,
    ptr: *const tomoyo_acl_info,
) -> bool {
    let acl: *const tomoyo_env_acl = container_of!(ptr, tomoyo_env_acl, head);

    tomoyo_path_matches_pattern((*r).param.environ.name, (*acl).env)
}

/**
 * tomoyo_audit_env_log - Audit environment variable name log.
 *
 * @r: Pointer to "struct tomoyo_request_info".
 *
 * Returns 0 on success, negative value otherwise.
 */
unsafe fn tomoyo_audit_env_log(r: *mut tomoyo_request_info) -> libc::c_int {
    /*
     * Original C annotation:
     * __must_hold_shared(&tomoyo_ss)
     */
    tomoyo_supervisor(
        r,
        c"misc env %s\n".as_ptr(),
        (*(*r).param.environ.name).name,
    )
}

/**
 * tomoyo_env_perm - Check permission for environment variable's name.
 *
 * @r:   Pointer to "struct tomoyo_request_info".
 * @env: The name of environment variable.
 *
 * Returns 0 on success, negative value otherwise.
 *
 * Caller holds tomoyo_read_lock().
 */
pub unsafe fn tomoyo_env_perm(
    r: *mut tomoyo_request_info,
    env: *const libc::c_char,
) -> libc::c_int {
    let mut environ: tomoyo_path_info = core::mem::zeroed();
    let mut error: libc::c_int;

    if env.is_null() || *env == 0 {
        return 0;
    }
    environ.name = env;
    tomoyo_fill_path_info(&mut environ);
    (*r).param_type = TOMOYO_TYPE_ENV_ACL;
    (*r).param.environ.name = &mut environ;
    loop {
        tomoyo_check_acl(r, Some(tomoyo_check_env_acl));
        error = tomoyo_audit_env_log(r);
        if error != TOMOYO_RETRY_REQUEST {
            break;
        }
    }
    error
}

/**
 * tomoyo_same_env_acl - Check for duplicated "struct tomoyo_env_acl" entry.
 *
 * @a: Pointer to "struct tomoyo_acl_info".
 * @b: Pointer to "struct tomoyo_acl_info".
 *
 * Returns true if @a == @b, false otherwise.
 */
unsafe fn tomoyo_same_env_acl(
    a: *const tomoyo_acl_info,
    b: *const tomoyo_acl_info,
) -> bool {
    let p1: *const tomoyo_env_acl = container_of!(a, tomoyo_env_acl, head);
    let p2: *const tomoyo_env_acl = container_of!(b, tomoyo_env_acl, head);

    (*p1).env == (*p2).env
}

/**
 * tomoyo_write_env - Write "struct tomoyo_env_acl" list.
 *
 * @param: Pointer to "struct tomoyo_acl_param".
 *
 * Returns 0 on success, negative value otherwise.
 *
 * Caller holds tomoyo_read_lock().
 */
unsafe fn tomoyo_write_env(param: *mut tomoyo_acl_param) -> libc::c_int {
    let mut e: tomoyo_env_acl = core::mem::zeroed();
    e.head.type_ = TOMOYO_TYPE_ENV_ACL;
    let mut error: libc::c_int = -libc::ENOMEM;
    let data: *const libc::c_char = tomoyo_read_token(param);

    if !tomoyo_correct_word(data) || !libc::strchr(data, b'=' as libc::c_int).is_null() {
        return -libc::EINVAL;
    }
    e.env = tomoyo_get_name(data);
    if e.env.is_null() {
        return error;
    }
    error = tomoyo_update_domain(
        &mut e.head,
        core::mem::size_of_val(&e),
        param,
        Some(tomoyo_same_env_acl),
        None,
    );
    tomoyo_put_name(e.env);
    error
}

/**
 * tomoyo_write_misc - Update environment variable list.
 *
 * @param: Pointer to "struct tomoyo_acl_param".
 *
 * Returns 0 on success, negative value otherwise.
 */
pub unsafe fn tomoyo_write_misc(param: *mut tomoyo_acl_param) -> libc::c_int {
    if tomoyo_str_starts(&mut (*param).data, c"env ".as_ptr()) {
        return tomoyo_write_env(param);
    }
    -libc::EINVAL
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
