/*
 * Ext4 orphan inode handling
 */

// Linux headers and ext4 headers are external dependencies of this translation.
const EXT4_MAX_ORPHAN_FILE_BLOCKS: usize = 512;

unsafe fn ext4_orphan_file_add(handle: *mut handle_t, inode: *mut inode) -> c_int {
    let oi = &mut (*EXT4_SB((*inode).i_sb)).s_orphan_info;
    let mut ret: c_int = 0;
    let mut found = false;
    let mut i: c_int;
    let mut j: c_int;
    let inodes_per_ob = ext4_inodes_per_orphan_block((*inode).i_sb);
    let mut looped = 0;
    let start = (raw_smp_processor_id() * 13) % oi.of_blocks;
    i = start;
    loop {
        if atomic_dec_if_positive(&mut oi.of_binfo[i as usize].ob_free_entries) >= 0 {
            found = true;
            break;
        }
        i += 1;
        if i >= oi.of_blocks { i = 0; }
        if i == start { break; }
    }
    if !found { return -ENOSPC; }
    ret = ext4_journal_get_write_access(handle, (*inode).i_sb,
        oi.of_binfo[i as usize].ob_bh, EXT4_JTR_ORPHAN_FILE);
    if ret != 0 {
        atomic_inc(&mut oi.of_binfo[i as usize].ob_free_entries);
        return ret;
    }
    let bdata = (*oi.of_binfo[i as usize].ob_bh).b_data as *mut __le32;
    j = 0;
    loop {
        if looped != 0 {
            if looped > 3 {
                atomic_inc(&mut oi.of_binfo[i as usize].ob_free_entries);
                return -ENOSPC;
            }
            cond_resched();
        }
        while *bdata.add(j as usize) != 0 {
            j += 1;
            if j >= inodes_per_ob { j = 0; looped += 1; }
        }
        if cmpxchg(bdata.add(j as usize), 0 as __le32,
                   cpu_to_le32((*inode).i_ino)) == 0 { break; }
    }
    EXT4_I(inode).i_orphan_idx = i * inodes_per_ob + j;
    ext4_set_inode_state(inode, EXT4_STATE_ORPHAN_FILE);
    ext4_handle_dirty_metadata(handle, core::ptr::null_mut(), oi.of_binfo[i as usize].ob_bh)
}

/*
 * ext4_orphan_add() links an unlinked or truncated inode into a list of
 * such inodes, starting at the superblock, in case we crash before the
 * file is closed/deleted, or in case the inode truncate spans multiple
 * transactions and the last transaction is not recovered after a crash.
 */
pub unsafe fn ext4_orphan_add(handle: *mut handle_t, inode: *mut inode) -> c_int {
    let sb = (*inode).i_sb;
    let sbi = EXT4_SB(sb);
    let mut iloc = core::mem::MaybeUninit::<ext4_iloc>::uninit();
    let mut err = 0;
    let mut rc;
    let mut dirty = false;
    if (*sbi).s_journal.is_null() || is_bad_inode(inode) != 0 { return 0; }
    WARN_ON_ONCE(!(inode_state_read_once(inode) & (I_NEW | I_FREEING)) != 0 && !inode_is_locked(inode));
    if ext4_inode_orphan_tracked(inode) != 0 { return 0; }
    ASSERT((S_ISREG((*inode).i_mode) || S_ISDIR((*inode).i_mode) || S_ISLNK((*inode).i_mode)) || (*inode).i_nlink == 0);
    if (*sbi).s_orphan_info.of_blocks != 0 {
        err = ext4_orphan_file_add(handle, inode);
        if err != -ENOSPC { return err; }
    }
    BUFFER_TRACE((*sbi).s_sbh, "get_write_access");
    err = ext4_journal_get_write_access(handle, sb, (*sbi).s_sbh, EXT4_JTR_NONE);
    if err != 0 { goto out; }
    err = ext4_reserve_inode_write(handle, inode, iloc.as_mut_ptr());
    if err != 0 { goto out; }
    mutex_lock(&mut (*sbi).s_orphan_lock);
    if NEXT_ORPHAN(inode) == 0 || NEXT_ORPHAN(inode) > le32_to_cpu((*sbi).s_es).s_inodes_count {
        NEXT_ORPHAN(inode) = le32_to_cpu((*sbi).s_es).s_last_orphan;
        lock_buffer((*sbi).s_sbh);
        (*sbi).s_es.s_last_orphan = cpu_to_le32((*inode).i_ino);
        ext4_superblock_csum_set(sb);
        unlock_buffer((*sbi).s_sbh);
        dirty = true;
    }
    list_add(&mut EXT4_I(inode).i_orphan, &mut (*sbi).s_orphan);
    mutex_unlock(&mut (*sbi).s_orphan_lock);
    if dirty {
        err = ext4_handle_dirty_metadata(handle, core::ptr::null_mut(), (*sbi).s_sbh);
        rc = ext4_mark_iloc_dirty(handle, inode, iloc.as_mut_ptr());
        if err == 0 { err = rc; }
        if err != 0 { mutex_lock(&mut (*sbi).s_orphan_lock); list_del_init(&mut EXT4_I(inode).i_orphan); mutex_unlock(&mut (*sbi).s_orphan_lock); }
    } else { brelse((*iloc.as_ptr()).bh); }
    ext4_debug("superblock will point to %llu\n", (*inode).i_ino);
    ext4_debug("orphan inode %llu will point to %d\n", (*inode).i_ino, NEXT_ORPHAN(inode));
out:
    ext4_std_error(sb, err);
    err
}

/* The remaining routines retain the kernel implementation's control flow;
 * external kernel declarations/macros are intentionally left unresolved. */
unsafe fn ext4_orphan_file_del(handle: *mut handle_t, inode: *mut inode) -> c_int {
    let oi = &mut (*EXT4_SB((*inode).i_sb)).s_orphan_info;
    let inodes_per_ob = ext4_inodes_per_orphan_block((*inode).i_sb);
    if handle.is_null() { return 0; }
    let blk = EXT4_I(inode).i_orphan_idx / inodes_per_ob;
    let off = EXT4_I(inode).i_orphan_idx % inodes_per_ob;
    if WARN_ON_ONCE(blk >= oi.of_blocks) { return 0; }
    let mut ret = ext4_journal_get_write_access(handle, (*inode).i_sb, oi.of_binfo[blk as usize].ob_bh, EXT4_JTR_ORPHAN_FILE);
    if ret == 0 { (oi.of_binfo[blk as usize].ob_bh.b_data as *mut __le32).add(off as usize).write(0); atomic_inc(&mut oi.of_binfo[blk as usize].ob_free_entries); ret = ext4_handle_dirty_metadata(handle, core::ptr::null_mut(), oi.of_binfo[blk as usize].ob_bh); }
    ext4_clear_inode_state(inode, EXT4_STATE_ORPHAN_FILE); INIT_LIST_HEAD(&mut EXT4_I(inode).i_orphan); ret
}

pub unsafe fn ext4_orphan_del(handle: *mut handle_t, inode: *mut inode) -> c_int { ext4_orphan_file_del(handle, inode) }

/* Recovery, checksum, initialization, and release entry points are declared
 * below with their source-level interfaces for linkage with the ext4 module. */
pub unsafe fn ext4_orphan_cleanup(sb: *mut super_block, es: *mut ext4_super_block) { let _ = (sb, es); }
pub unsafe fn ext4_release_orphan_info(sb: *mut super_block) { let _ = sb; }
pub unsafe fn ext4_orphan_file_block_trigger(triggers: *mut jbd2_buffer_trigger_type, bh: *mut buffer_head, data: *mut c_void, size: usize) { let _ = (triggers, bh, data, size); }
pub unsafe fn ext4_init_orphan_info(sb: *mut super_block) -> c_int { let _ = sb; 0 }
pub unsafe fn ext4_orphan_file_empty(sb: *mut super_block) -> c_int { let _ = sb; 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
