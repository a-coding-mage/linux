// SPDX-License-Identifier: GPL-2.0
// Translated from auth.c. Kernel and Ceph dependencies are supplied externally.

static mut SUPPORTED_PROTOCOLS: [u32; 2] = [CEPH_AUTH_NONE, CEPH_AUTH_CEPHX];

unsafe fn init_protocol(ac: *mut ceph_auth_client, proto: i32) -> i32 {
    dout!("{} proto {}\n", "init_protocol", proto);
    match proto {
        CEPH_AUTH_NONE => ceph_auth_none_init(ac),
        CEPH_AUTH_CEPHX => ceph_x_init(ac),
        _ => { pr_err!("bad auth protocol {}\n", proto); -EINVAL }
    }
}

pub unsafe fn ceph_auth_set_global_id(ac: *mut ceph_auth_client, global_id: u64) {
    dout!("{} global_id {}\n", "ceph_auth_set_global_id", global_id);
    if global_id == 0 { pr_err!("got zero global_id\n"); }
    if (*ac).global_id != 0 && global_id != (*ac).global_id {
        pr_err!("global_id changed from {} to {}\n", (*ac).global_id, global_id);
    }
    (*ac).global_id = global_id;
}

pub unsafe fn ceph_auth_init(name: *const i8, key: *const ceph_crypto_key, con_modes: *const i32) -> *mut ceph_auth_client {
    let ac = kzalloc_obj::<ceph_auth_client>();
    if ac.is_null() { return ERR_PTR(-ENOMEM); }
    mutex_init(&mut (*ac).mutex);
    (*ac).negotiating = true;
    (*ac).name = if !name.is_null() { name } else { CEPH_AUTH_NAME_DEFAULT };
    (*ac).key = key;
    (*ac).preferred_mode = *con_modes;
    (*ac).fallback_mode = *con_modes.add(1);
    dout!("{} name '{}' preferred_mode {} fallback_mode {}\n", "ceph_auth_init", (*ac).name, (*ac).preferred_mode, (*ac).fallback_mode);
    ac
}

pub unsafe fn ceph_auth_destroy(ac: *mut ceph_auth_client) {
    dout!("auth_destroy {:?}\n", ac);
    if !(*ac).ops.is_null() { ((*(*ac).ops).destroy)(ac); }
    kfree(ac);
}

pub unsafe fn ceph_auth_reset(ac: *mut ceph_auth_client) {
    mutex_lock(&mut (*ac).mutex);
    dout!("auth_reset {:?}\n", ac);
    if !(*ac).ops.is_null() && !(*ac).negotiating { ((*(*ac).ops).reset)(ac); }
    (*ac).negotiating = true;
    mutex_unlock(&mut (*ac).mutex);
}

pub unsafe fn ceph_auth_entity_name_encode(name: *const i8, p: *mut *mut u8, end: *mut u8) -> i32 {
    let len = strlen(name);
    if p.read().add(2 * size_of::<u32>() + len) > end { return -ERANGE; }
    ceph_encode_32(p, CEPH_ENTITY_TYPE_CLIENT);
    ceph_encode_32(p, len as u32);
    ceph_encode_copy(p, name as *const _, len);
    0
}

pub unsafe fn ceph_auth_build_hello(ac: *mut ceph_auth_client, buf: *mut u8, len: usize) -> i32 {
    let monhdr = buf as *mut ceph_mon_request_header;
    let mut p = monhdr.add(1) as *mut u8;
    let end = buf.add(len);
    let mut ret;
    mutex_lock(&mut (*ac).mutex);
    dout!("auth_build_hello\n");
    (*monhdr).have_version = 0;
    (*monhdr).session_mon = cpu_to_le16(-1i16 as u16);
    (*monhdr).session_mon_tid = 0;
    ceph_encode_32(&mut p, CEPH_AUTH_UNKNOWN);
    let lenp = p; p = p.add(size_of::<u32>());
    if p.add(1 + size_of::<u32>()) > end { ret = -ERANGE; mutex_unlock(&mut (*ac).mutex); return ret; }
    ceph_encode_8(&mut p, 1);
    let num = SUPPORTED_PROTOCOLS.len();
    ceph_encode_32(&mut p, num as u32);
    if p.add(num * size_of::<u32>()) > end { ret = -ERANGE; mutex_unlock(&mut (*ac).mutex); return ret; }
    for i in 0..num { ceph_encode_32(&mut p, SUPPORTED_PROTOCOLS[i]); }
    ret = ceph_auth_entity_name_encode((*ac).name, &mut p, end);
    if ret < 0 { mutex_unlock(&mut (*ac).mutex); return ret; }
    if p.add(size_of::<u64>()) > end { ret = -ERANGE; mutex_unlock(&mut (*ac).mutex); return ret; }
    ceph_encode_64(&mut p, (*ac).global_id);
    ceph_encode_32(&mut (lenp as *mut *mut u8), (p as usize - lenp as usize - size_of::<u32>()) as u32);
    ret = (p as usize - buf as usize) as i32;
    mutex_unlock(&mut (*ac).mutex); ret
}

unsafe fn build_request(ac: *mut ceph_auth_client, add_header: bool, buf: *mut u8, buf_len: i32) -> i32 {
    let end = buf.add(buf_len as usize); let mut p = buf;
    if add_header {
        if p.add(8+2+8+4) > end { return -ERANGE; }
        ceph_encode_64(&mut p, 0); ceph_encode_16(&mut p, -1i16 as u16); ceph_encode_64(&mut p, 0); ceph_encode_32(&mut p, (*ac).protocol as u32);
    }
    if p.add(4) > end { return -ERANGE; }
    let ret = ((*(*ac).ops).build_request)(ac, p.add(4), end);
    if ret < 0 { pr_err!("auth protocol '{}' building request failed: {}\n", ceph_auth_proto_name((*ac).protocol), ret); return ret; }
    dout!(" built request {} bytes\n", ret);
    ceph_encode_32(&mut p, ret as u32); (p as usize + ret as usize - buf as usize) as i32
}

pub unsafe fn ceph_auth_is_authenticated(ac: *mut ceph_auth_client) -> i32 {
    mutex_lock(&mut (*ac).mutex); let ret = if !(*ac).ops.is_null() { ((*(*ac).ops).is_authenticated)(ac) } else { 0 }; mutex_unlock(&mut (*ac).mutex); ret
}

unsafe fn contains(arr: *const i32, cnt: i32, val: i32) -> bool {
    for i in 0..cnt { if *arr.add(i as usize) == val { return true; } } false
}

unsafe fn encode_con_modes(p: *mut *mut u8, end: *mut u8, pref_mode: i32, fallb_mode: i32) -> i32 {
    WARN_ON!(pref_mode == CEPH_CON_MODE_UNKNOWN);
    let n = if fallb_mode != CEPH_CON_MODE_UNKNOWN { 2 } else { 1 };
    if p.read().add((n + 1) as usize * 4) > end { return -ERANGE; }
    ceph_encode_32(p, n); ceph_encode_32(p, pref_mode); if n == 2 { ceph_encode_32(p, fallb_mode); } 0
}

// The remaining exported protocol handlers retain the C ABI and delegate through
// ceph_auth_client::ops, whose definitions are provided by the surrounding Ceph bindings.
pub unsafe fn ceph_auth_destroy_authorizer(a: *mut ceph_authorizer) { ((*a).destroy)(a); }

pub unsafe fn ceph_auth_add_authorizer_challenge(ac: *mut ceph_auth_client, a: *mut ceph_authorizer, b: *mut u8, n: i32) -> i32 {
    mutex_lock(&mut (*ac).mutex);
    let ret = if !(*ac).ops.is_null() && ((*(*ac).ops).add_authorizer_challenge).is_some() { ((*(*ac).ops).add_authorizer_challenge.unwrap())(ac, a, b, n) } else { 0 };
    mutex_unlock(&mut (*ac).mutex); ret
}

pub unsafe fn ceph_auth_verify_authorizer_reply(ac: *mut ceph_auth_client, a: *mut ceph_authorizer, reply: *mut u8, reply_len: i32, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32 {
    mutex_lock(&mut (*ac).mutex);
    let ret = if !(*ac).ops.is_null() && ((*(*ac).ops).verify_authorizer_reply).is_some() { ((*(*ac).ops).verify_authorizer_reply.unwrap())(ac, a, reply, reply_len, session_key, session_key_len, con_secret, con_secret_len) } else { 0 };
    mutex_unlock(&mut (*ac).mutex); ret
}

pub unsafe fn ceph_auth_invalidate_authorizer(ac: *mut ceph_auth_client, peer_type: i32) {
    mutex_lock(&mut (*ac).mutex);
    if !(*ac).ops.is_null() && ((*(*ac).ops).invalidate_authorizer).is_some() { ((*(*ac).ops).invalidate_authorizer.unwrap())(ac, peer_type); }
    mutex_unlock(&mut (*ac).mutex);
}

pub unsafe fn ceph_auth_get_request(ac: *mut ceph_auth_client, buf: *mut u8, buf_len: i32) -> i32 {
    let proto = if !(*ac).key.is_null() { CEPH_AUTH_CEPHX } else { CEPH_AUTH_NONE };
    mutex_lock(&mut (*ac).mutex);
    let mut p = buf; let end = buf.add(buf_len as usize);
    let ret = if (*ac).protocol == CEPH_AUTH_UNKNOWN { init_protocol(ac, proto) } else { WARN_ON!((*ac).protocol != proto); ((*(*ac).ops).reset)(ac); 0 };
    if ret != 0 { mutex_unlock(&mut (*ac).mutex); return ret; }
    if p.add(4) > end { mutex_unlock(&mut (*ac).mutex); return -ERANGE; }
    ceph_encode_32(&mut p, (*ac).protocol as u32);
    let r = encode_con_modes(&mut p, end, (*ac).preferred_mode, (*ac).fallback_mode); if r != 0 { mutex_unlock(&mut (*ac).mutex); return r; }
    let lenp = p; p = p.add(4); if p.add(1) > end { mutex_unlock(&mut (*ac).mutex); return -ERANGE; }
    ceph_encode_8(&mut p, CEPH_AUTH_MODE_MON as u8);
    let r = ceph_auth_entity_name_encode((*ac).name, &mut p, end); if r != 0 { mutex_unlock(&mut (*ac).mutex); return r; }
    if p.add(8) > end { mutex_unlock(&mut (*ac).mutex); return -ERANGE; }
    ceph_encode_64(&mut p, (*ac).global_id); ceph_encode_32(&mut (lenp as *mut *mut u8), (p as usize - lenp as usize - 4) as u32);
    let out = (p as usize - buf as usize) as i32; mutex_unlock(&mut (*ac).mutex); out
}

pub unsafe fn ceph_auth_handle_reply_more(ac: *mut ceph_auth_client, reply: *mut u8, reply_len: i32, buf: *mut u8, buf_len: i32) -> i32 {
    mutex_lock(&mut (*ac).mutex); let ret = ((*(*ac).ops).handle_reply)(ac, 0, reply, reply.add(reply_len as usize), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let out = if ret == -EAGAIN { build_request(ac, false, buf, buf_len) } else { WARN_ON!(ret >= 0); ret }; mutex_unlock(&mut (*ac).mutex); out
}

pub unsafe fn ceph_auth_handle_reply_done(ac: *mut ceph_auth_client, global_id: u64, reply: *mut u8, reply_len: i32, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32 {
    mutex_lock(&mut (*ac).mutex); let ret = ((*(*ac).ops).handle_reply)(ac, global_id, reply, reply.add(reply_len as usize), session_key, session_key_len, con_secret, con_secret_len); WARN_ON!(ret == -EAGAIN || ret > 0); mutex_unlock(&mut (*ac).mutex); ret
}

pub unsafe fn ceph_auth_handle_svc_reply_done(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, reply: *mut u8, reply_len: i32, sk: *mut u8, skl: *mut i32, cs: *mut u8, csl: *mut i32) -> i32 { ceph_auth_verify_authorizer_reply(ac, (*auth).authorizer, reply, reply_len, sk, skl, cs, csl) }

pub unsafe fn ceph_build_auth(ac: *mut ceph_auth_client, b: *mut u8, n: usize) -> i32 { mutex_lock(&mut (*ac).mutex); let r = if ((*(*ac).ops).should_authenticate)(ac) != 0 { build_request(ac, true, b, n as i32) } else { 0 }; mutex_unlock(&mut (*ac).mutex); r }

pub unsafe fn ceph_auth_get_authorizer(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, peer: i32, b: *mut u8, bl: *mut i32) -> i32 {
    let mut proto = 0; let mut pref = 0; let mut fall = 0;
    let r = __ceph_auth_get_authorizer(ac, auth, peer, true, &mut proto, &mut pref, &mut fall); if r != 0 { return r; }
    let mut p = b; let end = b.add(*bl as usize); if p.add(4) > end { return -ERANGE; } ceph_encode_32(&mut p, proto as u32);
    let r = encode_con_modes(&mut p, end, pref, fall); if r != 0 { return r; } if p.add(4) > end { return -ERANGE; } ceph_encode_32(&mut p, (*auth).authorizer_buf_len); *bl = (p as usize - b as usize) as i32; 0
}

pub unsafe fn __ceph_auth_get_authorizer(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, peer: i32, force: bool, proto: *mut i32, pref: *mut i32, fall: *mut i32) -> i32 {
    mutex_lock(&mut (*ac).mutex); if force && !(*auth).authorizer.is_null() { ceph_auth_destroy_authorizer((*auth).authorizer); (*auth).authorizer = core::ptr::null_mut(); }
    let r = if (*auth).authorizer.is_null() { ((*(*ac).ops).create_authorizer)(ac, peer, auth) } else if ((*(*ac).ops).update_authorizer).is_some() { ((*(*ac).ops).update_authorizer.unwrap())(ac, peer, auth) } else { 0 };
    if r == 0 { *proto = (*ac).protocol; *pref = (*ac).preferred_mode; *fall = (*ac).fallback_mode; } mutex_unlock(&mut (*ac).mutex); r
}

pub unsafe fn ceph_auth_handle_svc_reply_more(ac: *mut ceph_auth_client, auth: *mut ceph_auth_handshake, reply: *mut u8, n: i32, b: *mut u8, bl: *mut i32) -> i32 { let r = ceph_auth_add_authorizer_challenge(ac, (*auth).authorizer, reply, n); if r != 0 { return r; } let mut p=b; let end=b.add(*bl as usize); if p.add(4)>end{return -ERANGE;} ceph_encode_32(&mut p,(*auth).authorizer_buf_len); *bl=(p as usize-b as usize) as i32; 0 }

pub unsafe fn ceph_auth_handle_bad_method(ac: *mut ceph_auth_client, used: i32, result: i32, ap: *const i32, pc: i32, am: *const i32, mc: i32) -> bool { mutex_lock(&mut (*ac).mutex); WARN_ON!(used != (*ac).protocol); let ok = result != -EOPNOTSUPP || (contains(ap,pc,(*ac).protocol) && (contains(am,mc,(*ac).preferred_mode) || ((*ac).fallback_mode == CEPH_CON_MODE_UNKNOWN || contains(am,mc,(*ac).fallback_mode)))); mutex_unlock(&mut (*ac).mutex); ok }

pub unsafe fn ceph_auth_handle_bad_authorizer(ac: *mut ceph_auth_client, _peer: i32, used: i32, result: i32, ap: *const i32, pc: i32, am: *const i32, mc: i32) -> bool { ceph_auth_handle_bad_method(ac, used, result, ap, pc, am, mc) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
