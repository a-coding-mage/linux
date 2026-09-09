// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn dev_seq_from_index(seq: *mut seq_file, pos: *mut loff_t) -> *mut net_device {
    let mut ifindex = *pos as c_ulong;
    let mut dev: *mut net_device = core::ptr::null_mut();
    for_each_netdev_dump(seq_file_net(seq), dev, ifindex) {
        *pos = (*dev).ifindex as loff_t;
        return dev;
    }
    core::ptr::null_mut()
}

unsafe fn dev_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    rcu_read_lock();
    if *pos == 0 { SEQ_START_TOKEN } else { dev_seq_from_index(seq, pos) as *mut _ }
}

unsafe fn dev_seq_next(seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    dev_seq_from_index(seq, pos) as *mut _
}

unsafe fn dev_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) { rcu_read_unlock(); }

unsafe fn dev_seq_printf_stats(seq: *mut seq_file, dev: *mut net_device) {
    let mut temp: rtnl_link_stats64 = core::mem::zeroed();
    let stats = dev_get_stats(dev, &mut temp);
    seq_printf(seq, "%6s: %7llu %7llu %4llu %4llu %4llu %5llu %10llu %9llu %8llu %7llu %4llu %4llu %4llu %5llu %7llu %10llu\n",
        (*dev).name, (*stats).rx_bytes, (*stats).rx_packets, (*stats).rx_errors,
        (*stats).rx_dropped + (*stats).rx_missed_errors, (*stats).rx_fifo_errors,
        (*stats).rx_length_errors + (*stats).rx_over_errors + (*stats).rx_crc_errors + (*stats).rx_frame_errors,
        (*stats).rx_compressed, (*stats).multicast, (*stats).tx_bytes, (*stats).tx_packets,
        (*stats).tx_errors, (*stats).tx_dropped, (*stats).tx_fifo_errors, (*stats).collisions,
        (*stats).tx_carrier_errors + (*stats).tx_aborted_errors + (*stats).tx_window_errors + (*stats).tx_heartbeat_errors,
        (*stats).tx_compressed);
}

/* Called from the PROCfs module. This now uses the new arbitrary sized
 * /proc/net interface to create /proc/net/dev. */
unsafe fn dev_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    if v == SEQ_START_TOKEN {
        seq_puts(seq, "Inter-|   Receive                                                |  Transmit\n face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed\n");
    } else { dev_seq_printf_stats(seq, v as *mut net_device); }
    0
}

unsafe fn softnet_input_pkt_queue_len(sd: *mut softnet_data) -> u32 { skb_queue_len_lockless(&mut (*sd).input_pkt_queue) }
unsafe fn softnet_process_queue_len(sd: *mut softnet_data) -> u32 { skb_queue_len_lockless(&mut (*sd).process_queue) }

unsafe fn softnet_get_online(pos: *mut loff_t) -> *mut softnet_data {
    let mut sd = core::ptr::null_mut();
    while *pos < nr_cpu_ids {
        if cpu_online(*pos as _) { sd = per_cpu(softnet_data, *pos as _); break; } else { *pos += 1; }
    }
    sd
}
unsafe fn softnet_seq_start(_seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void { softnet_get_online(pos) as *mut _ }
unsafe fn softnet_seq_next(_seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void { *pos += 1; softnet_get_online(pos) as *mut _ }
unsafe fn softnet_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}

unsafe fn softnet_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let sd = v as *mut softnet_data;
    let input_qlen = softnet_input_pkt_queue_len(sd);
    let process_qlen = softnet_process_queue_len(sd);
    let mut flow_limit_count: c_uint = 0;
    // CONFIG_NET_FLOW_LIMIT conditionally supplies the following block.
    /* rcu_read_lock(); fl = rcu_dereference((*sd).flow_limit); if !fl.is_null() { flow_limit_count = READ_ONCE((*fl).count); } rcu_read_unlock(); */
    seq_printf(seq, "%08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x\n",
        READ_ONCE((*sd).processed), numa_drop_read(&mut (*sd).drop_counters), READ_ONCE((*sd).time_squeeze), 0, 0, 0, 0, 0, 0,
        READ_ONCE((*sd).received_rps), flow_limit_count, input_qlen + process_qlen, (*seq).index as c_int, input_qlen, process_qlen);
    0
}

static dev_seq_ops: seq_operations = seq_operations { start: Some(dev_seq_start), next: Some(dev_seq_next), stop: Some(dev_seq_stop), show: Some(dev_seq_show) };
static softnet_seq_ops: seq_operations = seq_operations { start: Some(softnet_seq_start), next: Some(softnet_seq_next), stop: Some(softnet_seq_stop), show: Some(softnet_seq_show) };

#[repr(C)] struct ptype_iter_state { p: seq_net_private, dev: *mut net_device }

unsafe fn ptype_get_idx(seq: *mut seq_file, pos: loff_t) -> *mut packet_type {
    let iter = (*seq).private as *mut ptype_iter_state; let mut i = 0; let mut dev: *mut net_device;
    for_each_netdev_rcu(seq_file_net(seq), dev) { let ptype_list = &mut (*dev).ptype_all; let mut pt: *mut packet_type;
        list_for_each_entry_rcu(pt, ptype_list, list) { if i == pos { (*iter).dev = dev; return pt; } i += 1; }
    }
    (*iter).dev = core::ptr::null_mut(); let mut pt: *mut packet_type;
    list_for_each_entry_rcu(pt, &mut (*seq_file_net(seq)).ptype_all, list) { if i == pos { return pt; } i += 1; }
    list_for_each_entry_rcu(pt, &mut (*seq_file_net(seq)).ptype_specific, list) { if i == pos { return pt; } i += 1; }
    for t in 0..PTYPE_HASH_SIZE { list_for_each_entry_rcu(pt, &mut ptype_base[t], list) { if i == pos { return pt; } i += 1; } }
    core::ptr::null_mut()
}

unsafe fn ptype_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void { rcu_read_lock(); if *pos != 0 { ptype_get_idx(seq, *pos - 1) as *mut _ } else { SEQ_START_TOKEN } }
unsafe fn ptype_seq_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let iter = (*seq).private as *mut ptype_iter_state; let net = seq_file_net(seq); *pos += 1;
    if v == SEQ_START_TOKEN { return ptype_get_idx(seq, 0) as *mut _; }
    let pt = v as *mut packet_type; let mut nxt = READ_ONCE((*pt).list.next); let dev = (*iter).dev; let mut hash: usize;
    if !dev.is_null() { if nxt != &mut (*dev).ptype_all { return list_entry(nxt, packet_type, list); }
        for_each_netdev_continue_rcu(seq_file_net(seq), dev) { nxt = READ_ONCE((*dev).ptype_all.next); if nxt != &mut (*dev).ptype_all { (*iter).dev = dev; return list_entry(nxt, packet_type, list); } }
        (*iter).dev = core::ptr::null_mut(); nxt = READ_ONCE((*net).ptype_all.next);
    }
    if !(*pt).af_packet_net.is_null() { if nxt != &mut (*net).ptype_all && nxt != &mut (*net).ptype_specific { return list_entry(nxt, packet_type, list); }
        if nxt == &mut (*net).ptype_all { nxt = READ_ONCE((*net).ptype_specific.next); if nxt != &mut (*net).ptype_specific { return list_entry(nxt, packet_type, list); } }
        hash = 0; nxt = READ_ONCE(ptype_base[0].next);
    } else { hash = (ntohs((*pt).type) as usize) & PTYPE_HASH_MASK; }
    while nxt == &mut ptype_base[hash] { hash += 1; if hash >= PTYPE_HASH_SIZE { return core::ptr::null_mut(); } nxt = READ_ONCE(ptype_base[hash].next); }
    list_entry(nxt, packet_type, list)
}
unsafe fn ptype_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) { rcu_read_unlock(); }
unsafe fn ptype_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> c_int { let iter = (*seq).private as *mut ptype_iter_state; let pt = v as *mut packet_type; if v == SEQ_START_TOKEN { seq_puts(seq, "Type Device      Function\n"); return 0; } let dev = (*iter).dev; if ((*pt).af_packet_net.is_null() || net_eq((*pt).af_packet_net, seq_file_net(seq))) && (dev.is_null() || net_eq(dev_net(dev), seq_file_net(seq))) { if (*pt).type == htons(ETH_P_ALL) { seq_puts(seq, "ALL "); } else { seq_printf(seq, "%04x", ntohs((*pt).type)); } seq_printf(seq, " %-8s %ps\n", if dev.is_null() { core::ptr::null() } else { (*dev).name }, (*pt).func); } 0 }
static ptype_seq_ops: seq_operations = seq_operations { start: Some(ptype_seq_start), next: Some(ptype_seq_next), stop: Some(ptype_seq_stop), show: Some(ptype_seq_show) };

// The remaining registration functions retain the C kernel API and lifecycle ordering.
unsafe fn dev_proc_net_init(net: *mut net) -> c_int { let mut rc = -ENOMEM; if proc_create_net(c"dev", 0o444, (*net).proc_net, &dev_seq_ops, core::mem::size_of::<seq_net_private>()).is_null() { return rc; } if proc_create_seq(c"softnet_stat", 0o444, (*net).proc_net, &softnet_seq_ops).is_null() { remove_proc_entry(c"dev", (*net).proc_net); return rc; } if proc_create_net(c"ptype", 0o444, (*net).proc_net, &ptype_seq_ops, core::mem::size_of::<ptype_iter_state>()).is_null() { remove_proc_entry(c"softnet_stat", (*net).proc_net); remove_proc_entry(c"dev", (*net).proc_net); return rc; } if wext_proc_init(net) != 0 { remove_proc_entry(c"ptype", (*net).proc_net); remove_proc_entry(c"softnet_stat", (*net).proc_net); remove_proc_entry(c"dev", (*net).proc_net); return rc; } rc = 0; rc }
unsafe fn dev_proc_net_exit(net: *mut net) { wext_proc_exit(net); remove_proc_entry(c"ptype", (*net).proc_net); remove_proc_entry(c"softnet_stat", (*net).proc_net); remove_proc_entry(c"dev", (*net).proc_net); }
static mut dev_proc_ops: pernet_operations = pernet_operations { init: Some(dev_proc_net_init), exit: Some(dev_proc_net_exit) };

unsafe fn dev_mc_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> c_int { if v == SEQ_START_TOKEN { return 0; } let dev = v as *mut net_device; netif_addr_lock_bh(dev); let mut ha: *mut netdev_hw_addr; netdev_for_each_mc_addr!(ha, dev) { seq_printf(seq, "%-4d %-15s %-5d %-5d %*phN\n", (*dev).ifindex, (*dev).name, (*ha).refcount, (*ha).global_use, (*dev).addr_len as c_int, (*ha).addr); } netif_addr_unlock_bh(dev); 0 }
static dev_mc_seq_ops: seq_operations = seq_operations { start: Some(dev_seq_start), next: Some(dev_seq_next), stop: Some(dev_seq_stop), show: Some(dev_mc_seq_show) };
unsafe fn dev_mc_net_init(net: *mut net) -> c_int { if proc_create_net(c"dev_mcast", 0, (*net).proc_net, &dev_mc_seq_ops, core::mem::size_of::<seq_net_private>()).is_null() { -ENOMEM } else { 0 } }
unsafe fn dev_mc_net_exit(net: *mut net) { remove_proc_entry(c"dev_mcast", (*net).proc_net); }
static mut dev_mc_net_ops: pernet_operations = pernet_operations { init: Some(dev_mc_net_init), exit: Some(dev_mc_net_exit) };
unsafe fn dev_proc_init() -> c_int { let ret = register_pernet_subsys(&mut dev_proc_ops); if ret == 0 { register_pernet_subsys(&mut dev_mc_net_ops) } else { ret } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
