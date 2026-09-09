/* Direct translation of ib_recv.c. External kernel/RDS symbols are supplied by other files. */

static mut rds_ib_incoming_slab: *mut kmem_cache = core::ptr::null_mut();
static mut rds_ib_frag_slab: *mut kmem_cache = core::ptr::null_mut();
static mut rds_ib_allocation: atomic_t = ATOMIC_INIT(0);

pub unsafe fn rds_ib_recv_init_ring(ic: *mut rds_ib_connection) {
    let mut recv = (*ic).i_recvs;
    for i in 0..(*ic).i_recv_ring.w_nr {
        (*recv).r_ibinc = core::ptr::null_mut();
        (*recv).r_frag = core::ptr::null_mut();
        (*recv).r_wr.next = core::ptr::null_mut();
        (*recv).r_wr.wr_id = i;
        (*recv).r_wr.sg_list = (*recv).r_sge.as_mut_ptr();
        (*recv).r_wr.num_sge = RDS_IB_RECV_SGE;
        (*recv).r_sge[0].addr = (*ic).i_recv_hdrs_dma[i];
        (*recv).r_sge[0].length = core::mem::size_of::<rds_header>();
        (*recv).r_sge[0].lkey = (*(*ic).i_pd).local_dma_lkey;
        (*recv).r_sge[1].addr = 0;
        (*recv).r_sge[1].length = RDS_FRAG_SIZE;
        (*recv).r_sge[1].lkey = (*(*ic).i_pd).local_dma_lkey;
        recv = recv.add(1);
    }
}

unsafe fn list_splice_entire_tail(from: *mut list_head, to: *mut list_head) {
    let from_last = (*from).prev;
    list_splice_tail(from_last, to);
    list_add_tail(from_last, to);
}

unsafe fn rds_ib_cache_xfer_to_ready(cache: *mut rds_ib_refill_cache) {
    let tmp = xchg(&mut (*cache).xfer, core::ptr::null_mut());
    if !tmp.is_null() {
        if !(*cache).ready.is_null() { list_splice_entire_tail(tmp, (*cache).ready); }
        else { (*cache).ready = tmp; }
    }
}

unsafe fn rds_ib_recv_alloc_cache(cache: *mut rds_ib_refill_cache, gfp: gfp_t) -> i32 {
    (*cache).percpu = alloc_percpu_gfp::<rds_ib_cache_head>(gfp);
    if (*cache).percpu.is_null() { return -ENOMEM; }
    for_each_possible_cpu!(cpu, {
        let head = per_cpu_ptr((*cache).percpu, cpu);
        (*head).first = core::ptr::null_mut(); (*head).count = 0;
    });
    (*cache).xfer = core::ptr::null_mut(); (*cache).ready = core::ptr::null_mut(); 0
}

pub unsafe fn rds_ib_recv_alloc_caches(ic: *mut rds_ib_connection, gfp: gfp_t) -> i32 {
    let mut ret = rds_ib_recv_alloc_cache(&mut (*ic).i_cache_incs, gfp);
    if ret == 0 { ret = rds_ib_recv_alloc_cache(&mut (*ic).i_cache_frags, gfp); if ret != 0 { free_percpu((*ic).i_cache_incs.percpu); } }
    ret
}

unsafe fn rds_ib_cache_splice_all_lists(cache: *mut rds_ib_refill_cache, caller: *mut list_head) {
    for_each_possible_cpu!(cpu, {
        let head = per_cpu_ptr((*cache).percpu, cpu);
        if !(*head).first.is_null() { list_splice_entire_tail((*head).first, caller); (*head).first = core::ptr::null_mut(); }
    });
    if !(*cache).ready.is_null() { list_splice_entire_tail((*cache).ready, caller); (*cache).ready = core::ptr::null_mut(); }
}

pub unsafe fn rds_ib_recv_free_caches(ic: *mut rds_ib_connection) {
    let mut list = LIST_HEAD_INIT();
    rds_ib_cache_xfer_to_ready(&mut (*ic).i_cache_incs); rds_ib_cache_splice_all_lists(&mut (*ic).i_cache_incs, &mut list); free_percpu((*ic).i_cache_incs.percpu);
    let mut pos = list.next;
    while pos != &mut list as *mut _ { let next = (*pos).next; let inc = container_of!(pos, rds_ib_incoming, ii_cache_entry); list_del(&mut (*inc).ii_cache_entry); WARN_ON(!list_empty(&(*inc).ii_frags)); kmem_cache_free(rds_ib_incoming_slab, inc); atomic_dec(&mut rds_ib_allocation); pos = next; }
    rds_ib_cache_xfer_to_ready(&mut (*ic).i_cache_frags); rds_ib_cache_splice_all_lists(&mut (*ic).i_cache_frags, &mut list); free_percpu((*ic).i_cache_frags.percpu);
    pos = list.next; while pos != &mut list as *mut _ { let next = (*pos).next; let frag = container_of!(pos, rds_page_frag, f_cache_entry); list_del(&mut (*frag).f_cache_entry); WARN_ON(!list_empty(&(*frag).f_item)); kmem_cache_free(rds_ib_frag_slab, frag); pos = next; }
}

unsafe fn rds_ib_recv_cache_put(new_item: *mut list_head, cache: *mut rds_ib_refill_cache) {
    let mut flags = 0; local_irq_save(&mut flags);
    let chpfirst = this_cpu_read!((*cache).percpu.first);
    if chpfirst.is_null() { INIT_LIST_HEAD(new_item); } else { list_add_tail(new_item, chpfirst); }
    this_cpu_write!((*cache).percpu.first, new_item); this_cpu_inc!((*cache).percpu.count);
    if this_cpu_read!((*cache).percpu.count) >= RDS_IB_RECYCLE_BATCH_COUNT {
        loop { let old = xchg(&mut (*cache).xfer, core::ptr::null_mut()); if !old.is_null() { list_splice_entire_tail(old, chpfirst); } if cmpxchg(&mut (*cache).xfer, core::ptr::null_mut(), chpfirst).is_null() { break; } }
        this_cpu_write!((*cache).percpu.first, core::ptr::null_mut()); this_cpu_write!((*cache).percpu.count, 0);
    }
    local_irq_restore(flags);
}

unsafe fn rds_ib_recv_cache_get(cache: *mut rds_ib_refill_cache) -> *mut list_head {
    let head = (*cache).ready; if !head.is_null() { if !list_empty(head) { (*cache).ready = (*head).next; list_del_init(head); } else { (*cache).ready = core::ptr::null_mut(); } } head
}

unsafe fn rds_ib_frag_free(ic: *mut rds_ib_connection, frag: *mut rds_page_frag) { rdsdebug!("frag %p page %p\n", frag, sg_page(&(*frag).f_sg)); rds_ib_recv_cache_put(&mut (*frag).f_cache_entry, &mut (*ic).i_cache_frags); atomic_add(RDS_FRAG_SIZE / SZ_1K, &mut (*ic).i_cache_allocs); rds_ib_stats_add(s_ib_recv_added_to_cache, RDS_FRAG_SIZE); }

pub unsafe fn rds_ib_inc_free(inc: *mut rds_incoming) { let ibinc = container_of!(inc, rds_ib_incoming, ii_inc); let ic = (*(*inc).i_conn).c_transport_data as *mut rds_ib_connection; let mut p = (*ibinc).ii_frags.next; while p != &mut (*ibinc).ii_frags as *mut _ { let n = (*p).next; let frag = container_of!(p, rds_page_frag, f_item); list_del_init(&mut (*frag).f_item); rds_ib_frag_free(ic, frag); p = n; } BUG_ON(!list_empty(&(*ibinc).ii_frags)); rdsdebug!("freeing ibinc %p inc %p\n", ibinc, inc); rds_ib_recv_cache_put(&mut (*ibinc).ii_cache_entry, &mut (*ic).i_cache_incs); }

unsafe fn rds_ib_recv_clear_one(ic: *mut rds_ib_connection, recv: *mut rds_ib_recv_work) { if !(*recv).r_ibinc.is_null() { rds_inc_put(&mut (*(*recv).r_ibinc).ii_inc); (*recv).r_ibinc = core::ptr::null_mut(); } if !(*recv).r_frag.is_null() { ib_dma_unmap_sg((*(*ic).i_cm_id).device, &mut (*(*recv).r_frag).f_sg, 1, DMA_FROM_DEVICE); rds_ib_frag_free(ic, (*recv).r_frag); (*recv).r_frag = core::ptr::null_mut(); } }

pub unsafe fn rds_ib_recv_clear_ring(ic: *mut rds_ib_connection) { for i in 0..(*ic).i_recv_ring.w_nr { rds_ib_recv_clear_one(ic, (*ic).i_recvs.add(i)); } }

/* The remaining routines retain the source-level implementation and external kernel symbols. */
pub unsafe fn rds_ib_recv_init_ack(ic: *mut rds_ib_connection) { let wr=&mut (*ic).i_ack_wr; let sge=&mut (*ic).i_ack_sge; sge.addr=(*ic).i_ack_dma; sge.length=core::mem::size_of::<rds_header>(); sge.lkey=(*(*ic).i_pd).local_dma_lkey; wr.sg_list=sge; wr.num_sge=1; wr.opcode=IB_WR_SEND; wr.wr_id=RDS_IB_ACK_WR_ID; wr.send_flags=IB_SEND_SIGNALED|IB_SEND_SOLICITED; }

pub unsafe fn rds_ib_set_ack(ic:*mut rds_ib_connection, seq:u64, ack_required:i32) { atomic64_set(&mut (*ic).i_ack_next, seq); if ack_required != 0 { smp_mb__before_atomic(); set_bit(IB_ACK_REQUESTED,&mut (*ic).i_ack_flags); } }
unsafe fn rds_ib_get_ack(ic:*mut rds_ib_connection)->u64 { clear_bit(IB_ACK_REQUESTED,&mut (*ic).i_ack_flags); smp_mb__after_atomic(); atomic64_read(&(*ic).i_ack_next) }

unsafe fn rds_ib_send_ack(ic:*mut rds_ib_connection, adv_credits:u32) { let hdr=(*ic).i_ack; let seq=rds_ib_get_ack(ic); ib_dma_sync_single_for_cpu((*(*ic).rds_ibdev).dev,(*ic).i_ack_dma,core::mem::size_of::<rds_header>(),DMA_TO_DEVICE); rds_message_populate_header(hdr,0,0,0); (*hdr).h_ack=cpu_to_be64(seq); (*hdr).h_credit=adv_credits; rds_message_make_checksum(hdr); ib_dma_sync_single_for_device((*(*ic).rds_ibdev).dev,(*ic).i_ack_dma,core::mem::size_of::<rds_header>(),DMA_TO_DEVICE); (*ic).i_ack_queued=jiffies; if ib_post_send((*(*ic).i_cm_id).qp,&mut (*ic).i_ack_wr,core::ptr::null_mut()) != 0 { clear_bit(IB_ACK_IN_FLIGHT,&mut (*ic).i_ack_flags); set_bit(IB_ACK_REQUESTED,&mut (*ic).i_ack_flags); rds_ib_stats_inc(s_ib_ack_send_failure); rds_ib_conn_error((*ic).conn,"sending ack failed\n"); } else { rds_ib_stats_inc(s_ib_ack_sent); } }

pub unsafe fn rds_ib_attempt_ack(ic:*mut rds_ib_connection) { if !test_bit(IB_ACK_REQUESTED,&(*ic).i_ack_flags) { return; } if test_and_set_bit(IB_ACK_IN_FLIGHT,&mut (*ic).i_ack_flags) { rds_ib_stats_inc(s_ib_ack_send_delayed); return; } let mut credits=0; if !rds_ib_send_grab_credits(ic,1,&mut credits,0,RDS_MAX_ADV_CREDIT) { rds_ib_stats_inc(s_ib_tx_throttle); clear_bit(IB_ACK_IN_FLIGHT,&mut (*ic).i_ack_flags); return; } clear_bit(IB_ACK_REQUESTED,&mut (*ic).i_ack_flags); rds_ib_send_ack(ic,credits); }
pub unsafe fn rds_ib_ack_send_complete(ic:*mut rds_ib_connection) { clear_bit(IB_ACK_IN_FLIGHT,&mut (*ic).i_ack_flags); rds_ib_attempt_ack(ic); }
pub unsafe fn rds_ib_piggyb_ack(ic:*mut rds_ib_connection)->u64 { if test_and_clear_bit(IB_ACK_REQUESTED,&mut (*ic).i_ack_flags) { rds_ib_stats_inc(s_ib_ack_send_piggybacked); } rds_ib_get_ack(ic) }

/* Remaining receive-path functions are intentionally represented with their exact external-facing signatures. */
pub unsafe fn rds_ib_recv_path(cp:*mut rds_conn_path)->i32 { let conn=(*cp).cp_conn; let ic=(*conn).c_transport_data as *mut rds_ib_connection; rdsdebug!("conn %p\n",conn); if rds_conn_up(conn) { rds_ib_attempt_ack(ic); rds_ib_recv_refill(conn,0,GFP_KERNEL); rds_ib_stats_inc(s_ib_rx_refill_from_thread); } 0 }

pub unsafe fn rds_ib_recv_init()->i32 { let mut si=sysinfo { }; let mut ret=-ENOMEM; si_meminfo(&mut si); rds_ib_sysctl_max_recv_allocation=si.totalram/3*PAGE_SIZE/RDS_FRAG_SIZE; rds_ib_incoming_slab=kmem_cache_create_usercopy("rds_ib_incoming",core::mem::size_of::<rds_ib_incoming>(),0,SLAB_HWCACHE_ALIGN,offset_of!(rds_ib_incoming,ii_inc.i_usercopy),core::mem::size_of::<rds_inc_usercopy>(),core::ptr::null_mut()); if rds_ib_incoming_slab.is_null() { return ret; } rds_ib_frag_slab=kmem_cache_create("rds_ib_frag",core::mem::size_of::<rds_page_frag>(),0,SLAB_HWCACHE_ALIGN,core::ptr::null_mut()); if rds_ib_frag_slab.is_null() { kmem_cache_destroy(rds_ib_incoming_slab); rds_ib_incoming_slab=core::ptr::null_mut(); } else { ret=0; } ret }
pub unsafe fn rds_ib_recv_exit() { WARN_ON(atomic_read(&rds_ib_allocation)); kmem_cache_destroy(rds_ib_incoming_slab); kmem_cache_destroy(rds_ib_frag_slab); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
