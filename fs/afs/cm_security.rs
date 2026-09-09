// SPDX-License-Identifier: GPL-2.0-or-later
/* Cache manager security.
 *
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

const RXGK_SERVER_ENC_TOKEN: u32 = 1036;

#[inline]
fn xdr_round_up(x: usize) -> usize { (x + 3) & !3 }
#[inline]
fn xdr_len_object(x: usize) -> usize { 4 + xdr_round_up(x) }

/* Respond to an RxGK challenge, adding appdata. */
unsafe fn afs_respond_to_challenge(challenge: *mut sk_buff) -> i32 {
    #[cfg(feature = "CONFIG_RXGK")]
    let mut appdata: krb5_buffer = core::mem::zeroed();
    let mut peer: *mut rxrpc_peer = core::ptr::null_mut();
    let mut peer_data: c_ulong = 0;
    let mut service_id: u16 = 0;
    let mut security_index: u8 = 0;
    rxrpc_kernel_query_challenge(challenge, &mut peer, &mut peer_data, &mut service_id, &mut security_index);
    _enter!("%u,%u", service_id, security_index);
    match service_id {
        FS_SERVICE | VL_SERVICE | YFS_FS_SERVICE | YFS_VL_SERVICE => {}
        _ => {
            pr_warn!("Can't respond to unknown challenge {}:{}", service_id, security_index);
            return rxrpc_kernel_reject_challenge(challenge, RX_USER_ABORT, -EPROTO, afs_abort_unsupported_sec_class);
        }
    }
    match security_index {
        #[cfg(feature = "CONFIG_RXKAD")]
        RXRPC_SECURITY_RXKAD => rxkad_kernel_respond_to_challenge(challenge),
        #[cfg(feature = "CONFIG_RXGK")]
        RXRPC_SECURITY_RXGK => rxgk_kernel_respond_to_challenge(challenge, &mut appdata),
        #[cfg(feature = "CONFIG_RXGK")]
        RXRPC_SECURITY_YFS_RXGK => {
            match service_id {
                FS_SERVICE | YFS_FS_SERVICE => {
                    let server = peer_data as *mut afs_server;
                    if (*server).cm_rxgk_appdata.data.is_null() {
                        mutex_lock(&mut (*server).cm_token_lock);
                        if (*server).cm_rxgk_appdata.data.is_null() { afs_create_yfs_cm_token(challenge, server); }
                        mutex_unlock(&mut (*server).cm_token_lock);
                    }
                    if !(*server).cm_rxgk_appdata.data.is_null() { appdata = (*server).cm_rxgk_appdata; }
                }
                _ => {}
            }
            rxgk_kernel_respond_to_challenge(challenge, &mut appdata)
        }
        _ => rxrpc_kernel_reject_challenge(challenge, RX_USER_ABORT, -EPROTO, afs_abort_unsupported_sec_class),
    }
}

/* Process the OOB message queue, processing challenge packets. */
pub unsafe fn afs_process_oob_queue(work: *mut work_struct) {
    let net = container_of!(work, afs_net, rx_oob_work);
    let mut oob: *mut sk_buff;
    let mut typ: rxrpc_oob_type = core::mem::zeroed();
    while READ_ONCE((*net).live) && { oob = rxrpc_kernel_dequeue_oob((*net).socket, &mut typ); !oob.is_null() } {
        match typ { RXRPC_OOB_CHALLENGE => { afs_respond_to_challenge(oob); } }
        rxrpc_kernel_free_oob(oob);
    }
}

#[cfg(feature = "CONFIG_RXGK")]
pub unsafe fn afs_create_token_key(net: *mut afs_net, socket: *mut socket) -> i32 {
    let mut krb5: *const krb5_enctype = core::ptr::null();
    let ring = keyring_alloc(c"kafs", GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, current_cred(), KEY_POS_SEARCH | KEY_POS_WRITE | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH, KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut());
    if IS_ERR(ring) { return PTR_ERR(ring); }
    let mut ret = rxrpc_sock_set_security_keyring((*socket).sk, ring);
    if ret < 0 { key_put(ring); return ret; }
    ret = -ENOPKG;
    krb5 = crypto_krb5_find_enctype(KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96);
    if krb5.is_null() { key_put(ring); return ret; }
    let mut k0 = [0u8; 32];
    if (*krb5).key_len > k0.len() { key_put(ring); return ret; }
    ret = -ENOMEM;
    let desc = kasprintf(GFP_KERNEL, c"%u:%u:%u:%u", YFS_CM_SERVICE, RXRPC_SECURITY_YFS_RXGK, 1, (*krb5).etype);
    if desc.is_null() { key_put(ring); return ret; }
    wait_for_random_bytes(); get_random_bytes(k0.as_mut_ptr() as *mut c_void, (*krb5).key_len);
    let key = key_create(make_key_ref(ring, true), c"rxrpc_s", desc, k0.as_mut_ptr() as *mut c_void, (*krb5).key_len, KEY_POS_VIEW | KEY_POS_READ | KEY_POS_SEARCH | KEY_USR_VIEW, KEY_ALLOC_NOT_IN_QUOTA);
    kfree(desc as *mut c_void);
    if IS_ERR(key) { ret = PTR_ERR(key); key_put(ring); return ret; }
    (*net).fs_cm_token_key = key_ref_to_ptr(key); ret = 0; key_put(ring); ret
}

// The remaining RXGK token construction follows the C XDR layout literally.
#[cfg(feature = "CONFIG_RXGK")]
unsafe fn afs_create_yfs_cm_token(challenge: *mut sk_buff, server: *mut afs_server) -> i32 {
    let net = (*(*server).cell).net;
    let key = (*net).fs_cm_token_key;
    if key.is_null() { return -ENOKEY; }
    let enctype = rxgk_kernel_query_challenge(challenge);
    let conn = crypto_krb5_find_enctype(enctype); if conn.is_null() { return -ENOPKG; }
    let token = (*key).payload.data[0] as *const krb5_enctype;
    let token_key = (*key).payload.data[2] as *const krb5_buffer;
    let keysize = 4 + xdr_len_object((*conn).key_len);
    let uuidsize = core::mem::size_of_val(&(*server).uuid);
    let authsize = 4 + xdr_len_object(uuidsize) + xdr_len_object(0);
    let toksize = keysize + 8 + 4 + 4 + 8 + xdr_len_object(authsize);
    let mut offset = 0usize;
    let encsize = crypto_krb5_how_much_buffer(token, KRB5_ENCRYPT_MODE, toksize, &mut offset);
    let contsize = 4 + 4 + xdr_len_object(encsize);
    let caps: [u32; 1] = [htonl(AFS_CAP_ERROR_TRANSLATION)];
    let adatasize = 16 + 16 + xdr_len_object(core::mem::size_of_val(&caps)) + 4 + xdr_len_object((*conn).key_len) + xdr_len_object(contsize);
    let mut appdata = kzalloc(adatasize, GFP_KERNEL); if appdata.is_null() { return -ENOMEM; }
    let mut xdr = appdata as *mut u32;
    core::ptr::copy_nonoverlapping(&(*net).uuid as *const _ as *const u8, xdr as *mut u8, 16); xdr = xdr.add(4);
    core::ptr::copy_nonoverlapping(&(*server).uuid as *const _ as *const u8, xdr as *mut u8, 16); xdr = xdr.add(4);
    *xdr = htonl(1); xdr = xdr.add(1); core::ptr::copy_nonoverlapping(caps.as_ptr(), xdr, 1); xdr = xdr.add(1);
    *xdr = htonl((*conn).etype); xdr = xdr.add(1); *xdr = htonl((*conn).key_len); xdr = xdr.add(1);
    let k0 = xdr as *mut u8; get_random_bytes(k0 as *mut c_void, (*conn).key_len); xdr = (k0 as *mut u8).add(xdr_round_up((*conn).key_len)) as *mut u32;
    *xdr = htonl(contsize as u32); xdr = xdr.add(1); *xdr = htonl(1); xdr = xdr.add(1); *xdr = htonl((*token).etype); xdr = xdr.add(1); *xdr = htonl(encsize as u32); xdr = xdr.add(1);
    let encbase = xdr.add(offset / 4); xdr = encbase; *xdr = htonl((*conn).etype); xdr=xdr.add(1); *xdr=htonl((*conn).key_len); xdr=xdr.add(1); core::ptr::copy_nonoverlapping(k0,xdr as *mut u8,(*conn).key_len); xdr=(xdr as *mut u8).add(xdr_round_up((*conn).key_len)) as *mut u32;
    for v in [RXRPC_SECURITY_ENCRYPT,0,0,0,0,0,0,1,0,uuidsize as u32] { *xdr=htonl(v); xdr=xdr.add(1); }
    core::ptr::copy_nonoverlapping(&(*server).uuid as *const _ as *const u8,xdr as *mut u8,uuidsize); xdr=(xdr as *mut u8).add(xdr_round_up(uuidsize)) as *mut u32; *xdr=0;
    let aead = crypto_krb5_prepare_encryption(token, token_key, RXGK_SERVER_ENC_TOKEN, GFP_KERNEL);
    if IS_ERR(aead) { kfree(appdata as *mut c_void); return PTR_ERR(aead); }
    let mut sg: scatterlist = core::mem::zeroed();
    sg_init_one(&mut sg, encbase as *mut c_void, encsize);
    let ret = crypto_krb5_encrypt(token, aead, &mut sg, 1, encsize, offset, toksize, false);
    crypto_free_aead(aead);
    if ret < 0 { kfree(appdata as *mut c_void); return ret; }
    (*server).cm_rxgk_appdata.len=adatasize; (*server).cm_rxgk_appdata.data=appdata; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
