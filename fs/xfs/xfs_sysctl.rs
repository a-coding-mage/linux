// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2001-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Dependencies supplied by the surrounding kernel/XFS sources are intentionally
// left as external Rust declarations.

static mut xfs_table_header: *mut ctl_table_header = core::ptr::null_mut();

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn xfs_stats_clear_proc_handler(
    ctl: *const ctl_table,
    write: libc::c_int,
    buffer: *mut libc::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> libc::c_int {
    let valp = (*ctl).data as *mut libc::c_int;
    let ret = proc_dointvec_minmax(ctl, write, buffer, lenp, ppos);

    if ret == 0 && write != 0 && *valp != 0 {
        xfs_stats_clearall(xfsstats.xs_stats);
        xfs_stats_clear = 0;
    }

    ret
}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn xfs_panic_mask_proc_handler(
    ctl: *const ctl_table,
    write: libc::c_int,
    buffer: *mut libc::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> libc::c_int {
    let valp = (*ctl).data as *mut libc::c_int;
    let ret = proc_dointvec_minmax(ctl, write, buffer, lenp, ppos);
    if ret == 0 && write != 0 {
        xfs_panic_mask = *valp;
        // Preserved conditional intent: this block is compiled in DEBUG builds.
        #[cfg(DEBUG)]
        {
            xfs_panic_mask |= XFS_PTAG_SHUTDOWN_CORRUPT | XFS_PTAG_LOGRES;
        }
    }
    ret
}

unsafe extern "C" fn xfs_deprecated_dointvec_minmax(
    ctl: *const ctl_table,
    write: libc::c_int,
    buffer: *mut libc::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> libc::c_int {
    if write != 0 {
        printk_ratelimited(KERN_WARNING, c"XFS: %s sysctl option is deprecated.\n", (*ctl).procname);
    }
    proc_dointvec_minmax(ctl, write, buffer, lenp, ppos)
}

static xfs_table: [ctl_table; 11] = [
    ctl_table { procname: c"panic_mask", data: &raw mut xfs_params.panic_mask.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(xfs_panic_mask_proc_handler), extra1: &raw mut xfs_params.panic_mask.min as *mut _, extra2: &raw mut xfs_params.panic_mask.max as *mut _ },
    ctl_table { procname: c"error_level", data: &raw mut xfs_params.error_level.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.error_level.min as *mut _, extra2: &raw mut xfs_params.error_level.max as *mut _ },
    ctl_table { procname: c"xfssyncd_centisecs", data: &raw mut xfs_params.syncd_timer.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.syncd_timer.min as *mut _, extra2: &raw mut xfs_params.syncd_timer.max as *mut _ },
    ctl_table { procname: c"inherit_sync", data: &raw mut xfs_params.inherit_sync.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.inherit_sync.min as *mut _, extra2: &raw mut xfs_params.inherit_sync.max as *mut _ },
    ctl_table { procname: c"inherit_nodump", data: &raw mut xfs_params.inherit_nodump.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.inherit_nodump.min as *mut _, extra2: &raw mut xfs_params.inherit_nodump.max as *mut _ },
    ctl_table { procname: c"inherit_noatime", data: &raw mut xfs_params.inherit_noatim.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.inherit_noatim.min as *mut _, extra2: &raw mut xfs_params.inherit_noatim.max as *mut _ },
    ctl_table { procname: c"inherit_nosymlinks", data: &raw mut xfs_params.inherit_nosym.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.inherit_nosym.min as *mut _, extra2: &raw mut xfs_params.inherit_nosym.max as *mut _ },
    ctl_table { procname: c"rotorstep", data: &raw mut xfs_params.rotorstep.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.rotorstep.min as *mut _, extra2: &raw mut xfs_params.rotorstep.max as *mut _ },
    ctl_table { procname: c"inherit_nodefrag", data: &raw mut xfs_params.inherit_nodfrg.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.inherit_nodfrg.min as *mut _, extra2: &raw mut xfs_params.inherit_nodfrg.max as *mut _ },
    ctl_table { procname: c"filestream_centisecs", data: &raw mut xfs_params.fstrm_timer.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.fstrm_timer.min as *mut _, extra2: &raw mut xfs_params.fstrm_timer.max as *mut _ },
    ctl_table { procname: c"speculative_prealloc_lifetime", data: &raw mut xfs_params.blockgc_timer.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: &raw mut xfs_params.blockgc_timer.min as *mut _, extra2: &raw mut xfs_params.blockgc_timer.max as *mut _ },
    // please keep this the last entry
    #[cfg(CONFIG_PROC_FS)]
    ctl_table { procname: c"stats_clear", data: &raw mut xfs_params.stats_clear.val as *mut _, maxlen: core::mem::size_of::<libc::c_int>(), mode: 0o644, proc_handler: Some(xfs_stats_clear_proc_handler), extra1: &raw mut xfs_params.stats_clear.min as *mut _, extra2: &raw mut xfs_params.stats_clear.max as *mut _ },
];

#[no_mangle]
pub unsafe extern "C" fn xfs_sysctl_register() -> libc::c_int {
    xfs_table_header = register_sysctl(c"fs/xfs", xfs_table.as_ptr());
    if xfs_table_header.is_null() {
        return -ENOMEM;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfs_sysctl_unregister() {
    unregister_sysctl_table(xfs_table_header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
