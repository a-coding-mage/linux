// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/ext4 translation.

/* Checksumming functions */
unsafe fn ext4_mmp_csum(sb: *mut super_block, mmp: *mut mmp_struct) -> __le32 {
    let sbi = EXT4_SB(sb);
    let offset = core::mem::offset_of!(mmp_struct, mmp_checksum);
    let csum: __u32 = ext4_chksum((*sbi).s_csum_seed, mmp as *mut i8, offset);
    cpu_to_le32(csum)
}

unsafe fn ext4_mmp_csum_verify(sb: *mut super_block, mmp: *mut mmp_struct) -> i32 {
    if !ext4_has_feature_metadata_csum(sb) { return 1; }
    if (*mmp).mmp_checksum == ext4_mmp_csum(sb, mmp) { 1 } else { 0 }
}

unsafe fn ext4_mmp_csum_set(sb: *mut super_block, mmp: *mut mmp_struct) {
    if !ext4_has_feature_metadata_csum(sb) { return; }
    (*mmp).mmp_checksum = ext4_mmp_csum(sb, mmp);
}

/* Write the MMP block using REQ_SYNC to try to get the block on-disk faster. */
unsafe fn write_mmp_block_thawed(sb: *mut super_block, bh: *mut buffer_head) -> i32 {
    let mmp = (*bh).b_data as *mut mmp_struct;
    ext4_mmp_csum_set(sb, mmp);
    lock_buffer(bh);
    bh_submit(bh, REQ_OP_WRITE | REQ_SYNC | REQ_META | REQ_PRIO, bh_end_write);
    wait_on_buffer(bh);
    if !buffer_uptodate(bh) { return -EIO; }
    0
}

unsafe fn write_mmp_block(sb: *mut super_block, bh: *mut buffer_head) -> i32 {
    /* Protect against freezing so that dirty buffers are not created on a frozen filesystem. */
    write_mmp_block_thawed(sb, bh)
}

/* Read the MMP block. It must be read from disk, so clear the uptodate flag. */
unsafe fn read_mmp_block(sb: *mut super_block, bh: *mut *mut buffer_head,
                         mmp_block: ext4_fsblk_t) -> i32 {
    let mut ret: i32;
    if !(*bh).is_null() { clear_buffer_uptodate(*bh); }
    if (*bh).is_null() {
        *bh = sb_getblk(sb, mmp_block);
        if (*bh).is_null() { ret = -ENOMEM; return read_mmp_warn_exit(sb, bh, ret, mmp_block); }
    }
    lock_buffer(*bh);
    ret = ext4_read_bh(*bh, REQ_META | REQ_PRIO, core::ptr::null_mut(), false);
    if ret != 0 { return read_mmp_warn_exit(sb, bh, ret, mmp_block); }
    let mmp = (**bh).b_data as *mut mmp_struct;
    if le32_to_cpu((*mmp).mmp_magic) != EXT4_MMP_MAGIC { ret = -EFSCORRUPTED; return read_mmp_warn_exit(sb, bh, ret, mmp_block); }
    if ext4_mmp_csum_verify(sb, mmp) == 0 { ret = -EFSBADCRC; return read_mmp_warn_exit(sb, bh, ret, mmp_block); }
    0
}

unsafe fn read_mmp_warn_exit(sb: *mut super_block, bh: *mut *mut buffer_head,
                             ret: i32, mmp_block: ext4_fsblk_t) -> i32 {
    brelse(*bh); *bh = core::ptr::null_mut();
    ext4_warning(sb, "Error %d while reading MMP block %llu", ret, mmp_block);
    ret
}

/* Dump as much information as possible to help the admin. */
pub unsafe fn __dump_mmp_msg(sb: *mut super_block, mmp: *mut mmp_struct,
                             function: *const i8, line: u32, msg: *const i8) {
    __ext4_warning(sb, function, line, "%s", msg);
    __ext4_warning(sb, function, line,
        "MMP failure info: last update time: %llu, last update node: %.*s, last update device: %.*s",
        le64_to_cpu((*mmp).mmp_time), core::mem::size_of_val(&(*mmp).mmp_nodename), (*mmp).mmp_nodename,
        core::mem::size_of_val(&(*mmp).mmp_bdevname), (*mmp).mmp_bdevname);
}

unsafe fn kmmpd(data: *mut core::ffi::c_void) -> i32 {
    let sb = data as *mut super_block;
    let es = (*EXT4_SB(sb)).s_es;
    let bh = (*EXT4_SB(sb)).s_mmp_bh;
    let mmp = (*bh).b_data as *mut mmp_struct;
    let mmp_block = le64_to_cpu((*es).s_mmp_block);
    let mut seq: u32 = 0;
    let mut failed_writes: c_ulong = 0;
    let mmp_update_interval = le16_to_cpu((*es).s_mmp_update_interval) as i32;
    let mut mmp_check_interval: c_uint;
    let mut last_update_time: c_ulong;
    let mut diff: c_ulong;
    let mut retval = 0;
    (*mmp).mmp_time = cpu_to_le64(ktime_get_real_seconds());
    mmp_check_interval = max(EXT4_MMP_CHECK_MULT * mmp_update_interval as c_uint, EXT4_MMP_MIN_CHECK_INTERVAL);
    (*mmp).mmp_check_interval = cpu_to_le16(mmp_check_interval as u16);
    memcpy((*mmp).mmp_nodename.as_mut_ptr(), init_utsname()->nodename.as_ptr(), core::mem::size_of_val(&(*mmp).mmp_nodename));
    while !kthread_should_stop() && !ext4_emergency_state(sb) {
        if !ext4_has_feature_mmp(sb) { ext4_warning(sb, "kmmpd being stopped since MMP feature has been disabled."); break; }
        seq += 1; if seq > EXT4_MMP_SEQ_MAX { seq = 1; }
        (*mmp).mmp_seq = cpu_to_le32(seq); (*mmp).mmp_time = cpu_to_le64(ktime_get_real_seconds()); last_update_time = jiffies;
        retval = write_mmp_block(sb, bh);
        if retval != 0 { if failed_writes % 60 == 0 { ext4_error_err(sb, -retval, "Error writing to MMP block"); } failed_writes += 1; }
        diff = jiffies - last_update_time;
        if diff < mmp_update_interval as c_ulong * HZ { schedule_timeout_interruptible(mmp_update_interval as c_long * HZ - diff as c_long); }
        diff = jiffies - last_update_time;
        if diff > mmp_check_interval as c_ulong * HZ {
            let mut bh_check: *mut buffer_head = core::ptr::null_mut();
            if read_mmp_block(sb, &mut bh_check, mmp_block) != 0 { ext4_error_err(sb, -retval, "error reading MMP data: %d", retval); break; }
            let mmp_check = (*bh_check).b_data as *mut mmp_struct;
            if (*mmp).mmp_seq != (*mmp_check).mmp_seq || memcmp((*mmp).mmp_nodename.as_ptr(), (*mmp_check).mmp_nodename.as_ptr(), core::mem::size_of_val(&(*mmp).mmp_nodename)) != 0 { dump_mmp_msg(sb, mmp_check, "Error while updating MMP info. The filesystem seems to have been multiply mounted."); ext4_error_err(sb, EBUSY, "abort"); put_bh(bh_check); retval = -EBUSY; break; }
            put_bh(bh_check);
        }
        mmp_check_interval = clamp(EXT4_MMP_CHECK_MULT * diff as c_uint / HZ, EXT4_MMP_MIN_CHECK_INTERVAL, EXT4_MMP_MAX_CHECK_INTERVAL);
        (*mmp).mmp_check_interval = cpu_to_le16(mmp_check_interval as u16);
    }
    (*mmp).mmp_seq = cpu_to_le32(EXT4_MMP_SEQ_CLEAN); (*mmp).mmp_time = cpu_to_le64(ktime_get_real_seconds());
    retval = write_mmp_block(sb, bh);
    while !kthread_should_stop() { set_current_state(TASK_INTERRUPTIBLE); if !kthread_should_stop() { schedule(); } }
    set_current_state(TASK_RUNNING); retval
}

pub unsafe fn ext4_stop_mmpd(sbi: *mut ext4_sb_info) { if !(*sbi).s_mmp_tsk.is_null() { kthread_stop((*sbi).s_mmp_tsk); brelse((*sbi).s_mmp_bh); (*sbi).s_mmp_tsk = core::ptr::null_mut(); } }

unsafe fn mmp_new_seq() -> c_uint { get_random_u32_below(EXT4_MMP_SEQ_MAX + 1) }

pub unsafe fn ext4_multi_mount_protect(sb: *mut super_block, mmp_block: ext4_fsblk_t) -> i32 {
    let es = (*EXT4_SB(sb)).s_es; let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut mmp: *mut mmp_struct; let mut seq: u32; let mut mmp_check_interval = le16_to_cpu((*es).s_mmp_update_interval) as c_uint; let mut wait_time = 0; let retval: i32;
    if mmp_block < le32_to_cpu((*es).s_first_data_block) as ext4_fsblk_t || mmp_block >= ext4_blocks_count(es) { ext4_warning(sb, "Invalid MMP block in superblock"); return -EINVAL; }
    if read_mmp_block(sb, &mut bh, mmp_block) != 0 { return -EIO; }
    mmp = (*bh).b_data as *mut mmp_struct;
    if mmp_check_interval < EXT4_MMP_MIN_CHECK_INTERVAL { mmp_check_interval = EXT4_MMP_MIN_CHECK_INTERVAL; }
    if le16_to_cpu((*mmp).mmp_check_interval) as c_uint > mmp_check_interval { mmp_check_interval = le16_to_cpu((*mmp).mmp_check_interval) as c_uint; }
    seq = le32_to_cpu((*mmp).mmp_seq); if seq != EXT4_MMP_SEQ_CLEAN { if seq == EXT4_MMP_SEQ_FSCK { dump_mmp_msg(sb, mmp, "fsck is running on the filesystem"); brelse(bh); return -EBUSY; } wait_time = min(mmp_check_interval * 2 + 1, mmp_check_interval + 60); if schedule_timeout_interruptible(HZ * wait_time) != 0 { brelse(bh); return -ETIMEDOUT; } if read_mmp_block(sb, &mut bh, mmp_block) != 0 { return -EIO; } mmp = (*bh).b_data as *mut mmp_struct; if seq != le32_to_cpu((*mmp).mmp_seq) { dump_mmp_msg(sb, mmp, "Device is already active on another node."); brelse(bh); return -EBUSY; } }
    seq = mmp_new_seq(); (*mmp).mmp_seq = cpu_to_le32(seq); retval = write_mmp_block_thawed(sb, bh); if retval != 0 { brelse(bh); return retval; }
    if schedule_timeout_interruptible(HZ * wait_time) != 0 { brelse(bh); return -ETIMEDOUT; }
    if read_mmp_block(sb, &mut bh, mmp_block) != 0 { return -EIO; } mmp = (*bh).b_data as *mut mmp_struct;
    if seq != le32_to_cpu((*mmp).mmp_seq) { dump_mmp_msg(sb, mmp, "Device is already active on another node."); brelse(bh); return -EBUSY; }
    (*EXT4_SB(sb)).s_mmp_bh = bh; snprintf((*mmp).mmp_bdevname.as_mut_ptr(), core::mem::size_of_val(&(*mmp).mmp_bdevname), "%pg", (*bh).b_bdev);
    (*EXT4_SB(sb)).s_mmp_tsk = kthread_run(kmmpd, sb as *mut _, "kmmpd-%.*s", core::mem::size_of_val(&(*mmp).mmp_bdevname), (*mmp).mmp_bdevname);
    if IS_ERR((*EXT4_SB(sb)).s_mmp_tsk) { (*EXT4_SB(sb)).s_mmp_tsk = core::ptr::null_mut(); brelse(bh); return -ENOMEM; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
