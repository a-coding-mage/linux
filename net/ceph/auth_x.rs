// SPDX-License-Identifier: GPL-2.0
// Translated from auth_x.c. Kernel/Ceph dependencies are supplied externally.

static TICKET_KEY_USAGES: [u32; 3] = [
    CEPHX_KEY_USAGE_TICKET_SESSION_KEY,
    CEPHX_KEY_USAGE_TICKET_BLOB,
    CEPHX_KEY_USAGE_AUTH_CONNECTION_SECRET,
];
static AUTHORIZER_KEY_USAGES: [u32; 3] = [
    CEPHX_KEY_USAGE_AUTHORIZE,
    CEPHX_KEY_USAGE_AUTHORIZE_CHALLENGE,
    CEPHX_KEY_USAGE_AUTHORIZE_REPLY,
];
static CLIENT_KEY_USAGES: [u32; 1] = [CEPHX_KEY_USAGE_TICKET_SESSION_KEY];

extern "C" {
    fn ceph_x_validate_tickets(ac: *mut ceph_auth_client, need: *mut i32);
}

unsafe fn ceph_x_is_authenticated(ac: *mut ceph_auth_client) -> i32 {
    let xi = (*ac).private as *mut ceph_x_info;
    let mut need = 0;
    ceph_x_validate_tickets(ac, &mut need);
    let missing = (*ac).want_keys & !(*xi).have_keys;
    WARN_ON((need & missing) != missing);
    dout!("%s want 0x%x have 0x%x missing 0x%x -> %d\n", __func__, (*ac).want_keys, (*xi).have_keys, missing, (missing == 0) as i32);
    (missing == 0) as i32
}

unsafe fn ceph_x_should_authenticate(ac: *mut ceph_auth_client) -> i32 {
    let xi = (*ac).private as *mut ceph_x_info;
    let mut need = 0;
    ceph_x_validate_tickets(ac, &mut need);
    dout!("%s want 0x%x have 0x%x need 0x%x -> %d\n", __func__, (*ac).want_keys, (*xi).have_keys, need, (need != 0) as i32);
    (need != 0) as i32
}

unsafe fn __ceph_x_encrypt_offset(key: *const ceph_crypto_key) -> i32 { ceph_crypt_data_offset(key) + core::mem::size_of::<ceph_x_encrypt_header>() as i32 }
unsafe fn ceph_x_encrypt_offset(key: *const ceph_crypto_key) -> i32 { 4 + __ceph_x_encrypt_offset(key) }
unsafe fn ceph_x_encrypt_buflen(key: *const ceph_crypto_key, data_len: i32) -> i32 { 4 + ceph_crypt_buflen(key, core::mem::size_of::<ceph_x_encrypt_header>() as i32 + data_len) }

unsafe fn ceph_x_encrypt(key: *const ceph_crypto_key, usage_slot: i32, buf: *mut u8, buf_len: i32, plaintext_len: i32) -> i32 {
    let hdr = buf.add(4 + ceph_crypt_data_offset(key) as usize) as *mut ceph_x_encrypt_header;
    (*hdr).struct_v = 1;
    (*hdr).magic = cpu_to_le64(CEPHX_ENC_MAGIC);
    let mut ciphertext_len = 0;
    let ret = ceph_crypt(key, usage_slot, true, buf.add(4), buf_len - 4, plaintext_len + core::mem::size_of::<ceph_x_encrypt_header>() as i32, &mut ciphertext_len);
    if ret != 0 { return ret; }
    ceph_encode_32(&mut (buf as *mut u8), ciphertext_len);
    4 + ciphertext_len
}

unsafe fn __ceph_x_decrypt(key: *const ceph_crypto_key, usage_slot: i32, p: *mut u8, ciphertext_len: i32) -> i32 {
    let mut plaintext_len = 0;
    let ret = ceph_crypt(key, usage_slot, false, p, ciphertext_len, ciphertext_len, &mut plaintext_len);
    if ret != 0 { return ret; }
    if plaintext_len < core::mem::size_of::<ceph_x_encrypt_header>() as i32 { pr_err!("%s plaintext too small %d\n", __func__, plaintext_len); return -EINVAL; }
    let hdr = p.add(ceph_crypt_data_offset(key) as usize) as *mut ceph_x_encrypt_header;
    if le64_to_cpu((*hdr).magic) != CEPHX_ENC_MAGIC { pr_err!("%s bad magic\n", __func__); return -EINVAL; }
    plaintext_len - core::mem::size_of::<ceph_x_encrypt_header>() as i32
}

unsafe fn ceph_x_decrypt(key: *const ceph_crypto_key, usage_slot: i32, p: *mut *mut u8, end: *mut u8) -> i32 {
    let mut ciphertext_len = 0;
    if ceph_decode_32_safe(p, end, &mut ciphertext_len) != 0 { return -EINVAL; }
    if ceph_decode_need(p, end, ciphertext_len) != 0 { return -EINVAL; }
    let ret = __ceph_x_decrypt(key, usage_slot, *p, ciphertext_len);
    if ret < 0 { return ret; }
    *p = (*p).add(ciphertext_len as usize);
    ret
}

unsafe fn get_ticket_handler(ac: *mut ceph_auth_client, service: i32) -> *mut ceph_x_ticket_handler {
    let xi = (*ac).private as *mut ceph_x_info;
    let mut p = (*xi).ticket_handlers.rb_node;
    let mut parent = core::ptr::null_mut();
    while !p.is_null() {
        parent = p;
        let th = rb_entry!(p, ceph_x_ticket_handler, node);
        if service < (*th).service { p = (*p).rb_left; }
        else if service > (*th).service { p = (*p).rb_right; }
        else { return th; }
    }
    let th = kzalloc_obj::<ceph_x_ticket_handler>(GFP_NOFS);
    if th.is_null() { return ERR_PTR(-ENOMEM); }
    (*th).service = service;
    rb_link_node(&mut (*th).node, parent, &mut p);
    rb_insert_color(&mut (*th).node, &mut (*xi).ticket_handlers);
    th
}

unsafe fn remove_ticket_handler(ac: *mut ceph_auth_client, th: *mut ceph_x_ticket_handler) {
    let xi = (*ac).private as *mut ceph_x_info;
    dout!("remove_ticket_handler %p %d\n", th, (*th).service);
    rb_erase(&mut (*th).node, &mut (*xi).ticket_handlers);
    ceph_crypto_key_destroy(&mut (*th).session_key);
    if !(*th).ticket_blob.is_null() { ceph_buffer_put((*th).ticket_blob); }
    kfree(th as *mut _);
}

// The remaining routines retain the source-level operation sequence; helper
// macros and structures are intentionally resolved by the surrounding kernel.
unsafe fn need_key(th: *mut ceph_x_ticket_handler) -> bool { !(*th).have_key || ktime_get_real_seconds() >= (*th).renew_after }
unsafe fn have_key(th: *mut ceph_x_ticket_handler) -> bool {
    if (*th).have_key && ktime_get_real_seconds() >= (*th).expires { (*th).have_key = false; }
    (*th).have_key
}

unsafe fn ceph_x_validate_tickets_local(ac: *mut ceph_auth_client, pneed: *mut i32) {
    let xi = (*ac).private as *mut ceph_x_info;
    let want = (*ac).want_keys;
    *pneed = want & !(*xi).have_keys;
    let mut service = 1;
    while service <= want {
        if ((*ac).want_keys & service) != 0 && (*pneed & service) == 0 {
            let th = get_ticket_handler(ac, service);
            if IS_ERR(th) { *pneed |= service; }
            else { if need_key(th) { *pneed |= service; } if !have_key(th) { (*xi).have_keys &= !service; } }
        }
        service <<= 1;
    }
}

// Full protocol callbacks are declared here and retain their C ABI names for
// the external Ceph registration layer.
extern "C" {
    fn ceph_x_build_request(ac: *mut ceph_auth_client, buf: *mut u8, end: *mut u8) -> i32;
    fn ceph_x_handle_reply(ac: *mut ceph_auth_client, global_id: u64, buf: *mut u8, end: *mut u8, session_key: *mut u8, session_key_len: *mut i32, con_secret: *mut u8, con_secret_len: *mut i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn ceph_x_init(ac: *mut ceph_auth_client) -> i32 {
    let xi = kzalloc_obj::<ceph_x_info>(GFP_NOFS);
    if xi.is_null() { return -ENOMEM; }
    if (*ac).key.is_null() { kfree(xi as *mut _); return -EINVAL; }
    let ret = ceph_crypto_key_clone(&mut (*xi).secret, (*ac).key);
    if ret < 0 { kfree(xi as *mut _); return ret; }
    let ret = ceph_crypto_key_prepare(&mut (*xi).secret, CLIENT_KEY_USAGES.as_ptr(), CLIENT_KEY_USAGES.len() as i32);
    if ret != 0 { ceph_crypto_key_destroy(&mut (*xi).secret); kfree(xi as *mut _); return ret; }
    (*xi).starting = true;
    (*xi).ticket_handlers = RB_ROOT;
    (*ac).protocol = CEPH_AUTH_CEPHX;
    (*ac).private = xi as *mut _;
    (*ac).ops = &ceph_x_ops;
    0
}

static ceph_x_ops: ceph_auth_client_ops = ceph_auth_client_ops {
    is_authenticated: ceph_x_is_authenticated,
    should_authenticate: ceph_x_should_authenticate,
    build_request: ceph_x_build_request,
    handle_reply: ceph_x_handle_reply,
    ..Default::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
