// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/cls_fw.c	Classifier mapping ipchains' fwmark to traffic class.
 *
 * Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *
 * Changes:
 * Karlis Peisenieks <karlis@mt.lv> : 990415 : fw_walk off by one
 * Karlis Peisenieks <karlis@mt.lv> : 990415 : fw_delete killed all the filter (and kernel).
 * Alex <alex@pilotsoft.com> : 2004xxyy: Added Action extension
 */

// C headers provide the external kernel types, constants, and functions used below.

const HTSIZE: usize = 256;

#[repr(C)]
struct fw_head {
    mask: u32,
    ht: [*mut fw_filter; HTSIZE],
    rcu: rcu_head,
}

#[repr(C)]
struct fw_filter {
    next: *mut fw_filter,
    id: u32,
    res: tcf_result,
    ifindex: i32,
    exts: tcf_exts,
    tp: *mut tcf_proto,
    rwork: rcu_work,
}

unsafe fn fw_hash(mut handle: u32) -> u32 {
    handle ^= handle >> 16;
    handle ^= handle >> 8;
    handle % HTSIZE as u32
}

unsafe fn fw_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32 {
    let head = rcu_dereference_bh((*tp).root) as *mut fw_head;
    let mut r: i32;
    let id = (*skb).mark;

    if !head.is_null() {
        let id = id & (*head).mask;
        let mut f = rcu_dereference_bh((*head).ht[fw_hash(id) as usize]); as *mut fw_filter;
        while !f.is_null() {
            if (*f).id == id {
                *res = (*f).res;
                if !tcf_match_indev(skb, (*f).ifindex) { f = rcu_dereference_bh((*f).next) as *mut fw_filter; continue; }
                r = tcf_exts_exec(skb, &mut (*f).exts, res);
                if r >= 0 { return r; }
            }
            f = rcu_dereference_bh((*f).next) as *mut fw_filter;
        }
    } else {
        // Old method: classify the packet using its skb mark.
        let q: *mut Qdisc;
        if tcf_block_shared((*(*tp).chain).block) { return -1; }
        q = tcf_block_q((*(*tp).chain).block);
        if id != 0 && (TC_H_MAJ(id) == 0 || TC_H_MAJ(id ^ (*q).handle) == 0) {
            (*res).classid = id;
            (*res).class = 0;
            return 0;
        }
    }
    -1
}

unsafe fn fw_get(tp: *mut tcf_proto, handle: u32) -> *mut core::ffi::c_void {
    let head = rtnl_dereference((*tp).root) as *mut fw_head;
    if head.is_null() { return core::ptr::null_mut(); }
    let mut f = rtnl_dereference((*head).ht[fw_hash(handle) as usize]) as *mut fw_filter;
    while !f.is_null() {
        if (*f).id == handle { return f as *mut core::ffi::c_void; }
        f = rtnl_dereference((*f).next) as *mut fw_filter;
    }
    core::ptr::null_mut()
}

unsafe fn fw_init(_tp: *mut tcf_proto) -> i32 { 0 }

unsafe fn __fw_delete_filter(f: *mut fw_filter) {
    tcf_exts_destroy(&mut (*f).exts);
    tcf_exts_put_net(&mut (*f).exts);
    kfree(f as *mut core::ffi::c_void);
}

unsafe fn fw_delete_filter_work(work: *mut work_struct) {
    let f = container_of(to_rcu_work(work), core::mem::offset_of!(fw_filter, rwork)) as *mut fw_filter;
    rtnl_lock(); __fw_delete_filter(f); rtnl_unlock();
}

unsafe fn fw_destroy(tp: *mut tcf_proto, _rtnl_held: bool, _extack: *mut netlink_ext_ack) {
    let head = rtnl_dereference((*tp).root) as *mut fw_head;
    if head.is_null() { return; }
    for h in 0..HTSIZE {
        loop {
            let f = rtnl_dereference((*head).ht[h]) as *mut fw_filter;
            if f.is_null() { break; }
            RCU_INIT_POINTER((*head).ht[h], rtnl_dereference((*f).next));
            tcf_unbind_filter(tp, &mut (*f).res);
            if tcf_exts_get_net(&mut (*f).exts) { tcf_queue_work(&mut (*f).rwork, fw_delete_filter_work); }
            else { __fw_delete_filter(f); }
        }
    }
    kfree_rcu(head, rcu);
}

// The remaining classifier callbacks retain the kernel ABI and operation ordering.
// Their declarations use the corresponding external kernel types and helpers.
unsafe fn fw_delete(tp: *mut tcf_proto, arg: *mut core::ffi::c_void, last: *mut bool, _rtnl_held: bool, _extack: *mut netlink_ext_ack) -> i32 {
    let head = rtnl_dereference((*tp).root) as *mut fw_head;
    let f = arg as *mut fw_filter;
    if head.is_null() || f.is_null() { return -22; }
    let mut fp = &mut (*head).ht[fw_hash((*f).id) as usize] as *mut *mut fw_filter;
    let mut pfp = rtnl_dereference(*fp) as *mut fw_filter;
    while !pfp.is_null() {
        if pfp == f { RCU_INIT_POINTER(*fp, rtnl_dereference((*f).next)); tcf_unbind_filter(tp, &mut (*f).res); tcf_exts_get_net(&mut (*f).exts); tcf_queue_work(&mut (*f).rwork, fw_delete_filter_work); break; }
        fp = &mut (*pfp).next; pfp = rtnl_dereference(*fp) as *mut fw_filter;
    }
    *last = true;
    for h in 0..HTSIZE { if !rcu_access_pointer((*head).ht[h]).is_null() { *last = false; break; } }
    if pfp.is_null() { -22 } else { 0 }
}

#[repr(C)]
struct nla_policy { r#type: u16, len: u16 }
static FW_POLICY: [nla_policy; 4] = [
    nla_policy { r#type: 0, len: 0 },
    nla_policy { r#type: NLA_U32, len: 0 },
    nla_policy { r#type: NLA_STRING, len: IFNAMSIZ as u16 },
    nla_policy { r#type: NLA_U32, len: 0 },
];

unsafe fn fw_set_parms(net: *mut net, tp: *mut tcf_proto, f: *mut fw_filter,
                       tb: *mut *mut nlattr, tca: *mut *mut nlattr, base: usize,
                       flags: u32, extack: *mut netlink_ext_ack) -> i32 {
    let mut err = tcf_exts_validate(net, tp, tb, *tca.add(TCA_RATE as usize), &mut (*f).exts, flags, extack);
    if err < 0 { return err; }
    if !(*tb.add(TCA_FW_INDEV as usize)).is_null() {
        let ret = tcf_change_indev(net, *tb.add(TCA_FW_INDEV as usize), extack);
        if ret < 0 { return ret; } (*f).ifindex = ret;
    }
    let head = rtnl_dereference((*tp).root) as *mut fw_head;
    if !(*tb.add(TCA_FW_MASK as usize)).is_null() {
        if nla_get_u32(*tb.add(TCA_FW_MASK as usize)) != (*head).mask { return -22; }
    } else if (*head).mask != 0xffff_ffff { return -22; }
    if !(*tb.add(TCA_FW_CLASSID as usize)).is_null() {
        (*f).res.classid = nla_get_u32(*tb.add(TCA_FW_CLASSID as usize));
        tcf_bind_filter(tp, &mut (*f).res, base);
    }
    err = 0; err
}

unsafe fn fw_walk(tp: *mut tcf_proto, arg: *mut tcf_walker, _rtnl_held: bool) {
    let head = rtnl_dereference((*tp).root) as *mut fw_head;
    if head.is_null() { (*arg).stop = 1; }
    if (*arg).stop != 0 { return; }
    for h in 0..HTSIZE { let mut f = rtnl_dereference((*head).ht[h]) as *mut fw_filter;
        while !f.is_null() { if !tc_cls_stats_dump(tp, arg, f as *mut core::ffi::c_void) { return; } f = rtnl_dereference((*f).next) as *mut fw_filter; }
    }
}

unsafe fn fw_bind_class(fh: *mut core::ffi::c_void, classid: u32, cl: usize, q: *mut core::ffi::c_void, base: usize) {
    tc_cls_bind_class(classid, cl, q, &mut (*(fh as *mut fw_filter)).res, base);
}

extern "C" {
    fn register_tcf_proto_ops(ops: *mut tcf_proto_ops) -> i32;
    fn unregister_tcf_proto_ops(ops: *mut tcf_proto_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
