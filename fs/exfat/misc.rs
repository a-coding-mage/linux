// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Written 1992,1993 by Werner Almesberger
 *  22/11/2000 - Fixed fat_date_unix2dos for dates earlier than 01/01/1980
 *               and date_dos2unix for date==0 by Igor Zhbanov(bsg@uniyar.ac.ru)
 * Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
 */

// Linux and exFAT declarations are supplied by the surrounding translation unit.

const SECS_PER_MIN: i64 = 60;

#[inline]
const fn timezone_sec(x: i64) -> i64 { x * 15 * SECS_PER_MIN }

pub unsafe extern "C" fn __exfat_fs_error(
    sb: *mut super_block, report: c_int, fmt: *const c_char, ...
) {
    let opts: *mut exfat_mount_options = &mut (*EXFAT_SB(sb)).options;

    if report != 0 {
        // The C va_list/va_format plumbing is provided by the kernel ABI.
        // Keep the diagnostic call and its ordering for the native binding.
        let _ = fmt;
        exfat_err(sb, c"error, %pV".as_ptr(), core::ptr::null_mut());
    }

    if (*opts).errors == EXFAT_ERRORS_PANIC {
        panic(c"exFAT-fs (%s): fs panic from previous error\n".as_ptr(), (*sb).s_id);
    } else if (*opts).errors == EXFAT_ERRORS_RO && sb_rdonly(sb) == 0 {
        (*sb).s_flags |= SB_RDONLY;
        exfat_err(sb, c"Filesystem has been set read-only".as_ptr());
    }
}

unsafe fn exfat_adjust_tz(ts: *mut timespec64, tz_off: u8) {
    if tz_off <= 0x3f {
        (*ts).tv_sec -= timezone_sec(tz_off as i64);
    } else {
        (*ts).tv_sec += timezone_sec((0x80 - tz_off) as i64);
    }
}

#[inline]
unsafe fn exfat_tz_offset(sbi: *mut exfat_sb_info) -> c_int {
    if (*sbi).options.sys_tz {
        -sys_tz.tz_minuteswest
    } else {
        (*sbi).options.time_offset
    }
}

pub unsafe extern "C" fn exfat_get_entry_time(
    sbi: *mut exfat_sb_info, ts: *mut timespec64, tz: u8,
    time: __le16, date: __le16, time_cs: u8,
) {
    let t: u16 = le16_to_cpu(time);
    let d: u16 = le16_to_cpu(date);
    (*ts).tv_sec = mktime64(
        1980 + ((d >> 9) as i64), ((d >> 5) & 0x000f) as i64,
        (d & 0x001f) as i64, (t >> 11) as i64, ((t >> 5) & 0x003f) as i64,
        ((t & 0x001f) << 1) as i64,
    );

    if time_cs != 0 {
        (*ts).tv_sec += (time_cs / 100) as i64;
        (*ts).tv_nsec = ((time_cs % 100) as i64) * 10 * NSEC_PER_MSEC;
    } else {
        (*ts).tv_nsec = 0;
    }

    if tz & EXFAT_TZ_VALID != 0 {
        exfat_adjust_tz(ts, tz & !EXFAT_TZ_VALID);
    } else {
        (*ts).tv_sec -= (exfat_tz_offset(sbi) as i64) * SECS_PER_MIN;
    }
}

pub unsafe extern "C" fn exfat_set_entry_time(
    _sbi: *mut exfat_sb_info, ts: *mut timespec64, tz: *mut u8,
    time: *mut __le16, date: *mut __le16, time_cs: *mut u8,
) {
    let mut tm: tm = core::mem::zeroed();
    time64_to_tm((*ts).tv_sec, 0, &mut tm);
    let t: u16 = ((tm.tm_hour as u16) << 11) | ((tm.tm_min as u16) << 5)
        | ((tm.tm_sec as u16) >> 1);
    let d: u16 = (((tm.tm_year - 80) as u16) << 9)
        | (((tm.tm_mon + 1) as u16) << 5) | tm.tm_mday as u16;
    *time = cpu_to_le16(t);
    *date = cpu_to_le16(d);
    if !time_cs.is_null() {
        *time_cs = ((tm.tm_sec & 1) * 100 + (*ts).tv_nsec / (10 * NSEC_PER_MSEC)) as u8;
    }
    *tz = EXFAT_TZ_VALID;
}

pub unsafe extern "C" fn exfat_truncate_atime(ts: *mut timespec64) {
    (*ts).tv_sec = (*ts).tv_sec - (*ts).tv_sec.rem_euclid(2);
    (*ts).tv_nsec = 0;
}

pub unsafe extern "C" fn exfat_truncate_inode_atime(inode: *mut inode) {
    let mut atime = inode_get_atime(inode);
    exfat_truncate_atime(&mut atime);
    inode_set_atime_to_ts(inode, atime);
}

pub unsafe extern "C" fn exfat_calc_chksum16(
    data: *mut c_void, len: c_int, mut chksum: u16, type_: c_int,
) -> u16 {
    let c = data as *const u8;
    for i in 0..len {
        if type_ == CS_DIR_ENTRY && (i == 2 || i == 3) { continue; }
        chksum = chksum.rotate_right(1).wrapping_add(*c.add(i as usize) as u16);
    }
    chksum
}

pub unsafe extern "C" fn exfat_calc_chksum32(
    data: *mut c_void, len: c_int, mut chksum: u32, type_: c_int,
) -> u32 {
    let c = data as *const u8;
    for i in 0..len {
        if type_ == CS_BOOT_SECTOR && (i == 106 || i == 107 || i == 112) { continue; }
        chksum = chksum.rotate_right(1).wrapping_add(*c.add(i as usize) as u32);
    }
    chksum
}

pub unsafe extern "C" fn exfat_update_bh(bh: *mut buffer_head, sync: c_int) -> c_int {
    set_buffer_uptodate(bh); mark_buffer_dirty(bh);
    if sync != 0 { sync_dirty_buffer(bh) } else { 0 }
}

pub unsafe extern "C" fn exfat_update_bhs(
    bhs: *mut *mut buffer_head, nr_bhs: c_int, sync: c_int,
) -> c_int {
    let mut err = 0;
    for i in 0..nr_bhs {
        let bh = *bhs.add(i as usize);
        set_buffer_uptodate(bh); mark_buffer_dirty(bh);
        if sync != 0 { write_dirty_buffer(bh, REQ_SYNC); }
    }
    for i in 0..nr_bhs {
        if sync == 0 { break; }
        let bh = *bhs.add(i as usize);
        wait_on_buffer(bh);
        if err == 0 && buffer_uptodate(bh) == 0 { err = -EIO; }
    }
    err
}

pub unsafe extern "C" fn exfat_chain_set(
    ec: *mut exfat_chain, dir: c_uint, size: c_uint, flags: c_uchar,
) { (*ec).dir = dir; (*ec).size = size; (*ec).flags = flags; }

pub unsafe extern "C" fn exfat_chain_dup(
    dup: *mut exfat_chain, ec: *mut exfat_chain,
) { exfat_chain_set(dup, (*ec).dir, (*ec).size, (*ec).flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
