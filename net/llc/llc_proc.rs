// SPDX-License-Identifier: GPL-2.0
/*
 * proc_llc.c - proc interface for LLC
 *
 * Copyright (c) 2001 by Jay Schulist <jschlst@samba.org>
 *                     2002-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn llc_ui_format_mac(seq: *mut seq_file, addr: *const u8) {
    seq_printf(seq, "%pM", addr);
}

unsafe fn llc_get_sk_idx(mut pos: loff_t) -> *mut sock {
    let mut sap: *mut llc_sap;
    let mut sk: *mut sock = core::ptr::null_mut();
    let mut i: i32;

    list_for_each_entry_rcu!(sap, &llc_sap_list, node) {
        spin_lock_bh(&(*sap).sk_lock);
        i = 0;
        while i < LLC_SK_LADDR_HASH_ENTRIES {
            let head: *mut hlist_nulls_head = &mut (*sap).sk_laddr_hash[i as usize];
            let mut node: *mut hlist_nulls_node;
            sk_nulls_for_each!(sk, node, head) {
                if pos == 0 {
                    return sk; // keep the lock
                }
                pos -= 1;
            }
            i += 1;
        }
        spin_unlock_bh(&(*sap).sk_lock);
    }
    core::ptr::null_mut()
}

unsafe fn llc_seq_start(_seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let mut l = *pos;
    rcu_read_lock_bh();
    if l != 0 {
        l -= 1;
        llc_get_sk_idx(l) as *mut core::ffi::c_void
    } else {
        SEQ_START_TOKEN
    }
}

unsafe fn laddr_hash_next(sap: *mut llc_sap, mut bucket: i32) -> *mut sock {
    let mut sk: *mut sock = core::ptr::null_mut();
    bucket += 1;
    while bucket < LLC_SK_LADDR_HASH_ENTRIES {
        let mut node: *mut hlist_nulls_node;
        sk_nulls_for_each!(sk, node, &(*sap).sk_laddr_hash[bucket as usize]) {
            return sk;
        }
        bucket += 1;
    }
    sk
}

unsafe fn llc_seq_next(_seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    let mut sk: *mut sock;
    if v == SEQ_START_TOKEN {
        sk = llc_get_sk_idx(0);
        return sk as *mut core::ffi::c_void;
    }
    sk = v as *mut sock;
    let next = sk_nulls_next(sk);
    if !next.is_null() {
        return next as *mut core::ffi::c_void;
    }
    let llc: *mut llc_sock = llc_sk(sk);
    let sap = (*llc).sap;
    sk = laddr_hash_next(sap, llc_sk_laddr_hashfn(sap, &(*llc).laddr));
    if !sk.is_null() {
        return sk as *mut core::ffi::c_void;
    }
    spin_unlock_bh(&(*sap).sk_lock);
    list_for_each_entry_continue_rcu!(sap, &llc_sap_list, node) {
        spin_lock_bh(&(*sap).sk_lock);
        sk = laddr_hash_next(sap, -1);
        if !sk.is_null() {
            break; // keep the lock
        }
        spin_unlock_bh(&(*sap).sk_lock);
    }
    sk as *mut core::ffi::c_void
}

unsafe fn llc_seq_stop(_seq: *mut seq_file, v: *mut core::ffi::c_void) {
    if !v.is_null() && v != SEQ_START_TOKEN {
        let sk = v as *mut sock;
        let llc = llc_sk(sk);
        let sap = (*llc).sap;
        spin_unlock_bh(&(*sap).sk_lock);
    }
    rcu_read_unlock_bh();
}

unsafe fn llc_seq_socket_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_puts(seq, "SKt Mc local_mac_sap        remote_mac_sap       tx_queue rx_queue st uid link\n");
        return 0;
    }
    let sk = v as *mut sock;
    let llc = llc_sk(sk);
    seq_printf(seq, "%2X  %2X ", (*sk).sk_type, 0);
    if !(*llc).dev.is_null() {
        llc_ui_format_mac(seq, (*(*llc).dev).dev_addr);
    } else {
        let addr = [0u8; 6];
        llc_ui_format_mac(seq, addr.as_ptr());
    }
    seq_printf(seq, "@%02X ", (*llc).sap.laddr.lsap);
    llc_ui_format_mac(seq, (*llc).daddr.mac.as_ptr());
    seq_printf(seq, "@%02X %8d %8d %2d %3u %4d\n", (*llc).daddr.lsap,
        sk_wmem_alloc_get(sk), sk_rmem_alloc_get(sk) - (*llc).copied_seq,
        (*sk).sk_state, from_kuid_munged(seq_user_ns(seq), sk_uid(sk)), (*llc).link);
    0
}

static const char *const llc_conn_state_names[] = [
    "adm", "setup", "normal", "busy", "rej", "await", "await_busy",
    "await_rej", "d_conn", "reset", "error", "temp",
];

unsafe fn llc_seq_core_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_puts(seq, "Connection list:\n" /* continued header */
            "dsap state      retr txw rxw pf ff sf df rs cs tack tpfc trs tbs blog busr\n");
        return 0;
    }
    let sk = v as *mut sock;
    let llc = llc_sk(sk);
    seq_printf(seq, " %02X  %-10s %3d  %3d %3d %2d %2d %2d %2d %2d %2d %4d %4d %3d %3d %4d %4d\n",
        (*llc).daddr.lsap, llc_conn_state_names[(*llc).state as usize], (*llc).retry_count,
        (*llc).k, (*llc).rw, (*llc).p_flag, (*llc).f_flag, (*llc).s_flag,
        (*llc).data_flag, (*llc).remote_busy_flag, (*llc).cause_flag,
        timer_pending(&(*llc).ack_timer.timer), timer_pending(&(*llc).pf_cycle_timer.timer),
        timer_pending(&(*llc).rej_sent_timer.timer), timer_pending(&(*llc).busy_state_timer.timer),
        (!(*sk).sk_backlog.tail.is_null()) as i32, sock_owned_by_user_nocheck(sk));
    0
}

static const struct_seq_operations! llc_seq_socket_ops = seq_operations! {
    start: llc_seq_start, next: llc_seq_next, stop: llc_seq_stop, show: llc_seq_socket_show,
};
static const struct_seq_operations! llc_seq_core_ops = seq_operations! {
    start: llc_seq_start, next: llc_seq_next, stop: llc_seq_stop, show: llc_seq_core_show,
};

static mut llc_proc_dir: *mut proc_dir_entry = core::ptr::null_mut();

unsafe fn llc_proc_init() -> i32 {
    let mut rc = -ENOMEM;
    let mut p: *mut proc_dir_entry;
    llc_proc_dir = proc_mkdir("llc", init_net.proc_net);
    if llc_proc_dir.is_null() { return rc; }
    p = proc_create_seq("socket", 0o444, llc_proc_dir, &llc_seq_socket_ops);
    if p.is_null() {
        remove_proc_entry("llc", init_net.proc_net);
        return rc;
    }
    p = proc_create_seq("core", 0o444, llc_proc_dir, &llc_seq_core_ops);
    if p.is_null() {
        remove_proc_entry("socket", llc_proc_dir);
        remove_proc_entry("llc", init_net.proc_net);
        return rc;
    }
    rc = 0;
    rc
}

unsafe fn llc_proc_exit() {
    remove_proc_entry("socket", llc_proc_dir);
    remove_proc_entry("core", llc_proc_dir);
    remove_proc_entry("llc", init_net.proc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
