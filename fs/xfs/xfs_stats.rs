// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS/kernel translation.

static mut xfsstats: xstats = xstats { /* external layout supplied elsewhere */ };

unsafe fn counter_val(stats: *mut xfsstats_percpu, idx: i32) -> i32 {
    let mut val: i32 = 0;
    let mut cpu: i32;
    for_each_possible_cpu!(cpu) {
        val += *((per_cpu_ptr(stats, cpu) as *mut u32).add(idx as usize)) as i32;
    }
    val
}

unsafe fn xfs_stats_format(stats: *mut xfsstats_percpu, buf: *mut c_char) -> i32 {
    let mut i: i32;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut xs_xstrat_bytes: u64 = 0;
    let mut xs_write_bytes: u64 = 0;
    let mut xs_read_bytes: u64 = 0;
    let mut xs_defer_relog: u64 = 0;
    let mut xs_gc_bytes: u64 = 0;

    #[repr(C)]
    struct XstatsEntry {
        desc: *mut c_char,
        endpoint: i32,
    }

    let xstats: [XstatsEntry; 30] = [
        XstatsEntry { desc: c"extent_alloc".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_abt_lookup) },
        XstatsEntry { desc: c"abt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_blk_mapr) },
        XstatsEntry { desc: c"blk_map".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_bmbt_lookup) },
        XstatsEntry { desc: c"bmbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_dir_lookup) },
        XstatsEntry { desc: c"dir".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_trans_sync) },
        XstatsEntry { desc: c"trans".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_ig_attempts) },
        XstatsEntry { desc: c"ig".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_log_writes) },
        XstatsEntry { desc: c"log".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_try_logspace) },
        XstatsEntry { desc: c"push_ail".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_xstrat_quick) },
        XstatsEntry { desc: c"xstrat".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_write_calls) },
        XstatsEntry { desc: c"rw".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_attr_get) },
        XstatsEntry { desc: c"attr".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_iflush_count) },
        XstatsEntry { desc: c"icluster".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_inodes_active) },
        XstatsEntry { desc: c"vnodes".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xb_get) },
        XstatsEntry { desc: c"buf".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_abtb_2) },
        XstatsEntry { desc: c"abtb2".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_abtc_2) },
        XstatsEntry { desc: c"abtc2".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_bmbt_2) },
        XstatsEntry { desc: c"bmbt2".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_ibt_2) },
        XstatsEntry { desc: c"ibt2".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_fibt_2) },
        XstatsEntry { desc: c"fibt2".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rmap_2) },
        XstatsEntry { desc: c"rmapbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_refcbt_2) },
        XstatsEntry { desc: c"refcntbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rmap_mem_2) },
        XstatsEntry { desc: c"rmapbt_mem".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rcbag_2) },
        XstatsEntry { desc: c"rcbagbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rtrmap_2) },
        XstatsEntry { desc: c"rtrmapbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rtrmap_mem_2) },
        XstatsEntry { desc: c"rtrmapbt_mem".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_rtrefcbt_2) },
        XstatsEntry { desc: c"rtrefcntbt".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_qm_dqreclaims) },
        // We print both series of quota information together.
        XstatsEntry { desc: c"qm".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_gc_read_calls) },
        XstatsEntry { desc: c"zoned".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_inodes_meta) },
        XstatsEntry { desc: c"metafile".as_ptr() as *mut c_char, endpoint: xfsstats_offset!(xs_xstrat_bytes) },
    ];

    // Loop over all stats groups.
    i = 0;
    while i < xstats.len() as i32 {
        len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"%s".as_ptr(), xstats[i as usize].desc);
        while j < xstats[i as usize].endpoint {
            len += scnprintf(buf.add(len as usize), PATH_MAX - len, c" %u".as_ptr(), counter_val(stats, j));
            j += 1;
        }
        len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"\n".as_ptr());
        i += 1;
    }

    for_each_possible_cpu!(i) {
        xs_xstrat_bytes += (*per_cpu_ptr(stats, i)).s.xs_xstrat_bytes as u64;
        xs_write_bytes += (*per_cpu_ptr(stats, i)).s.xs_write_bytes as u64;
        xs_read_bytes += (*per_cpu_ptr(stats, i)).s.xs_read_bytes as u64;
        xs_defer_relog += (*per_cpu_ptr(stats, i)).s.xs_defer_relog as u64;
        xs_gc_bytes += (*per_cpu_ptr(stats, i)).s.xs_gc_bytes as u64;
    }

    len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"xpc %llu %llu %llu\n".as_ptr(), xs_xstrat_bytes, xs_write_bytes, xs_read_bytes);
    len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"defer_relog %llu\n".as_ptr(), xs_defer_relog);
    len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"debug %u\n".as_ptr(), if cfg!(feature = "DEBUG") { 1 } else { 0 });
    len += scnprintf(buf.add(len as usize), PATH_MAX - len, c"gc xpc %llu\n".as_ptr(), xs_gc_bytes);
    len
}

unsafe fn xfs_stats_clearall(stats: *mut xfsstats_percpu) {
    let mut xs_inodes_active: u32;
    let mut xs_inodes_meta: u32;
    let mut c: i32;
    xfs_notice!(core::ptr::null_mut(), "Clearing xfsstats");
    for_each_possible_cpu!(c) {
        preempt_disable();
        // Save the active / meta inode counters, as they are stateful.
        xs_inodes_active = (*per_cpu_ptr(stats, c)).s.xs_inodes_active;
        xs_inodes_meta = (*per_cpu_ptr(stats, c)).s.xs_inodes_meta;
        core::ptr::write_bytes(per_cpu_ptr(stats, c) as *mut u8, 0, core::mem::size_of::<xfsstats_percpu>());
        (*per_cpu_ptr(stats, c)).s.xs_inodes_active = xs_inodes_active;
        (*per_cpu_ptr(stats, c)).s.xs_inodes_meta = xs_inodes_meta;
        preempt_enable();
    }
}

// The following legacy procfs interfaces are present only when CONFIG_PROC_FS
// and, where indicated, CONFIG_XFS_QUOTA are enabled in the C build.
#[cfg(feature = "CONFIG_XFS_QUOTA")]
const XFSSTAT_START_XQMSTAT: i32 = xfsstats_offset!(xs_qm_dqreclaims);
#[cfg(feature = "CONFIG_XFS_QUOTA")]
const XFSSTAT_END_XQMSTAT: i32 = xfsstats_offset!(xs_qm_dquot);

#[cfg(feature = "CONFIG_XFS_QUOTA")]
unsafe fn xqm_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    seq_printf!(m, c"%d\t%d\t%d\t%u\t%s\n", 0, counter_val((*core::ptr::addr_of_mut!(xfsstats)).xs_stats, XFSSTAT_END_XQMSTAT), 0, counter_val((*core::ptr::addr_of_mut!(xfsstats)).xs_stats, XFSSTAT_END_XQMSTAT + 1), if cfg!(feature = "CONFIG_XFS_RT") { c"rtquota" } else { c"quota" });
    0
}

#[cfg(feature = "CONFIG_XFS_QUOTA")]
unsafe fn xqmstat_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    let mut j = XFSSTAT_START_XQMSTAT;
    seq_puts!(m, c"qm");
    while j < XFSSTAT_END_XQMSTAT {
        seq_printf!(m, c" %u", counter_val((*core::ptr::addr_of_mut!(xfsstats)).xs_stats, j));
        j += 1;
    }
    seq_putc!(m, b'\n' as c_int);
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn xfs_init_procfs() -> i32 {
    if proc_mkdir(c"fs/xfs".as_ptr(), core::ptr::null_mut()).is_null() { return -ENOMEM; }
    if proc_symlink(c"fs/xfs/stat".as_ptr(), core::ptr::null_mut(), c"/sys/fs/xfs/stats/stats".as_ptr()).is_null() { remove_proc_subtree(c"fs/xfs".as_ptr(), core::ptr::null_mut()); return -ENOMEM; }
    #[cfg(feature = "CONFIG_XFS_QUOTA")]
    {
        if proc_create_single(c"fs/xfs/xqmstat".as_ptr(), 0, core::ptr::null_mut(), xqmstat_proc_show).is_null() || proc_create_single(c"fs/xfs/xqm".as_ptr(), 0, core::ptr::null_mut(), xqm_proc_show).is_null() { remove_proc_subtree(c"fs/xfs".as_ptr(), core::ptr::null_mut()); return -ENOMEM; }
    }
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn xfs_cleanup_procfs() { remove_proc_subtree(c"fs/xfs".as_ptr(), core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
