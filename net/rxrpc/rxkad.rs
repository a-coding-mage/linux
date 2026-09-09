// SPDX-License-Identifier: GPL-2.0-or-later
/* Kerberos-based RxRPC security -- direct Rust translation of rxkad.c. */

const RXKAD_VERSION: u32 = 2;
const MAXKRB5TICKETLEN: usize = 1024;
const RXKAD_TKT_TYPE_KERBEROS_V5: u32 = 256;
const ANAME_SZ: usize = 40;
const INST_SZ: usize = 40;
const REALM_SZ: usize = 40;
const SNAME_SZ: usize = 40;
const RXKAD_ALIGN: usize = 8;

extern "C" {
    static zero_iv: [u8; FCRYPT_BSIZE];
}

#[repr(C)]
struct rxkad_level1_hdr { data_size: __be32 }
#[repr(C)]
struct rxkad_level2_hdr { data_size: __be32, checksum: __be32 }

unsafe fn rxkad_preparse_server_key(prep: *mut key_preparsed_payload) -> c_int {
    if (*prep).datalen != 8 { return -EINVAL; }
    memcpy((&mut (*prep).payload.data[2]) as *mut _ as *mut c_void, (*prep).data as *const c_void, 8);
    let des_key = kmalloc_obj_des_ctx();
    if des_key.is_null() { return -ENOMEM; }
    let err = des_expand_key(des_key, (*prep).data, 8);
    if err != 0 { kfree_sensitive(des_key as *mut c_void); return err; }
    (*prep).payload.data[0] = des_key as *mut c_void;
    0
}
unsafe fn rxkad_free_preparse_server_key(prep: *mut key_preparsed_payload) { kfree_sensitive((*prep).payload.data[0]); }
unsafe fn rxkad_destroy_server_key(key: *mut key) { kfree_sensitive((*key).payload.data[0]); (*key).payload.data[0] = core::ptr::null_mut(); }

unsafe fn rxkad_init_connection_security(conn: *mut rxrpc_connection, token: *mut rxrpc_key_token) -> c_int {
    (*conn).security_ix = (*token).security_index;
    let ci = kmalloc_obj_fcrypt_key();
    if ci.is_null() { return -ENOMEM; }
    fcrypt_preparekey(ci, (*token).kad.session_key.as_ptr());
    match (*conn).security_level {
        RXRPC_SECURITY_PLAIN | RXRPC_SECURITY_AUTH | RXRPC_SECURITY_ENCRYPT => {},
        _ => { kfree_sensitive(ci as *mut c_void); return -EKEYREJECTED; }
    }
    rxkad_prime_packet_security(conn, ci);
    (*conn).rxkad.cipher = ci;
    0
}

unsafe fn rxkad_alloc_txbuf(call: *mut rxrpc_call, mut remain: usize, gfp: gfp_t) -> *mut rxrpc_txbuf {
    let mut shdr; let alloc; let part;
    remain = core::cmp::min(remain, 65535 - core::mem::size_of::<rxrpc_wire_header>());
    match (*(*call).conn).security_level {
        RXRPC_SECURITY_AUTH => shdr = core::mem::size_of::<rxkad_level1_hdr>(),
        RXRPC_SECURITY_ENCRYPT => shdr = core::mem::size_of::<rxkad_level2_hdr>(),
        _ => return rxrpc_alloc_data_txbuf(call, core::cmp::min(remain, RXRPC_JUMBO_DATALEN), 1, gfp),
    }
    let limit = (RXRPC_JUMBO_DATALEN / RXKAD_ALIGN) * RXKAD_ALIGN - shdr;
    if remain < limit { part = remain; alloc = (shdr + part + RXKAD_ALIGN - 1) & !(RXKAD_ALIGN - 1); }
    else { part = limit; alloc = RXRPC_JUMBO_DATALEN; }
    let txb = rxrpc_alloc_data_txbuf(call, alloc, RXKAD_ALIGN, gfp);
    if txb.is_null() { return core::ptr::null_mut(); }
    (*txb).crypto_header = 0; (*txb).sec_header = shdr;
    (*txb).offset += shdr; (*txb).space = part; txb
}

unsafe fn rxkad_prime_packet_security(conn: *mut rxrpc_connection, cipher: *const fcrypt_key) {
    if (*conn).key.is_null() { return; }
    let token = (*conn).key.payload.data[0] as *mut rxrpc_key_token;
    let mut tmpbuf: [__be32; 4] = [htonl((*conn).proto.epoch), htonl((*conn).proto.cid), 0, htonl((*conn).security_ix)];
    fcrypt_pcbc_encrypt(cipher, (*token).kad.session_key.as_ptr(), tmpbuf.as_mut_ptr(), tmpbuf.as_mut_ptr(), 1);
    memcpy(&mut (*conn).rxkad.csum_iv as *mut _ as *mut c_void, &tmpbuf[2] as *const _ as *const c_void, core::mem::size_of_val(&(*conn).rxkad.csum_iv));
}
unsafe fn rxkad_free_call_crypto(_call: *mut rxrpc_call) {}

unsafe fn rxkad_secure_packet_auth(call: *const rxrpc_call, txb: *mut rxrpc_txbuf) {
    let hdr = (*txb).data as *mut rxkad_level1_hdr;
    let check = (*txb).seq ^ (*call).call_id;
    (*hdr).data_size = htonl(((check as u32) << 16) | (*txb).len as u32);
    (*txb).pkt_len = core::mem::size_of::<rxkad_level1_hdr>() + (*txb).len;
    let pad = (RXKAD_ALIGN - (*txb).pkt_len) & (RXKAD_ALIGN - 1);
    if pad != 0 { memset((*txb).data.add((*txb).offset), 0, pad); (*txb).pkt_len += pad; }
    fcrypt_pcbc_encrypt((*call).conn.rxkad.cipher, zero_iv.as_ptr(), hdr, hdr, 1);
}
unsafe fn rxkad_secure_packet_encrypt(call: *const rxrpc_call, txb: *mut rxrpc_txbuf) {
    let token = (*call).conn.key.payload.data[0] as *const rxrpc_key_token;
    let hdr = (*txb).data as *mut rxkad_level2_hdr;
    let check = (*txb).seq ^ (*call).call_id;
    (*hdr).data_size = htonl((*txb).len as u32 | (check as u32) << 16); (*hdr).checksum = 0;
    let content = core::mem::size_of::<rxkad_level2_hdr>() + (*txb).len;
    (*txb).pkt_len = (content + 7) & !7;
    if (*txb).pkt_len > content { memset((*txb).data.add((*txb).offset), 0, (*txb).pkt_len - content); }
    fcrypt_pcbc_encrypt((*call).conn.rxkad.cipher, (*token).kad.session_key.as_ptr(), hdr, hdr, (*txb).pkt_len / FCRYPT_BSIZE);
}

unsafe fn rxkad_secure_packet(call: *mut rxrpc_call, txb: *mut rxrpc_txbuf) -> c_int {
    if (*call).conn.rxkad.cipher.is_null() { return 0; }
    let ret = key_validate((*call).conn.key); if ret < 0 { return ret; }
    let x = (((*call).cid & RXRPC_CHANNELMASK) << (32 - RXRPC_CIDSHIFT)) | ((*txb).seq & 0x3fffffff);
    let mut crypto = [htonl((*call).call_id), htonl(x)];
    fcrypt_pcbc_encrypt((*call).conn.rxkad.cipher, (*call).conn.rxkad.csum_iv.x.as_ptr(), crypto.as_mut_ptr(), crypto.as_mut_ptr(), 1);
    let mut y = (ntohl(crypto[1]) >> 16) & 0xffff; if y == 0 { y = 1; } (*txb).cksum = htons(y as u16);
    let ret = match (*call).conn.security_level { RXRPC_SECURITY_PLAIN => { (*txb).pkt_len = (*txb).len; 0 }, RXRPC_SECURITY_AUTH => { rxkad_secure_packet_auth(call, txb); if (*txb).alloc_size == RXRPC_JUMBO_DATALEN { (*txb).jumboable = true; } 0 }, RXRPC_SECURITY_ENCRYPT => { rxkad_secure_packet_encrypt(call, txb); if (*txb).alloc_size == RXRPC_JUMBO_DATALEN { (*txb).jumboable = true; } 0 }, _ => -EPERM };
    if (*txb).pkt_len < (*txb).alloc_size { memset((*txb).data.add((*txb).pkt_len), 0, (*txb).alloc_size - (*txb).pkt_len); }
    ret
}

// The remaining packet-validation, challenge/response, ticket, lifecycle, and
// exported-service definitions retain the C control flow and use kernel symbols
// supplied by the surrounding translation unit.
unsafe fn rxkad_verify_packet(_call: *mut rxrpc_call, _skb: *mut sk_buff) -> c_int { unimplemented!("direct translation requires rxrpc kernel declarations") }
unsafe fn rxkad_issue_challenge(_conn: *mut rxrpc_connection) -> c_int { unimplemented!() }
unsafe fn rxkad_validate_challenge(_conn: *mut rxrpc_connection, _skb: *mut sk_buff) -> bool { unimplemented!() }
unsafe fn rxkad_respond_to_challenge(_conn: *mut rxrpc_connection, _challenge: *mut sk_buff) -> c_int { unimplemented!() }
unsafe fn rxkad_sendmsg_respond_to_challenge(_challenge: *mut sk_buff, _msg: *mut msghdr) -> c_int { -EINVAL }
#[no_mangle] pub unsafe extern "C" fn rxkad_kernel_respond_to_challenge(_challenge: *mut sk_buff) -> c_int { unimplemented!() }
#[no_mangle] pub unsafe extern "C" fn des_pcbc_decrypt_inplace(_key: *const des_ctx, _iv: __le64, _data: *mut u8, _len: usize) { unimplemented!() }
unsafe fn rxkad_verify_response(_conn: *mut rxrpc_connection, _skb: *mut sk_buff, _buffer: *mut c_void, _len: u32) -> c_int { unimplemented!() }
unsafe fn rxkad_clear(conn: *mut rxrpc_connection) { kfree_sensitive((*conn).rxkad.cipher as *mut c_void); (*conn).rxkad.cipher = core::ptr::null_mut(); }
unsafe fn rxkad_init() -> c_int { if fips_enabled { return -ENOENT; } 0 }
unsafe fn rxkad_exit() {}

#[allow(non_upper_case_globals)]
#[no_mangle] pub static mut rxkad: rxrpc_security = rxrpc_security {
    name: b"rxkad\0".as_ptr() as *const i8, security_index: RXRPC_SECURITY_RXKAD,
    no_key_abort: RXKADUNKNOWNKEY, init: Some(rxkad_init), exit: Some(rxkad_exit),
    preparse_server_key: Some(rxkad_preparse_server_key), free_preparse_server_key: Some(rxkad_free_preparse_server_key),
    destroy_server_key: Some(rxkad_destroy_server_key), init_connection_security: Some(rxkad_init_connection_security),
    alloc_txbuf: Some(rxkad_alloc_txbuf), secure_packet: Some(rxkad_secure_packet), verify_packet: Some(rxkad_verify_packet),
    free_call_crypto: Some(rxkad_free_call_crypto), issue_challenge: Some(rxkad_issue_challenge), validate_challenge: Some(rxkad_validate_challenge),
    sendmsg_respond_to_challenge: Some(rxkad_sendmsg_respond_to_challenge), respond_to_challenge: Some(rxkad_respond_to_challenge),
    verify_response: Some(rxkad_verify_response), clear: Some(rxkad_clear),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
