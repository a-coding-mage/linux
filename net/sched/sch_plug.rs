// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sch_plug.c Queue traffic until an explicit release command
 *
 * There are two ways to use this qdisc:
 * 1. A simple "instantaneous" plug/unplug operation, by issuing an alternating
 *    sequence of TCQ_PLUG_BUFFER & TCQ_PLUG_RELEASE_INDEFINITE commands.
 *
 * 2. For network output buffering (a.k.a output commit) functionality.
 *    Output commit property is commonly used by applications using checkpoint
 *    based fault-tolerance to ensure that the checkpoint from which a system
 *    is being restored is consistent w.r.t outside world.
 *
 *    Consider for e.g. Remus - a Virtual Machine checkpointing system,
 *    wherein a VM is checkpointed, say every 50ms. The checkpoint is replicated
 *    asynchronously to the backup host, while the VM continues executing the
 *    next epoch speculatively.
 *
 *    The following is a typical sequence of output buffer operations:
 *       1.At epoch i, start_buffer(i)
 *       2. At end of epoch i (i.e. after 50ms):
 *          2.1 Stop VM and take checkpoint(i).
 *          2.2 start_buffer(i+1) and Resume VM
 *       3. While speculatively executing epoch(i+1), asynchronously replicate
 *          checkpoint(i) to backup host.
 *       4. When checkpoint_ack(i) is received from backup, release_buffer(i)
 *    Thus, this Qdisc would receive the following sequence of commands:
 *       TCQ_PLUG_BUFFER (epoch i)
 *       .. TCQ_PLUG_BUFFER (epoch i+1)
 *       ....TCQ_PLUG_RELEASE_ONE (epoch i)
 *       ......TCQ_PLUG_BUFFER (epoch i+2)
 *       ........
 */

// C includes and symbols supplied by the kernel/networking environment are
// intentionally left as external Rust dependencies.

#[repr(C)]
struct plug_sched_data {
    /* If true, the dequeue function releases all packets
     * from head to end of the queue. The queue turns into
     * a pass-through queue for newly arriving packets.
     */
    unplug_indefinite: bool,

    throttled: bool,

    /* Queue Limit in bytes */
    limit: u32,

    /* Number of packets (output) from the current speculatively
     * executing epoch.
     */
    pkts_current_epoch: u32,

    /* Number of packets corresponding to the recently finished
     * epoch. These will be released when we receive a
     * TCQ_PLUG_RELEASE_ONE command. This command is typically
     * issued after committing a checkpoint at the target.
     */
    pkts_last_epoch: u32,

    /*
     * Number of packets from the head of the queue, that can
     * be released (committed checkpoint).
     */
    pkts_to_release: u32,
}

unsafe fn plug_enqueue(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut *mut sk_buff,
) -> i32 {
    let q = qdisc_priv(sch) as *mut plug_sched_data;

    if (likely(((*sch).qstats.backlog as u64) + (*skb).len as u64 <= (*q).limit as u64)) {
        if !(*q).unplug_indefinite {
            (*q).pkts_current_epoch += 1;
        }
        return qdisc_enqueue_tail(skb, sch);
    }

    qdisc_drop(skb, sch, to_free)
}

unsafe fn plug_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv(sch) as *mut plug_sched_data;

    if (*q).throttled {
        return core::ptr::null_mut();
    }

    if !(*q).unplug_indefinite {
        if (*q).pkts_to_release == 0 {
            /* No more packets to dequeue. Block the queue
             * and wait for the next release command.
             */
            (*q).throttled = true;
            return core::ptr::null_mut();
        }
        (*q).pkts_to_release -= 1;
    }

    qdisc_dequeue_head(sch)
}

unsafe fn plug_init(
    sch: *mut Qdisc,
    opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let q = qdisc_priv(sch) as *mut plug_sched_data;

    (*q).pkts_current_epoch = 0;
    (*q).pkts_last_epoch = 0;
    (*q).pkts_to_release = 0;
    (*q).unplug_indefinite = false;

    if opt.is_null() {
        (*q).limit = qdisc_dev(sch).tx_queue_len * psched_mtu(qdisc_dev(sch));
    } else {
        let ctl = nla_data(opt) as *mut tc_plug_qopt;

        if nla_len(opt) < core::mem::size_of::<tc_plug_qopt>() {
            return -EINVAL;
        }

        (*q).limit = (*ctl).limit;
    }

    (*q).throttled = true;
    0
}

/* Receives 4 types of messages:
 * TCQ_PLUG_BUFFER: Inset a plug into the queue and
 *  buffer any incoming packets
 * TCQ_PLUG_RELEASE_ONE: Dequeue packets from queue head
 *  to beginning of the next plug.
 * TCQ_PLUG_RELEASE_INDEFINITE: Dequeue all packets from queue.
 *  Stop buffering packets until the next TCQ_PLUG_BUFFER
 *  command is received (just act as a pass-thru queue).
 * TCQ_PLUG_LIMIT: Increase/decrease queue size
 */
unsafe fn plug_change(
    sch: *mut Qdisc,
    opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let q = qdisc_priv(sch) as *mut plug_sched_data;
    let msg = nla_data(opt) as *mut tc_plug_qopt;

    if nla_len(opt) < core::mem::size_of::<tc_plug_qopt>() {
        return -EINVAL;
    }

    match (*msg).action {
        TCQ_PLUG_BUFFER => {
            /* Save size of the current buffer */
            (*q).pkts_last_epoch = (*q).pkts_current_epoch;
            (*q).pkts_current_epoch = 0;
            if (*q).unplug_indefinite {
                (*q).throttled = true;
            }
            (*q).unplug_indefinite = false;
        }
        TCQ_PLUG_RELEASE_ONE => {
            /* Add packets from the last complete buffer to the
             * packets to be released set.
             */
            (*q).pkts_to_release += (*q).pkts_last_epoch;
            (*q).pkts_last_epoch = 0;
            (*q).throttled = false;
            netif_schedule_queue((*sch).dev_queue);
        }
        TCQ_PLUG_RELEASE_INDEFINITE => {
            (*q).unplug_indefinite = true;
            (*q).pkts_to_release = 0;
            (*q).pkts_last_epoch = 0;
            (*q).pkts_current_epoch = 0;
            (*q).throttled = false;
            netif_schedule_queue((*sch).dev_queue);
        }
        TCQ_PLUG_LIMIT => {
            /* Limit is supplied in bytes */
            (*q).limit = (*msg).limit;
        }
        _ => return -EINVAL,
    }

    0
}

static mut plug_qdisc_ops: Qdisc_ops = Qdisc_ops {
    id: "plug",
    priv_size: core::mem::size_of::<plug_sched_data>(),
    enqueue: Some(plug_enqueue),
    dequeue: Some(plug_dequeue),
    peek: Some(qdisc_peek_dequeued),
    init: Some(plug_init),
    change: Some(plug_change),
    reset: Some(qdisc_reset_queue),
    owner: THIS_MODULE,
};

// MODULE_ALIAS_NET_SCH("plug");

unsafe fn plug_module_init() -> i32 {
    register_qdisc(&raw mut plug_qdisc_ops)
}

unsafe fn plug_module_exit() {
    unregister_qdisc(&raw mut plug_qdisc_ops);
}

// module_init(plug_module_init)
// module_exit(plug_module_exit)
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Qdisc to plug and unplug traffic via netlink control");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
