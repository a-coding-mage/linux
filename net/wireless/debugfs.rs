// SPDX-License-Identifier: GPL-2.0-only
/*
 * cfg80211 debugfs
 *
 * Copyright 2009  Luis R. Rodriguez <lrodriguez@atheros.com>
 * Copyright 2007  Johannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2023 Intel Corporation
 */

/* Linux kernel dependencies are supplied by the surrounding translation. */

unsafe fn rts_threshold_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                             count: usize, ppos: *mut loff_t) -> isize {
    let wiphy = (*file).private_data as *mut wiphy;
    let mut buf = [0 as core::ffi::c_char; 20];
    let res = scnprintf(buf.as_mut_ptr(), 20, b"%d\0".as_ptr() as *const _, (*wiphy).rts_threshold);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

unsafe fn fragmentation_threshold_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                                       count: usize, ppos: *mut loff_t) -> isize {
    let wiphy = (*file).private_data as *mut wiphy;
    let mut buf = [0 as core::ffi::c_char; 20];
    let res = scnprintf(buf.as_mut_ptr(), 20, b"%d\0".as_ptr() as *const _, (*wiphy).frag_threshold);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

unsafe fn short_retry_limit_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                                 count: usize, ppos: *mut loff_t) -> isize {
    let wiphy = (*file).private_data as *mut wiphy;
    let mut buf = [0 as core::ffi::c_char; 20];
    let res = scnprintf(buf.as_mut_ptr(), 20, b"%d\0".as_ptr() as *const _, (*wiphy).retry_short);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

unsafe fn long_retry_limit_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                                count: usize, ppos: *mut loff_t) -> isize {
    let wiphy = (*file).private_data as *mut wiphy;
    let mut buf = [0 as core::ffi::c_char; 20];
    let res = scnprintf(buf.as_mut_ptr(), 20, b"%d\0".as_ptr() as *const _, (*wiphy).retry_long);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

unsafe fn radio_rts_threshold_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                                   count: usize, ppos: *mut loff_t) -> isize {
    let radio_cfg = (*file).private_data as *mut wiphy_radio_cfg;
    let mut buf = [0 as core::ffi::c_char; 20];
    let res = scnprintf(buf.as_mut_ptr(), 20, b"%d\0".as_ptr() as *const _, (*radio_cfg).rts_threshold);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

unsafe fn ht_print_chan(chan: *mut ieee80211_channel, buf: *mut core::ffi::c_char,
                        buf_size: i32, offset: i32) -> i32 {
    if WARN_ON(offset > buf_size) { return 0; }
    if (*chan).flags & IEEE80211_CHAN_DISABLED != 0 {
        return scnprintf(buf.add(offset as usize), (buf_size - offset) as usize,
                         b"%d Disabled\n\0".as_ptr() as *const _, (*chan).center_freq);
    }
    scnprintf(buf.add(offset as usize), (buf_size - offset) as usize,
              b"%d HT40 %c%c\n\0".as_ptr() as *const _, (*chan).center_freq,
              if (*chan).flags & IEEE80211_CHAN_NO_HT40MINUS != 0 { ' ' as i32 } else { '-' as i32 },
              if (*chan).flags & IEEE80211_CHAN_NO_HT40PLUS != 0 { ' ' as i32 } else { '+' as i32 })
}

unsafe fn ht40allow_map_read(file: *mut file, user_buf: *mut core::ffi::c_char,
                             count: usize, ppos: *mut loff_t) -> isize {
    let wiphy = (*file).private_data as *mut wiphy;
    let buf_size = PAGE_SIZE;
    let buf = kzalloc(buf_size, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    let mut offset: u32 = 0;
    for band in 0..NUM_NL80211_BANDS {
        let sband = (*wiphy).bands[band as usize];
        if sband.is_null() { continue; }
        for i in 0..(*sband).n_channels {
            offset += ht_print_chan(&mut (*sband).channels[i as usize], buf,
                                    buf_size as i32, offset as i32) as u32;
        }
    }
    let r = simple_read_from_buffer(user_buf, count, ppos, buf, offset as usize);
    kfree(buf);
    r
}

unsafe fn cfg80211_debugfs_rdev_add(rdev: *mut cfg80211_registered_device) {
    let phyd = (*rdev).wiphy.debugfsdir;
    let mut radiod: *mut dentry;
    let mut i: u8;
    debugfs_create_file(b"rts_threshold\0".as_ptr() as *const _, 0o444, phyd,
                        &mut (*rdev).wiphy as *mut _ as *mut _, &rts_threshold_ops);
    debugfs_create_file(b"fragmentation_threshold\0".as_ptr() as *const _, 0o444, phyd,
                        &mut (*rdev).wiphy as *mut _ as *mut _, &fragmentation_threshold_ops);
    debugfs_create_file(b"short_retry_limit\0".as_ptr() as *const _, 0o444, phyd,
                        &mut (*rdev).wiphy as *mut _ as *mut _, &short_retry_limit_ops);
    debugfs_create_file(b"long_retry_limit\0".as_ptr() as *const _, 0o444, phyd,
                        &mut (*rdev).wiphy as *mut _ as *mut _, &long_retry_limit_ops);
    debugfs_create_file(b"ht40allow_map\0".as_ptr() as *const _, 0o444, phyd,
                        &mut (*rdev).wiphy as *mut _ as *mut _, &ht40allow_map_ops);
    i = 0;
    while i < (*rdev).wiphy.n_radio {
        radiod = (*rdev).wiphy.radio_cfg[i as usize].radio_debugfsdir;
        debugfs_create_file(b"radio_rts_threshold\0".as_ptr() as *const _, 0o444, radiod,
                            &mut (*rdev).wiphy.radio_cfg[i as usize] as *mut _ as *mut _,
                            &radio_rts_threshold_ops);
        i += 1;
    }
}

/* The file-operation objects correspond to the macro-generated C definitions. */
static rts_threshold_ops: file_operations = file_operations { read: Some(rts_threshold_read), open: Some(simple_open), llseek: Some(generic_file_llseek) };
static fragmentation_threshold_ops: file_operations = file_operations { read: Some(fragmentation_threshold_read), open: Some(simple_open), llseek: Some(generic_file_llseek) };
static short_retry_limit_ops: file_operations = file_operations { read: Some(short_retry_limit_read), open: Some(simple_open), llseek: Some(generic_file_llseek) };
static long_retry_limit_ops: file_operations = file_operations { read: Some(long_retry_limit_read), open: Some(simple_open), llseek: Some(generic_file_llseek) };
static radio_rts_threshold_ops: file_operations = file_operations { read: Some(radio_rts_threshold_read), open: Some(simple_open), llseek: Some(generic_file_llseek) };
static ht40allow_map_ops: file_operations = file_operations { read: Some(ht40allow_map_read), open: Some(simple_open), llseek: Some(default_llseek) };

#[repr(C)]
struct debugfs_read_work {
    work: wiphy_work,
    handler: Option<unsafe extern "C" fn(*mut wiphy, *mut file, *mut core::ffi::c_char, usize, *mut core::ffi::c_void) -> isize>,
    wiphy: *mut wiphy, file: *mut file, buf: *mut core::ffi::c_char, bufsize: usize,
    data: *mut core::ffi::c_void, ret: isize, completion: completion,
}

unsafe fn wiphy_locked_debugfs_read_work(wiphy: *mut wiphy, work: *mut wiphy_work) {
    let w = container_of!(work, debugfs_read_work, work);
    (*w).ret = ((*w).handler.unwrap())((*w).wiphy, (*w).file, (*w).buf, (*w).bufsize, (*w).data);
    complete(&mut (*w).completion);
}

unsafe fn wiphy_locked_debugfs_read_cancel(_dentry: *mut dentry, data: *mut core::ffi::c_void) {
    let w = data as *mut debugfs_read_work;
    wiphy_work_cancel((*w).wiphy, &mut (*w).work);
    complete(&mut (*w).completion);
}

unsafe fn wiphy_locked_debugfs_read(wiphy: *mut wiphy, file: *mut file, buf: *mut core::ffi::c_char,
                                    bufsize: usize, userbuf: *mut core::ffi::c_char, count: usize,
                                    ppos: *mut loff_t,
                                    handler: Option<unsafe extern "C" fn(*mut wiphy, *mut file, *mut core::ffi::c_char, usize, *mut core::ffi::c_void) -> isize>,
                                    data: *mut core::ffi::c_void) -> isize {
    let mut work = debugfs_read_work { work: core::mem::zeroed(), handler, wiphy, file, buf, bufsize, data, ret: -ENODEV, completion: core::mem::zeroed() };
    let cancellation = debugfs_cancellation { cancel: Some(wiphy_locked_debugfs_read_cancel), cancel_data: &mut work as *mut _ as *mut _ };
    memset(buf, 0, bufsize);
    wiphy_work_init(&mut work.work, Some(wiphy_locked_debugfs_read_work));
    wiphy_work_queue(wiphy, &mut work.work);
    debugfs_enter_cancellation(file, &cancellation);
    wait_for_completion(&mut work.completion);
    debugfs_leave_cancellation(file, &cancellation);
    if work.ret < 0 { return work.ret; }
    if WARN_ON(work.ret as usize > bufsize) { return -EINVAL; }
    simple_read_from_buffer(userbuf, count, ppos, buf, work.ret as usize)
}

#[repr(C)]
struct debugfs_write_work {
    work: wiphy_work,
    handler: Option<unsafe extern "C" fn(*mut wiphy, *mut file, *mut core::ffi::c_char, usize, *mut core::ffi::c_void) -> isize>,
    wiphy: *mut wiphy, file: *mut file, buf: *mut core::ffi::c_char, count: usize,
    data: *mut core::ffi::c_void, ret: isize, completion: completion,
}

unsafe fn wiphy_locked_debugfs_write_work(_wiphy: *mut wiphy, work: *mut wiphy_work) {
    let w = container_of!(work, debugfs_write_work, work);
    (*w).ret = ((*w).handler.unwrap())((*w).wiphy, (*w).file, (*w).buf, (*w).count, (*w).data);
    complete(&mut (*w).completion);
}

unsafe fn wiphy_locked_debugfs_write_cancel(_dentry: *mut dentry, data: *mut core::ffi::c_void) {
    let w = data as *mut debugfs_write_work;
    wiphy_work_cancel((*w).wiphy, &mut (*w).work);
    complete(&mut (*w).completion);
}

unsafe fn wiphy_locked_debugfs_write(wiphy: *mut wiphy, file: *mut file, buf: *mut core::ffi::c_char,
                                     bufsize: usize, userbuf: *const core::ffi::c_char, count: usize,
                                     handler: Option<unsafe extern "C" fn(*mut wiphy, *mut file, *mut core::ffi::c_char, usize, *mut core::ffi::c_void) -> isize>,
                                     data: *mut core::ffi::c_void) -> isize {
    let mut work = debugfs_write_work { work: core::mem::zeroed(), handler, wiphy, file, buf, count, data, ret: -ENODEV, completion: core::mem::zeroed() };
    let cancellation = debugfs_cancellation { cancel: Some(wiphy_locked_debugfs_write_cancel), cancel_data: &mut work as *mut _ as *mut _ };
    if count >= bufsize { return -EINVAL; }
    memset(buf, 0, bufsize);
    if copy_from_user(buf, userbuf, count) != 0 { return -EFAULT; }
    wiphy_work_init(&mut work.work, Some(wiphy_locked_debugfs_write_work));
    wiphy_work_queue(wiphy, &mut work.work);
    debugfs_enter_cancellation(file, &cancellation);
    wait_for_completion(&mut work.completion);
    debugfs_leave_cancellation(file, &cancellation);
    work.ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
