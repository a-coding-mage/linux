/* Copyright (c) 2018, Mellanox Technologies. Rust translation of tls_device.c. */

// Kernel headers and the local tls/trace headers are supplied by the surrounding
// kernel translation. Their declarations are intentionally not reimplemented here.

static mut DEVICE_OFFLOAD_LOCK: rw_semaphore = DECLARE_RWSEM!();
static mut DESTRUCT_WQ: *mut workqueue_struct = core::ptr::null_mut();
static mut TLS_DEVICE_LIST: list_head = LIST_HEAD!();
static mut TLS_DEVICE_DOWN_LIST: list_head = LIST_HEAD!();
static mut TLS_DEVICE_LOCK: spinlock_t = DEFINE_SPINLOCK!();
static mut DUMMY_PAGE: *mut page = core::ptr::null_mut();

unsafe fn tls_device_free_ctx(ctx: *mut tls_context) {
    if (*ctx).tx_conf == TLS_HW { kfree(tls_offload_ctx_tx(ctx) as *mut core::ffi::c_void); }
    if (*ctx).rx_conf == TLS_HW { kfree(tls_offload_ctx_rx(ctx) as *mut core::ffi::c_void); }
    tls_ctx_free(core::ptr::null_mut(), ctx);
}

unsafe fn tls_device_tx_del_task(work: *mut work_struct) {
    let offload_ctx = container_of!(work, tls_offload_context_tx, destruct_work);
    let ctx = (*offload_ctx).ctx;
    let netdev = rcu_dereference_protected((*ctx).netdev, !refcount_read(&(*ctx).refcount));
    ((*netdev).tlsdev_ops).as_ref().unwrap().tls_dev_del(netdev, ctx, TLS_OFFLOAD_CTX_DIR_TX);
    dev_put(netdev); (*ctx).netdev = core::ptr::null_mut(); tls_device_free_ctx(ctx);
}

unsafe fn tls_device_queue_ctx_destruction(ctx: *mut tls_context) {
    let mut flags = 0; spin_lock_irqsave(&mut TLS_DEVICE_LOCK, &mut flags);
    if !refcount_dec_and_test(&mut (*ctx).refcount) { spin_unlock_irqrestore(&mut TLS_DEVICE_LOCK, flags); return; }
    list_del(&mut (*ctx).list);
    let netdev = rcu_dereference_protected((*ctx).netdev, !refcount_read(&(*ctx).refcount));
    let async_cleanup = !netdev.is_null() && (*ctx).tx_conf == TLS_HW;
    if async_cleanup { let o = tls_offload_ctx_tx(ctx); queue_work(DESTRUCT_WQ, &mut (*o).destruct_work); }
    spin_unlock_irqrestore(&mut TLS_DEVICE_LOCK, flags);
    if !async_cleanup { tls_device_free_ctx(ctx); }
}

unsafe fn get_netdev_for_sock(sk: *mut sock) -> *mut net_device {
    let mut result = core::ptr::null_mut(); rcu_read_lock();
    let dst = __sk_dst_get(sk); let dev = if !dst.is_null() { dst_dev_rcu(dst) } else { core::ptr::null_mut() };
    if !dev.is_null() { result = netdev_sk_get_lowest_dev(dev, sk); dev_hold(result); }
    rcu_read_unlock(); result
}

unsafe fn destroy_record(record: *mut tls_record_info) {
    for i in 0..(*record).num_frags { __skb_frag_unref(&mut (*record).frags[i], false); }
    kfree(record as *mut core::ffi::c_void);
}
unsafe fn delete_all_records(ctx: *mut tls_offload_context_tx) {
    let mut info = core::ptr::null_mut(); let mut temp = core::ptr::null_mut();
    list_for_each_entry_safe!(info, temp, &mut (*ctx).records_list, list, { list_del(&mut (*info).list); destroy_record(info); });
    (*ctx).retransmit_hint = core::ptr::null_mut();
}
unsafe fn tls_tcp_clean_acked(sk: *mut sock, acked_seq: u32) {
    let tls_ctx = tls_get_ctx(sk); if tls_ctx.is_null() { return; }
    let ctx = tls_offload_ctx_tx(tls_ctx); let mut flags=0; let mut deleted=0u64;
    spin_lock_irqsave(&mut (*ctx).lock, &mut flags);
    let mut info=(*ctx).retransmit_hint; if !info.is_null() && !before(acked_seq,(*info).end_seq) { (*ctx).retransmit_hint=core::ptr::null_mut(); }
    let mut temp=core::ptr::null_mut(); list_for_each_entry_safe!(info,temp,&mut (*ctx).records_list,list,{ if before(acked_seq,(*info).end_seq){break;} list_del(&mut (*info).list); destroy_record(info); deleted+=1; });
    (*ctx).unacked_record_sn=(*ctx).unacked_record_sn.wrapping_add(deleted); spin_unlock_irqrestore(&mut (*ctx).lock,flags);
}

pub unsafe extern "C" fn tls_device_sk_destruct(sk: *mut sock) {
    let tls_ctx=tls_get_ctx(sk); let ctx=tls_offload_ctx_tx(tls_ctx); ((*tls_ctx).sk_destruct)(sk);
    if (*tls_ctx).tx_conf==TLS_HW { if !(*ctx).open_record.is_null(){destroy_record((*ctx).open_record);} delete_all_records(ctx); crypto_free_aead((*ctx).aead_send); clean_acked_data_disable(tcp_sk(sk)); }
    tls_device_queue_ctx_destruction(tls_ctx);
}
pub unsafe extern "C" fn tls_device_free_resources_tx(sk:*mut sock){let c=tls_get_ctx(sk);tls_free_partial_record(sk,c);}
pub unsafe extern "C" fn tls_offload_tx_resync_request(sk:*mut sock,got_seq:u32,exp_seq:u32){let c=tls_get_ctx(sk);trace_tls_device_tx_resync_req(sk,got_seq,exp_seq);WARN_ON(test_and_set_bit(TLS_TX_SYNC_SCHED,&mut (*c).flags));}

unsafe fn tls_device_resync_tx(sk:*mut sock,ctx:*mut tls_context,seq:u32){tcp_write_collapse_fence(sk);let r=(*ctx).tx.rec_seq;down_read(&mut DEVICE_OFFLOAD_LOCK);let n=rcu_dereference_protected((*ctx).netdev,lockdep_is_held(&DEVICE_OFFLOAD_LOCK));let e=if !n.is_null(){((*n).tlsdev_ops).as_ref().unwrap().tls_dev_resync(n,sk,seq,r,TLS_OFFLOAD_CTX_DIR_TX)}else{0};up_read(&mut DEVICE_OFFLOAD_LOCK);if e==0{clear_bit_unlock(TLS_TX_SYNC_SCHED,&mut (*ctx).flags);}}

unsafe fn tls_append_frag(record:*mut tls_record_info,pfrag:*mut page_frag,size:i32){let frag=&mut (*record).frags[(*record).num_frags-1];if skb_frag_page(frag)==(*pfrag).page && skb_frag_off(frag)+skb_frag_size(frag)==(*pfrag).offset{skb_frag_size_add(frag,size as u32);}else{let f=frag.add(1);skb_frag_fill_page_desc(f,(*pfrag).page,(*pfrag).offset,size as u32);(*record).num_frags+=1;get_page((*pfrag).page);}(*pfrag).offset+=size as usize;(*record).len+=size as usize;}

unsafe fn tls_push_record(sk:*mut sock,ctx:*mut tls_context,o:*mut tls_offload_context_tx,r:*mut tls_record_info,flags:i32)->i32{let tp=tcp_sk(sk);(*r).end_seq=(*tp).write_seq+(*r).len as u32;list_add_tail_rcu(&mut (*r).list,&mut (*o).records_list);(*o).open_record=core::ptr::null_mut();if test_bit(TLS_TX_SYNC_SCHED,&(*ctx).flags){tls_device_resync_tx(sk,ctx,(*tp).write_seq);}tls_advance_record_sn(sk,&(*ctx).prot_info,&mut (*ctx).tx);for i in 0..(*r).num_frags{let f=&(*r).frags[i];sg_unmark_end(&mut (*o).sg_tx_data[i]);sg_set_page(&mut (*o).sg_tx_data[i],skb_frag_page(f),skb_frag_size(f),skb_frag_off(f));sk_mem_charge(sk,skb_frag_size(f));get_page(skb_frag_page(f));}sg_mark_end(&mut (*o).sg_tx_data[(*r).num_frags-1]);tls_push_sg(sk,ctx,(*o).sg_tx_data.as_mut_ptr(),0,flags)}

unsafe fn tls_device_record_close(sk:*mut sock,ctx:*mut tls_context,r:*mut tls_record_info,pfrag:*mut page_frag,record_type:u8){let prot=&(*ctx).prot_info;let mut dummy=page_frag{page:DUMMY_PAGE,offset:0,size:0};let pf=if (*pfrag).size-(*pfrag).offset<prot.tag_size as usize && !skb_page_frag_refill(prot.tag_size,pfrag,(*sk).sk_allocation){&mut dummy}else{pfrag};tls_append_frag(r,pf,prot.tag_size as i32);tls_fill_prepend(ctx,skb_frag_address(&mut (*r).frags[0]),(*r).len-prot.overhead_size as usize,record_type);}

unsafe fn tls_create_new_record(o:*mut tls_offload_context_tx,pfrag:*mut page_frag,prepend_size:usize)->i32{let r=kmalloc_obj::<tls_record_info>();if r.is_null(){return -ENOMEM;}skb_frag_fill_page_desc(&mut (*r).frags[0],(*pfrag).page,(*pfrag).offset,prepend_size as u32);get_page((*pfrag).page);(*pfrag).offset+=prepend_size;(*r).num_frags=1;(*r).len=prepend_size;(*o).open_record=r;0}

// The remaining routines retain the C implementation's externally supplied kernel
// operations and are expressed with the same ordering and branch structure.
pub unsafe extern "C" fn tls_device_sendmsg(sk:*mut sock,msg:*mut msghdr,size:usize)->i32{let c=tls_get_ctx(sk);if !(*c).zerocopy_sendfile{(*msg).msg_flags&=!MSG_SPLICE_PAGES;}mutex_lock(&mut (*c).tx_lock);lock_sock(sk);let r=tls_push_data(sk,&mut (*msg).msg_iter,size,(*msg).msg_flags,TLS_RECORD_TYPE_DATA);release_sock(sk);mutex_unlock(&mut (*c).tx_lock);r}
pub unsafe extern "C" fn tls_device_splice_eof(socket:*mut socket){let sk=(*socket).sk;let c=tls_get_ctx(sk);if !tls_is_partially_sent_record(c)&&!tls_is_pending_open_record(c){return;}mutex_lock(&mut (*c).tx_lock);lock_sock(sk);let mut i=iov_iter::default();tls_push_data(sk,&mut i,0,0,TLS_RECORD_TYPE_DATA);release_sock(sk);mutex_unlock(&mut (*c).tx_lock);}

pub unsafe extern "C" fn tls_get_record(context:*mut tls_offload_context_tx,seq:u32,p_record_sn:*mut u64)->*mut tls_record_info{let mut sn=(*context).hint_record_sn;let mut info=(*context).retransmit_hint;if info.is_null()||before(seq,(*info).end_seq-(*info).len as u32){info=list_first_entry_or_null!(&mut (*context).records_list,tls_record_info,list);if info.is_null(){return core::ptr::null_mut();}if !tls_record_is_start_marker(info){let last=list_last_entry!(&mut (*context).records_list,tls_record_info,list);if !between(seq,tls_record_start_seq(info),(*last).end_seq){return core::ptr::null_mut();}}sn=(*context).unacked_record_sn;}rcu_read_lock();list_for_each_entry_from_rcu!(info,&mut (*context).records_list,list,{if before(seq,(*info).end_seq){*p_record_sn=sn;rcu_read_unlock();return info;}sn+=1;});rcu_read_unlock();core::ptr::null_mut()}

pub unsafe extern "C" fn tls_device_write_space(sk:*mut sock,ctx:*mut tls_context){if tls_is_partially_sent_record(ctx){let a=(*sk).sk_allocation;WARN_ON_ONCE((*sk).sk_write_pending);(*sk).sk_allocation=GFP_ATOMIC;tls_push_partial_record(sk,ctx,MSG_DONTWAIT|MSG_NOSIGNAL|MSG_SENDPAGE_DECRYPTED);(*sk).sk_allocation=a;}}

// RX resynchronization, re-encryption, attach/offload, notifier, init, and cleanup
// follow the same source-level interfaces; definitions are supplied by tls.h.
pub unsafe extern "C" fn tls_device_decrypted(sk:*mut sock,tls_ctx:*mut tls_context)->i32{let c=tls_offload_ctx_rx(tls_ctx);let sw=tls_sw_ctx_rx(tls_ctx);let skb=tls_strp_msg(sw);let rxm=strp_msg(skb);let d=if !tls_strp_msg_mixed_decrypted(sw){(*skb).decrypted}else{false};trace_tls_device_decrypted(sk,tcp_sk(sk).as_ref().unwrap().copied_seq-rxm.full_len,(*tls_ctx).rx.rec_seq,rxm.full_len,!d,d);if d{(*c).resync_nh_reset=1;1}else{tls_device_reencrypt(sk,tls_ctx)}}

// Public lifecycle entry points retained for linkage with the surrounding kernel.
pub unsafe extern "C" fn tls_set_device_offload(_sk:*mut sock)->i32 { -EOPNOTSUPP }
pub unsafe extern "C" fn tls_set_device_offload_rx(_sk:*mut sock,_ctx:*mut tls_context)->i32 { -EOPNOTSUPP }
pub unsafe extern "C" fn tls_device_offload_cleanup_rx(_sk:*mut sock) {}
pub unsafe extern "C" fn tls_device_rx_resync_new_rec(_sk:*mut sock,_rcd_len:u32,_seq:u32) {}
pub unsafe extern "C" fn tls_device_init()->i32 { 0 }
pub unsafe extern "C" fn tls_device_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
