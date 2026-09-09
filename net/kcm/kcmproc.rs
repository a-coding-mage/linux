// SPDX-License-Identifier: GPL-2.0
// Linux kernel dependencies are supplied by the surrounding translation.

#[cfg(CONFIG_PROC_FS)]
use core::ffi::c_void;

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_get_first(seq: *mut seq_file) -> *mut kcm_mux {
    let net = seq_file_net(seq);
    let knet = net_generic(net, kcm_net_id);
    list_first_or_null_rcu(&mut (*knet).mux_list, core::mem::offset_of!(kcm_mux, kcm_mux_list))
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_get_next(mux: *mut kcm_mux) -> *mut kcm_mux {
    let knet = (*mux).knet;
    list_next_or_null_rcu(&mut (*knet).mux_list, &mut (*mux).kcm_mux_list,
                          core::mem::offset_of!(kcm_mux, kcm_mux_list))
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_get_idx(seq: *mut seq_file, mut pos: loff_t) -> *mut kcm_mux {
    let net = seq_file_net(seq);
    let knet = net_generic(net, kcm_net_id);
    let mut m: *mut kcm_mux = core::ptr::null_mut();
    list_for_each_entry_rcu!(m, &mut (*knet).mux_list, kcm_mux_list, {
        if pos == 0 { return m; }
        pos -= 1;
    });
    core::ptr::null_mut()
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let p = if v == SEQ_START_TOKEN { kcm_get_first(seq) as *mut c_void }
            else { kcm_get_next(v as *mut kcm_mux) as *mut c_void };
    *pos += 1;
    p
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    rcu_read_lock();
    if *pos == 0 { SEQ_START_TOKEN } else { kcm_get_idx(seq, *pos - 1) as *mut c_void }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_seq_stop(_seq: *mut seq_file, _v: *mut c_void) { rcu_read_unlock(); }

#[cfg(CONFIG_PROC_FS)]
#[repr(C)]
struct kcm_proc_mux_state { p: seq_net_private, idx: i32 }

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_format_mux_header(seq: *mut seq_file) {
    let net = seq_file_net(seq);
    let knet = net_generic(net, kcm_net_id);
    seq_printf(seq, "*** KCM statistics (%d MUX) ****\n", (*knet).count);
    seq_printf(seq, "%-14s %-10s %-16s %-10s %-16s %-8s %-8s %-8s %-8s %s",
        "Object", "RX-Msgs", "RX-Bytes", "TX-Msgs", "TX-Bytes", "Recv-Q", "Rmem", "Send-Q", "Smem", "Status");
    // XXX: pdsts header stuff here
    seq_puts(seq, "\n");
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_format_sock(kcm: *mut kcm_sock, seq: *mut seq_file, _i: i32, _len: *mut i32) {
    seq_printf(seq, "   kcm-%-7u %-10llu %-16llu %-10llu %-16llu %-8d %-8d %-8d %-8s ",
        (*kcm).index, (*kcm).stats.rx_msgs, (*kcm).stats.rx_bytes, (*kcm).stats.tx_msgs,
        (*kcm).stats.tx_bytes, (*kcm).sk.sk_receive_queue.qlen,
        sk_rmem_alloc_get(&mut (*kcm).sk), (*kcm).sk.sk_write_queue.qlen, "-");
    if !(*kcm).tx_psock.is_null() { seq_printf(seq, "Psck-%u ", (*(*kcm).tx_psock).index); }
    if (*kcm).tx_wait { seq_puts(seq, "TxWait "); }
    if (*kcm).tx_wait_more { seq_puts(seq, "WMore "); }
    if (*kcm).rx_wait { seq_puts(seq, "RxWait "); }
    seq_puts(seq, "\n");
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_format_psock(psock: *mut kcm_psock, seq: *mut seq_file, _i: i32, _len: *mut i32) {
    seq_printf(seq, "   psock-%-5u %-10llu %-16llu %-10llu %-16llu %-8d %-8d %-8d %-8d ",
        (*psock).index, (*psock).strp.stats.msgs, (*psock).strp.stats.bytes,
        (*psock).stats.tx_msgs, (*psock).stats.tx_bytes, (*psock).sk.sk_receive_queue.qlen,
        atomic_read(&(*psock).sk.sk_rmem_alloc), (*psock).sk.sk_write_queue.qlen,
        refcount_read(&(*psock).sk.sk_wmem_alloc));
    if (*psock).done { seq_puts(seq, "Done "); }
    if (*psock).tx_stopped { seq_puts(seq, "TxStop "); }
    if (*psock).strp.stopped { seq_puts(seq, "RxStop "); }
    if !(*psock).tx_kcm.is_null() { seq_printf(seq, "Rsvd-%d ", (*(*psock).tx_kcm).index); }
    if !(*psock).strp.paused && !(*psock).ready_rx_msg {
        if (*psock).sk.sk_receive_queue.qlen != 0 {
            if (*psock).strp.need_bytes != 0 { seq_printf(seq, "RxWait=%u ", (*psock).strp.need_bytes); }
            else { seq_puts(seq, "RxWait "); }
        }
    } else {
        if (*psock).strp.paused { seq_puts(seq, "RxPause "); }
        if (*psock).ready_rx_msg { seq_puts(seq, "RdyRx "); }
    }
    seq_puts(seq, "\n");
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_format_mux(mux: *mut kcm_mux, _idx: loff_t, seq: *mut seq_file) {
    let mut i = 0; let mut len = 0; let mut kcm: *mut kcm_sock; let mut psock: *mut kcm_psock;
    seq_printf(seq, "%-6s%-8s %-10llu %-16llu %-10llu %-16llu %-8s %-8s %-8s %-8s ", "mux", "", (*mux).stats.rx_msgs, (*mux).stats.rx_bytes, (*mux).stats.tx_msgs, (*mux).stats.tx_bytes, "-", "-", "-", "-");
    seq_printf(seq, "KCMs: %d, Psocks %d\n", (*mux).kcm_socks_cnt, (*mux).psocks_cnt);
    spin_lock_bh(&mut (*mux).lock);
    list_for_each_entry!(kcm, &mut (*mux).kcm_socks, kcm_sock_list, { kcm_format_sock(kcm, seq, i, &mut len); i += 1; });
    i = 0;
    list_for_each_entry!(psock, &mut (*mux).psocks, psock_list, { kcm_format_psock(psock, seq, i, &mut len); i += 1; });
    spin_unlock_bh(&mut (*mux).lock);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_seq_show(seq: *mut seq_file, v: *mut c_void) -> i32 {
    let state = (*seq).private as *mut kcm_proc_mux_state;
    if v == SEQ_START_TOKEN { (*state).idx = 0; kcm_format_mux_header(seq); }
    else { kcm_format_mux(v as *mut kcm_mux, (*state).idx as loff_t, seq); (*state).idx += 1; }
    0
}

#[cfg(CONFIG_PROC_FS)]
static KCM_SEQ_OPS: seq_operations = seq_operations { show: Some(kcm_seq_show), start: Some(kcm_seq_start), next: Some(kcm_seq_next), stop: Some(kcm_seq_stop) };

// The remaining statistics aggregation and per-network registration functions retain
// the kernel's external data structures and helpers exactly as declared by its headers.
#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_stats_seq_show(seq: *mut seq_file, _v: *mut c_void) -> i32 {
    let net = (*seq).private as *mut net; let knet = net_generic(net, kcm_net_id);
    let mut psock_stats = core::mem::zeroed::<kcm_psock_stats>();
    let mut mux_stats = core::mem::zeroed::<kcm_mux_stats>();
    let mut strp_stats = core::mem::zeroed::<strp_aggr_stats>();
    mutex_lock(&mut (*knet).mutex);
    aggregate_mux_stats(&(*knet).aggregate_mux_stats, &mut mux_stats);
    aggregate_psock_stats(&(*knet).aggregate_psock_stats, &mut psock_stats);
    aggregate_strp_stats(&(*knet).aggregate_strp_stats, &mut strp_stats);
    let mut mux: *mut kcm_mux; let mut psock: *mut kcm_psock;
    list_for_each_entry!(mux, &mut (*knet).mux_list, kcm_mux_list, {
        spin_lock_bh(&mut (*mux).lock); aggregate_mux_stats(&(*mux).stats, &mut mux_stats);
        aggregate_psock_stats(&(*mux).aggregate_psock_stats, &mut psock_stats);
        aggregate_strp_stats(&(*mux).aggregate_strp_stats, &mut strp_stats);
        list_for_each_entry!(psock, &mut (*mux).psocks, psock_list, { aggregate_psock_stats(&(*psock).stats, &mut psock_stats); save_strp_stats(&(*psock).strp, &mut strp_stats); });
        spin_unlock_bh(&mut (*mux).lock);
    });
    mutex_unlock(&mut (*knet).mutex);
    seq_printf(seq, "%-8s %-10s %-16s %-10s %-16s %-10s %-10s %-10s %-10s %-10s\n", "MUX", "RX-Msgs", "RX-Bytes", "TX-Msgs", "TX-Bytes", "TX-Retries", "Attach", "Unattach", "UnattchRsvd", "RX-RdyDrops");
    seq_printf(seq, "%-8s %-10llu %-16llu %-10llu %-16llu %-10u %-10u %-10u %-10u %-10u\n", "", mux_stats.rx_msgs, mux_stats.rx_bytes, mux_stats.tx_msgs, mux_stats.tx_bytes, mux_stats.tx_retries, mux_stats.psock_attach, mux_stats.psock_unattach_rsvd, mux_stats.psock_unattach, mux_stats.rx_ready_drops);
    seq_printf(seq, "%-8s %-10s %-16s %-10s %-16s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s %-10s\n", "Psock", "RX-Msgs", "RX-Bytes", "TX-Msgs", "TX-Bytes", "Reserved", "Unreserved", "RX-Aborts", "RX-Intr", "RX-Unrecov", "RX-MemFail", "RX-NeedMor", "RX-BadLen", "RX-TooBig", "RX-Timeout", "TX-Aborts");
    seq_printf(seq, "%-8s %-10llu %-16llu %-10llu %-16llu %-10llu %-10llu %-10u %-10u %-10u %-10u %-10u %-10u %-10u %-10u %-10u\n", "", strp_stats.msgs, strp_stats.bytes, psock_stats.tx_msgs, psock_stats.tx_bytes, psock_stats.reserved, psock_stats.unreserved, strp_stats.aborts, strp_stats.interrupted, strp_stats.unrecov_intr, strp_stats.mem_fail, strp_stats.need_more_hdr, strp_stats.bad_hdr_len, strp_stats.msg_too_big, strp_stats.msg_timeouts, psock_stats.tx_aborts);
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_proc_init_net(net: *mut net) -> i32 {
    if proc_create_net_single("kcm_stats", 0o444, (*net).proc_net, kcm_stats_seq_show, core::ptr::null_mut()).is_null() { return -12; }
    if proc_create_net("kcm", 0o444, (*net).proc_net, &KCM_SEQ_OPS, core::mem::size_of::<kcm_proc_mux_state>()).is_null() { remove_proc_entry("kcm_stats", (*net).proc_net); return -12; }
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_proc_exit_net(net: *mut net) { remove_proc_entry("kcm", (*net).proc_net); remove_proc_entry("kcm_stats", (*net).proc_net); }

#[cfg(CONFIG_PROC_FS)]
static mut KCM_NET_OPS: pernet_operations = pernet_operations { init: Some(kcm_proc_init_net), exit: Some(kcm_proc_exit_net) };

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_proc_init() -> i32 { register_pernet_subsys(&mut KCM_NET_OPS) }

#[cfg(CONFIG_PROC_FS)]
unsafe fn kcm_proc_exit() { unregister_pernet_subsys(&mut KCM_NET_OPS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
