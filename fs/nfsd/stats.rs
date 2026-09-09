// SPDX-License-Identifier: GPL-2.0
/*
 * procfs-based user access to knfsd statistics
 *
 * /proc/net/rpc/nfsd
 *
 * Format:
 *	rc <hits> <misses> <nocache>
 *			Statistsics for the reply cache
 *	fh <stale> <deprecated filehandle cache stats>
 *			statistics for filehandle lookup
 *	io <bytes-read> <bytes-written>
 *			statistics for IO throughput
 *	th <threads> <deprecated thread usage histogram stats>
 *			number of threads
 *	ra <deprecated ra-cache stats>
 *
 *	plus generic RPC stats (see net/sunrpc/stats.c)
 *
 * Copyright (C) 1995, 1996, 1997 Olaf Kirch <okir@monad.swb.de>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/seq_file.h, linux/module.h, linux/sunrpc/stats.h,
// net/net_namespace.h, nfsd.h, netns.h, stats.h

unsafe fn nfsd_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let net: *mut net = pde_data(file_inode((*seq).file));
    let nn: *mut nfsd_net = net_generic(net, nfsd_net_id);
    let mut i: i32;

    seq_printf(
        seq,
        c"rc %lld %lld %lld\nfh %lld 0 0 0 0\nio %lld %lld\n",
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_RC_HITS]),
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_RC_MISSES]),
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_RC_NOCACHE]),
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_FH_STALE]),
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_IO_READ]),
        percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_IO_WRITE]),
    );

    /* thread usage: */
    seq_printf(seq, c"th %u 0", atomic_read(&nfsd_th_cnt));

    /* deprecated thread usage histogram stats */
    i = 0;
    while i < 10 {
        seq_puts(seq, c" 0.000");
        i += 1;
    }

    /* deprecated ra-cache stats */
    seq_puts(seq, c"\nra 0 0 0 0 0 0 0 0 0 0 0 0\n");

    /* show my rpc info */
    svc_seq_show(seq, &mut (*nn).nfsd_svcstats);

    // C conditional: CONFIG_NFSD_V4
    #[cfg(feature = "CONFIG_NFSD_V4")]
    {
        /* Show count for individual nfsv4 operations */
        /* Writing operation numbers 0 1 2 also for maintaining uniformity */
        seq_printf(seq, c"proc4ops %u", LAST_NFS4_OP + 1);
        i = 0;
        while i <= LAST_NFS4_OP {
            seq_printf(
                seq,
                c" %lld",
                percpu_counter_sum_positive(&mut (*nn).counter[NFSD_STATS_NFS4_OP(i)]),
            );
            i += 1;
        }
        seq_printf(
            seq,
            c"\nwdeleg_getattr %lld",
            percpu_counter_sum_positive(&mut (*nn).cb_counter[OP_CB_GETATTR]),
        );

        seq_putc(seq, b'\n' as i32);
    }

    0
}

// DEFINE_PROC_SHOW_ATTRIBUTE(nfsd);
extern "C" {
    static nfsd_proc_ops: proc_ops;
}

unsafe fn nfsd_proc_stat_init(net: *mut net) -> *mut proc_dir_entry {
    let nn: *mut nfsd_net = net_generic(net, nfsd_net_id);

    svc_proc_register(net, &mut (*nn).nfsd_svcstats, &nfsd_proc_ops)
}

unsafe fn nfsd_proc_stat_shutdown(net: *mut net) {
    svc_proc_unregister(net, c"nfsd");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
