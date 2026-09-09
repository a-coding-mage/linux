// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * This code REQUIRES 2.4 with seq_file support
 *
 * History
 * 2002/10/06 Arnaldo Carvalho de Melo seq_file support
 */

// C dependencies: linux init/proc_fs/seq_file/export, net_namespace, sock, x25.
// CONFIG_PROC_FS is a build-time condition supplied by the surrounding kernel.

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_route_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    read_lock_bh(&raw mut x25_route_list_lock);
    seq_list_start_head(&raw mut x25_route_list, *pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_route_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    seq_list_next(v, &raw mut x25_route_list, pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_route_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    read_unlock_bh(&raw mut x25_route_list_lock);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_route_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let mut rt: *mut x25_route = list_entry(v, core::mem::offset_of!(x25_route, node));
    if v == (&raw mut x25_route_list).cast() {
        seq_puts(seq, "Address          Digits  Device\n");
        return 0;
    }
    rt = v.cast();
    seq_printf(seq, "%-15s  %-6d  %-5s\n", (*rt).address.x25_addr,
        (*rt).sigdigits, if !(*rt).dev.is_null() { (*rt).dev.as_ref().unwrap().name } else { "???" });
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_socket_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    read_lock_bh(&raw mut x25_list_lock);
    seq_hlist_start_head(&raw mut x25_list, *pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_socket_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    seq_hlist_next(v, &raw mut x25_list, pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_socket_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    read_unlock_bh(&raw mut x25_list_lock);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_socket_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_printf(seq, "dest_addr  src_addr   dev   lci st vs vr va   t  t2 t21 t22 t23 Snd-Q Rcv-Q inode\n");
        return 0;
    }
    let s: *mut sock = sk_entry(v);
    let x25: *mut x25_sock = x25_sk(s);
    let devname = if (*x25).neighbour.is_null() || (*(*x25).neighbour).dev.is_null() {
        "???"
    } else { (*(*(*x25).neighbour).dev).name };
    seq_printf(seq, "%-10s %-10s %-5s %3.3X  %d  %d  %d  %d %3lu %3lu %3lu %3lu %3lu %5d %5d %llu\n",
        if (*x25).dest_addr.x25_addr[0] == 0 { "*" } else { (*x25).dest_addr.x25_addr },
        if (*x25).source_addr.x25_addr[0] == 0 { "*" } else { (*x25).source_addr.x25_addr },
        devname, (*x25).lci & 0x0fff, (*x25).state, (*x25).vs, (*x25).vr, (*x25).va,
        x25_display_timer(s) / HZ, (*x25).t2 / HZ, (*x25).t21 / HZ, (*x25).t22 / HZ,
        (*x25).t23 / HZ, sk_wmem_alloc_get(s), sk_rmem_alloc_get(s),
        if !(*s).sk_socket.is_null() { SOCK_INODE((*s).sk_socket).i_ino } else { 0u64 });
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_forward_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    read_lock_bh(&raw mut x25_forward_list_lock);
    seq_list_start_head(&raw mut x25_forward_list, *pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_forward_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    seq_list_next(v, &raw mut x25_forward_list, pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_forward_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    read_unlock_bh(&raw mut x25_forward_list_lock);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn x25_seq_forward_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == (&raw mut x25_forward_list).cast() {
        seq_printf(seq, "lci dev1       dev2\n");
        return 0;
    }
    let f: *mut x25_forward = v.cast();
    seq_printf(seq, "%d %-10s %-10s\n", (*f).lci, (*(*f).dev1).name, (*(*f).dev2).name);
    0
}

#[cfg(CONFIG_PROC_FS)]
static x25_seq_route_ops: seq_operations = seq_operations { start: Some(x25_seq_route_start), next: Some(x25_seq_route_next), stop: Some(x25_seq_route_stop), show: Some(x25_seq_route_show) };
#[cfg(CONFIG_PROC_FS)]
static x25_seq_socket_ops: seq_operations = seq_operations { start: Some(x25_seq_socket_start), next: Some(x25_seq_socket_next), stop: Some(x25_seq_socket_stop), show: Some(x25_seq_socket_show) };
#[cfg(CONFIG_PROC_FS)]
static x25_seq_forward_ops: seq_operations = seq_operations { start: Some(x25_seq_forward_start), next: Some(x25_seq_forward_next), stop: Some(x25_seq_forward_stop), show: Some(x25_seq_forward_show) };

unsafe fn x25_proc_init() -> i32 {
    #[cfg(CONFIG_PROC_FS)] {
        if proc_mkdir("x25", (*init_net).proc_net).is_null() { return -ENOMEM; }
        if proc_create_seq("x25/route", 0o444, (*init_net).proc_net, &x25_seq_route_ops).is_null() { remove_proc_subtree("x25", (*init_net).proc_net); return -ENOMEM; }
        if proc_create_seq("x25/socket", 0o444, (*init_net).proc_net, &x25_seq_socket_ops).is_null() { remove_proc_subtree("x25", (*init_net).proc_net); return -ENOMEM; }
        if proc_create_seq("x25/forward", 0o444, (*init_net).proc_net, &x25_seq_forward_ops).is_null() { remove_proc_subtree("x25", (*init_net).proc_net); return -ENOMEM; }
        return 0;
    }
    #[cfg(not(CONFIG_PROC_FS))]
    { 0 }
}

unsafe fn x25_proc_exit() {
    #[cfg(CONFIG_PROC_FS)]
    remove_proc_subtree("x25", (*init_net).proc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
