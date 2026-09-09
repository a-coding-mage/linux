// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/* proc.c - procfs support for Protocol family CAN core module */

// C dependencies and build-time kernel configuration are supplied by other files.

const CAN_PROC_STATS: &str = "stats";
const CAN_PROC_RESET_STATS: &str = "reset_stats";
const CAN_PROC_RCVLIST_ALL: &str = "rcvlist_all";
const CAN_PROC_RCVLIST_FIL: &str = "rcvlist_fil";
const CAN_PROC_RCVLIST_INV: &str = "rcvlist_inv";
const CAN_PROC_RCVLIST_SFF: &str = "rcvlist_sff";
const CAN_PROC_RCVLIST_EFF: &str = "rcvlist_eff";
const CAN_PROC_RCVLIST_ERR: &str = "rcvlist_err";

static mut user_reset: i32 = 0;

static rx_list_name: [&str; 4] = ["rx_err", "rx_all", "rx_fil", "rx_inv"];

unsafe fn can_init_stats(net: *mut net) {
    let pkg_stats = (*net).can.pkg_stats;
    let rcv_lists_stats = (*net).can.rcv_lists_stats;
    // Called from timer or process context, matching the original memset semantics.
    core::ptr::write_bytes(pkg_stats as *mut u8, 0, core::mem::size_of::<can_pkg_stats>());
    (*pkg_stats).jiffies_init = jiffies;
    (*rcv_lists_stats).stats_reset += 1;
    if user_reset != 0 {
        user_reset = 0;
        (*rcv_lists_stats).user_reset += 1;
    }
}

unsafe fn calc_rate(oldjif: usize, newjif: usize, count: usize) -> usize {
    if oldjif == newjif { return 0; }
    if count > (usize::MAX / HZ) {
        printk(KERN_ERR, "can: calc_rate: count exceeded! %ld\n", count);
        return 99999999;
    }
    (count * HZ) / (newjif - oldjif)
}

pub unsafe fn can_stat_update(t: *mut timer_list) {
    let net = timer_container_of!(t, can.stattimer);
    let pkg_stats = (*net).can.pkg_stats;
    let j = jiffies;
    let rx_frames = atomic_long_read(&(*pkg_stats).rx_frames);
    let tx_frames = atomic_long_read(&(*pkg_stats).tx_frames);
    let matches = atomic_long_read(&(*pkg_stats).matches);
    let rx_frames_delta = atomic_long_read(&(*pkg_stats).rx_frames_delta);
    let tx_frames_delta = atomic_long_read(&(*pkg_stats).tx_frames_delta);
    let matches_delta = atomic_long_read(&(*pkg_stats).matches_delta);
    if user_reset != 0 { can_init_stats(net); }
    if j < (*pkg_stats).jiffies_init { can_init_stats(net); }
    if rx_frames > (isize::MAX as _ / HZ) { can_init_stats(net); }
    if tx_frames > (isize::MAX as _ / HZ) { can_init_stats(net); }
    if matches > (isize::MAX as _ / 100) { can_init_stats(net); }
    if rx_frames != 0 { (*pkg_stats).total_rx_match_ratio = (matches * 100) / rx_frames; }
    (*pkg_stats).total_tx_rate = calc_rate((*pkg_stats).jiffies_init, j, tx_frames as _);
    (*pkg_stats).total_rx_rate = calc_rate((*pkg_stats).jiffies_init, j, rx_frames as _);
    if rx_frames_delta != 0 { (*pkg_stats).current_rx_match_ratio = (matches_delta * 100) / rx_frames_delta; }
    (*pkg_stats).current_tx_rate = calc_rate(0, HZ, tx_frames_delta as _);
    (*pkg_stats).current_rx_rate = calc_rate(0, HZ, rx_frames_delta as _);
    if (*pkg_stats).max_tx_rate < (*pkg_stats).current_tx_rate { (*pkg_stats).max_tx_rate = (*pkg_stats).current_tx_rate; }
    if (*pkg_stats).max_rx_rate < (*pkg_stats).current_rx_rate { (*pkg_stats).max_rx_rate = (*pkg_stats).current_rx_rate; }
    if (*pkg_stats).max_rx_match_ratio < (*pkg_stats).current_rx_match_ratio { (*pkg_stats).max_rx_match_ratio = (*pkg_stats).current_rx_match_ratio; }
    atomic_long_set(&(*pkg_stats).tx_frames_delta, 0);
    atomic_long_set(&(*pkg_stats).rx_frames_delta, 0);
    atomic_long_set(&(*pkg_stats).matches_delta, 0);
    mod_timer(&mut (*net).can.stattimer, round_jiffies(jiffies + HZ));
}

unsafe fn can_print_rcvlist(m: *mut seq_file, rx_list: *mut hlist_head, dev: *mut net_device) {
    hlist_for_each_entry_rcu!(r, rx_list, list, {
        let fmt = if ((*r).can_id & CAN_EFF_FLAG) != 0 {
            "   %-5s  %08x  %08x  %pK  %pK  %8ld  %s\n"
        } else { "   %-5s     %03x    %08x  %pK  %pK  %8ld  %s\n" };
        seq_printf!(m, fmt, DNAME(dev), (*r).can_id, (*r).mask, (*r).func, (*r).data,
                    atomic_long_read(&(*r).matches), (*r).ident);
    });
}

unsafe fn can_print_recv_banner(m: *mut seq_file) {
    if IS_ENABLED!(CONFIG_64BIT) {
        seq_puts!(m, "  device   can_id   can_mask      function          userdata       matches  ident\n");
    } else { seq_puts!(m, "  device   can_id   can_mask  function  userdata   matches  ident\n"); }
}

unsafe fn can_stats_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*m).private as *mut net;
    let p = (*net).can.pkg_stats; let s = (*net).can.rcv_lists_stats;
    seq_putc!(m, '\n');
    seq_printf!(m, " %8ld transmitted frames (TXF)\n", atomic_long_read(&(*p).tx_frames));
    seq_printf!(m, " %8ld received frames (RXF)\n", atomic_long_read(&(*p).rx_frames));
    seq_printf!(m, " %8ld matched frames (RXMF)\n", atomic_long_read(&(*p).matches));
    seq_putc!(m, '\n');
    if (*net).can.stattimer.function == Some(can_stat_update) {
        seq_printf!(m, " %8ld %% total match ratio (RXMR)\n", (*p).total_rx_match_ratio);
        seq_printf!(m, " %8ld frames/s total tx rate (TXR)\n", (*p).total_tx_rate);
        seq_printf!(m, " %8ld frames/s total rx rate (RXR)\n", (*p).total_rx_rate);
        seq_putc!(m, '\n');
        seq_printf!(m, " %8ld %% current match ratio (CRXMR)\n", (*p).current_rx_match_ratio);
        seq_printf!(m, " %8ld frames/s current tx rate (CTXR)\n", (*p).current_tx_rate);
        seq_printf!(m, " %8ld frames/s current rx rate (CRXR)\n", (*p).current_rx_rate);
        seq_putc!(m, '\n');
        seq_printf!(m, " %8ld %% max match ratio (MRXMR)\n", (*p).max_rx_match_ratio);
        seq_printf!(m, " %8ld frames/s max tx rate (MTXR)\n", (*p).max_tx_rate);
        seq_printf!(m, " %8ld frames/s max rx rate (MRXR)\n", (*p).max_rx_rate);
        seq_putc!(m, '\n');
    }
    seq_printf!(m, " %8ld current receive list entries (CRCV)\n", (*s).rcv_entries);
    seq_printf!(m, " %8ld maximum receive list entries (MRCV)\n", (*s).rcv_entries_max);
    if (*s).stats_reset != 0 { seq_printf!(m, "\n %8ld statistic resets (STR)\n", (*s).stats_reset); }
    if (*s).user_reset != 0 { seq_printf!(m, " %8ld user statistic resets (USTR)\n", (*s).user_reset); }
    seq_putc!(m, '\n'); 0
}

unsafe fn can_reset_stats_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*m).private as *mut net; let s = (*net).can.rcv_lists_stats; let p = (*net).can.pkg_stats;
    user_reset = 1;
    if (*net).can.stattimer.function == Some(can_stat_update) {
        seq_printf!(m, "Scheduled statistic reset #%ld.\n", (*s).stats_reset + 1);
    } else {
        if (*p).jiffies_init != jiffies { can_init_stats(net); }
        seq_printf!(m, "Performed statistic reset #%ld.\n", (*s).stats_reset);
    } 0
}

unsafe fn can_rcvlist_proc_show_one(m: *mut seq_file, idx: i32, dev: *mut net_device, lists: *mut can_dev_rcv_lists) {
    if !hlist_empty!(&mut (*lists).rx[idx as usize]) { can_print_recv_banner(m); can_print_rcvlist(m, &mut (*lists).rx[idx as usize], dev); }
    else { seq_printf!(m, "  (%s: no entry)\n", DNAME(dev)); }
}

unsafe fn can_rcvlist_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let idx = pde_data!((*m).file.f_inode) as isize as i32; let net = (*m).private as *mut net;
    seq_printf!(m, "\nreceive list '%s':\n", rx_list_name[idx as usize]); rcu_read_lock!();
    can_rcvlist_proc_show_one(m, idx, core::ptr::null_mut(), (*net).can.rx_alldev_list);
    for_each_netdev_rcu!(net, dev, { let ml = can_get_ml_priv(dev); if !ml.is_null() { can_rcvlist_proc_show_one(m, idx, dev, &mut (*ml).dev_rcv_lists); } });
    rcu_read_unlock!(); seq_putc!(m, '\n'); 0
}

unsafe fn can_rcvlist_proc_show_array(m: *mut seq_file, dev: *mut net_device, a: *mut hlist_head, n: usize) {
    let mut empty = true; for i in 0..n { if !hlist_empty!(&mut *a.add(i)) { empty = false; break; } }
    if !empty { can_print_recv_banner(m); for i in 0..n { if !hlist_empty!(&mut *a.add(i)) { can_print_rcvlist(m, a.add(i), dev); } } }
    else { seq_printf!(m, "  (%s: no entry)\n", DNAME(dev)); }
}

unsafe fn can_rcvlist_sff_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*m).private as *mut net; seq_puts!(m, "\nreceive list 'rx_sff':\n"); rcu_read_lock!();
    let l = (*net).can.rx_alldev_list; can_rcvlist_proc_show_array(m, core::ptr::null_mut(), (*l).rx_sff.as_mut_ptr(), (*l).rx_sff.len());
    for_each_netdev_rcu!(net, dev, { let ml = can_get_ml_priv(dev); if !ml.is_null() { let l = &mut (*ml).dev_rcv_lists; can_rcvlist_proc_show_array(m, dev, l.rx_sff.as_mut_ptr(), l.rx_sff.len()); } });
    rcu_read_unlock!(); seq_putc!(m, '\n'); 0
}

unsafe fn can_rcvlist_eff_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net = (*m).private as *mut net; seq_puts!(m, "\nreceive list 'rx_eff':\n"); rcu_read_lock!();
    let l = (*net).can.rx_alldev_list; can_rcvlist_proc_show_array(m, core::ptr::null_mut(), (*l).rx_eff.as_mut_ptr(), (*l).rx_eff.len());
    for_each_netdev_rcu!(net, dev, { let ml = can_get_ml_priv(dev); if !ml.is_null() { let l = &mut (*ml).dev_rcv_lists; can_rcvlist_proc_show_array(m, dev, l.rx_eff.as_mut_ptr(), l.rx_eff.len()); } });
    rcu_read_unlock!(); seq_putc!(m, '\n'); 0
}

pub unsafe fn can_init_proc(net: *mut net) {
    (*net).can.proc_dir = proc_net_mkdir!(net, "can", (*net).proc_net);
    if (*net).can.proc_dir.is_null() { printk!(KERN_INFO, "can: failed to create /proc/net/can . CONFIG_PROC_FS missing?\n"); return; }
    (*net).can.pde_stats = proc_create_net_single!(CAN_PROC_STATS, 0o644, (*net).can.proc_dir, can_stats_proc_show, core::ptr::null_mut());
    (*net).can.pde_reset_stats = proc_create_net_single!(CAN_PROC_RESET_STATS, 0o644, (*net).can.proc_dir, can_reset_stats_proc_show, core::ptr::null_mut());
    (*net).can.pde_rcvlist_err = proc_create_net_single!(CAN_PROC_RCVLIST_ERR, 0o644, (*net).can.proc_dir, can_rcvlist_proc_show, RX_ERR as *mut _);
    (*net).can.pde_rcvlist_all = proc_create_net_single!(CAN_PROC_RCVLIST_ALL, 0o644, (*net).can.proc_dir, can_rcvlist_proc_show, RX_ALL as *mut _);
    (*net).can.pde_rcvlist_fil = proc_create_net_single!(CAN_PROC_RCVLIST_FIL, 0o644, (*net).can.proc_dir, can_rcvlist_proc_show, RX_FIL as *mut _);
    (*net).can.pde_rcvlist_inv = proc_create_net_single!(CAN_PROC_RCVLIST_INV, 0o644, (*net).can.proc_dir, can_rcvlist_proc_show, RX_INV as *mut _);
    (*net).can.pde_rcvlist_eff = proc_create_net_single!(CAN_PROC_RCVLIST_EFF, 0o644, (*net).can.proc_dir, can_rcvlist_eff_proc_show, core::ptr::null_mut());
    (*net).can.pde_rcvlist_sff = proc_create_net_single!(CAN_PROC_RCVLIST_SFF, 0o644, (*net).can.proc_dir, can_rcvlist_sff_proc_show, core::ptr::null_mut());
}

pub unsafe fn can_remove_proc(net: *mut net) {
    if (*net).can.proc_dir.is_null() { return; }
    if !(*net).can.pde_stats.is_null() { remove_proc_entry!(CAN_PROC_STATS, (*net).can.proc_dir); }
    if !(*net).can.pde_reset_stats.is_null() { remove_proc_entry!(CAN_PROC_RESET_STATS, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_err.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_ERR, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_all.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_ALL, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_fil.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_FIL, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_inv.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_INV, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_eff.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_EFF, (*net).can.proc_dir); }
    if !(*net).can.pde_rcvlist_sff.is_null() { remove_proc_entry!(CAN_PROC_RCVLIST_SFF, (*net).can.proc_dir); }
    remove_proc_entry!("can", (*net).proc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
