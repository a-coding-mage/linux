// SPDX-License-Identifier: GPL-2.0-only
/* iptables module for using new netfilter netlink queue
 *
 * (C) 2005 by Harald Welte <laforge@netfilter.org>
 */

// Dependencies supplied by the kernel and netfilter headers are intentionally
// referenced here rather than reimplemented.

static mut jhash_initval: u32 = 0;

unsafe extern "C" {
    fn nfqueue_hash(
        skb: *mut sk_buff,
        queue: u32,
        queues_total: u32,
        family: u8,
        initval: u32,
    ) -> u32;
    fn init_hashrandom(initval: *mut u32);
    fn raw_smp_processor_id() -> i32;
    fn xt_register_targets(targets: *mut xt_target, n: usize) -> i32;
    fn xt_unregister_targets(targets: *mut xt_target, n: usize);
}

#[repr(C)]
struct sk_buff;
#[repr(C)]
struct xt_action_param {
    targinfo: *const core::ffi::c_void,
}
#[repr(C)]
struct xt_tgchk_param {
    targinfo: *const core::ffi::c_void,
    target: *const xt_target,
}
#[repr(C)]
struct xt_target {
    name: *const u8,
    revision: u8,
    family: u16,
    target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    targetsize: usize,
    me: *mut core::ffi::c_void,
}

#[repr(C)]
struct xt_NFQ_info;
#[repr(C)]
struct xt_NFQ_info_v1 {
    queuenum: u32,
    queues_total: u32,
}
#[repr(C)]
struct xt_NFQ_info_v2 {
    queuenum: u32,
    queues_total: u32,
    bypass: u8,
}
#[repr(C)]
struct xt_NFQ_info_v3 {
    queuenum: u32,
    queues_total: u32,
    flags: u32,
}

// Constants supplied by the corresponding kernel headers.
const NFPROTO_UNSPEC: u16 = 0;
const NFQ_FLAG_CPU_FANOUT: u32 = 1 << 0;
const NFQ_FLAG_BYPASS: u32 = 1 << 1;
const NFQ_FLAG_MASK: u32 = NFQ_FLAG_CPU_FANOUT | NFQ_FLAG_BYPASS;
const NF_VERDICT_FLAG_QUEUE_BYPASS: u32 = 1 << 16;

#[inline]
unsafe fn nf_queue_nr(queue: u32) -> u32 {
    // NF_QUEUE_NR(queue)
    queue << 16
}

unsafe extern "C" fn nfqueue_tg(
    _skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let tinfo = (*par).targinfo as *const xt_NFQ_info;
    // The opaque v0 structure's first field is queuenum, as defined by xt_NFQ.h.
    let queue = *(tinfo as *const u16) as u32;
    nf_queue_nr(queue)
}

unsafe extern "C" fn nfqueue_tg_v1(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_NFQ_info_v1;
    let mut queue = (*info).queuenum;

    if (*info).queues_total > 1 {
        queue = nfqueue_hash(skb, queue, (*info).queues_total, 0, jhash_initval);
    }
    nf_queue_nr(queue)
}

unsafe extern "C" fn nfqueue_tg_v2(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_NFQ_info_v2;
    let mut ret = nfqueue_tg_v1(skb, par);

    if (*info).bypass != 0 {
        ret |= NF_VERDICT_FLAG_QUEUE_BYPASS;
    }
    ret
}

unsafe extern "C" fn nfqueue_tg_check(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *const xt_NFQ_info_v3;
    let maxid: u32;

    init_hashrandom(&raw mut jhash_initval);

    if (*info).queues_total == 0 {
        return -22;
    }
    maxid = (*info).queues_total - 1 + (*info).queuenum;
    if maxid > 0xffff {
        return -34;
    }
    if (*(*par).target).revision == 2 && (*info).flags > 1 {
        return -22;
    }
    if (*(*par).target).revision == 3 && (*info).flags & !NFQ_FLAG_MASK != 0 {
        return -22;
    }

    0
}

unsafe extern "C" fn nfqueue_tg_v3(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_NFQ_info_v3;
    let mut queue = (*info).queuenum;

    if (*info).queues_total > 1 {
        if (*info).flags & NFQ_FLAG_CPU_FANOUT != 0 {
            let cpu = raw_smp_processor_id();
            queue = (*info).queuenum + (cpu as u32) % (*info).queues_total;
        } else {
            queue = nfqueue_hash(skb, queue, (*info).queues_total, 0, jhash_initval);
        }
    }

    let mut ret = nf_queue_nr(queue);
    if (*info).flags & NFQ_FLAG_BYPASS != 0 {
        ret |= NF_VERDICT_FLAG_QUEUE_BYPASS;
    }
    ret
}

static mut nfqueue_tg_reg: [xt_target; 4] = [
    xt_target {
        name: b"NFQUEUE\0".as_ptr(), revision: 0, family: NFPROTO_UNSPEC,
        target: Some(nfqueue_tg), checkentry: None,
        targetsize: core::mem::size_of::<xt_NFQ_info>(), me: core::ptr::null_mut(),
    },
    xt_target {
        name: b"NFQUEUE\0".as_ptr(), revision: 1, family: NFPROTO_UNSPEC,
        target: Some(nfqueue_tg_v1), checkentry: Some(nfqueue_tg_check),
        targetsize: core::mem::size_of::<xt_NFQ_info_v1>(), me: core::ptr::null_mut(),
    },
    xt_target {
        name: b"NFQUEUE\0".as_ptr(), revision: 2, family: NFPROTO_UNSPEC,
        target: Some(nfqueue_tg_v2), checkentry: Some(nfqueue_tg_check),
        targetsize: core::mem::size_of::<xt_NFQ_info_v2>(), me: core::ptr::null_mut(),
    },
    xt_target {
        name: b"NFQUEUE\0".as_ptr(), revision: 3, family: NFPROTO_UNSPEC,
        target: Some(nfqueue_tg_v3), checkentry: Some(nfqueue_tg_check),
        targetsize: core::mem::size_of::<xt_NFQ_info_v3>(), me: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn nfqueue_tg_init() -> i32 {
    xt_register_targets(nfqueue_tg_reg.as_mut_ptr(), nfqueue_tg_reg.len())
}

unsafe extern "C" fn nfqueue_tg_exit() {
    xt_unregister_targets(nfqueue_tg_reg.as_mut_ptr(), nfqueue_tg_reg.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
