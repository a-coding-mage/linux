// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/stats.c
 *
 * procfs-based user access to generic RPC statistics. The stats files
 * reside in /proc/net/rpc.
 *
 * The read routines assume that the buffer passed in is just big enough.
 * If you implement an RPC service that has its own stats routine which
 * appends the generic RPC stats, make sure you don't exceed the PAGE_SIZE
 * limit.
 *
 * Copyright (C) 1995, 1996, 1997 Olaf Kirch <okir@monad.swb.de>
 */

// Kernel dependencies supplied by other translation units.

const RPCDBG_FACILITY: u32 = RPCDBG_MISC;

unsafe fn rpc_proc_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let statp = (*seq).private as *const rpc_stat;
    let prog = (*statp).program;
    let mut i: u32 = 0;
    let mut j: u32;

    seq_printf(seq, b"net %u %u %u %u\n\0".as_ptr() as *const i8,
        (*statp).netcnt, (*statp).netudpcnt, (*statp).nettcpcnt, (*statp).nettcpconn);
    seq_printf(seq, b"rpc %u %u %u\n\0".as_ptr() as *const i8,
        (*statp).rpccnt, (*statp).rpcretrans, (*statp).rpcauthrefresh);

    while i < (*prog).nrvers {
        let vers = *(*prog).version.add(i as usize);
        if !vers.is_null() {
            seq_printf(seq, b"proc%u %u\0".as_ptr() as *const i8, (*vers).number, (*vers).nrprocs);
            j = 0;
            while j < (*vers).nrprocs {
                seq_printf(seq, b" %u\0".as_ptr() as *const i8, *(*vers).counts.add(j as usize));
                j += 1;
            }
            seq_putc(seq, b'\n' as i32);
        }
        i += 1;
    }
    0
}

unsafe fn rpc_proc_open(inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, Some(rpc_proc_show), pde_data(inode))
}

static rpc_proc_ops: proc_ops = proc_ops {
    proc_open: Some(rpc_proc_open),
    proc_read: Some(seq_read),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release),
};

unsafe fn svc_seq_show(seq: *mut seq_file, statp: *const svc_stat) {
    let prog = (*statp).program;
    let mut i: u32 = 0;
    let mut j: u32;
    let mut k: u32;
    let mut count: c_ulong;

    seq_printf(seq, b"net %u %u %u %u\n\0".as_ptr() as *const i8,
        (*statp).netcnt, (*statp).netudpcnt, (*statp).nettcpcnt, (*statp).nettcpconn);
    seq_printf(seq, b"rpc %u %u %u %u %u\n\0".as_ptr() as *const i8,
        (*statp).rpccnt, (*statp).rpcbadfmt + (*statp).rpcbadauth + (*statp).rpcbadclnt,
        (*statp).rpcbadfmt, (*statp).rpcbadauth, (*statp).rpcbadclnt);

    while i < (*prog).pg_nvers {
        let vers = *(*prog).pg_vers.add(i as usize);
        if !vers.is_null() {
            seq_printf(seq, b"proc%d %u\0".as_ptr() as *const i8, i as i32, (*vers).vs_nproc);
            j = 0;
            while j < (*vers).vs_nproc {
                count = 0;
                for_each_possible_cpu!(k, {
                    count += per_cpu!((*statp).vs_count[i as usize][j as usize], k);
                });
                seq_printf(seq, b" %lu\0".as_ptr() as *const i8, count);
                j += 1;
            }
            seq_putc(seq, b'\n' as i32);
        }
        i += 1;
    }
}

unsafe fn rpc_alloc_iostats(clnt: *mut rpc_clnt) -> *mut rpc_iostats {
    let stats = kzalloc_objs::<rpc_iostats>((*clnt).cl_maxproc);
    if !stats.is_null() {
        for i in 0..(*clnt).cl_maxproc {
            spin_lock_init(&mut (*stats.add(i as usize)).om_lock);
        }
    }
    stats
}

unsafe fn rpc_free_iostats(stats: *mut rpc_iostats) { kfree(stats as *mut core::ffi::c_void); }

unsafe fn rpc_count_iostats_metrics(task: *const rpc_task, op_metrics: *mut rpc_iostats) {
    let req = (*task).tk_rqstp;
    if op_metrics.is_null() || req.is_null() { return; }
    let now = ktime_get();
    spin_lock(&mut (*op_metrics).om_lock);
    (*op_metrics).om_ops += 1;
    (*op_metrics).om_ntrans += core::cmp::max((*req).rq_ntrans, 1);
    (*op_metrics).om_timeouts += (*task).tk_timeouts;
    (*op_metrics).om_bytes_sent += (*req).rq_xmit_bytes_sent;
    (*op_metrics).om_bytes_recv += (*req).rq_reply_bytes_recvd;
    let mut backlog = 0;
    if ktime_to_ns((*req).rq_xtime) != 0 {
        backlog = ktime_sub((*req).rq_xtime, (*task).tk_start);
        (*op_metrics).om_queue = ktime_add((*op_metrics).om_queue, backlog);
    }
    (*op_metrics).om_rtt = ktime_add((*op_metrics).om_rtt, (*req).rq_rtt);
    let execute = ktime_sub(now, (*task).tk_start);
    (*op_metrics).om_execute = ktime_add((*op_metrics).om_execute, execute);
    if (*task).tk_status < 0 { (*op_metrics).om_error_status += 1; }
    spin_unlock(&mut (*op_metrics).om_lock);
    trace_rpc_stats_latency((*req).rq_task, backlog, (*req).rq_rtt, execute);
}

unsafe fn rpc_count_iostats(task: *const rpc_task, stats: *mut rpc_iostats) {
    rpc_count_iostats_metrics(task, stats.add((*(*task).tk_msg.rpc_proc).p_statidx as usize));
}

unsafe fn _print_name(seq: *mut seq_file, op: u32, procs: *const rpc_procinfo) {
    if !(*procs.add(op as usize)).p_name.is_null() {
        seq_printf(seq, b"\t%12s: \0".as_ptr() as *const i8, (*procs.add(op as usize)).p_name);
    } else if op == 0 { seq_printf(seq, b"\t        NULL: \0".as_ptr() as *const i8); }
    else { seq_printf(seq, b"\t%12u: \0".as_ptr() as *const i8, op); }
}

unsafe fn _add_rpc_iostats(a: *mut rpc_iostats, b: *mut rpc_iostats) {
    (*a).om_ops += (*b).om_ops; (*a).om_ntrans += (*b).om_ntrans; (*a).om_timeouts += (*b).om_timeouts;
    (*a).om_bytes_sent += (*b).om_bytes_sent; (*a).om_bytes_recv += (*b).om_bytes_recv;
    (*a).om_queue = ktime_add((*a).om_queue, (*b).om_queue); (*a).om_rtt = ktime_add((*a).om_rtt, (*b).om_rtt);
    (*a).om_execute = ktime_add((*a).om_execute, (*b).om_execute); (*a).om_error_status += (*b).om_error_status;
}

unsafe fn _print_rpc_iostats(seq: *mut seq_file, stats: *mut rpc_iostats, op: i32, procs: *const rpc_procinfo) {
    _print_name(seq, op as u32, procs);
    seq_printf(seq, b"%lu %lu %lu %llu %llu %llu %llu %llu %lu\n\0".as_ptr() as *const i8,
        (*stats).om_ops, (*stats).om_ntrans, (*stats).om_timeouts, (*stats).om_bytes_sent,
        (*stats).om_bytes_recv, ktime_to_ms((*stats).om_queue), ktime_to_ms((*stats).om_rtt),
        ktime_to_ms((*stats).om_execute), (*stats).om_error_status);
}

unsafe fn do_print_stats(_clnt: *mut rpc_clnt, xprt: *mut rpc_xprt, seqv: *mut core::ffi::c_void) -> i32 {
    ((*(*xprt).ops).print_stats)(xprt, seqv as *mut seq_file); 0
}

unsafe fn rpc_clnt_show_stats(seq: *mut seq_file, clnt: *mut rpc_clnt) {
    let maxproc = (*clnt).cl_maxproc;
    if (*clnt).cl_metrics.is_null() { return; }
    seq_printf(seq, b"\tRPC iostats version: %s  \0".as_ptr() as *const i8, RPC_IOSTATS_VERS.as_ptr());
    seq_printf(seq, b"p/v: %u/%u (%s)\n\0".as_ptr() as *const i8, (*clnt).cl_prog, (*clnt).cl_vers, (*(*clnt).cl_program).name);
    rpc_clnt_iterate_for_each_xprt(clnt, Some(do_print_stats), seq as *mut core::ffi::c_void);
    seq_printf(seq, b"\tper-op statistics\n\0".as_ptr() as *const i8);
    for op in 0..maxproc {
        let mut stats: rpc_iostats = core::mem::zeroed();
        let mut next = clnt;
        loop {
            _add_rpc_iostats(&mut stats, &mut *(*next).cl_metrics.add(op as usize));
            if next == (*next).cl_parent { break; }
            next = (*next).cl_parent;
            if next.is_null() { break; }
        }
        _print_rpc_iostats(seq, &mut stats, op as i32, (*clnt).cl_procinfo);
    }
}

unsafe fn do_register(net: *mut net, name: *const i8, data: *mut core::ffi::c_void, proc_ops: *const proc_ops) -> *mut proc_dir_entry {
    let sn = net_generic(net, sunrpc_net_id);
    proc_create_data(name, 0, (*sn).proc_net_rpc, proc_ops, data)
}

unsafe fn rpc_proc_register(net: *mut net, statp: *mut rpc_stat) -> *mut proc_dir_entry {
    do_register(net, (*(*statp).program).name, statp as *mut core::ffi::c_void, &rpc_proc_ops)
}
unsafe fn rpc_proc_unregister(net: *mut net, name: *const i8) {
    let sn = net_generic(net, sunrpc_net_id); remove_proc_entry(name, (*sn).proc_net_rpc);
}
unsafe fn svc_proc_register(net: *mut net, statp: *mut svc_stat, proc_ops: *const proc_ops) -> *mut proc_dir_entry {
    do_register(net, (*(*statp).program).pg_name, net as *mut core::ffi::c_void, proc_ops)
}
unsafe fn svc_proc_unregister(net: *mut net, name: *const i8) {
    let sn = net_generic(net, sunrpc_net_id); remove_proc_entry(name, (*sn).proc_net_rpc);
}
unsafe fn rpc_proc_init(net: *mut net) -> i32 {
    let sn = net_generic(net, sunrpc_net_id);
    (*sn).proc_net_rpc = proc_mkdir(b"rpc\0".as_ptr() as *const i8, (*net).proc_net);
    if (*sn).proc_net_rpc.is_null() { return -12; }
    0
}
unsafe fn rpc_proc_exit(net: *mut net) { remove_proc_entry(b"rpc\0".as_ptr() as *const i8, (*net).proc_net); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
