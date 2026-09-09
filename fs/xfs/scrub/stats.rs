// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the XFS platform, kernel, scrub, and trace headers.

#[repr(C)]
struct xchk_scrub_stats {
    /* all 32-bit counters here */

    /* checking stats */
    invocations: u32,
    clean: u32,
    corrupt: u32,
    preen: u32,
    xfail: u32,
    xcorrupt: u32,
    incomplete: u32,
    warning: u32,
    retries: u32,

    /* repair stats */
    repair_invocations: u32,
    repair_success: u32,

    /* all 64-bit items here */

    /* runtimes */
    checktime_us: u64,
    repairtime_us: u64,

    /* non-counter state must go at the end for clearall */
    css_lock: spinlock_t,
}

#[repr(C)]
struct xchk_stats {
    cs_debugfs: *mut dentry,
    cs_stats: [xchk_scrub_stats; XFS_SCRUB_TYPE_NR],
}

static mut global_stats: xchk_stats = unsafe { core::mem::zeroed() };

static name_map: [&'static str; XFS_SCRUB_TYPE_NR] = [
    "sb", "agf", "agfl", "agi", "bnobt", "cntbt", "inobt", "finobt",
    "rmapbt", "refcountbt", "inode", "bmapbtd", "bmapbta", "bmapbtc",
    "directory", "xattr", "symlink", "parent", "rtbitmap", "rtsummary",
    "usrquota", "grpquota", "prjquota", "fscounters", "quotacheck", "nlinks",
    "dirtree", "metapath", "rgsuper", "rtrmapbt", "rtrefcountbt",
];

/* Format the scrub stats into a text buffer, similar to pcp style. */
unsafe fn xchk_stats_format(cs: *mut xchk_stats, mut buf: *mut u8, mut remaining: usize) -> isize {
    let mut css = (*cs).cs_stats.as_mut_ptr();
    let mut copied: isize = 0;
    let mut ret: isize = 0;

    for i in 0..XFS_SCRUB_TYPE_NR {
        let _ = i;
        ret = scnprintf(buf, remaining, b"%s %u %u %u %u %u %u %u %u %u %llu %u %u %llu\n\0".as_ptr(),
            name_map[i].as_ptr(), (*css).invocations, (*css).clean, (*css).corrupt,
            (*css).preen, (*css).xfail, (*css).xcorrupt, (*css).incomplete,
            (*css).warning, (*css).retries, (*css).checktime_us,
            (*css).repair_invocations, (*css).repair_success, (*css).repairtime_us);
        if ret <= 0 { break; }
        remaining -= ret as usize;
        copied += ret;
        buf = buf.add(ret as usize);
        css = css.add(1);
    }
    if copied > 0 { copied } else { ret }
}

/* Estimate the worst case buffer size required to hold the whole report. */
unsafe fn xchk_stats_estimate_bufsize(cs: *mut xchk_stats) -> usize {
    let mut ret = 0usize;
    let field_width = 11 * (core::mem::offset_of!(xchk_scrub_stats, checktime_us) / core::mem::size_of::<u32>());
    let field_width = field_width + 21 * ((core::mem::offset_of!(xchk_scrub_stats, css_lock)
        - core::mem::offset_of!(xchk_scrub_stats, checktime_us)) / core::mem::size_of::<u64>());
    for i in 0..XFS_SCRUB_TYPE_NR {
        let _ = cs;
        ret += 1 + name_map[i].len();
        ret += field_width + 1;
    }
    ret
}

/* Clear all counters. */
unsafe fn xchk_stats_clearall(cs: *mut xchk_stats) {
    let mut css = (*cs).cs_stats.as_mut_ptr();
    for _ in 0..XFS_SCRUB_TYPE_NR {
        spin_lock(&mut (*css).css_lock);
        core::ptr::write_bytes(css as *mut u8, 0, core::mem::offset_of!(xchk_scrub_stats, css_lock));
        spin_unlock(&mut (*css).css_lock);
        css = css.add(1);
    }
}

const XFS_SCRUB_OFLAG_UNCLEAN: u32 = XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_PREEN |
    XFS_SCRUB_OFLAG_XFAIL | XFS_SCRUB_OFLAG_XCORRUPT |
    XFS_SCRUB_OFLAG_INCOMPLETE | XFS_SCRUB_OFLAG_WARNING;

unsafe fn xchk_stats_merge_one(cs: *mut xchk_stats, sm: *const xfs_scrub_metadata, run: *const xchk_stats_run) {
    if (*sm).sm_type >= XFS_SCRUB_TYPE_NR { ASSERT((*sm).sm_type < XFS_SCRUB_TYPE_NR); return; }
    let css = &mut (*cs).cs_stats[(*sm).sm_type as usize];
    spin_lock(&mut css.css_lock);
    css.invocations += 1;
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_UNCLEAN == 0 { css.clean += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { css.corrupt += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_PREEN != 0 { css.preen += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_XFAIL != 0 { css.xfail += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_XCORRUPT != 0 { css.xcorrupt += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_INCOMPLETE != 0 { css.incomplete += 1; }
    if (*sm).sm_flags & XFS_SCRUB_OFLAG_WARNING != 0 { css.warning += 1; }
    css.retries += (*run).retries;
    css.checktime_us += howmany_64((*run).scrub_ns, NSEC_PER_USEC);
    if (*run).repair_attempted { css.repair_invocations += 1; }
    if (*run).repair_succeeded { css.repair_success += 1; }
    css.repairtime_us += howmany_64((*run).repair_ns, NSEC_PER_USEC);
    spin_unlock(&mut css.css_lock);
}

/* Merge these scrub-run stats into the global and mount stat data. */
unsafe fn xchk_stats_merge(mp: *mut xfs_mount, sm: *const xfs_scrub_metadata, run: *const xchk_stats_run) {
    xchk_stats_merge_one(&mut global_stats, sm, run);
    xchk_stats_merge_one((*mp).m_scrub_stats, sm, run);
}

/* The remaining debugfs and allocation interfaces are direct declarations of
 * the corresponding C implementation, preserving their external ABI. */
extern "C" {
    fn xchk_stats_register(cs: *mut xchk_stats, parent: *mut dentry);
    fn xchk_stats_unregister(cs: *mut xchk_stats);
    fn xchk_global_stats_setup(parent: *mut dentry) -> i32;
    fn xchk_global_stats_teardown();
    fn xchk_mount_stats_alloc(mp: *mut xfs_mount) -> i32;
    fn xchk_mount_stats_free(mp: *mut xfs_mount);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
