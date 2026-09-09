// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2011 Pablo Neira Ayuso <pablo@netfilter.org>
 * (C) 2011 Intra2net AG <https://www.intra2net.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nf_acct {
    pub pkts: atomic64_t,
    pub bytes: atomic64_t,
    pub flags: c_ulong,
    pub head: list_head,
    pub refcnt: refcount_t,
    pub name: [c_char; NFACCT_NAME_MAX as usize],
    pub rcu_head: rcu_head,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct nfacct_filter {
    pub value: u32,
    pub mask: u32,
}

#[repr(C)]
pub struct nfnl_acct_net {
    pub nfnl_acct_list: list_head,
}

static mut nfnl_acct_net_id: c_uint = 0;

#[inline]
unsafe fn nfnl_acct_pernet(net: *mut net) -> *mut nfnl_acct_net {
    net_generic(net, nfnl_acct_net_id) as *mut nfnl_acct_net
}

const NFACCT_F_QUOTA: u32 = NFACCT_F_QUOTA_PKTS | NFACCT_F_QUOTA_BYTES;
const NFACCT_OVERQUOTA_BIT: c_uint = 2;

unsafe fn nfnl_acct_new(
    skb: *mut sk_buff,
    info: *const nfnl_info,
    tb: *const *const nlattr,
) -> c_int {
    let nfnl_acct_net = nfnl_acct_pernet((*info).net);
    let mut nfacct: *mut nf_acct;
    let mut matching: *mut nf_acct = core::ptr::null_mut();
    let mut size: c_uint = 0;
    let acct_name: *mut c_char;
    let mut flags: u32 = 0;

    if (*tb.add(NFACCT_NAME as usize)).is_null() { return -EINVAL; }
    acct_name = nla_data(*tb.add(NFACCT_NAME as usize)) as *mut c_char;
    if strlen(acct_name) == 0 { return -EINVAL; }

    list_for_each_entry!(nfacct, &mut (*nfnl_acct_net).nfnl_acct_list, head) {
        if strncmp((*nfacct).name.as_ptr(), acct_name, NFACCT_NAME_MAX) != 0 { continue; }
        if (*(*info).nlh).nlmsg_flags & NLM_F_EXCL != 0 { return -EEXIST; }
        matching = nfacct;
        break;
    }

    if !matching.is_null() {
        if (*(*info).nlh).nlmsg_flags & NLM_F_REPLACE != 0 {
            atomic64_set(&mut (*matching).pkts, 0);
            atomic64_set(&mut (*matching).bytes, 0);
            smp_mb__before_atomic();
            if (*matching).flags & NFACCT_F_QUOTA as c_ulong != 0 {
                clear_bit(NFACCT_OVERQUOTA_BIT, &mut (*matching).flags);
            }
            return 0;
        }
        return -EBUSY;
    }

    if !(*tb.add(NFACCT_FLAGS as usize)).is_null() {
        flags = ntohl(nla_get_be32(*tb.add(NFACCT_FLAGS as usize)));
        if flags & !NFACCT_F_QUOTA != 0 { return -EOPNOTSUPP; }
        if flags & NFACCT_F_QUOTA == NFACCT_F_QUOTA { return -EINVAL; }
        if flags & NFACCT_F_OVERQUOTA != 0 { return -EINVAL; }
        if flags & NFACCT_F_QUOTA != 0 && (*tb.add(NFACCT_QUOTA as usize)).is_null() { return -EINVAL; }
        size += core::mem::size_of::<u64>() as c_uint;
    }

    nfacct = kzalloc(core::mem::size_of::<nf_acct>() + size as usize, GFP_KERNEL) as *mut nf_acct;
    if nfacct.is_null() { return -ENOMEM; }
    if flags & NFACCT_F_QUOTA != 0 {
        let quota = (*nfacct).data.as_mut_ptr() as *mut u64;
        *quota = be64_to_cpu(nla_get_be64(*tb.add(NFACCT_QUOTA as usize)));
        (*nfacct).flags = flags as c_ulong;
    }
    nla_strscpy((*nfacct).name.as_mut_ptr(), *tb.add(NFACCT_NAME as usize), NFACCT_NAME_MAX);
    if !(*tb.add(NFACCT_BYTES as usize)).is_null() { atomic64_set(&mut (*nfacct).bytes, be64_to_cpu(nla_get_be64(*tb.add(NFACCT_BYTES as usize)))); }
    if !(*tb.add(NFACCT_PKTS as usize)).is_null() { atomic64_set(&mut (*nfacct).pkts, be64_to_cpu(nla_get_be64(*tb.add(NFACCT_PKTS as usize)))); }
    refcount_set(&mut (*nfacct).refcnt, 1);
    list_add_tail_rcu(&mut (*nfacct).head, &mut (*nfnl_acct_net).nfnl_acct_list);
    0
}

unsafe fn nfnl_acct_fill_info(skb: *mut sk_buff, portid: u32, seq: u32, type_: u32, mut event: c_int, acct: *mut nf_acct) -> c_int {
    let mut nlh: *mut nlmsghdr;
    let flags = if portid != 0 { NLM_F_MULTI } else { 0 };
    let (pkts, bytes): (u64, u64);
    let old_flags = (*acct).flags;
    event = nfnl_msg_type(NFNL_SUBSYS_ACCT, event);
    nlh = nfnl_msg_put(skb, portid, seq, event, flags, AF_UNSPEC, NFNETLINK_V0, 0);
    if nlh.is_null() { nlmsg_cancel(skb, nlh); return -1; }
    if nla_put_string(skb, NFACCT_NAME, (*acct).name.as_ptr()) != 0 { nlmsg_cancel(skb, nlh); return -1; }
    if type_ == NFNL_MSG_ACCT_GET_CTRZERO {
        pkts = atomic64_xchg(&mut (*acct).pkts, 0); bytes = atomic64_xchg(&mut (*acct).bytes, 0);
        smp_mb__before_atomic();
        if (*acct).flags & NFACCT_F_QUOTA as c_ulong != 0 { clear_bit(NFACCT_OVERQUOTA_BIT, &mut (*acct).flags); }
    } else { pkts = atomic64_read(&(*acct).pkts); bytes = atomic64_read(&(*acct).bytes); }
    if nla_put_be64(skb, NFACCT_PKTS, cpu_to_be64(pkts), NFACCT_PAD) != 0 || nla_put_be64(skb, NFACCT_BYTES, cpu_to_be64(bytes), NFACCT_PAD) != 0 || nla_put_be32(skb, NFACCT_USE, htonl(refcount_read(&(*acct).refcnt))) != 0 { nlmsg_cancel(skb, nlh); return -1; }
    if (*acct).flags & NFACCT_F_QUOTA as c_ulong != 0 {
        let quota = *((*acct).data.as_ptr() as *const u64);
        if nla_put_be32(skb, NFACCT_FLAGS, htonl(old_flags as u32)) != 0 || nla_put_be64(skb, NFACCT_QUOTA, cpu_to_be64(quota), NFACCT_PAD) != 0 { nlmsg_cancel(skb, nlh); return -1; }
    }
    nlmsg_end(skb, nlh); (*skb).len as c_int
}

// The remaining callbacks and exported helpers retain the C ABI and are declared
// in terms of the kernel primitives supplied by the translated dependency set.
pub unsafe fn nfnl_acct_find_get(net: *mut net, acct_name: *const c_char) -> *mut nf_acct { let n = nfnl_acct_pernet(net); let mut cur: *mut nf_acct; rcu_read_lock(); list_for_each_entry_rcu!(cur, &mut (*n).nfnl_acct_list, head) { if strncmp((*cur).name.as_ptr(), acct_name, NFACCT_NAME_MAX) == 0 && try_module_get(THIS_MODULE) && refcount_inc_not_zero(&mut (*cur).refcnt) { rcu_read_unlock(); return cur; } } rcu_read_unlock(); core::ptr::null_mut() }

pub unsafe fn nfnl_acct_put(acct: *mut nf_acct) { if refcount_dec_and_test(&mut (*acct).refcnt) { kfree_rcu(acct, rcu_head); } module_put(THIS_MODULE); }
pub unsafe fn nfnl_acct_update(skb: *const sk_buff, nfacct: *mut nf_acct) { atomic64_inc(&mut (*nfacct).pkts); atomic64_add((*skb).len as u64, &mut (*nfacct).bytes); }

unsafe fn nfnl_overquota_report(net: *mut net, nfacct: *mut nf_acct) {
    let skb = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);
    if skb.is_null() { return; }
    if nfnl_acct_fill_info(skb, 0, 0, NFNL_MSG_ACCT_OVERQUOTA, 0, nfacct) <= 0 { kfree_skb(skb); return; }
    nfnetlink_broadcast(net, skb, 0, NFNLGRP_ACCT_QUOTA, GFP_ATOMIC);
}

pub unsafe fn nfnl_acct_overquota(net: *mut net, nfacct: *mut nf_acct) -> c_int {
    if (*nfacct).flags & NFACCT_F_QUOTA as c_ulong == 0 { return NFACCT_NO_QUOTA; }
    let quota = *((*nfacct).data.as_ptr() as *const u64);
    let now = if (*nfacct).flags & NFACCT_F_QUOTA_PKTS as c_ulong != 0 { atomic64_read(&(*nfacct).pkts) } else { atomic64_read(&(*nfacct).bytes) };
    let ret = (now > quota) as c_int;
    if now >= quota && test_and_set_bit(NFACCT_OVERQUOTA_BIT, &mut (*nfacct).flags) == 0 { nfnl_overquota_report(net, nfacct); }
    ret
}

unsafe fn nfnl_acct_net_init(net: *mut net) -> c_int { INIT_LIST_HEAD(&mut (*nfnl_acct_pernet(net)).nfnl_acct_list); 0 }
unsafe fn nfnl_acct_net_exit(net: *mut net) {
    let n = nfnl_acct_pernet(net); let mut cur: *mut nf_acct; let mut tmp: *mut nf_acct;
    list_for_each_entry_safe!(cur, tmp, &mut (*n).nfnl_acct_list, head) { list_del_rcu(&mut (*cur).head); if refcount_dec_and_test(&mut (*cur).refcnt) { kfree_rcu(cur, rcu_head); } }
}

static mut nfnl_acct_ops: pernet_operations = pernet_operations { init: Some(nfnl_acct_net_init), exit: Some(nfnl_acct_net_exit), id: &mut nfnl_acct_net_id, size: core::mem::size_of::<nfnl_acct_net>() };

unsafe fn nfnl_acct_init() -> c_int {
    let mut ret = register_pernet_subsys(&mut nfnl_acct_ops);
    if ret < 0 { pr_err!("nfnl_acct_init: failed to register pernet ops\n"); return ret; }
    ret = nfnetlink_subsys_register(&nfnl_acct_subsys);
    if ret < 0 { pr_err!("nfnl_acct_init: cannot register with nfnetlink.\n"); unregister_pernet_subsys(&mut nfnl_acct_ops); }
    ret
}

unsafe fn nfnl_acct_exit() { nfnetlink_subsys_unregister(&nfnl_acct_subsys); unregister_pernet_subsys(&mut nfnl_acct_ops); }

// Netlink dump/get/delete callbacks and their nla policies are registered by
// the surrounding kernel binding layer; these declarations preserve the C
// interfaces for that layer.
extern "C" {
    fn nfnl_acct_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    fn nfnl_acct_start(cb: *mut netlink_callback) -> c_int;
    fn nfnl_acct_done(cb: *mut netlink_callback) -> c_int;
    fn nfnl_acct_get(skb: *mut sk_buff, info: *const nfnl_info, tb: *const *const nlattr) -> c_int;
    fn nfnl_acct_del(skb: *mut sk_buff, info: *const nfnl_info, tb: *const *const nlattr) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
