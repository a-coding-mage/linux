/* Translated from tls.h. Kernel includes and externally supplied types are
 * intentionally left as external dependencies. */

use core::ffi::{c_char, c_int, c_long, c_uchar, c_void};

pub const TLS_PAGE_ORDER: u32 = 0; // min_t(unsigned int, PAGE_ALLOC_COSTLY_ORDER, TLS_MAX_PAYLOAD_SIZE >> PAGE_SHIFT)

#[repr(C)]
pub struct tls_cipher_desc {
    pub nonce: u32,
    pub iv: u32,
    pub key: u32,
    pub salt: u32,
    pub tag: u32,
    pub rec_seq: u32,
    pub iv_offset: u32,
    pub key_offset: u32,
    pub salt_offset: u32,
    pub rec_seq_offset: u32,
    pub cipher_name: *mut c_char,
    pub offloadable: bool,
    pub crypto_info: usize,
}

pub const TLS_CIPHER_MIN: u32 = TLS_CIPHER_AES_GCM_128;
pub const TLS_CIPHER_MAX: u32 = TLS_CIPHER_ARIA_GCM_256;

unsafe extern "C" {
    pub static tls_cipher_desc: [tls_cipher_desc; (TLS_CIPHER_MAX + 1 - TLS_CIPHER_MIN) as usize];
}

#[inline]
pub unsafe fn get_cipher_desc(cipher_type: u16) -> *const tls_cipher_desc {
    if (cipher_type as u32) < TLS_CIPHER_MIN || (cipher_type as u32) > TLS_CIPHER_MAX {
        core::ptr::null()
    } else {
        tls_cipher_desc.as_ptr().add(cipher_type as usize - TLS_CIPHER_MIN as usize)
    }
}

#[inline]
pub unsafe fn crypto_info_iv(crypto_info: *mut tls_crypto_info, cipher_desc: *const tls_cipher_desc) -> *mut c_char {
    (crypto_info as *mut c_char).add((*cipher_desc).iv_offset as usize)
}
#[inline]
pub unsafe fn crypto_info_key(crypto_info: *mut tls_crypto_info, cipher_desc: *const tls_cipher_desc) -> *mut c_char {
    (crypto_info as *mut c_char).add((*cipher_desc).key_offset as usize)
}
#[inline]
pub unsafe fn crypto_info_salt(crypto_info: *mut tls_crypto_info, cipher_desc: *const tls_cipher_desc) -> *mut c_char {
    (crypto_info as *mut c_char).add((*cipher_desc).salt_offset as usize)
}
#[inline]
pub unsafe fn crypto_info_rec_seq(crypto_info: *mut tls_crypto_info, cipher_desc: *const tls_cipher_desc) -> *mut c_char {
    (crypto_info as *mut c_char).add((*cipher_desc).rec_seq_offset as usize)
}

#[repr(C)]
pub struct tls_rec {
    pub list: list_head,
    pub tx_ready: c_int,
    pub tx_flags: c_int,
    pub msg_plaintext: sk_msg,
    pub msg_encrypted: sk_msg,
    pub sg_aead_in: [scatterlist; 2],
    pub sg_aead_out: [scatterlist; 2],
    pub content_type: c_char,
    pub sg_content_type: scatterlist,
    pub sk: *mut sock,
    pub aad_space: [c_char; TLS_AAD_SPACE_SIZE as usize],
    pub iv_data: [u8; TLS_MAX_IV_SIZE as usize],
    pub aead_req: aead_request,
}

unsafe extern "C" {
    pub fn tls_proc_init(net: *mut net) -> c_int;
    pub fn tls_proc_fini(net: *mut net);
    pub fn tls_ctx_create(sk: *mut sock) -> *mut tls_context;
    pub fn tls_ctx_free(sk: *mut sock, ctx: *mut tls_context);
    pub fn update_sk_prot(sk: *mut sock, ctx: *mut tls_context);
    pub fn wait_on_pending_writer(sk: *mut sock, timeo: *mut c_long) -> c_int;
    pub fn tls_err_abort(sk: *mut sock, err: c_int);
    pub fn tls_strp_abort_strp(strp: *mut tls_strparser, err: c_int);
    pub fn init_prot_info(prot: *mut tls_prot_info, crypto_info: *const tls_crypto_info, cipher_desc: *const tls_cipher_desc) -> c_int;
    pub fn tls_set_sw_offload(sk: *mut sock, tx: c_int, new_crypto_info: *mut tls_crypto_info) -> c_int;
    pub fn tls_update_rx_zc_capable(tls_ctx: *mut tls_context);
    pub fn tls_sw_strparser_arm(sk: *mut sock, ctx: *mut tls_context);
    pub fn tls_sw_strparser_done(tls_ctx: *mut tls_context);
    pub fn tls_sw_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
    pub fn tls_sw_splice_eof(sock: *mut socket);
    pub fn tls_sw_cancel_work_tx(tls_ctx: *mut tls_context);
    pub fn tls_sw_release_resources_tx(sk: *mut sock);
    pub fn tls_sw_free_ctx_tx(tls_ctx: *mut tls_context);
    pub fn tls_sw_free_resources_rx(sk: *mut sock);
    pub fn tls_sw_release_resources_rx(sk: *mut sock);
    pub fn tls_sw_free_ctx_rx(tls_ctx: *mut tls_context);
    pub fn tls_sw_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub fn tls_sw_sock_is_readable(sk: *mut sock) -> bool;
    pub fn tls_sw_splice_read(sock: *mut socket, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize;
    pub fn tls_sw_read_sock(sk: *mut sock, desc: *mut read_descriptor_t, read_actor: sk_read_actor_t) -> c_int;
    pub fn tls_device_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
    pub fn tls_device_splice_eof(sock: *mut socket);
    pub fn tls_tx_records(sk: *mut sock, flags: c_int) -> c_int;
    pub fn tls_sw_write_space(sk: *mut sock, ctx: *mut tls_context);
    pub fn tls_device_write_space(sk: *mut sock, ctx: *mut tls_context);
    pub fn tls_process_cmsg(sk: *mut sock, msg: *mut msghdr, record_type: *mut c_uchar) -> c_int;
    pub fn decrypt_skb(sk: *mut sock, sgout: *mut scatterlist) -> c_int;
    pub fn tls_sw_fallback_init(sk: *mut sock, offload_ctx: *mut tls_offload_context_tx, crypto_info: *mut tls_crypto_info) -> c_int;
    pub fn tls_strp_dev_init() -> c_int;
    pub fn tls_strp_dev_exit();
    pub fn tls_strp_done(strp: *mut tls_strparser);
    pub fn __tls_strp_done(strp: *mut tls_strparser);
    pub fn tls_strp_stop(strp: *mut tls_strparser);
    pub fn tls_strp_init(strp: *mut tls_strparser, sk: *mut sock) -> c_int;
    pub fn tls_strp_data_ready(strp: *mut tls_strparser);
    pub fn tls_strp_check_rcv(strp: *mut tls_strparser, announce: bool);
    pub fn tls_strp_msg_consume(strp: *mut tls_strparser);
    pub fn tls_rx_msg_size(strp: *mut tls_strparser, skb: *mut sk_buff) -> c_int;
    pub fn tls_rx_msg_maybe_announce(strp: *mut tls_strparser);
    pub fn tls_strp_msg_load(strp: *mut tls_strparser, force_refresh: bool) -> bool;
    pub fn tls_strp_msg_cow(ctx: *mut tls_sw_context_rx) -> c_int;
    pub fn tls_strp_msg_detach(ctx: *mut tls_sw_context_rx) -> *mut sk_buff;
    pub fn tls_strp_msg_hold(strp: *mut tls_strparser, dst: *mut sk_buff_head) -> c_int;
}

#[inline]
pub unsafe fn tls_msg(skb: *mut sk_buff) -> *mut tls_msg {
    &mut (*(skb as *mut sk_skb_cb)).tls
}
#[inline]
pub unsafe fn tls_strp_msg(ctx: *mut tls_sw_context_rx) -> *mut sk_buff { (*ctx).strp.anchor }
#[inline]
pub unsafe fn tls_strp_msg_ready(ctx: *mut tls_sw_context_rx) -> bool { READ_ONCE((*ctx).strp.msg_ready) }
#[inline]
pub unsafe fn tls_strp_msg_mixed_decrypted(ctx: *mut tls_sw_context_rx) -> bool { (*ctx).strp.mixed_decrypted }

#[cfg(feature = "CONFIG_TLS_DEVICE")]
unsafe extern "C" {
    pub fn tls_device_init() -> c_int;
    pub fn tls_device_cleanup();
    pub fn tls_set_device_offload(sk: *mut sock) -> c_int;
    pub fn tls_device_free_resources_tx(sk: *mut sock);
    pub fn tls_set_device_offload_rx(sk: *mut sock, ctx: *mut tls_context) -> c_int;
    pub fn tls_device_offload_cleanup_rx(sk: *mut sock);
    pub fn tls_device_rx_resync_new_rec(sk: *mut sock, rcd_len: u32, seq: u32);
    pub fn tls_device_decrypted(sk: *mut sock, tls_ctx: *mut tls_context) -> c_int;
}
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_init() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_cleanup() {}
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_set_device_offload(_sk: *mut sock) -> c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_free_resources_tx(_sk: *mut sock) {}
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_set_device_offload_rx(_sk: *mut sock, _ctx: *mut tls_context) -> c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_offload_cleanup_rx(_sk: *mut sock) {}
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_rx_resync_new_rec(_sk: *mut sock, _rcd_len: u32, _seq: u32) {}
#[cfg(not(feature = "CONFIG_TLS_DEVICE"))]
pub unsafe fn tls_device_decrypted(_sk: *mut sock, _tls_ctx: *mut tls_context) -> c_int { 0 }

unsafe extern "C" {
    pub fn tls_push_sg(sk: *mut sock, ctx: *mut tls_context, sg: *mut scatterlist, first_offset: u16, flags: c_int) -> c_int;
    pub fn tls_push_partial_record(sk: *mut sock, ctx: *mut tls_context, flags: c_int) -> c_int;
    pub fn tls_free_partial_record(sk: *mut sock, ctx: *mut tls_context);
}

#[inline]
pub unsafe fn tls_is_partially_sent_record(ctx: *mut tls_context) -> bool { (*ctx).partially_sent_record != 0 }
#[inline]
pub unsafe fn tls_is_pending_open_record(tls_ctx: *mut tls_context) -> bool { (*tls_ctx).pending_open_record_frags }

#[inline]
pub unsafe fn tls_bigint_increment(seq: *mut c_uchar, len: c_int) -> bool {
    let mut i = len - 1;
    while i >= 0 { *seq.add(i as usize) = (*seq.add(i as usize)).wrapping_add(1); if *seq.add(i as usize) != 0 { break; } i -= 1; }
    i == -1
}

#[inline]
pub unsafe fn tls_bigint_subtract(seq: *mut c_uchar, n: c_int) {
    let p = seq as *mut u64;
    let rcd_sn = u64::from_be(*p);
    *p = (rcd_sn.wrapping_sub(n as u64)).to_be();
}

#[inline]
pub unsafe fn tls_advance_record_sn(sk: *mut sock, prot: *mut tls_prot_info, ctx: *mut cipher_context) {
    if tls_bigint_increment((*ctx).rec_seq, (*prot).rec_seq_size as c_int) { tls_err_abort(sk, -EBADMSG); }
    if (*prot).version != TLS_1_3_VERSION && (*prot).cipher_type != TLS_CIPHER_CHACHA20_POLY1305 { tls_bigint_increment((*ctx).iv.add((*prot).salt_size as usize), (*prot).iv_size as c_int); }
}

#[inline]
pub unsafe fn tls_xor_iv_with_seq(prot: *mut tls_prot_info, iv: *mut c_char, seq: *mut c_char) {
    if (*prot).version == TLS_1_3_VERSION || (*prot).cipher_type == TLS_CIPHER_CHACHA20_POLY1305 { for i in 0..8 { *iv.add(i + 4) = (*iv.add(i + 4) as u8 ^ *seq.add(i) as u8) as c_char; } }
}

#[inline]
pub unsafe fn tls_fill_prepend(ctx: *mut tls_context, buf: *mut c_char, plaintext_len: usize, record_type: c_uchar) {
    let prot = &(*ctx).prot_info; let iv_size = prot.iv_size as usize; let mut pkt_len = plaintext_len + prot.tag_size as usize;
    if prot.version != TLS_1_3_VERSION && prot.cipher_type != TLS_CIPHER_CHACHA20_POLY1305 { pkt_len += iv_size; core::ptr::copy_nonoverlapping((*ctx).tx.iv.add(prot.salt_size as usize), buf.add(TLS_NONCE_OFFSET as usize), iv_size); }
    *buf = if prot.version == TLS_1_3_VERSION { TLS_RECORD_TYPE_DATA as c_char } else { record_type as c_char }; *buf.add(1) = TLS_1_2_VERSION_MINOR as c_char; *buf.add(2) = TLS_1_2_VERSION_MAJOR as c_char; *buf.add(3) = (pkt_len >> 8) as c_char; *buf.add(4) = pkt_len as c_char;
}

#[inline]
pub unsafe fn tls_make_aad(buf: *mut c_char, mut size: usize, record_sequence: *mut c_char, record_type: c_uchar, prot: *mut tls_prot_info) {
    if (*prot).version != TLS_1_3_VERSION { core::ptr::copy_nonoverlapping(record_sequence, buf, (*prot).rec_seq_size as usize); } else { size += (*prot).tag_size as usize; }
    let off = if (*prot).version != TLS_1_3_VERSION { 8 } else { 0 }; let b = buf.add(off); *b = if (*prot).version == TLS_1_3_VERSION { TLS_RECORD_TYPE_DATA as c_char } else { record_type as c_char }; *b.add(1) = TLS_1_2_VERSION_MAJOR as c_char; *b.add(2) = TLS_1_2_VERSION_MINOR as c_char; *b.add(3) = (size >> 8) as c_char; *b.add(4) = size as c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
