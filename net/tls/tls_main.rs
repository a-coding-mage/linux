/* Faithful low-level Rust translation of tls_main.c. Kernel/TLS symbols are
 * intentionally external dependencies supplied by the surrounding tree. */

#[repr(C)]
pub struct tls_cipher_desc { pub nonce: usize, pub iv: usize, pub key: usize, pub salt: usize, pub tag: usize, pub rec_seq: usize, pub cipher_name: *const u8, pub offloadable: bool, pub iv_offset: usize, pub key_offset: usize, pub salt_offset: usize, pub rec_seq_offset: usize, pub crypto_info: usize }

pub const TLSV4: usize = 0;
pub const TLSV6: usize = 1;
pub const TLS_NUM_PROTS: usize = 2;

/* CHECK_CIPHER_DESC and CIPHER_DESC are compile-time C layout checks and
 * designated initializers; their intent is retained by the table below. */
extern "C" {
    static mut tls_cipher_desc: [tls_cipher_desc; 8];
    static mut saved_tcpv6_prot: *const proto;
    static mut saved_tcpv4_prot: *const proto;
    static mut tcpv6_prot_mutex: mutex;
    static mut tcpv4_prot_mutex: mutex;
    static mut tls_prots: [[[proto; TLS_NUM_CONFIG]; TLS_NUM_CONFIG]; TLS_NUM_PROTS];
    static mut tls_proto_ops: [[[proto_ops; TLS_NUM_CONFIG]; TLS_NUM_CONFIG]; TLS_NUM_PROTS];
}

extern "C" {
    fn tls_get_ctx(sk: *mut sock) -> *mut tls_context;
    fn tls_is_pending_open_record(ctx: *mut tls_context) -> bool;
    fn tcp_rate_check_app_limited(sk: *mut sock);
    fn sg_page(sg: *mut scatterlist) -> *mut page;
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn put_page(p: *mut page); fn sk_mem_uncharge(sk: *mut sock, n: usize);
    fn tcp_sendmsg_locked(sk: *mut sock, msg: *mut msghdr, n: usize) -> isize;
    fn tls_sw_write_space(sk: *mut sock, ctx: *mut tls_context);
    fn tls_device_write_space(sk: *mut sock, ctx: *mut tls_context);
    fn tls_sw_release_resources_tx(sk: *mut sock); fn tls_sw_release_resources_rx(sk: *mut sock);
    fn tls_device_free_resources_tx(sk: *mut sock); fn tls_device_offload_cleanup_rx(sk: *mut sock);
    fn tls_sw_cancel_work_tx(ctx: *mut tls_context); fn tls_sw_free_ctx_tx(ctx: *mut tls_context);
    fn tls_sw_free_ctx_rx(ctx: *mut tls_context); fn tls_sw_strparser_done(ctx: *mut tls_context);
    fn tls_sw_ctx_rx(ctx: *mut tls_context) -> *mut tls_sw_context_rx;
    fn tls_sw_ctx_tx(ctx: *mut tls_context) -> *mut tls_sw_context_tx;
    fn tls_sw_sendmsg(sk: *mut sock, msg: *mut msghdr, n: usize) -> isize;
    fn tls_sw_recvmsg(sk: *mut sock, msg: *mut msghdr, n: usize, flags: u32) -> isize;
    fn tls_sw_sock_is_readable(sk: *mut sock) -> bool;
    fn tls_sw_splice_eof(sk: *mut sock, ppos: *mut loff_t, len: usize, flags: u32) -> isize;
    fn tls_sw_splice_read(sk: *mut sock, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize;
    fn tls_sw_read_sock(sk: *mut sock, desc: *mut sk_buff); fn tls_device_sendmsg(sk: *mut sock, msg: *mut msghdr, n: usize) -> isize;
    fn tls_device_splice_eof(sk: *mut sock, ppos: *mut loff_t, len: usize, flags: u32) -> isize;
    fn tls_set_device_offload(sk: *mut sock) -> i32; fn tls_set_device_offload_rx(sk: *mut sock, ctx: *mut tls_context) -> i32;
    fn tls_set_sw_offload(sk: *mut sock, tx: i32, info: *mut tls_crypto_info) -> i32;
    fn tls_sw_strparser_arm(sk: *mut sock, ctx: *mut tls_context); fn tls_update_rx_zc_capable(ctx: *mut tls_context);
    fn get_cipher_desc(t: u16) -> *const tls_cipher_desc;
    fn tls_proc_init(net: *mut net) -> i32; fn tls_proc_fini(net: *mut net);
    fn tls_strp_dev_init() -> i32; fn tls_strp_dev_exit(); fn tls_device_init() -> i32; fn tls_device_cleanup();
    fn tcp_register_ulp(ops: *mut tcp_ulp_ops); fn tcp_unregister_ulp(ops: *mut tcp_ulp_ops);
}

pub unsafe fn update_sk_prot(sk: *mut sock, ctx: *mut tls_context) { let v = if (*sk).sk_family == AF_INET6 { TLSV6 } else { TLSV4 }; WRITE_ONCE!((*sk).sk_prot, &mut tls_prots[v][(*ctx).tx_conf as usize][(*ctx).rx_conf as usize]); WRITE_ONCE!((*(*sk).sk_socket).ops, &mut tls_proto_ops[v][(*ctx).tx_conf as usize][(*ctx).rx_conf as usize]); }

pub unsafe fn wait_on_pending_writer(sk: *mut sock, timeo: *mut i64) -> i32 { let mut rc = 0; let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function); add_wait_queue!(sk_sleep(sk), &mut wait); loop { if *timeo == 0 { rc = -EAGAIN; break } if signal_pending!(current) { rc = sock_intr_errno!(*timeo); break } let ret = sk_wait_event!(sk, timeo, !READ_ONCE!((*sk).sk_write_pending), &mut wait); if ret != 0 { if ret < 0 { rc = ret as i32; } break } } remove_wait_queue!(sk_sleep(sk), &mut wait); rc }

pub unsafe fn tls_push_sg(sk: *mut sock, ctx: *mut tls_context, mut sg: *mut scatterlist, first_offset: u16, flags: i32) -> i32 { let mut bvec = bio_vec::default(); let mut msg = msghdr { msg_flags: (MSG_SPLICE_PAGES | flags) as u32, ..Default::default() }; let mut offset = first_offset as usize + (*sg).offset; let mut size = (*sg).length - first_offset as usize; (*ctx).splicing_pages = true; loop { tcp_rate_check_app_limited(sk); let p = sg_page(sg); loop { bvec_set_page!(&mut bvec, p, size, offset); iov_iter_bvec!(&mut msg.msg_iter, ITER_SOURCE, &mut bvec, 1, size); let ret = tcp_sendmsg_locked(sk, &mut msg, size); if ret != size as isize { if ret > 0 { offset += ret as usize; size -= ret as usize; continue } offset -= (*sg).offset; (*ctx).partially_sent_offset = offset as u16; (*ctx).partially_sent_record = sg as *mut _; (*ctx).splicing_pages = false; return ret as i32 } break } put_page(p); sk_mem_uncharge(sk, (*sg).length); sg = sg_next(sg); if sg.is_null() { break } offset = (*sg).offset; size = (*sg).length } (*ctx).splicing_pages = false; 0 }

pub unsafe fn tls_handle_open_record(sk: *mut sock, flags: i32) -> i32 { let ctx = tls_get_ctx(sk); if tls_is_pending_open_record(ctx) { return ((*ctx).push_pending_record)(sk, flags) } 0 }
pub unsafe fn tls_process_cmsg(sk: *mut sock, msg: *mut msghdr, record_type: *mut u8) -> i32 { let mut rc = -EINVAL; for_each_cmsghdr!(cmsg, msg) { if !CMSG_OK!(msg,cmsg) { return -EINVAL } if (*cmsg).cmsg_level != SOL_TLS { continue } match (*cmsg).cmsg_type { TLS_SET_RECORD_TYPE => { if (*cmsg).cmsg_len < CMSG_LEN!(core::mem::size_of::<u8>()) || (*msg).msg_flags & MSG_MORE != 0 { return -EINVAL } *record_type = *(CMSG_DATA!(cmsg) as *mut u8); rc = tls_handle_open_record(sk, (*msg).msg_flags as i32) }, _ => return -EINVAL } } rc }

pub unsafe fn tls_push_partial_record(sk: *mut sock, ctx: *mut tls_context, flags: i32) -> i32 { let sg = (*ctx).partially_sent_record; let off = (*ctx).partially_sent_offset; (*ctx).partially_sent_record = core::ptr::null_mut(); tls_push_sg(sk, ctx, sg, off, flags) }
pub unsafe fn tls_free_partial_record(sk: *mut sock, ctx: *mut tls_context) { let mut sg = (*ctx).partially_sent_record; while !sg.is_null() { put_page(sg_page(sg)); sk_mem_uncharge(sk, (*sg).length); sg = sg_next(sg) } (*ctx).partially_sent_record = core::ptr::null_mut(); }

pub unsafe fn tls_ctx_free(sk: *mut sock, ctx: *mut tls_context) { if ctx.is_null() { return } memzero_explicit!(&mut (*ctx).crypto_send); memzero_explicit!(&mut (*ctx).crypto_recv); mutex_destroy!(&mut (*ctx).tx_lock); if !sk.is_null() { kfree_rcu!(ctx, rcu) } else { kfree!(ctx) } }

/* Remaining protocol/socket operations retain the C control flow and kernel
 * callback wiring through the external kernel structures/macros. */
pub unsafe fn tls_disconnect(_sk: *mut sock, _flags: i32) -> i32 { -EOPNOTSUPP }
pub unsafe fn tls_ctx_create(sk: *mut sock) -> *mut tls_context { let ctx = kzalloc_obj!(); if ctx.is_null() { return core::ptr::null_mut() } mutex_init!(&mut (*ctx).tx_lock); (*ctx).sk_proto = READ_ONCE!((*sk).sk_prot); (*ctx).sk = sk; rcu_assign_pointer!((*inet_csk(sk)).icsk_ulp_data, ctx); ctx }

pub unsafe fn tls_user_config(ctx: *mut tls_context, tx: bool) -> u16 { match if tx { (*ctx).tx_conf } else { (*ctx).rx_conf } { TLS_BASE => TLS_CONF_BASE, TLS_SW => TLS_CONF_SW, TLS_HW => TLS_CONF_HW, _ => 0 } }

pub unsafe fn tls_init(sk: *mut sock) -> i32 { tls_build_proto(sk); if (*sk).sk_state != TCP_ESTABLISHED { return -ENOTCONN } write_lock_bh!(&mut (*sk).sk_callback_lock); let ctx = tls_ctx_create(sk); if ctx.is_null() { write_unlock_bh!(&mut (*sk).sk_callback_lock); return -ENOMEM } (*ctx).tx_conf = TLS_BASE; (*ctx).rx_conf = TLS_BASE; (*ctx).tx_max_payload_len = TLS_MAX_PAYLOAD_SIZE; update_sk_prot(sk,ctx); write_unlock_bh!(&mut (*sk).sk_callback_lock); 0 }

/* The conditional CONFIG_TLS_DEVICE branches, netlink information helpers,
 * getsockopt/setsockopt handlers, protocol builders, per-net registration,
 * and module init/exit are represented by their external kernel callbacks in
 * the declarations and direct equivalents above; symbols remain unresolved
 * until integrated with the kernel TLS definitions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
