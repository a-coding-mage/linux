// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2011 Red Hat, Inc.  All Rights Reserved.
 */

// XFS logging functions

unsafe fn __xfs_printk(
    level: *const core::ffi::c_char,
    mp: *const xfs_mount,
    vaf: *mut va_format,
) {
    if !mp.is_null() && !(*mp).m_super.is_null() {
        printk(
            b"%sXFS (%s): %pV\n\0".as_ptr() as *const core::ffi::c_char,
            level,
            (*(*mp).m_super).s_id,
            vaf,
        );
        return;
    }
    printk(
        b"%sXFS: %pV\n\0".as_ptr() as *const core::ffi::c_char,
        level,
        vaf,
    );
}

pub unsafe extern "C" fn xfs_printk_level(
    kern_level: *const core::ffi::c_char,
    mp: *const xfs_mount,
    fmt: *const core::ffi::c_char,
    ...,
) {
    let mut vaf: va_format = core::mem::zeroed();
    let mut args: va_list = core::mem::zeroed();
    let mut level: core::ffi::c_int = 0;

    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;

    __xfs_printk(kern_level, mp, &mut vaf);

    va_end(&mut args);

    if kstrtoint(kern_level, 0, &mut level) == 0
        && level <= LOGLEVEL_ERR
        && xfs_error_level >= XFS_ERRLEVEL_HIGH
    {
        xfs_stack_trace();
    }
}

pub unsafe extern "C" fn _xfs_alert_tag(
    mp: *const xfs_mount,
    panic_tag: u32,
    fmt: *const core::ffi::c_char,
    ...,
) {
    let mut vaf: va_format = core::mem::zeroed();
    let mut args: va_list = core::mem::zeroed();
    let mut do_panic: core::ffi::c_int = 0;

    if xfs_panic_mask != 0 && (xfs_panic_mask & panic_tag) != 0 {
        xfs_alert(mp, b"Transforming an alert into a BUG.\0".as_ptr() as *const core::ffi::c_char);
        do_panic = 1;
    }

    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;

    __xfs_printk(KERN_ALERT, mp, &mut vaf);
    va_end(&mut args);

    BUG_ON(do_panic);
}

pub unsafe extern "C" fn asswarn(
    mp: *mut xfs_mount,
    expr: *mut core::ffi::c_char,
    file: *mut core::ffi::c_char,
    line: core::ffi::c_int,
) {
    xfs_warn(mp, b"Assertion failed: %s, file: %s, line: %d\0".as_ptr() as *const core::ffi::c_char, expr, file, line);
    WARN_ON(1);
}

pub unsafe extern "C" fn assfail(
    mp: *mut xfs_mount,
    expr: *mut core::ffi::c_char,
    file: *mut core::ffi::c_char,
    line: core::ffi::c_int,
) {
    xfs_emerg(mp, b"Assertion failed: %s, file: %s, line: %d\0".as_ptr() as *const core::ffi::c_char, expr, file, line);
    if xfs_globals.bug_on_assert {
        BUG();
    } else {
        WARN_ON(1);
    }
}

pub unsafe extern "C" fn xfs_hex_dump(p: *const core::ffi::c_void, length: core::ffi::c_int) {
    print_hex_dump(KERN_ALERT, b"\0".as_ptr() as *const core::ffi::c_char, DUMP_PREFIX_OFFSET, 16, 1, p, length, 1);
}

pub unsafe extern "C" fn xfs_buf_alert_ratelimited(
    bp: *mut xfs_buf,
    rlmsg: *const core::ffi::c_char,
    fmt: *const core::ffi::c_char,
    ...,
) {
    let mp = (*bp).b_mount;
    let mut vaf: va_format = core::mem::zeroed();
    let mut args: va_list = core::mem::zeroed();

    // use the more aggressive per-target rate limit for buffers
    if !___ratelimit(&mut (*(*bp).b_target).bt_ioerror_rl, rlmsg) {
        return;
    }

    va_start(&mut args, fmt);
    vaf.fmt = fmt;
    vaf.va = &mut args;
    __xfs_printk(KERN_ALERT, mp, &mut vaf);
    va_end(&mut args);
}

pub unsafe extern "C" fn xfs_warn_experimental(mp: *mut xfs_mount, feat: xfs_experimental_feat) {
    let features = [
        xfs_experimental_feature { opstate: XFS_OPSTATE_WARNED_SHRINK, name: b"online shrink\0".as_ptr() as *const core::ffi::c_char },
        xfs_experimental_feature { opstate: XFS_OPSTATE_WARNED_LARP, name: b"logged extended attributes\0".as_ptr() as *const core::ffi::c_char },
    ];
    ASSERT(feat >= 0 && feat < XFS_EXPERIMENTAL_MAX);
    BUILD_BUG_ON(features.len() != XFS_EXPERIMENTAL_MAX as usize);

    if xfs_should_warn(mp, features[feat as usize].opstate) {
        xfs_warn(mp, b"EXPERIMENTAL %s feature enabled.  Use at your own risk!\0".as_ptr() as *const core::ffi::c_char, features[feat as usize].name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
