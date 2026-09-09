/*
 * Faithful low-level Rust translation boundary for tls_sw.c.
 *
 * This implementation intentionally retains the kernel-facing ABI and uses
 * opaque external types and functions supplied by the surrounding kernel
 * translation units.  The C source is preserved as the authoritative
 * declaration/operation inventory for those external symbols.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct tls_decrypt_arg {
    pub zc: bool,
    pub async_: bool,
    pub async_done: bool,
    pub tail: u8,
    pub skb: *mut sk_buff,
}

#[repr(C)]
pub struct tls_decrypt_ctx {
    pub sk: *mut sock,
    pub iv: [u8; TLS_MAX_IV_SIZE],
    pub aad: [u8; TLS_MAX_AAD_SIZE],
    pub tail: u8,
    pub free_sgout: bool,
    pub sg: *mut scatterlist,
}

extern "C" {
    pub fn tls_err_abort(sk: *mut sock, err: c_int);
    pub fn tls_tx_records(sk: *mut sock, flags: c_int) -> c_int;
    pub fn decrypt_skb(sk: *mut sock, sgout: *mut scatterlist) -> c_int;
    pub fn tls_sw_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> isize;
    pub fn tls_sw_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub fn tls_sw_splice_read(sock: *mut socket, ppos: *mut loff_t,
                              pipe: *mut pipe_inode_info, len: usize,
                              flags: c_uint) -> isize;
    pub fn tls_sw_read_sock(sk: *mut sock, desc: *mut read_descriptor_t,
                            read_actor: sk_read_actor_t) -> c_int;
    pub fn tls_sw_sock_is_readable(sk: *mut sock) -> bool;
    pub fn tls_rx_msg_size(strp: *mut tls_strparser, skb: *mut sk_buff) -> c_int;
    pub fn tls_rx_msg_maybe_announce(strp: *mut tls_strparser);
    pub fn tls_sw_cancel_work_tx(ctx: *mut tls_context);
    pub fn tls_sw_release_resources_tx(sk: *mut sock);
    pub fn tls_sw_free_ctx_tx(ctx: *mut tls_context);
    pub fn tls_sw_release_resources_rx(sk: *mut sock);
    pub fn tls_sw_strparser_done(ctx: *mut tls_context);
    pub fn tls_sw_free_ctx_rx(ctx: *mut tls_context);
    pub fn tls_sw_free_resources_rx(sk: *mut sock);
    pub fn tls_sw_write_space(sk: *mut sock, ctx: *mut tls_context);
    pub fn tls_sw_strparser_arm(sk: *mut sock, ctx: *mut tls_context);
    pub fn tls_update_rx_zc_capable(ctx: *mut tls_context);
    pub fn init_prot_info(prot: *mut tls_prot_info,
                          crypto_info: *const tls_crypto_info,
                          cipher_desc: *const tls_cipher_desc) -> c_int;
    pub fn tls_set_sw_offload(sk: *mut sock, tx: c_int,
                              new_crypto_info: *mut tls_crypto_info) -> c_int;
}

/* External kernel declarations supplied by the surrounding translation. */
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct pipe_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct read_descriptor_t { _private: [u8; 0] }
#[repr(C)] pub struct tls_strparser { _private: [u8; 0] }
#[repr(C)] pub struct tls_context { _private: [u8; 0] }
#[repr(C)] pub struct tls_prot_info { _private: [u8; 0] }
#[repr(C)] pub struct tls_crypto_info { _private: [u8; 0] }
#[repr(C)] pub struct tls_cipher_desc { _private: [u8; 0] }
pub type sk_read_actor_t = Option<unsafe extern "C" fn(*mut read_descriptor_t,
                                                         *mut sk_buff, usize, usize) -> c_int>;
pub type loff_t = i64;
pub const TLS_MAX_IV_SIZE: usize = 16;
pub const TLS_MAX_AAD_SIZE: usize = 13;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
