// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of linux/fs/fat/misc.c. External kernel symbols are
 * supplied by the surrounding filesystem implementation. */

const SECS_PER_MIN: i64 = 60;
const SECS_PER_HOUR: i64 = 60 * 60;
const SECS_PER_DAY: i64 = SECS_PER_HOUR * 24;
const DAYS_DELTA: i64 = 365 * 10 + 2;
const YEAR_2100: i64 = 120;

static mut DAYS_IN_YEAR: [i64; 16] =
    [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 0, 0, 0];

#[inline]
unsafe fn fat_tz_offset(sbi: *const msdos_sb_info) -> i64 {
    if (*sbi).options.tz_set {
        -(*sbi).options.time_offset as i64
    } else {
        sys_tz.tz_minuteswest as i64
    } * SECS_PER_MIN
}

pub unsafe fn __fat_fs_error(sb: *mut super_block, report: i32, fmt: *const i8, mut args: ...) {
    let opts = &mut (*MSDOS_SB(sb)).options;
    if report != 0 {
        let mut vaf = va_format { fmt, va: &mut args };
        fat_msg(sb, KERN_ERR, c"error, %pV", &mut vaf);
    }
    if opts.errors == FAT_ERRORS_PANIC {
        panic(c"FAT-fs (%s): fs panic from previous error\n", (*sb).s_id);
    } else if opts.errors == FAT_ERRORS_RO && sb_rdonly(sb) == 0 {
        (*sb).s_flags |= SB_RDONLY;
        fat_msg(sb, KERN_ERR, c"Filesystem has been set read-only");
    }
}

pub unsafe fn _fat_msg(sb: *mut super_block, level: *const i8, fmt: *const i8, mut args: ...) {
    let mut vaf = va_format { fmt, va: &mut args };
    _printk(FAT_PRINTK_PREFIX c"%pV\n", level, (*sb).s_id, &mut vaf);
}

pub unsafe fn fat_clusters_flush(sb: *mut super_block) -> i32 {
    let sbi = MSDOS_SB(sb);
    if !is_fat32(sbi) { return 0; }
    let bh = sb_bread(sb, (*sbi).fsinfo_sector);
    if bh.is_null() {
        fat_msg(sb, KERN_ERR, c"bread failed in fat_clusters_flush");
        return -EIO;
    }
    let fsinfo = (*bh).b_data as *mut fat_boot_fsinfo;
    if !IS_FSINFO(fsinfo) {
        fat_msg(sb, KERN_ERR, c"Invalid FSINFO signature: 0x%08x, 0x%08x (sector = %lu)",
            le32_to_cpu((*fsinfo).signature1), le32_to_cpu((*fsinfo).signature2), (*sbi).fsinfo_sector);
    } else {
        if (*sbi).free_clusters != -1 { (*fsinfo).free_clusters = cpu_to_le32((*sbi).free_clusters); }
        if (*sbi).prev_free != -1 { (*fsinfo).next_cluster = cpu_to_le32((*sbi).prev_free); }
        mark_buffer_dirty(bh);
    }
    brelse(bh);
    0
}

pub unsafe fn fat_chain_add(inode: *mut inode, new_dclus: i32, nr_cluster: i32) -> i32 {
    let sb = (*inode).i_sb;
    let sbi = MSDOS_SB(sb);
    let (mut ret, mut new_fclus, mut last) = (0, 0, 0);
    if (*MSDOS_I(inode)).i_start != 0 {
        let (mut fclus, mut dclus) = (0, 0);
        ret = fat_get_cluster(inode, FAT_ENT_EOF, &mut fclus, &mut dclus);
        if ret < 0 { return ret; }
        new_fclus = fclus + 1; last = dclus;
    }
    if last != 0 {
        let mut fatent = core::mem::zeroed::<fat_entry>();
        fatent_init(&mut fatent);
        ret = fat_ent_read(inode, &mut fatent, last);
        if ret >= 0 {
            let wait = inode_needs_sync(inode);
            let old = ret;
            ret = fat_ent_write(inode, &mut fatent, new_dclus, wait);
            if ret < 0 { fat_ent_write(inode, &mut fatent, old, wait); }
            fatent_brelse(&mut fatent);
        }
        if ret < 0 { return ret; }
    } else {
        (*MSDOS_I(inode)).i_start = new_dclus;
        (*MSDOS_I(inode)).i_logstart = new_dclus;
        mark_inode_dirty(inode);
        if S_ISDIR((*inode).i_mode) && IS_DIRSYNC(inode) {
            ret = sync_inode_metadata(inode, 1);
            if ret != 0 { return ret; }
        }
    }
    if new_fclus != ((*inode).i_blocks >> ((*sbi).cluster_bits - 9)) {
        fat_fs_error_ratelimit(sb, c"clusters badly computed (%d != %llu)", new_fclus,
            (*inode).i_blocks >> ((*sbi).cluster_bits - 9));
        fat_cache_inval_inode(inode);
    }
    (*inode).i_blocks += nr_cluster << ((*sbi).cluster_bits - 9);
    0
}

pub unsafe fn fat_time_fat2unix(sbi: *mut msdos_sb_info, ts: *mut timespec64,
    __time: __le16, __date: __le16, time_cs: u8) {
    let time = le16_to_cpu(__time) as i64;
    let date = le16_to_cpu(__date) as i64;
    let year = date >> 9;
    let month = core::cmp::max(1, (date >> 5) & 0xf);
    let day = core::cmp::max(1, date & 0x1f) - 1;
    let mut leap_day = (year + 3) / 4;
    if year > YEAR_2100 { leap_day -= 1; }
    if (year & 3) == 0 && year != YEAR_2100 && month > 2 { leap_day += 1; }
    let mut second = (time & 0x1f) * 2 + ((time >> 5) & 0x3f) * SECS_PER_MIN +
        (time >> 11) * SECS_PER_HOUR + (year * 365 + leap_day + DAYS_IN_YEAR[month as usize] + day + DAYS_DELTA) * SECS_PER_DAY;
    second += fat_tz_offset(sbi);
    (*ts).tv_sec = second + (time_cs / 100) as i64;
    (*ts).tv_nsec = if time_cs != 0 { ((time_cs % 100) as i64) * 10_000_000 } else { 0 };
}

pub unsafe fn fat_time_unix2fat(sbi: *mut msdos_sb_info, ts: *mut timespec64,
    time: *mut __le16, date: *mut __le16, time_cs: *mut u8) {
    let mut tm = core::mem::zeroed::<tm>();
    time64_to_tm((*ts).tv_sec, -fat_tz_offset(sbi), &mut tm);
    if tm.tm_year < 80 { *time = 0; *date = cpu_to_le16((1 << 5) | 1); if !time_cs.is_null() { *time_cs = 0; } return; }
    if tm.tm_year > 207 { *time = cpu_to_le16((23 << 11) | (59 << 5) | 29); *date = cpu_to_le16((127 << 9) | (12 << 5) | 31); if !time_cs.is_null() { *time_cs = 199; } return; }
    tm.tm_year -= 80; tm.tm_mon += 1; tm.tm_sec >>= 1;
    *time = cpu_to_le16((tm.tm_hour << 11) | (tm.tm_min << 5) | tm.tm_sec);
    *date = cpu_to_le16((tm.tm_year << 9) | (tm.tm_mon << 5) | tm.tm_mday);
    if !time_cs.is_null() { *time_cs = (((*ts).tv_sec & 1) * 100 + (*ts).tv_nsec / 10_000_000) as u8; }
}

#[inline] unsafe fn fat_timespec64_trunc_2secs(ts: timespec64) -> timespec64 {
    timespec64 { tv_sec: ts.tv_sec & !1, tv_nsec: 0 }
}

pub unsafe fn fat_truncate_atime(sbi: *const msdos_sb_info, ts: *const timespec64) -> timespec64 {
    let mut seconds = (*ts).tv_sec - fat_tz_offset(sbi);
    let remainder = seconds.rem_euclid(SECS_PER_DAY);
    seconds = seconds + fat_tz_offset(sbi) - remainder;
    timespec64 { tv_sec: seconds, tv_nsec: 0 }
}

pub unsafe fn fat_truncate_time(inode: *mut inode, now: *mut timespec64, flags: u32) {
    let sbi = MSDOS_SB((*inode).i_sb);
    if (*inode).i_ino == MSDOS_ROOT_INO { return; }
    let mut ts;
    if now.is_null() { ts = current_time(inode); now = &mut ts; }
    if flags & FAT_UPDATE_ATIME != 0 { inode_set_atime_to_ts(inode, fat_truncate_atime(sbi, now)); }
    if flags & FAT_UPDATE_CMTIME != 0 { let mtime = fat_timespec64_trunc_2secs(*now); inode_set_mtime_to_ts(inode, mtime); inode_set_ctime_to_ts(inode, mtime); }
}

pub unsafe fn fat_update_time(inode: *mut inode, ty: fs_update_time, _flags: u32) -> i32 {
    if (*inode).i_ino != MSDOS_ROOT_INO { fat_truncate_time(inode, core::ptr::null_mut(), if ty == FS_UPD_ATIME { FAT_UPDATE_ATIME } else { FAT_UPDATE_CMTIME }); __mark_inode_dirty(inode, inode_time_dirty_flag(inode)); }
    0
}

pub unsafe fn fat_sync_bhs(bhs: *mut *mut buffer_head, nr_bhs: i32) -> i32 {
    let mut err = 0;
    for i in 0..nr_bhs { write_dirty_buffer(*bhs.add(i as usize), 0); }
    for i in 0..nr_bhs { let bh = *bhs.add(i as usize); wait_on_buffer(bh); if err == 0 && buffer_uptodate(bh) == 0 { err = -EIO; } }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
