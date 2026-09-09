// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * netdebug.c
 *
 * debug functionality for o2net
 *
 * Copyright (C) 2005, 2008 Oracle.  All rights reserved.
 */

// CONFIG_DEBUG_FS conditional preserved from the original source.

const O2NET_DEBUG_DIR: &[u8] = b"o2net\0";
const SC_DEBUG_NAME: &[u8] = b"sock_containers\0";
const NST_DEBUG_NAME: &[u8] = b"send_tracking\0";
const STATS_DEBUG_NAME: &[u8] = b"stats\0";
const NODES_DEBUG_NAME: &[u8] = b"connected_nodes\0";

const SHOW_SOCK_CONTAINERS: i32 = 0;
const SHOW_SOCK_STATS: i32 = 1;

static mut O2NET_DENTRY: *mut dentry = core::ptr::null_mut();
static mut O2NET_DEBUG_LOCK: spinlock_t = unsafe { core::mem::zeroed() };
static mut SOCK_CONTAINERS: list_head = unsafe { core::mem::zeroed() };
static mut SEND_TRACKING: list_head = unsafe { core::mem::zeroed() };

pub unsafe fn o2net_debug_add_nst(nst: *mut o2net_send_tracking) {
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    list_add(&mut (*nst).st_net_debug_item, &raw mut SEND_TRACKING);
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
}

pub unsafe fn o2net_debug_del_nst(nst: *mut o2net_send_tracking) {
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    if !list_empty(&raw mut (*nst).st_net_debug_item) {
        list_del_init(&mut (*nst).st_net_debug_item);
    }
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
}

unsafe fn next_nst(mut nst_start: *mut o2net_send_tracking) -> *mut o2net_send_tracking {
    let mut nst: *mut o2net_send_tracking;
    let mut ret: *mut o2net_send_tracking = core::ptr::null_mut();
    assert_spin_locked(&raw mut O2NET_DEBUG_LOCK);
    list_for_each_entry!(nst, &mut (*nst_start).st_net_debug_item, st_net_debug_item, {
        if core::ptr::eq(&(*nst).st_net_debug_item, &raw mut SEND_TRACKING) { break; }
        if !(*nst).st_task.is_null() { ret = nst; break; }
    });
    ret
}

unsafe fn nst_seq_start(seq: *mut seq_file, _pos: *mut loff_t) -> *mut core::ffi::c_void {
    let dummy_nst = (*seq).private as *mut o2net_send_tracking;
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    let nst = next_nst(dummy_nst);
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
    nst.cast()
}

unsafe fn nst_seq_next(seq: *mut seq_file, _v: *mut core::ffi::c_void, _pos: *mut loff_t) -> *mut core::ffi::c_void {
    let dummy_nst = (*seq).private as *mut o2net_send_tracking;
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    let nst = next_nst(dummy_nst);
    list_del_init(&mut (*dummy_nst).st_net_debug_item);
    if !nst.is_null() { list_add(&mut (*dummy_nst).st_net_debug_item, &mut (*nst).st_net_debug_item); }
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
    nst.cast()
}

unsafe fn nst_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let dummy_nst = (*seq).private as *mut o2net_send_tracking;
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    let nst = next_nst(dummy_nst);
    if !nst.is_null() {
        let now = ktime_get();
        let sock = ktime_to_us(ktime_sub(now, (*nst).st_sock_time));
        let send = ktime_to_us(ktime_sub(now, (*nst).st_send_time));
        let status = ktime_to_us(ktime_sub(now, (*nst).st_status_time));
        seq_printf(seq, b"%p:\n  pid:          %lu\n  tgid:         %lu\n  process name: %s\n  node:         %u\n  sc:           %p\n  message id:   %d\n  message type: %u\n  message key:  0x%08x\n  sock acquiry: %lld usecs ago\n  send start:   %lld usecs ago\n  wait start:   %lld usecs ago\n\0".as_ptr().cast(), nst, task_pid_nr((*nst).st_task) as c_ulong, (*(*nst).st_task).tgid as c_ulong, (*(*nst).st_task).comm.as_ptr(), (*nst).st_node, (*nst).st_sc, (*nst).st_id, (*nst).st_msg_type, (*nst).st_msg_key, sock, send, status);
    }
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
    0
}

unsafe fn nst_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}

static NST_SEQ_OPS: seq_operations = seq_operations { start: Some(nst_seq_start), next: Some(nst_seq_next), stop: Some(nst_seq_stop), show: Some(nst_seq_show) };

unsafe fn nst_fop_open(_inode: *mut inode, file: *mut file) -> i32 {
    let dummy_nst = __seq_open_private(file, &raw const NST_SEQ_OPS, core::mem::size_of::<o2net_send_tracking>()) as *mut o2net_send_tracking;
    if dummy_nst.is_null() { return -ENOMEM; }
    o2net_debug_add_nst(dummy_nst);
    0
}

unsafe fn nst_fop_release(inode: *mut inode, file: *mut file) -> i32 {
    let seq = (*file).private_data as *mut seq_file;
    o2net_debug_del_nst((*seq).private as *mut o2net_send_tracking);
    seq_release_private(inode, file)
}

pub unsafe fn o2net_debug_add_sc(sc: *mut o2net_sock_container) {
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    list_add(&mut (*sc).sc_net_debug_item, &raw mut SOCK_CONTAINERS);
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
}

pub unsafe fn o2net_debug_del_sc(sc: *mut o2net_sock_container) {
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    list_del_init(&mut (*sc).sc_net_debug_item);
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK);
}

#[repr(C)]
struct o2net_sock_debug { dbg_ctxt: i32, dbg_sock: *mut o2net_sock_container }

unsafe fn next_sc(sc_start: *mut o2net_sock_container) -> *mut o2net_sock_container {
    let mut sc: *mut o2net_sock_container;
    let mut ret = core::ptr::null_mut();
    assert_spin_locked(&raw mut O2NET_DEBUG_LOCK);
    list_for_each_entry!(sc, &mut (*sc_start).sc_net_debug_item, sc_net_debug_item, {
        if core::ptr::eq(&(*sc).sc_net_debug_item, &raw mut SOCK_CONTAINERS) { break; }
        if !(*sc).sc_page.is_null() { ret = sc; break; }
    });
    ret
}

unsafe fn sc_seq_start(seq: *mut seq_file, _pos: *mut loff_t) -> *mut core::ffi::c_void {
    let sd = (*seq).private as *mut o2net_sock_debug;
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK); let sc = next_sc((*sd).dbg_sock); spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK); sc.cast()
}

unsafe fn sc_seq_next(seq: *mut seq_file, _v: *mut core::ffi::c_void, _pos: *mut loff_t) -> *mut core::ffi::c_void {
    let sd = (*seq).private as *mut o2net_sock_debug;
    spin_lock_bh(&raw mut O2NET_DEBUG_LOCK);
    let sc = next_sc((*sd).dbg_sock); list_del_init(&mut (*(*sd).dbg_sock).sc_net_debug_item);
    if !sc.is_null() { list_add(&mut (*(*sd).dbg_sock).sc_net_debug_item, &mut (*sc).sc_net_debug_item); }
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK); sc.cast()
}

unsafe fn sc_show_sock_stats(seq: *mut seq_file, sc: *mut o2net_sock_container) {
    if sc.is_null() { return; }
    seq_printf(seq, b"%d,%u,%lu,%lld,%lld,%lld,%lu,%lld\n\0".as_ptr().cast(), 1, (*(*sc).sc_node).nd_num, sc_send_count(sc), sc_tv_acquiry_total_ns(sc), sc_tv_send_total_ns(sc), sc_tv_status_total_ns(sc), sc_recv_count(sc), sc_tv_process_total_ns(sc));
}

unsafe fn sc_show_sock_container(seq: *mut seq_file, sc: *mut o2net_sock_container) {
    if sc.is_null() { return; }
    let mut saddr: __be32 = 0; let mut daddr: __be32 = 0; let mut sport: __be16 = 0; let mut dport: __be16 = 0;
    if !(*sc).sc_sock.is_null() { let inet = inet_sk((*(*sc).sc_sock).sk); saddr = (*inet).inet_saddr; daddr = (*inet).inet_daddr; sport = (*inet).inet_sport; dport = (*inet).inet_dport; }
    seq_printf(seq, b"%p:\n  krefs:           %d\n  sock:            %pI4:%u -> %pI4:%u\n  remote node:     %s\n  page off:        %zu\n  handshake ok:    %u\n  timer:           %lld usecs\n  data ready:      %lld usecs\n  advance start:   %lld usecs\n  advance stop:    %lld usecs\n  func start:      %lld usecs\n  func stop:       %lld usecs\n  func key:        0x%08x\n  func type:       %u\n\0".as_ptr().cast(), sc, kref_read(&(*sc).sc_kref), &saddr, if !(*sc).sc_sock.is_null() { ntohs(sport) } else { 0 }, &daddr, if !(*sc).sc_sock.is_null() { ntohs(dport) } else { 0 }, (*(*sc).sc_node).nd_name.as_ptr(), (*sc).sc_page_off, (*sc).sc_handshake_ok, ktime_to_us((*sc).sc_tv_timer), ktime_to_us((*sc).sc_tv_data_ready), ktime_to_us((*sc).sc_tv_advance_start), ktime_to_us((*sc).sc_tv_advance_stop), ktime_to_us((*sc).sc_tv_func_start), ktime_to_us((*sc).sc_tv_func_stop), (*sc).sc_msg_key, (*sc).sc_msg_type);
}

unsafe fn sc_seq_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let sd = (*seq).private as *mut o2net_sock_debug; spin_lock_bh(&raw mut O2NET_DEBUG_LOCK); let sc = next_sc((*sd).dbg_sock);
    if !sc.is_null() { if (*sd).dbg_ctxt == SHOW_SOCK_CONTAINERS { sc_show_sock_container(seq, sc); } else { sc_show_sock_stats(seq, sc); } }
    spin_unlock_bh(&raw mut O2NET_DEBUG_LOCK); 0
}
unsafe fn sc_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {}

static SC_SEQ_OPS: seq_operations = seq_operations { start: Some(sc_seq_start), next: Some(sc_seq_next), stop: Some(sc_seq_stop), show: Some(sc_seq_show) };

// These helpers preserve the CONFIG_OCFS2_FS_STATS build-time alternatives.
unsafe fn sc_send_count(s: *mut o2net_sock_container) -> c_ulong { (*s).sc_send_count }
unsafe fn sc_recv_count(s: *mut o2net_sock_container) -> c_ulong { (*s).sc_recv_count }
unsafe fn sc_tv_acquiry_total_ns(s: *mut o2net_sock_container) -> c_longlong { ktime_to_ns((*s).sc_tv_acquiry_total) }
unsafe fn sc_tv_send_total_ns(s: *mut o2net_sock_container) -> c_longlong { ktime_to_ns((*s).sc_tv_send_total) }
unsafe fn sc_tv_status_total_ns(s: *mut o2net_sock_container) -> c_longlong { ktime_to_ns((*s).sc_tv_status_total) }
unsafe fn sc_tv_process_total_ns(s: *mut o2net_sock_container) -> c_longlong { ktime_to_ns((*s).sc_tv_process_total) }

unsafe fn sc_common_open(file: *mut file, ctxt: i32) -> i32 {
    let dummy_sc = kzalloc_obj::<o2net_sock_container>(); if dummy_sc.is_null() { return -ENOMEM; }
    let sd = __seq_open_private(file, &raw const SC_SEQ_OPS, core::mem::size_of::<o2net_sock_debug>()) as *mut o2net_sock_debug;
    if sd.is_null() { kfree(dummy_sc.cast()); return -ENOMEM; }
    (*sd).dbg_ctxt = ctxt; (*sd).dbg_sock = dummy_sc; o2net_debug_add_sc(dummy_sc); 0
}
unsafe fn sc_fop_release(inode: *mut inode, file: *mut file) -> i32 { let seq = (*file).private_data as *mut seq_file; let sd = (*seq).private as *mut o2net_sock_debug; o2net_debug_del_sc((*sd).dbg_sock); kfree((*sd).dbg_sock.cast()); seq_release_private(inode, file) }
unsafe fn stats_fop_open(_inode: *mut inode, file: *mut file) -> i32 { sc_common_open(file, SHOW_SOCK_STATS) }
unsafe fn sc_fop_open(_inode: *mut inode, file: *mut file) -> i32 { sc_common_open(file, SHOW_SOCK_CONTAINERS) }

unsafe fn o2net_fill_bitmap(buf: *mut u8, _len: i32) -> i32 { let mut map = [0usize; BITS_TO_LONGS(O2NM_MAX_NODES)]; let mut i: i32 = -1; let mut out = 0; o2net_fill_node_map(map.as_mut_ptr(), O2NM_MAX_NODES); while { i = find_next_bit(map.as_ptr(), O2NM_MAX_NODES, (i + 1) as usize) as i32; i < O2NM_MAX_NODES } { out += scnprintf(buf.add(out as usize), PAGE_SIZE - out as usize, b"%d \0".as_ptr().cast(), i); } out += scnprintf(buf.add(out as usize), PAGE_SIZE - out as usize, b"\n\0".as_ptr().cast()); out }
unsafe fn nodes_fop_open(inode: *mut inode, file: *mut file) -> i32 { let buf = kmalloc(PAGE_SIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; } i_size_write(inode, o2net_fill_bitmap(buf, PAGE_SIZE as i32) as loff_t); (*file).private_data = buf.cast(); 0 }
unsafe fn o2net_debug_release(_inode: *mut inode, file: *mut file) -> i32 { kfree((*file).private_data); 0 }
unsafe fn o2net_debug_read(file: *mut file, buf: *mut u8, nbytes: usize, ppos: *mut loff_t) -> isize { simple_read_from_buffer(buf, nbytes, ppos, (*file).private_data, i_size_read((*(*file).f_mapping).host)) }

static NST_SEQ_FOPS: file_operations = file_operations { open: Some(nst_fop_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(nst_fop_release) };
static STATS_SEQ_FOPS: file_operations = file_operations { open: Some(stats_fop_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(sc_fop_release) };
static SC_SEQ_FOPS: file_operations = file_operations { open: Some(sc_fop_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(sc_fop_release) };
static NODES_FOPS: file_operations = file_operations { open: Some(nodes_fop_open), release: Some(o2net_debug_release), read: Some(o2net_debug_read), llseek: Some(generic_file_llseek) };

pub unsafe fn o2net_debugfs_exit() { debugfs_remove_recursive(O2NET_DENTRY); }
pub unsafe fn o2net_debugfs_init() { let mode: umode_t = S_IFREG | S_IRUSR; O2NET_DENTRY = debugfs_create_dir(O2NET_DEBUG_DIR.as_ptr().cast(), core::ptr::null_mut()); debugfs_create_file(NST_DEBUG_NAME.as_ptr().cast(), mode, O2NET_DENTRY, core::ptr::null_mut(), &raw const NST_SEQ_FOPS); debugfs_create_file(SC_DEBUG_NAME.as_ptr().cast(), mode, O2NET_DENTRY, core::ptr::null_mut(), &raw const SC_SEQ_FOPS); debugfs_create_file(STATS_DEBUG_NAME.as_ptr().cast(), mode, O2NET_DENTRY, core::ptr::null_mut(), &raw const STATS_SEQ_FOPS); debugfs_create_file(NODES_DEBUG_NAME.as_ptr().cast(), mode, O2NET_DENTRY, core::ptr::null_mut(), &raw const NODES_FOPS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
