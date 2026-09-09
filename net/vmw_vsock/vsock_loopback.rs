// SPDX-License-Identifier: GPL-2.0-only
/* loopback transport for vsock using virtio_transport_common APIs
 *
 * Copyright (C) 2013-2019 Red Hat, Inc.
 * Authors: Asias He <asias@redhat.com>
 *          Stefan Hajnoczi <stefanha@redhat.com>
 *          Stefano Garzarella <sgarzare@redhat.com>
 *
 */

#[repr(C)]
struct VsockLoopback {
    workqueue: *mut workqueue_struct,
    pkt_queue: sk_buff_head,
    pkt_work: work_struct,
}

static mut THE_VSOCK_LOOPBACK: VsockLoopback = VsockLoopback {
    workqueue: core::ptr::null_mut(),
    pkt_queue: sk_buff_head {},
    pkt_work: work_struct {},
};

unsafe fn vsock_loopback_get_local_cid() -> u32 {
    VMADDR_CID_LOCAL
}

unsafe fn vsock_loopback_send_pkt(skb: *mut sk_buff, _net: *mut net) -> i32 {
    let vsock = &mut THE_VSOCK_LOOPBACK;
    let len = (*skb).len as i32;

    virtio_vsock_skb_queue_tail(&mut vsock.pkt_queue, skb);
    queue_work(vsock.workqueue, &mut vsock.pkt_work);

    len
}

unsafe fn vsock_loopback_cancel_pkt(vsk: *mut vsock_sock) -> i32 {
    let vsock = &mut THE_VSOCK_LOOPBACK;

    virtio_transport_purge_skbs(vsk, &mut vsock.pkt_queue);

    0
}

unsafe extern "C" fn vsock_loopback_seqpacket_allow(
    vsk: *mut vsock_sock,
    remote_cid: u32,
) -> bool;

unsafe fn vsock_loopback_stream_allow(
    _vsk: *mut vsock_sock,
    _cid: u32,
    _port: u32,
) -> bool {
    true
}

unsafe fn vsock_loopback_msgzerocopy_allow() -> bool {
    true
}

static mut loopback_transport: virtio_transport = virtio_transport {
    transport: transport {
        module: THIS_MODULE,

        get_local_cid: Some(vsock_loopback_get_local_cid),

        init: Some(virtio_transport_do_socket_init),
        destruct: Some(virtio_transport_destruct),
        release: Some(virtio_transport_release),
        connect: Some(virtio_transport_connect),
        shutdown: Some(virtio_transport_shutdown),
        cancel_pkt: Some(vsock_loopback_cancel_pkt),

        dgram_bind: Some(virtio_transport_dgram_bind),
        dgram_dequeue: Some(virtio_transport_dgram_dequeue),
        dgram_enqueue: Some(virtio_transport_dgram_enqueue),
        dgram_allow: Some(virtio_transport_dgram_allow),

        stream_dequeue: Some(virtio_transport_stream_dequeue),
        stream_enqueue: Some(virtio_transport_stream_enqueue),
        stream_has_data: Some(virtio_transport_stream_has_data),
        stream_has_space: Some(virtio_transport_stream_has_space),
        stream_rcvhiwat: Some(virtio_transport_stream_rcvhiwat),
        stream_is_active: Some(virtio_transport_stream_is_active),
        stream_allow: Some(vsock_loopback_stream_allow),

        seqpacket_dequeue: Some(virtio_transport_seqpacket_dequeue),
        seqpacket_enqueue: Some(virtio_transport_seqpacket_enqueue),
        seqpacket_allow: Some(vsock_loopback_seqpacket_allow),
        seqpacket_has_data: Some(virtio_transport_seqpacket_has_data),

        msgzerocopy_allow: Some(vsock_loopback_msgzerocopy_allow),

        notify_poll_in: Some(virtio_transport_notify_poll_in),
        notify_poll_out: Some(virtio_transport_notify_poll_out),
        notify_recv_init: Some(virtio_transport_notify_recv_init),
        notify_recv_pre_block: Some(virtio_transport_notify_recv_pre_block),
        notify_recv_pre_dequeue: Some(virtio_transport_notify_recv_pre_dequeue),
        notify_recv_post_dequeue: Some(virtio_transport_notify_recv_post_dequeue),
        notify_send_init: Some(virtio_transport_notify_send_init),
        notify_send_pre_block: Some(virtio_transport_notify_send_pre_block),
        notify_send_pre_enqueue: Some(virtio_transport_notify_send_pre_enqueue),
        notify_send_post_enqueue: Some(virtio_transport_notify_send_post_enqueue),
        notify_buffer_size: Some(virtio_transport_notify_buffer_size),
        notify_set_rcvlowat: Some(virtio_transport_notify_set_rcvlowat),

        unsent_bytes: Some(virtio_transport_unsent_bytes),

        read_skb: Some(virtio_transport_read_skb),
    },

    send_pkt: Some(vsock_loopback_send_pkt),
};

unsafe fn vsock_loopback_seqpacket_allow(
    vsk: *mut vsock_sock,
    _remote_cid: u32,
) -> bool {
    vsock_net_mode_global(vsk)
}

unsafe fn vsock_loopback_work(work: *mut work_struct) {
    let vsock = container_of!(work, VsockLoopback, pkt_work);
    let mut pkts = sk_buff_head {};
    let mut skb: *mut sk_buff;

    skb_queue_head_init(&mut pkts);

    spin_lock_bh(&mut (*vsock).pkt_queue.lock);
    skb_queue_splice_init(&mut (*vsock).pkt_queue, &mut pkts);
    spin_unlock_bh(&mut (*vsock).pkt_queue.lock);

    while {
        skb = __skb_dequeue(&mut pkts);
        !skb.is_null()
    } {
        /* Decrement the bytes_unsent counter without deallocating skb
         * It is freed by the receiver.
         */
        virtio_transport_consume_skb_sent(skb, false);
        virtio_transport_deliver_tap_pkt(skb);
        virtio_transport_recv_pkt(
            &mut loopback_transport,
            skb,
            sock_net((*skb).sk),
        );
    }
}

unsafe fn vsock_loopback_init() -> i32 {
    let vsock = &mut THE_VSOCK_LOOPBACK;
    let ret: i32;

    vsock.workqueue = alloc_workqueue(c"vsock-loopback", WQ_PERCPU, 0);
    if vsock.workqueue.is_null() {
        return -ENOMEM;
    }

    skb_queue_head_init(&mut vsock.pkt_queue);
    INIT_WORK(&mut vsock.pkt_work, Some(vsock_loopback_work));

    ret = vsock_core_register(&mut loopback_transport.transport, VSOCK_TRANSPORT_F_LOCAL);
    if ret != 0 {
        destroy_workqueue(vsock.workqueue);
        return ret;
    }

    0
}

unsafe fn vsock_loopback_exit() {
    let vsock = &mut THE_VSOCK_LOOPBACK;

    vsock_core_unregister(&mut loopback_transport.transport);

    flush_work(&mut vsock.pkt_work);

    virtio_vsock_skb_queue_purge(&mut vsock.pkt_queue);

    destroy_workqueue(vsock.workqueue);
}

module_init!(vsock_loopback_init);
module_exit!(vsock_loopback_exit);
MODULE_LICENSE!("GPL v2");
MODULE_AUTHOR!("Stefano Garzarella <sgarzare@redhat.com>");
MODULE_DESCRIPTION!("loopback transport for vsock");
MODULE_ALIAS_NETPROTO!(PF_VSOCK);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
