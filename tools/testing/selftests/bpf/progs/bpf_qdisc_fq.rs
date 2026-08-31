// SPDX-License-Identifier: GPL-2.0

/* bpf_fq is intended for testing the bpf qdisc infrastructure and not a direct
 * copy of sch_fq. bpf_fq implements the scheduling algorithm of sch_fq before
 * 29f834aa326e ("net_sched: sch_fq: add 3 bands and WRR scheduling") was
 * introduced. It gives each flow a fair chance to transmit packets in a
 * round-robin fashion. Note that for flow pacing, bpf_fq currently only
 * respects skb->tstamp but not skb->sk->sk_pacing_rate. In addition, if there
 * are multiple bpf_fq instances, they will have a shared view of flows and
 * configuration since some key data structure such as fq_prio_flows,
 * fq_nonprio_flows, and fq_bpf_data are global.
 *
 * To use bpf_fq alone without running selftests, use the following commands.
 *
 * 1. Register bpf_fq to the kernel
 *     bpftool struct_ops register bpf_qdisc_fq.bpf.o /sys/fs/bpf
 * 2. Add bpf_fq to an interface
 *     tc qdisc add dev <interface name> root handle <handle> bpf_fq
 * 3. Delete bpf_fq attached to the interface
 *     tc qdisc delete dev <interface name> root
 * 4. Unregister bpf_fq
 *     bpftool struct_ops unregister name fq
 *
 * The qdisc name, bpf_fq, used in tc commands is defined by Qdisc_ops.id.
 * The struct_ops_map_name, fq, used in the bpftool command is the name of the
 * Qdisc_ops.
 *
 * SEC(".struct_ops")
 * struct Qdisc_ops fq = {
 *         ...
 *         .id        = "bpf_fq",
 * };
 */

// C dependencies: <vmlinux.h>, <errno.h>, <bpf/bpf_helpers.h>,
// "bpf_experimental.h", "bpf_qdisc_common.h".

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

const NSEC_PER_USEC: i64 = 1000;
const NSEC_PER_SEC: i64 = 1000000000;

const NUM_QUEUE: u32 = 1 << 20;

#[repr(C)]
pub struct fq_bpf_data {
    pub quantum: u32,
    pub initial_quantum: u32,
    pub flow_refill_delay: u32,
    pub flow_plimit: u32,
    pub horizon: u64,
    pub orphan_mask: u32,
    pub timer_slack: u32,
    pub time_next_delayed_flow: u64,
    pub unthrottle_latency_ns: u64,
    pub horizon_drop: u8,
    pub new_flow_cnt: u32,
    pub old_flow_cnt: u32,
    pub ktime_cache: u64,
}

const CLS_RET_PRIO: i32 = 0;
const CLS_RET_NONPRIO: i32 = 1;
const CLS_RET_ERR: i32 = 2;

#[repr(C)]
pub struct skb_node {
    pub tstamp: u64,
    pub skb: *mut sk_buff,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct fq_flow_node {
    pub credit: i32,
    pub qlen: u32,
    pub age: u64,
    pub time_next_packet: u64,
    pub list_node: bpf_list_node,
    pub rb_node: bpf_rb_node,
    pub queue: bpf_rb_root,
    pub lock: bpf_spin_lock,
    pub refcount: bpf_refcount,
}

#[repr(C)]
pub struct dequeue_nonprio_ctx {
    pub stop_iter: bool,
    pub expire: u64,
    pub now: u64,
}

#[repr(C)]
pub struct remove_flows_ctx {
    pub gc_only: bool,
    pub reset_cnt: u32,
    pub reset_max: u32,
}

#[repr(C)]
pub struct unset_throttled_flows_ctx {
    pub unset_all: bool,
    pub now: u64,
}

#[repr(C)]
pub struct fq_stashed_flow {
    pub flow: *mut fq_flow_node,
}

// Map definitions translated from BPF map declaration macros.
#[link_section = ".maps"]
static mut fq_nonprio_flows: bpf_map = bpf_map {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: NUM_QUEUE,
};

#[link_section = ".maps"]
static mut fq_prio_flows: bpf_map = bpf_map {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
};

static mut fq_delayed_lock: bpf_spin_lock = unsafe { core::mem::zeroed() };
static mut fq_delayed: bpf_rb_root = unsafe { core::mem::zeroed() };

static mut fq_new_flows_lock: bpf_spin_lock = unsafe { core::mem::zeroed() };
static mut fq_new_flows: bpf_list_head = unsafe { core::mem::zeroed() };

static mut fq_old_flows_lock: bpf_spin_lock = unsafe { core::mem::zeroed() };
static mut fq_old_flows: bpf_list_head = unsafe { core::mem::zeroed() };

static mut q: fq_bpf_data = unsafe { core::mem::zeroed() };

unsafe fn bpf_kptr_xchg_back(map_val: *mut core::ffi::c_void, ptr: *mut core::ffi::c_void) {
    let ret: *mut core::ffi::c_void;

    ret = bpf_kptr_xchg(map_val, ptr);
    if !ret.is_null() {
        bpf_obj_drop(ret);
    }
}

unsafe fn skbn_tstamp_less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let skbn_a: *mut skb_node;
    let skbn_b: *mut skb_node;

    skbn_a = container_of_skb_node_node(a);
    skbn_b = container_of_skb_node_node(b as *mut bpf_rb_node);

    (*skbn_a).tstamp < (*skbn_b).tstamp
}

unsafe fn fn_time_next_packet_less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let flow_a: *mut fq_flow_node;
    let flow_b: *mut fq_flow_node;

    flow_a = container_of_fq_flow_node_rb_node(a);
    flow_b = container_of_fq_flow_node_rb_node(b as *mut bpf_rb_node);

    (*flow_a).time_next_packet < (*flow_b).time_next_packet
}

unsafe fn fq_flows_add_head(
    head: *mut bpf_list_head,
    lock: *mut bpf_spin_lock,
    flow: *mut fq_flow_node,
    flow_cnt: *mut u32,
) {
    bpf_spin_lock(lock);
    bpf_list_push_front(head, &mut (*flow).list_node);
    bpf_spin_unlock(lock);
    *flow_cnt = (*flow_cnt).wrapping_add(1);
}

unsafe fn fq_flows_add_tail(
    head: *mut bpf_list_head,
    lock: *mut bpf_spin_lock,
    flow: *mut fq_flow_node,
    flow_cnt: *mut u32,
) {
    bpf_spin_lock(lock);
    bpf_list_push_back(head, &mut (*flow).list_node);
    bpf_spin_unlock(lock);
    *flow_cnt = (*flow_cnt).wrapping_add(1);
}

unsafe fn fq_flows_remove_front(
    head: *mut bpf_list_head,
    lock: *mut bpf_spin_lock,
    node: *mut *mut bpf_list_node,
    flow_cnt: *mut u32,
) {
    bpf_spin_lock(lock);
    *node = bpf_list_pop_front(head);
    bpf_spin_unlock(lock);
    *flow_cnt = (*flow_cnt).wrapping_sub(1);
}

unsafe fn fq_flows_is_empty(head: *mut bpf_list_head, lock: *mut bpf_spin_lock) -> bool {
    let empty: bool;

    bpf_spin_lock(lock);
    empty = bpf_list_empty(head);
    bpf_spin_unlock(lock);

    empty
}

/* flow->age is used to denote the state of the flow (not-detached, detached, throttled)
 * as well as the timestamp when the flow is detached.
 *
 * 0: not-detached
 * 1 - (~0ULL-1): detached
 * ~0ULL: throttled
 */
unsafe fn fq_flow_set_detached(flow: *mut fq_flow_node) {
    (*flow).age = bpf_jiffies64();
}

unsafe fn fq_flow_is_detached(flow: *mut fq_flow_node) -> bool {
    (*flow).age != 0 && (*flow).age != !0u64
}

unsafe fn sk_listener(sk: *mut sock) -> bool {
    ((1u32 << (*sk).__sk_common.skc_state) & (TCPF_LISTEN | TCPF_NEW_SYN_RECV)) != 0
}

unsafe fn fq_gc();

unsafe fn fq_new_flow(flow_map: *mut bpf_map, sflow: *mut *mut fq_stashed_flow, hash: u64) -> i32 {
    let tmp: fq_stashed_flow = core::mem::zeroed();
    let flow: *mut fq_flow_node;
    let mut ret: i32;

    flow = bpf_obj_new_fq_flow_node();
    if flow.is_null() {
        return -ENOMEM;
    }

    (*flow).credit = q.initial_quantum as i32;
    (*flow).qlen = 0;
    (*flow).age = 1;
    (*flow).time_next_packet = 0;

    ret = bpf_map_update_elem(flow_map, &hash as *const _ as *const core::ffi::c_void, &tmp as *const _ as *const core::ffi::c_void, 0);
    if ret == -ENOMEM || ret == -E2BIG {
        fq_gc();
        bpf_map_update_elem(&mut fq_nonprio_flows, &hash as *const _ as *const core::ffi::c_void, &tmp as *const _ as *const core::ffi::c_void, 0);
    }

    *sflow = bpf_map_lookup_elem(flow_map, &hash as *const _ as *const core::ffi::c_void) as *mut fq_stashed_flow;
    if (*sflow).is_null() {
        bpf_obj_drop(flow as *mut core::ffi::c_void);
        return -ENOMEM;
    }

    bpf_kptr_xchg_back(&mut (**sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);
    0
}

unsafe fn fq_classify(skb: *mut sk_buff, sflow: *mut *mut fq_stashed_flow) -> i32 {
    let sk: *mut sock = (*skb).sk;
    let mut ret: i32 = CLS_RET_NONPRIO;
    let mut hash: u64 = 0;

    if ((*skb).priority & TC_PRIO_MAX) == TC_PRIO_CONTROL {
        *sflow = bpf_map_lookup_elem(&mut fq_prio_flows, &hash as *const _ as *const core::ffi::c_void) as *mut fq_stashed_flow;
        ret = CLS_RET_PRIO;
    } else {
        if sk.is_null() || sk_listener(sk) {
            hash = (bpf_skb_get_hash(skb) & q.orphan_mask) as u64;
            /* Avoid collision with an existing flow hash, which
             * only uses the lower 32 bits of hash, by setting the
             * upper half of hash to 1.
             */
            hash |= 1u64 << 32;
        } else if (*sk).__sk_common.skc_state == TCP_CLOSE {
            hash = (bpf_skb_get_hash(skb) & q.orphan_mask) as u64;
            hash |= 1u64 << 32;
        } else {
            hash = (*sk).__sk_common.skc_hash as u64;
        }
        *sflow = bpf_map_lookup_elem(&mut fq_nonprio_flows, &hash as *const _ as *const core::ffi::c_void) as *mut fq_stashed_flow;
    }

    if (*sflow).is_null() {
        ret = if fq_new_flow(&mut fq_nonprio_flows, sflow, hash) < 0 {
            CLS_RET_ERR
        } else {
            CLS_RET_NONPRIO
        };
    }

    ret
}

unsafe fn fq_packet_beyond_horizon(skb: *mut sk_buff) -> bool {
    (*skb).tstamp as i64 > q.ktime_cache.wrapping_add(q.horizon) as i64
}

#[link_section = "struct_ops/bpf_fq_enqueue"]
pub unsafe extern "C" fn bpf_fq_enqueue(
    mut skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut flow: *mut fq_flow_node = core::ptr::null_mut();
    let flow_copy: *mut fq_flow_node;
    let mut sflow: *mut fq_stashed_flow = core::ptr::null_mut();
    let time_to_send: u64;
    let jiffies: u64;
    let skbn: *mut skb_node;
    let ret: i32;

    if (*sch).q.qlen >= (*sch).limit {
        bpf_qdisc_skb_drop(skb, to_free);
        (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
        return NET_XMIT_DROP;
    }

    if (*skb).tstamp == 0 {
        q.ktime_cache = bpf_ktime_get_ns();
        time_to_send = q.ktime_cache;
    } else {
        if fq_packet_beyond_horizon(skb) {
            q.ktime_cache = bpf_ktime_get_ns();
            if fq_packet_beyond_horizon(skb) {
                if q.horizon_drop != 0 {
                    bpf_qdisc_skb_drop(skb, to_free);
                    (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
                    return NET_XMIT_DROP;
                }

                (*skb).tstamp = q.ktime_cache.wrapping_add(q.horizon);
            }
        }
        time_to_send = (*skb).tstamp;
    }

    ret = fq_classify(skb, &mut sflow);
    if ret == CLS_RET_ERR {
        bpf_qdisc_skb_drop(skb, to_free);
        (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
        return NET_XMIT_DROP;
    }

    flow = bpf_kptr_xchg(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void) as *mut fq_flow_node;
    if flow.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
        (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
        return NET_XMIT_DROP;
    }

    if ret == CLS_RET_NONPRIO {
        if (*flow).qlen >= q.flow_plimit {
            bpf_kptr_xchg_back(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);
            bpf_qdisc_skb_drop(skb, to_free);
            (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
            return NET_XMIT_DROP;
        }

        if fq_flow_is_detached(flow) {
            flow_copy = bpf_refcount_acquire(flow);

            jiffies = bpf_jiffies64();
            if jiffies.wrapping_sub((*flow_copy).age.wrapping_add(q.flow_refill_delay as u64)) as i64 > 0 {
                if (*flow_copy).credit < q.quantum as i32 {
                    (*flow_copy).credit = q.quantum as i32;
                }
            }
            (*flow_copy).age = 0;
            fq_flows_add_tail(&mut fq_new_flows, &mut fq_new_flows_lock, flow_copy, &mut q.new_flow_cnt);
        }
    }

    skbn = bpf_obj_new_skb_node();
    if skbn.is_null() {
        bpf_kptr_xchg_back(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);
        bpf_qdisc_skb_drop(skb, to_free);
        (*sch).qstats.drops = (*sch).qstats.drops.wrapping_add(1);
        return NET_XMIT_DROP;
    }

    (*skb).tstamp = time_to_send;
    (*skbn).tstamp = (*skb).tstamp;

    (*sch).qstats.backlog = (*sch).qstats.backlog.wrapping_add(qdisc_pkt_len(skb));

    skb = bpf_kptr_xchg(&mut (*skbn).skb as *mut _ as *mut core::ffi::c_void, skb as *mut core::ffi::c_void) as *mut sk_buff;
    if !skb.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
    }

    bpf_spin_lock(&mut (*flow).lock);
    bpf_rbtree_add(&mut (*flow).queue, &mut (*skbn).node, skbn_tstamp_less);
    bpf_spin_unlock(&mut (*flow).lock);

    (*flow).qlen = (*flow).qlen.wrapping_add(1);
    bpf_kptr_xchg_back(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);

    (*sch).q.qlen = (*sch).q.qlen.wrapping_add(1);
    NET_XMIT_SUCCESS
}

unsafe fn fq_unset_throttled_flows(_index: u32, ctx: *mut unset_throttled_flows_ctx) -> i32 {
    let mut node: *mut bpf_rb_node = core::ptr::null_mut();
    let mut flow: *mut fq_flow_node;

    bpf_spin_lock(&mut fq_delayed_lock);

    node = bpf_rbtree_first(&mut fq_delayed);
    if node.is_null() {
        bpf_spin_unlock(&mut fq_delayed_lock);
        return 1;
    }

    flow = container_of_fq_flow_node_rb_node(node);
    if !(*ctx).unset_all && (*flow).time_next_packet > (*ctx).now {
        q.time_next_delayed_flow = (*flow).time_next_packet;
        bpf_spin_unlock(&mut fq_delayed_lock);
        return 1;
    }

    node = bpf_rbtree_remove(&mut fq_delayed, &mut (*flow).rb_node);

    bpf_spin_unlock(&mut fq_delayed_lock);

    if node.is_null() {
        return 1;
    }

    flow = container_of_fq_flow_node_rb_node(node);
    (*flow).age = 0;
    fq_flows_add_tail(&mut fq_old_flows, &mut fq_old_flows_lock, flow, &mut q.old_flow_cnt);

    0
}

unsafe fn fq_flow_set_throttled(flow: *mut fq_flow_node) {
    (*flow).age = !0u64;

    if q.time_next_delayed_flow > (*flow).time_next_packet {
        q.time_next_delayed_flow = (*flow).time_next_packet;
    }

    bpf_spin_lock(&mut fq_delayed_lock);
    bpf_rbtree_add(&mut fq_delayed, &mut (*flow).rb_node, fn_time_next_packet_less);
    bpf_spin_unlock(&mut fq_delayed_lock);
}

unsafe fn fq_check_throttled(now: u64) {
    let mut ctx = unset_throttled_flows_ctx {
        unset_all: false,
        now,
    };
    let sample: core::ffi::c_ulong;

    if q.time_next_delayed_flow > now {
        return;
    }

    sample = now.wrapping_sub(q.time_next_delayed_flow) as core::ffi::c_ulong;
    q.unthrottle_latency_ns = q.unthrottle_latency_ns.wrapping_sub(q.unthrottle_latency_ns >> 3);
    q.unthrottle_latency_ns = q.unthrottle_latency_ns.wrapping_add((sample >> 3) as u64);

    q.time_next_delayed_flow = !0u64;
    bpf_loop(NUM_QUEUE, fq_unset_throttled_flows, &mut ctx as *mut _ as *mut core::ffi::c_void, 0);
}

unsafe fn fq_dequeue_nonprio_flows(_index: u32, ctx: *mut dequeue_nonprio_ctx) -> *mut sk_buff {
    let mut time_next_packet: u64;
    let time_to_send: u64;
    let mut rb_node: *mut bpf_rb_node;
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let head: *mut bpf_list_head;
    let mut node: *mut bpf_list_node = core::ptr::null_mut();
    let lock: *mut bpf_spin_lock;
    let flow: *mut fq_flow_node;
    let skbn: *mut skb_node;
    let is_empty: bool;
    let cnt: *mut u32;

    if q.new_flow_cnt != 0 {
        head = &mut fq_new_flows;
        lock = &mut fq_new_flows_lock;
        cnt = &mut q.new_flow_cnt;
    } else if q.old_flow_cnt != 0 {
        head = &mut fq_old_flows;
        lock = &mut fq_old_flows_lock;
        cnt = &mut q.old_flow_cnt;
    } else {
        if q.time_next_delayed_flow != !0u64 {
            (*ctx).expire = q.time_next_delayed_flow;
        }
        (*ctx).stop_iter = true;
        return skb;
    }

    fq_flows_remove_front(head, lock, &mut node, cnt);
    if node.is_null() {
        (*ctx).stop_iter = true;
        return skb;
    }

    flow = container_of_fq_flow_node_list_node(node);
    if (*flow).credit <= 0 {
        (*flow).credit = (*flow).credit.wrapping_add(q.quantum as i32);
        fq_flows_add_tail(&mut fq_old_flows, &mut fq_old_flows_lock, flow, &mut q.old_flow_cnt);
        return core::ptr::null_mut();
    }

    bpf_spin_lock(&mut (*flow).lock);
    rb_node = bpf_rbtree_first(&mut (*flow).queue);
    if rb_node.is_null() {
        bpf_spin_unlock(&mut (*flow).lock);
        is_empty = fq_flows_is_empty(&mut fq_old_flows, &mut fq_old_flows_lock);
        if head == &mut fq_new_flows as *mut bpf_list_head && !is_empty {
            fq_flows_add_tail(&mut fq_old_flows, &mut fq_old_flows_lock, flow, &mut q.old_flow_cnt);
        } else {
            fq_flow_set_detached(flow);
            bpf_obj_drop(flow as *mut core::ffi::c_void);
        }
        return core::ptr::null_mut();
    }

    skbn = container_of_skb_node_node(rb_node);
    time_to_send = (*skbn).tstamp;

    time_next_packet = if time_to_send > (*flow).time_next_packet {
        time_to_send
    } else {
        (*flow).time_next_packet
    };
    if (*ctx).now < time_next_packet {
        bpf_spin_unlock(&mut (*flow).lock);
        (*flow).time_next_packet = time_next_packet;
        fq_flow_set_throttled(flow);
        return core::ptr::null_mut();
    }

    rb_node = bpf_rbtree_remove(&mut (*flow).queue, rb_node);
    bpf_spin_unlock(&mut (*flow).lock);

    if rb_node.is_null() {
        fq_flows_add_head(head, lock, flow, cnt);
        (*ctx).stop_iter = true;
        return skb;
    }

    let skbn_removed = container_of_skb_node_node(rb_node);
    skb = bpf_kptr_xchg(&mut (*skbn_removed).skb as *mut _ as *mut core::ffi::c_void, skb as *mut core::ffi::c_void) as *mut sk_buff;
    bpf_obj_drop(skbn_removed as *mut core::ffi::c_void);

    if skb.is_null() {
        fq_flows_add_head(head, lock, flow, cnt);
        (*ctx).stop_iter = true;
        return skb;
    }

    (*flow).credit = (*flow).credit.wrapping_sub((*qdisc_skb_cb(skb)).pkt_len);
    (*flow).qlen = (*flow).qlen.wrapping_sub(1);

    fq_flows_add_head(head, lock, flow, cnt);
    (*ctx).stop_iter = true;
    skb
}

unsafe fn fq_dequeue_prio() -> *mut sk_buff {
    let mut flow: *mut fq_flow_node = core::ptr::null_mut();
    let sflow: *mut fq_stashed_flow;
    let mut rb_node: *mut bpf_rb_node;
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut skbn: *mut skb_node;
    let hash: u64 = 0;

    sflow = bpf_map_lookup_elem(&mut fq_prio_flows, &hash as *const _ as *const core::ffi::c_void) as *mut fq_stashed_flow;
    if sflow.is_null() {
        return core::ptr::null_mut();
    }

    flow = bpf_kptr_xchg(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void) as *mut fq_flow_node;
    if flow.is_null() {
        return core::ptr::null_mut();
    }

    bpf_spin_lock(&mut (*flow).lock);
    rb_node = bpf_rbtree_first(&mut (*flow).queue);
    if rb_node.is_null() {
        bpf_spin_unlock(&mut (*flow).lock);
        bpf_kptr_xchg_back(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);
        return skb;
    }

    skbn = container_of_skb_node_node(rb_node);
    rb_node = bpf_rbtree_remove(&mut (*flow).queue, &mut (*skbn).node);
    bpf_spin_unlock(&mut (*flow).lock);

    if !rb_node.is_null() {
        skbn = container_of_skb_node_node(rb_node);
        skb = bpf_kptr_xchg(&mut (*skbn).skb as *mut _ as *mut core::ffi::c_void, skb as *mut core::ffi::c_void) as *mut sk_buff;
        bpf_obj_drop(skbn as *mut core::ffi::c_void);
    }

    bpf_kptr_xchg_back(&mut (*sflow).flow as *mut _ as *mut core::ffi::c_void, flow as *mut core::ffi::c_void);

    skb
}

#[link_section = "struct_ops/bpf_fq_dequeue"]
pub unsafe extern "C" fn bpf_fq_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let mut cb_ctx: dequeue_nonprio_ctx = core::mem::zeroed();
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut i: i32;

    if (*sch).q.qlen == 0 {
        return core::ptr::null_mut();
    }

    skb = fq_dequeue_prio();
    if !skb.is_null() {
        (*sch).q.qlen = (*sch).q.qlen.wrapping_sub(1);
        (*sch).qstats.backlog = (*sch).qstats.backlog.wrapping_sub(qdisc_pkt_len(skb));
        bpf_qdisc_bstats_update(sch, skb);
        return skb;
    }

    q.ktime_cache = bpf_ktime_get_ns();
    cb_ctx.now = q.ktime_cache;
    fq_check_throttled(q.ktime_cache);
    i = 0;
    while i < (*sch).limit as i32 {
        skb = fq_dequeue_nonprio_flows(i as u32, &mut cb_ctx);
        if cb_ctx.stop_iter {
            break;
        }
        i += 1;
    }

    if !skb.is_null() {
        (*sch).q.qlen = (*sch).q.qlen.wrapping_sub(1);
        (*sch).qstats.backlog = (*sch).qstats.backlog.wrapping_sub(qdisc_pkt_len(skb));
        bpf_qdisc_bstats_update(sch, skb);
        return skb;
    }

    if cb_ctx.expire != 0 {
        bpf_qdisc_watchdog_schedule(sch, cb_ctx.expire, q.timer_slack);
    }
    core::ptr::null_mut()
}

unsafe fn fq_remove_flows_in_list(_index: u32, _ctx: *mut core::ffi::c_void) -> i32 {
    let mut node: *mut bpf_list_node;
    let flow: *mut fq_flow_node;

    bpf_spin_lock(&mut fq_new_flows_lock);
    node = bpf_list_pop_front(&mut fq_new_flows);
    bpf_spin_unlock(&mut fq_new_flows_lock);
    if node.is_null() {
        bpf_spin_lock(&mut fq_old_flows_lock);
        node = bpf_list_pop_front(&mut fq_old_flows);
        bpf_spin_unlock(&mut fq_old_flows_lock);
        if node.is_null() {
            return 1;
        }
    }

    flow = container_of_fq_flow_node_list_node(node);
    bpf_obj_drop(flow as *mut core::ffi::c_void);

    0
}

extern "C" {
    static CONFIG_HZ: u32;
}

/* limit number of collected flows per round */
const FQ_GC_MAX: u32 = 8;

unsafe fn fq_gc_age() -> u32 {
    3u32.wrapping_mul(CONFIG_HZ)
}

unsafe fn fq_gc_candidate(flow: *mut fq_flow_node) -> bool {
    let jiffies: u64 = bpf_jiffies64();

    fq_flow_is_detached(flow)
        && (jiffies.wrapping_sub((*flow).age.wrapping_add(fq_gc_age() as u64)) as i64 > 0)
}

unsafe fn fq_remove_flows(
    flow_map: *mut bpf_map,
    hash: *mut u64,
    sflow: *mut fq_stashed_flow,
    ctx: *mut remove_flows_ctx,
) -> i32 {
    if !(*sflow).flow.is_null() && (!(*ctx).gc_only || fq_gc_candidate((*sflow).flow)) {
        bpf_map_delete_elem(flow_map, hash as *const core::ffi::c_void);
        (*ctx).reset_cnt = (*ctx).reset_cnt.wrapping_add(1);
    }

    if (*ctx).reset_cnt < (*ctx).reset_max {
        0
    } else {
        1
    }
}

unsafe fn fq_gc() {
    let mut cb_ctx = remove_flows_ctx {
        gc_only: true,
        reset_cnt: 0,
        reset_max: FQ_GC_MAX,
    };

    bpf_for_each_map_elem(&mut fq_nonprio_flows, fq_remove_flows, &mut cb_ctx as *mut _ as *mut core::ffi::c_void, 0);
}

#[link_section = "struct_ops/bpf_fq_reset"]
pub unsafe extern "C" fn bpf_fq_reset(sch: *mut Qdisc) {
    let mut utf_ctx = unset_throttled_flows_ctx {
        unset_all: true,
        now: 0,
    };
    let mut rf_ctx = remove_flows_ctx {
        gc_only: false,
        reset_cnt: 0,
        reset_max: NUM_QUEUE,
    };
    let mut sflow: *mut fq_stashed_flow = core::ptr::null_mut();
    let hash: u64 = 0;

    (*sch).q.qlen = 0;
    (*sch).qstats.backlog = 0;

    bpf_for_each_map_elem(&mut fq_nonprio_flows, fq_remove_flows, &mut rf_ctx as *mut _ as *mut core::ffi::c_void, 0);

    rf_ctx.reset_cnt = 0;
    bpf_for_each_map_elem(&mut fq_prio_flows, fq_remove_flows, &mut rf_ctx as *mut _ as *mut core::ffi::c_void, 0);
    fq_new_flow(&mut fq_prio_flows, &mut sflow, hash);

    bpf_loop(NUM_QUEUE, fq_remove_flows_in_list, core::ptr::null_mut(), 0);
    q.new_flow_cnt = 0;
    q.old_flow_cnt = 0;

    bpf_loop(NUM_QUEUE, fq_unset_throttled_flows, &mut utf_ctx as *mut _ as *mut core::ffi::c_void, 0);
}

#[link_section = "struct_ops/bpf_fq_init"]
pub unsafe extern "C" fn bpf_fq_init(
    sch: *mut Qdisc,
    _opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let dev: *mut net_device = (*(*sch).dev_queue).dev;
    let psched_mtu: u32 = (*dev).mtu.wrapping_add((*dev).hard_header_len);
    let mut sflow: *mut fq_stashed_flow = core::ptr::null_mut();
    let hash: u64 = 0;

    if fq_new_flow(&mut fq_prio_flows, &mut sflow, hash) < 0 {
        return -ENOMEM;
    }

    (*sch).limit = 10000;
    q.initial_quantum = 10u32.wrapping_mul(psched_mtu);
    q.quantum = 2u32.wrapping_mul(psched_mtu);
    q.flow_refill_delay = 40;
    q.flow_plimit = 100;
    q.horizon = 10u64.wrapping_mul(NSEC_PER_SEC as u64);
    q.horizon_drop = 1;
    q.orphan_mask = 1024 - 1;
    q.timer_slack = 10u32.wrapping_mul(NSEC_PER_USEC as u32);
    q.time_next_delayed_flow = !0u64;
    q.unthrottle_latency_ns = 0u64;
    q.new_flow_cnt = 0;
    q.old_flow_cnt = 0;

    0
}

#[link_section = "struct_ops"]
pub unsafe extern "C" fn bpf_fq_destroy(_sch: *mut Qdisc) {}

#[link_section = ".struct_ops"]
#[no_mangle]
pub static fq: Qdisc_ops = Qdisc_ops {
    enqueue: bpf_fq_enqueue as *const core::ffi::c_void,
    dequeue: bpf_fq_dequeue as *const core::ffi::c_void,
    reset: bpf_fq_reset as *const core::ffi::c_void,
    init: bpf_fq_init as *const core::ffi::c_void,
    destroy: bpf_fq_destroy as *const core::ffi::c_void,
    id: b"bpf_fq\0".as_ptr() as *const i8,
};
