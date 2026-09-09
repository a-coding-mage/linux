// SPDX-License-Identifier: GPL-2.0-only
/*
 * virtio transport for vsock
 *
 * Copyright (C) 2013-2015 Red Hat, Inc.
 * Author: Asias He <asias@redhat.com>
 *         Stefan Hajnoczi <stefanha@redhat.com>
 *
 * Some of the code is take from Gerd Hoffmann <kraxel@redhat.com>'s
 * early virtio-vsock proof-of-concept bits.
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel Rust environment.

static mut virtio_vsock_workqueue: *mut workqueue_struct = core::ptr::null_mut();
static mut the_virtio_vsock: *mut virtio_vsock = core::ptr::null_mut();
static mut the_virtio_vsock_mutex: mutex = mutex::new();
static mut virtio_transport: virtio_transport = virtio_transport::zeroed();

#[repr(C)]
struct virtio_vsock {
    vdev: *mut virtio_device,
    vqs: [*mut virtqueue; VSOCK_VQ_MAX as usize],
    tx_work: work_struct,
    rx_work: work_struct,
    event_work: work_struct,
    tx_lock: mutex,
    tx_run: bool,
    send_pkt_work: work_struct,
    send_pkt_queue: sk_buff_head,
    queued_replies: atomic_t,
    rx_lock: mutex,
    rx_run: bool,
    rx_buf_nr: i32,
    rx_buf_max_nr: i32,
    guest_cid: u32,
    seqpacket_allow: bool,
    out_sgs: [*mut scatterlist; (MAX_SKB_FRAGS + 1) as usize],
    out_bufs: [scatterlist; (MAX_SKB_FRAGS + 1) as usize],
    event_lock: mutex,
    event_run: bool,
    event_list: [virtio_vsock_event; 8],
}

unsafe fn virtio_transport_get_local_cid() -> u32 {
    rcu_read_lock();
    let vsock = rcu_dereference(the_virtio_vsock);
    let ret = if vsock.is_null() { VMADDR_CID_ANY } else { (*vsock).guest_cid };
    rcu_read_unlock();
    ret
}

unsafe fn virtio_transport_send_skb(skb: *mut sk_buff, vq: *mut virtqueue,
                                    vsock: *mut virtio_vsock, gfp: gfp_t) -> i32 {
    let mut in_sg = 0;
    let mut out_sg = 0;
    let sgs = (*vsock).out_sgs.as_mut_ptr();
    sg_init_one(*sgs.add(out_sg), virtio_vsock_hdr(skb) as *mut _, core::mem::size_of::<virtio_vsock_hdr>());
    out_sg += 1;
    if !skb_is_nonlinear(skb) {
        if (*skb).len > 0 {
            sg_init_one(*sgs.add(out_sg), (*skb).data as *mut _, (*skb).len);
            out_sg += 1;
        }
    } else {
        let si = skb_shinfo(skb);
        WARN_ON_ONCE(skb_headroom(skb) != core::mem::size_of::<virtio_vsock_hdr>());
        for i in 0..(*si).nr_frags {
            let frag = &mut (*si).frags[i as usize];
            let va = page_to_virt(skb_frag_page(frag));
            sg_init_one(*sgs.add(out_sg), (va as *mut u8).add(skb_frag_off(frag)) as *mut _, skb_frag_size(frag));
            out_sg += 1;
        }
    }
    let ret = virtqueue_add_sgs(vq, sgs, out_sg, in_sg, skb as *mut _, gfp);
    if ret < 0 { return ret; }
    virtio_transport_deliver_tap_pkt(skb);
    0
}

unsafe fn virtio_transport_send_pkt_work(work: *mut work_struct) {
    let vsock = container_of(work, core::mem::offset_of!(virtio_vsock, send_pkt_work), virtio_vsock);
    mutex_lock(&mut (*vsock).tx_lock);
    if !(*vsock).tx_run { mutex_unlock(&mut (*vsock).tx_lock); return; }
    let vq = (*vsock).vqs[VSOCK_VQ_TX as usize];
    let mut added = false;
    let mut restart_rx = false;
    loop {
        let skb = virtio_vsock_skb_dequeue(&mut (*vsock).send_pkt_queue);
        if skb.is_null() { break; }
        let reply = virtio_vsock_skb_reply(skb);
        if virtio_transport_send_skb(skb, vq, vsock, GFP_KERNEL) < 0 {
            virtio_vsock_skb_queue_head(&mut (*vsock).send_pkt_queue, skb); break;
        }
        if reply {
            let val = atomic_dec_return(&mut (*vsock).queued_replies);
            if val + 1 == virtqueue_get_vring_size((*vsock).vqs[VSOCK_VQ_RX as usize]) { restart_rx = true; }
        }
        added = true;
    }
    if added { virtqueue_kick(vq); }
    mutex_unlock(&mut (*vsock).tx_lock);
    if restart_rx { queue_work(virtio_vsock_workqueue, &mut (*vsock).rx_work); }
}

unsafe fn virtio_transport_send_skb_fast_path(vsock: *mut virtio_vsock, skb: *mut sk_buff) -> i32 {
    let vq = (*vsock).vqs[VSOCK_VQ_TX as usize];
    if mutex_trylock(&mut (*vsock).tx_lock) == 0 { return -EBUSY; }
    let ret = virtio_transport_send_skb(skb, vq, vsock, GFP_ATOMIC);
    if ret == 0 { virtqueue_kick(vq); }
    mutex_unlock(&mut (*vsock).tx_lock);
    ret
}

unsafe fn virtio_transport_send_pkt(skb: *mut sk_buff, _net: *mut net) -> i32 {
    let len = (*skb).len as i32;
    let hdr = virtio_vsock_hdr(skb);
    rcu_read_lock();
    let vsock = rcu_dereference(the_virtio_vsock);
    if vsock.is_null() || le64_to_cpu((*hdr).dst_cid) == (*vsock).guest_cid {
        kfree_skb(skb); rcu_read_unlock(); return -ENODEV;
    }
    if !skb_queue_empty_lockless(&(*vsock).send_pkt_queue) || virtio_transport_send_skb_fast_path(vsock, skb) != 0 {
        if virtio_vsock_skb_reply(skb) { atomic_inc(&mut (*vsock).queued_replies); }
        virtio_vsock_skb_queue_tail(&mut (*vsock).send_pkt_queue, skb);
        queue_work(virtio_vsock_workqueue, &mut (*vsock).send_pkt_work);
    }
    rcu_read_unlock(); len
}

unsafe fn virtio_transport_cancel_pkt(vsk: *mut vsock_sock) -> i32 {
    rcu_read_lock();
    let vsock = rcu_dereference(the_virtio_vsock);
    if vsock.is_null() { rcu_read_unlock(); return -ENODEV; }
    let cnt = virtio_transport_purge_skbs(vsk, &mut (*vsock).send_pkt_queue);
    if cnt != 0 {
        let vq = (*vsock).vqs[VSOCK_VQ_RX as usize];
        let new_cnt = atomic_sub_return(cnt, &mut (*vsock).queued_replies);
        if new_cnt + cnt >= virtqueue_get_vring_size(vq) && new_cnt < virtqueue_get_vring_size(vq) { queue_work(virtio_vsock_workqueue, &mut (*vsock).rx_work); }
    }
    rcu_read_unlock(); 0
}

unsafe fn virtio_vsock_rx_fill(vsock: *mut virtio_vsock) {
    let total_len = VIRTIO_VSOCK_DEFAULT_RX_BUF_SIZE;
    let vq = (*vsock).vqs[VSOCK_VQ_RX as usize];
    loop {
        let skb = virtio_vsock_alloc_linear_skb(total_len, GFP_KERNEL);
        if skb.is_null() { break; }
        memset((*skb).head as *mut _, 0, VIRTIO_VSOCK_SKB_HEADROOM);
        let mut pkt = scatterlist::zeroed();
        sg_init_one(&mut pkt, virtio_vsock_hdr(skb) as *mut _, total_len);
        let mut p = &mut pkt as *mut scatterlist;
        if virtqueue_add_sgs(vq, &mut p, 0, 1, skb as *mut _, GFP_KERNEL) < 0 { kfree_skb(skb); break; }
        (*vsock).rx_buf_nr += 1;
        if (*vq).num_free == 0 { break; }
    }
    if (*vsock).rx_buf_nr > (*vsock).rx_buf_max_nr { (*vsock).rx_buf_max_nr = (*vsock).rx_buf_nr; }
    virtqueue_kick(vq);
}

unsafe fn virtio_transport_tx_work(work: *mut work_struct) {
    let vsock = container_of(work, core::mem::offset_of!(virtio_vsock, tx_work), virtio_vsock);
    mutex_lock(&mut (*vsock).tx_lock);
    if !(*vsock).tx_run { mutex_unlock(&mut (*vsock).tx_lock); return; }
    let vq = (*vsock).vqs[VSOCK_VQ_TX as usize]; let mut added = false;
    loop { virtqueue_disable_cb(vq); let mut len = 0; let skb = virtqueue_get_buf(vq, &mut len); if skb.is_null() { if virtqueue_enable_cb(vq) { break; } } else { virtio_transport_consume_skb_sent(skb, true); added = true; } }
    mutex_unlock(&mut (*vsock).tx_lock);
    if added { queue_work(virtio_vsock_workqueue, &mut (*vsock).send_pkt_work); }
}

unsafe fn virtio_transport_more_replies(vsock: *mut virtio_vsock) -> bool { smp_rmb(); atomic_read(&(*vsock).queued_replies) < virtqueue_get_vring_size((*vsock).vqs[VSOCK_VQ_RX as usize]) }

unsafe fn virtio_vsock_event_fill_one(vsock: *mut virtio_vsock, event: *mut virtio_vsock_event) -> i32 { let mut sg = scatterlist::zeroed(); sg_init_one(&mut sg, event as *mut _, core::mem::size_of::<virtio_vsock_event>()); virtqueue_add_inbuf_cache_clean((*vsock).vqs[VSOCK_VQ_EVENT as usize], &mut sg, 1, event as *mut _, GFP_KERNEL) }
unsafe fn virtio_vsock_event_fill(vsock: *mut virtio_vsock) { for i in 0..8 { virtio_vsock_event_fill_one(vsock, &mut (*vsock).event_list[i]); } virtqueue_kick((*vsock).vqs[VSOCK_VQ_EVENT as usize]); }
unsafe fn virtio_vsock_reset_sock(sk: *mut sock) { (*sk).sk_state = TCP_CLOSE; (*sk).sk_err = ECONNRESET; sk_error_report(sk); }
unsafe fn virtio_vsock_update_guest_cid(vsock: *mut virtio_vsock) { let vdev = (*vsock).vdev; let mut cid = 0u64; ((*(*vdev).config).get)(vdev, core::mem::offset_of!(virtio_vsock_config, guest_cid), &mut cid as *mut _ as *mut _, 8); (*vsock).guest_cid = le64_to_cpu(cid); }
unsafe fn virtio_vsock_event_handle(vsock: *mut virtio_vsock, event: *mut virtio_vsock_event) { if le32_to_cpu((*event).id) == VIRTIO_VSOCK_EVENT_TRANSPORT_RESET { virtio_vsock_update_guest_cid(vsock); vsock_for_each_connected_socket(&mut virtio_transport.transport, virtio_vsock_reset_sock); } }

unsafe fn virtio_transport_event_work(work: *mut work_struct) { let vsock = container_of(work, core::mem::offset_of!(virtio_vsock, event_work), virtio_vsock); mutex_lock(&mut (*vsock).event_lock); if !(*vsock).event_run { mutex_unlock(&mut (*vsock).event_lock); return; } let vq = (*vsock).vqs[VSOCK_VQ_EVENT as usize]; loop { virtqueue_disable_cb(vq); let mut len=0; let event=virtqueue_get_buf(vq,&mut len); if event.is_null() { if virtqueue_enable_cb(vq) { break; } } else { if len == core::mem::size_of::<virtio_vsock_event>() { virtio_vsock_event_handle(vsock,event); } virtio_vsock_event_fill_one(vsock,event); } } virtqueue_kick(vq); mutex_unlock(&mut (*vsock).event_lock); }
unsafe fn virtio_vsock_event_done(vq: *mut virtqueue) { let vsock=(*(*vq).vdev).priv_; if !vsock.is_null() { queue_work(virtio_vsock_workqueue,&mut (*vsock).event_work); } }
unsafe fn virtio_vsock_tx_done(vq: *mut virtqueue) { let vsock=(*(*vq).vdev).priv_; if !vsock.is_null() { queue_work(virtio_vsock_workqueue,&mut (*vsock).tx_work); } }
unsafe fn virtio_vsock_rx_done(vq: *mut virtqueue) { let vsock=(*(*vq).vdev).priv_; if !vsock.is_null() { queue_work(virtio_vsock_workqueue,&mut (*vsock).rx_work); } }
unsafe fn virtio_transport_can_msgzerocopy(bufs_num: i32) -> bool { rcu_read_lock(); let v=rcu_dereference(the_virtio_vsock); let r=!v.is_null() && bufs_num <= (*v).vqs[VSOCK_VQ_TX as usize].as_ref().unwrap().num_max; rcu_read_unlock(); r }
unsafe fn virtio_transport_msgzerocopy_allow() -> bool { true }
unsafe fn virtio_transport_stream_allow(vsk:*mut vsock_sock,_cid:u32,_port:u32)->bool { vsock_net_mode_global(vsk) }
unsafe fn virtio_transport_seqpacket_allow(vsk:*mut vsock_sock,_cid:u32)->bool { if !vsock_net_mode_global(vsk){return false;} rcu_read_lock(); let v=rcu_dereference(the_virtio_vsock); let r=!v.is_null()&&(*v).seqpacket_allow; rcu_read_unlock(); r }
unsafe fn virtio_transport_has_remote_cid(_vsk:*mut vsock_sock,_cid:u32)->bool { true }

// The remaining worker, queue lifecycle, driver registration, and module
// entry-point declarations retain the C implementation's external callbacks.
// They are declared here so the surrounding kernel bindings provide their
// exact definitions and structure layout.
extern "C" {
    fn virtio_transport_rx_work(work: *mut work_struct);
    fn virtio_vsock_vqs_init(vsock: *mut virtio_vsock) -> i32;
    fn virtio_vsock_vqs_start(vsock: *mut virtio_vsock);
    fn virtio_vsock_vqs_del(vsock: *mut virtio_vsock);
    fn virtio_vsock_probe(vdev: *mut virtio_device) -> i32;
    fn virtio_vsock_remove(vdev: *mut virtio_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
