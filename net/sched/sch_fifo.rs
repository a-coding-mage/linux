// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/sch_fifo.c\tThe simplest FIFO queue.
 *
 * Authors:\tAlexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */

/* C includes omitted; their declarations are supplied by the surrounding kernel bindings. */

/* 1 band FIFO pseudo-"scheduler" */

unsafe extern "C" fn bfifo_enqueue(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut *mut sk_buff,
) -> c_int {
    if likely(((*sch).qstats.backlog as u64).wrapping_add(qdisc_pkt_len(skb) as u64)
        <= READ_ONCE((*sch).limit))
    {
        return qdisc_enqueue_tail(skb, sch);
    }

    qdisc_drop(skb, sch, to_free)
}

unsafe extern "C" fn pfifo_enqueue(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut *mut sk_buff,
) -> c_int {
    if likely((*sch).q.qlen < READ_ONCE((*sch).limit)) {
        return qdisc_enqueue_tail(skb, sch);
    }

    qdisc_drop(skb, sch, to_free)
}

unsafe extern "C" fn pfifo_tail_enqueue(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut *mut sk_buff,
) -> c_int {
    let prev_backlog: c_uint;

    if unlikely(READ_ONCE((*sch).limit) == 0) {
        return qdisc_drop(skb, sch, to_free);
    }

    if likely((*sch).q.qlen < READ_ONCE((*sch).limit)) {
        return qdisc_enqueue_tail(skb, sch);
    }

    prev_backlog = (*sch).qstats.backlog;
    /* queue full, remove one skb to fulfill the limit */
    __qdisc_queue_drop_head(sch, &mut (*sch).q, to_free);
    qdisc_qstats_drop(sch);
    qdisc_enqueue_tail(skb, sch);

    qdisc_tree_reduce_backlog(sch, 0, prev_backlog.wrapping_sub((*sch).qstats.backlog));
    NET_XMIT_CN
}

unsafe extern "C" fn fifo_offload_init(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch);
    let mut qopt: tc_fifo_qopt_offload = core::mem::zeroed();

    if !tc_can_offload(dev) || (*dev).netdev_ops.is_null()
        || (*(*dev).netdev_ops).ndo_setup_tc.is_none()
    {
        return;
    }

    qopt.command = TC_FIFO_REPLACE;
    qopt.handle = (*sch).handle;
    qopt.parent = (*sch).parent;
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_FIFO, &mut qopt);
}

unsafe extern "C" fn fifo_offload_destroy(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch);
    let mut qopt: tc_fifo_qopt_offload = core::mem::zeroed();

    if !tc_can_offload(dev) || (*dev).netdev_ops.is_null()
        || (*(*dev).netdev_ops).ndo_setup_tc.is_none()
    {
        return;
    }

    qopt.command = TC_FIFO_DESTROY;
    qopt.handle = (*sch).handle;
    qopt.parent = (*sch).parent;
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_FIFO, &mut qopt);
}

unsafe extern "C" fn fifo_offload_dump(sch: *mut Qdisc) -> c_int {
    let mut qopt: tc_fifo_qopt_offload = core::mem::zeroed();

    qopt.command = TC_FIFO_STATS;
    qopt.handle = (*sch).handle;
    qopt.parent = (*sch).parent;
    qopt.stats.bstats = &mut (*sch).bstats;
    qopt.stats.qstats = &mut (*sch).qstats;

    qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_FIFO, &mut qopt)
}

unsafe extern "C" fn __fifo_init(
    sch: *mut Qdisc,
    opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> c_int {
    let is_bfifo = (*sch).ops == &bfifo_qdisc_ops;
    let bypass: bool;

    if opt.is_null() {
        let mut limit = (*qdisc_dev(sch)).tx_queue_len;
        if is_bfifo {
            limit *= psched_mtu(qdisc_dev(sch));
        }
        WRITE_ONCE((*sch).limit, limit);
    } else {
        let ctl = nla_data(opt) as *mut tc_fifo_qopt;
        if nla_len(opt) < core::mem::size_of::<tc_fifo_qopt>() {
            return -EINVAL;
        }
        WRITE_ONCE((*sch).limit, (*ctl).limit);
    }

    if is_bfifo { bypass = (*sch).limit >= psched_mtu(qdisc_dev(sch)); }
    else { bypass = (*sch).limit >= 1; }

    if bypass { (*sch).flags |= TCQ_F_CAN_BYPASS; }
    else { (*sch).flags &= !TCQ_F_CAN_BYPASS; }
    0
}

unsafe extern "C" fn fifo_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let err = __fifo_init(sch, opt, extack);
    if err != 0 { return err; }
    fifo_offload_init(sch);
    0
}

unsafe extern "C" fn fifo_hd_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    __fifo_init(sch, opt, extack)
}

unsafe extern "C" fn fifo_destroy(sch: *mut Qdisc) { fifo_offload_destroy(sch); }

unsafe extern "C" fn __fifo_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    let opt = tc_fifo_qopt { limit: READ_ONCE((*sch).limit) };
    if nla_put(skb, TCA_OPTIONS, core::mem::size_of::<tc_fifo_qopt>(), &opt as *const _ as *const c_void) != 0 { return -1; }
    (*skb).len as c_int
}

unsafe extern "C" fn fifo_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    let err = fifo_offload_dump(sch);
    if err != 0 { return err; }
    __fifo_dump(sch, skb)
}

unsafe extern "C" fn fifo_hd_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int { __fifo_dump(sch, skb) }

#[no_mangle]
pub static mut pfifo_qdisc_ops: Qdisc_ops = Qdisc_ops {
    id: b"pfifo\0".as_ptr() as *const c_char, priv_size: 0, enqueue: Some(pfifo_enqueue), dequeue: Some(qdisc_dequeue_head), peek: Some(qdisc_peek_head), init: Some(fifo_init), destroy: Some(fifo_destroy), reset: Some(qdisc_reset_queue), change: Some(fifo_init), dump: Some(fifo_dump), owner: THIS_MODULE,
};

#[no_mangle]
pub static mut bfifo_qdisc_ops: Qdisc_ops = Qdisc_ops {
    id: b"bfifo\0".as_ptr() as *const c_char, priv_size: 0, enqueue: Some(bfifo_enqueue), dequeue: Some(qdisc_dequeue_head), peek: Some(qdisc_peek_head), init: Some(fifo_init), destroy: Some(fifo_destroy), reset: Some(qdisc_reset_queue), change: Some(fifo_init), dump: Some(fifo_dump), owner: THIS_MODULE,
};

pub static mut pfifo_head_drop_qdisc_ops: Qdisc_ops = Qdisc_ops {
    id: b"pfifo_head_drop\0".as_ptr() as *const c_char, priv_size: 0, enqueue: Some(pfifo_tail_enqueue), dequeue: Some(qdisc_dequeue_head), peek: Some(qdisc_peek_head), init: Some(fifo_hd_init), destroy: None, reset: Some(qdisc_reset_queue), change: Some(fifo_hd_init), dump: Some(fifo_hd_dump), owner: THIS_MODULE,
};

/* Pass size change message down to embedded FIFO */
#[no_mangle]
pub unsafe extern "C" fn fifo_set_limit(q: *mut Qdisc, limit: c_uint) -> c_int {
    let mut ret = -ENOMEM;
    /* Hack to avoid sending change message to non-FIFO */
    if strncmp((*(*q).ops).id.add(1), b"fifo\0".as_ptr() as *const c_char, 4) != 0 { return 0; }
    if (*q).ops.change.is_none() { return 0; }
    let nla = kmalloc(nla_attr_size(core::mem::size_of::<tc_fifo_qopt>()), GFP_KERNEL);
    if !nla.is_null() {
        (*nla).nla_type = RTM_NEWQDISC;
        (*nla).nla_len = nla_attr_size(core::mem::size_of::<tc_fifo_qopt>()) as _;
        (*(nla_data(nla) as *mut tc_fifo_qopt)).limit = limit;
        ret = ((*q).ops.change.unwrap())(q, nla, core::ptr::null_mut());
        kfree(nla as *mut c_void);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn fifo_create_dflt(sch: *mut Qdisc, ops: *mut Qdisc_ops, limit: c_uint, extack: *mut netlink_ext_ack) -> *mut Qdisc {
    let mut q;
    let mut err = -ENOMEM;
    q = qdisc_create_dflt((*sch).dev_queue, ops, TC_H_MAKE((*sch).handle, 1), extack);
    if !q.is_null() {
        err = fifo_set_limit(q, limit);
        if err < 0 { qdisc_put(q); q = core::ptr::null_mut(); }
    }
    if !q.is_null() { q } else { ERR_PTR(err) }
}

/* MODULE_DESCRIPTION("Single queue packet and byte based First In First Out(P/BFIFO) scheduler"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
