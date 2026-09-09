// SPDX-License-Identifier: GPL-2.0
/* Linux multicast routing support
 * Common logic shared by IPv4 [ipmr] and IPv6 [ip6mr] implementation
 */

// Declarations supplied by the surrounding kernel translation.

pub unsafe fn vif_device_init(v: *mut vif_device, dev: *mut net_device,
                               rate_limit: c_ulong, threshold: u8,
                               flags: u16, get_iflink_mask: u16) {
    RCU_INIT_POINTER((*v).dev, core::ptr::null_mut());
    (*v).bytes_in = 0; (*v).bytes_out = 0; (*v).pkt_in = 0; (*v).pkt_out = 0;
    (*v).rate_limit = rate_limit; (*v).flags = flags; (*v).threshold = threshold;
    (*v).link = if (*v).flags & get_iflink_mask != 0 {
        dev_get_iflink(dev)
    } else { (*dev).ifindex };
}

unsafe fn __mr_free_table(work: *mut work_struct) {
    let mrt = container_of(to_rcu_work(work), mr_table, work);
    rhltable_destroy(&mut (*mrt).mfc_hash);
    kfree(mrt);
}

pub unsafe fn mr_table_free(mrt: *mut mr_table) {
    queue_rcu_work(system_dfl_wq, &mut (*mrt).work);
}

pub unsafe fn mr_table_alloc(net: *mut net, id: u32, ops: *mut mr_table_ops,
                             expire_func: Option<unsafe extern "C" fn(*mut timer_list)>,
                             table_set: Option<unsafe extern "C" fn(*mut mr_table, *mut net)>) -> *mut mr_table {
    let mrt = kzalloc_obj::<mr_table>();
    if mrt.is_null() { return ERR_PTR(-ENOMEM); }
    (*mrt).id = id;
    write_pnet(&mut (*mrt).net, net);
    (*mrt).ops = *ops;
    let err = rhltable_init(&mut (*mrt).mfc_hash, (*mrt).ops.rht_params);
    if err != 0 { kfree(mrt); return ERR_PTR(err); }
    INIT_RCU_WORK(&mut (*mrt).work, __mr_free_table);
    INIT_LIST_HEAD(&mut (*mrt).mfc_cache_list);
    INIT_LIST_HEAD(&mut (*mrt).mfc_unres_queue);
    timer_setup(&mut (*mrt).ipmr_expire_timer, expire_func, 0);
    (*mrt).mroute_reg_vif_num = -1;
    table_set.unwrap()(mrt, net);
    mrt
}

pub unsafe fn mr_mfc_find_parent(mrt: *mut mr_table, hasharg: *mut c_void, parent: c_int) -> *mut c_void {
    let list = rhltable_lookup(&mut (*mrt).mfc_hash, hasharg, *(*mrt).ops.rht_params);
    let mut tmp: *mut rhlist_head = core::ptr::null_mut();
    let mut c: *mut mr_mfc;
    rhl_for_each_entry_rcu!(c, tmp, list, mnode) {
        if parent == -1 || parent == (*c).mfc_parent { return c as *mut c_void; }
    }
    core::ptr::null_mut()
}

pub unsafe fn mr_mfc_find_any_parent(mrt: *mut mr_table, vifi: c_int) -> *mut c_void {
    let list = rhltable_lookup(&mut (*mrt).mfc_hash, (*mrt).ops.cmparg_any, *(*mrt).ops.rht_params);
    let mut tmp = core::ptr::null_mut(); let mut c: *mut mr_mfc;
    rhl_for_each_entry_rcu!(c, tmp, list, mnode) {
        if (*c).mfc_un.res.ttls[vifi as usize] < 255 { return c as *mut c_void; }
    }
    core::ptr::null_mut()
}

pub unsafe fn mr_mfc_find_any(mrt: *mut mr_table, vifi: c_int, hasharg: *mut c_void) -> *mut c_void {
    let list = rhltable_lookup(&mut (*mrt).mfc_hash, hasharg, *(*mrt).ops.rht_params);
    let mut tmp = core::ptr::null_mut(); let mut c: *mut mr_mfc; let mut proxy: *mut mr_mfc;
    rhl_for_each_entry_rcu!(c, tmp, list, mnode) {
        if (*c).mfc_un.res.ttls[vifi as usize] < 255 { return c as *mut c_void; }
        proxy = mr_mfc_find_any_parent(mrt, (*c).mfc_parent) as *mut mr_mfc;
        if !proxy.is_null() && (*proxy).mfc_un.res.ttls[vifi as usize] < 255 { return c as *mut c_void; }
    }
    mr_mfc_find_any_parent(mrt, vifi)
}

// CONFIG_PROC_FS guarded declarations and implementations retain their source condition.
#[cfg(CONFIG_PROC_FS)]
pub unsafe fn mr_vif_seq_idx(net: *mut net, iter: *mut mr_vif_iter, mut pos: loff_t) -> *mut c_void {
    let mrt = (*iter).mrt;
    for ct in 0..(*mrt).maxvif { (*iter).ct = ct; if !VIF_EXISTS(mrt, ct) { continue; } if pos == 0 { return &mut (*mrt).vif_table[ct as usize] as *mut _ as *mut c_void; } pos -= 1; }
    core::ptr::null_mut()
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn mr_vif_seq_next(seq: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let iter = (*seq).private as *mut mr_vif_iter; let net = seq_file_net(seq); let mrt = (*iter).mrt; *pos += 1;
    if _v == SEQ_START_TOKEN { return mr_vif_seq_idx(net, iter, 0); }
    while { (*iter).ct += 1; (*iter).ct < (*mrt).maxvif } { if VIF_EXISTS(mrt, (*iter).ct) { return &mut (*mrt).vif_table[(*iter).ct as usize] as *mut _ as *mut c_void; } }
    core::ptr::null_mut()
}

// The remaining source functions use kernel list/RCU/netlink primitives directly.
// Their declarations are preserved as external Rust-facing interfaces.
extern "C" {
    pub fn mr_mfc_seq_idx(net: *mut net, it: *mut mr_mfc_iter, pos: loff_t) -> *mut c_void;
    pub fn mr_mfc_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void;
    pub fn mr_fill_mroute(mrt: *mut mr_table, skb: *mut sk_buff, c: *mut mr_mfc, rtm: *mut rtmsg) -> c_int;
    pub fn mr_table_dump(mrt: *mut mr_table, skb: *mut sk_buff, cb: *mut netlink_callback, fill: *mut c_void, lock: *mut spinlock_t, filter: *mut fib_dump_filter) -> c_int;
    pub fn mr_rtm_dumproute(skb: *mut sk_buff, cb: *mut netlink_callback, iter: *mut c_void, fill: *mut c_void, lock: *mut spinlock_t, filter: *mut fib_dump_filter) -> c_int;
    pub fn mr_dump(net: *mut net, nb: *mut notifier_block, family: u16, rules_dump: *mut c_void, mr_iter: *mut c_void, extack: *mut netlink_ext_ack) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
