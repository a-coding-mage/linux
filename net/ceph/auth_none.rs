// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/ceph translation unit:
// linux/ceph/ceph_debug.h, linux/err.h, linux/module.h, linux/random.h,
// linux/slab.h, linux/ceph/decode.h, linux/ceph/auth.h, and auth_none.h.

unsafe fn reset(ac: *mut ceph_auth_client) {
    let xi = (*ac).private as *mut ceph_auth_none_info;

    (*xi).starting = true;
}

unsafe fn destroy(ac: *mut ceph_auth_client) {
    kfree((*ac).private);
    (*ac).private = core::ptr::null_mut();
}

unsafe fn is_authenticated(ac: *mut ceph_auth_client) -> i32 {
    let xi = (*ac).private as *mut ceph_auth_none_info;

    (!(*xi).starting) as i32
}

unsafe fn should_authenticate(ac: *mut ceph_auth_client) -> i32 {
    let xi = (*ac).private as *mut ceph_auth_none_info;

    (*xi).starting as i32
}

unsafe fn ceph_auth_none_build_authorizer(
    ac: *mut ceph_auth_client,
    au: *mut ceph_none_authorizer,
) -> i32 {
    let mut p = (*au).buf.as_mut_ptr() as *mut core::ffi::c_void;
    let end = p.add(core::mem::size_of_val(&(*au).buf));
    let ret: i32;

    if !ceph_encode_8_safe(&mut p, end, 1) {
        return -ERANGE;
    }
    ret = ceph_auth_entity_name_encode((*ac).name, &mut p, end);
    if ret < 0 {
        return ret;
    }

    if !ceph_encode_64_safe(&mut p, end, (*ac).global_id) {
        return -ERANGE;
    }
    (*au).buf_len = p.offset_from((*au).buf.as_mut_ptr() as *mut core::ffi::c_void) as usize;
    dout!("%s built authorizer len %d\n", "ceph_auth_none_build_authorizer", (*au).buf_len);
    return 0;
}

unsafe fn build_request(
    _ac: *mut ceph_auth_client,
    _buf: *mut core::ffi::c_void,
    _end: *mut core::ffi::c_void,
) -> i32 {
    0
}

/*
 * the generic auth code decode the global_id, and we carry no actual
 * authenticate state, so nothing happens here.
 */
unsafe fn handle_reply(
    ac: *mut ceph_auth_client,
    global_id: u64,
    _buf: *mut core::ffi::c_void,
    _end: *mut core::ffi::c_void,
    _session_key: *mut u8,
    _session_key_len: *mut i32,
    _con_secret: *mut u8,
    _con_secret_len: *mut i32,
) -> i32 {
    let xi = (*ac).private as *mut ceph_auth_none_info;

    (*xi).starting = false;
    ceph_auth_set_global_id(ac, global_id);
    0
}

unsafe fn ceph_auth_none_destroy_authorizer(a: *mut ceph_authorizer) {
    kfree(a as *mut core::ffi::c_void);
}

/*
 * build an 'authorizer' with our entity_name and global_id.  it is
 * identical for all services we connect to.
 */
unsafe fn ceph_auth_none_create_authorizer(
    ac: *mut ceph_auth_client,
    _peer_type: i32,
    auth: *mut ceph_auth_handshake,
) -> i32 {
    let au = kmalloc_obj::<ceph_none_authorizer>(GFP_NOFS);
    if au.is_null() {
        return -ENOMEM;
    }

    (*au).base.destroy = Some(ceph_auth_none_destroy_authorizer);

    let ret = ceph_auth_none_build_authorizer(ac, au);
    if ret != 0 {
        kfree(au as *mut core::ffi::c_void);
        return ret;
    }

    (*auth).authorizer = au as *mut ceph_authorizer;
    (*auth).authorizer_buf = (*au).buf.as_mut_ptr();
    (*auth).authorizer_buf_len = (*au).buf_len;
    (*auth).authorizer_reply_buf = core::ptr::null_mut();
    (*auth).authorizer_reply_buf_len = 0;

    0
}

static ceph_auth_none_ops: ceph_auth_client_ops = ceph_auth_client_ops {
    reset: Some(reset),
    destroy: Some(destroy),
    is_authenticated: Some(is_authenticated),
    should_authenticate: Some(should_authenticate),
    build_request: Some(build_request),
    handle_reply: Some(handle_reply),
    create_authorizer: Some(ceph_auth_none_create_authorizer),
};

pub unsafe fn ceph_auth_none_init(ac: *mut ceph_auth_client) -> i32 {
    dout!("ceph_auth_none_init %p\n", ac);
    let xi = kzalloc_obj::<ceph_auth_none_info>(GFP_NOFS);
    if xi.is_null() {
        return -ENOMEM;
    }

    (*xi).starting = true;

    (*ac).protocol = CEPH_AUTH_NONE;
    (*ac).private = xi as *mut core::ffi::c_void;
    (*ac).ops = &ceph_auth_none_ops;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
