// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2008 Red Hat, Inc.  All rights reserved.
 */

// Linux and GFS2 headers supplied by the surrounding translation unit.

static mut gfs2_freeze_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn gfs2_ail_error(gl: *mut gfs2_glock, bh: *const buffer_head) {
    let sdp = glock_sbd(gl);
    fs_err(sdp, "AIL buffer {:p}: blocknr {} state 0x{:08x} mapping {:p} page state 0x{:x}\n",
           bh, (*bh).b_blocknr as u64, (*bh).b_state, (*bh).b_folio.mapping,
           (*bh).b_folio.flags.f);
    fs_err(sdp, "AIL glock {}:{} mapping {:p}\n", glock_type(gl), glock_number(gl),
           gfs2_glock2aspace(gl));
    gfs2_lm(sdp, "AIL error\n");
    gfs2_withdraw(sdp);
}

unsafe fn __gfs2_ail_flush(gl: *mut gfs2_glock, fsync: bool, mut nr_revokes: u32) {
    let sdp = glock_sbd(gl);
    let head = &mut (*gl).gl_ail_list;
    let b_state: c_ulong = (1 << BH_Dirty) | (1 << BH_Pinned) | (1 << BH_Lock);
    spin_lock(&mut (*sdp).sd_log_lock);
    spin_lock(&mut (*sdp).sd_ail_lock);
    list_for_each_entry_safe_reverse!(bd, tmp, head, bd_ail_gl_list, gfs2_bufdata) {
        if nr_revokes == 0 { break; }
        let bh = (*bd).bd_bh;
        if (*bh).b_state & b_state != 0 {
            if fsync { continue; }
            gfs2_ail_error(gl, bh);
        }
        gfs2_trans_add_revoke(sdp, bd);
        nr_revokes -= 1;
    }
    GLOCK_BUG_ON!(gl, !fsync && atomic_read(&(*gl).gl_ail_count) != 0);
    spin_unlock(&mut (*sdp).sd_ail_lock);
    spin_unlock(&mut (*sdp).sd_log_lock);
}

unsafe fn gfs2_ail_empty_gl(gl: *mut gfs2_glock) -> c_int {
    let sdp = glock_sbd(gl);
    let mut tr: gfs2_trans = core::mem::zeroed();
    let revokes = atomic_read(&(*gl).gl_ail_count) as u32;
    let mut ret = 0;
    if revokes == 0 {
        let (have_revokes, log_in_flight);
        spin_lock(&mut (*sdp).sd_log_lock);
        have_revokes = !list_empty(&(*sdp).sd_log_revokes);
        log_in_flight = atomic_read(&(*sdp).sd_log_in_flight);
        spin_unlock(&mut (*sdp).sd_log_lock);
        if !have_revokes {
            if log_in_flight != 0 { log_flush_wait(sdp); }
            return 0;
        }
    } else {
        set_bit(TR_ONSTACK, &mut tr.tr_flags);
        ret = __gfs2_trans_begin(&mut tr, sdp, 0, revokes, _RET_IP_);
        if ret != 0 {
            fs_err(sdp, "Transaction error {}: Unable to write revokes.", ret);
        } else {
            __gfs2_ail_flush(gl, false, revokes);
            gfs2_trans_end(sdp);
        }
    }
    if ret == 0 { gfs2_log_flush(sdp, core::ptr::null_mut(), GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_AIL_EMPTY_GL); }
    ret
}

pub unsafe fn gfs2_ail_flush(gl: *mut gfs2_glock, fsync: bool) {
    let sdp = glock_sbd(gl);
    let revokes = atomic_read(&(*gl).gl_ail_count) as u32;
    if revokes == 0 { return; }
    if gfs2_trans_begin(sdp, 0, revokes) != 0 { return; }
    __gfs2_ail_flush(gl, fsync, revokes);
    gfs2_trans_end(sdp);
    gfs2_log_flush(sdp, core::ptr::null_mut(), GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_AIL_FLUSH);
}

unsafe fn gfs2_rgrp_metasync(gl: *mut gfs2_glock) -> c_int {
    let sdp = glock_sbd(gl);
    let metamapping = gfs2_aspace(sdp);
    let rgd = gfs2_glock2rgrp(gl);
    let bsize = (*sdp).sd_sb.sb_bsize;
    let start = ((*rgd).rd_addr * bsize as u64) & PAGE_MASK as i64;
    let end = page_align(((*rgd).rd_addr + (*rgd).rd_length) * bsize as u64) as i64 - 1;
    filemap_fdatawrite_range(metamapping, start, end);
    let error = filemap_fdatawait_range(metamapping, start, end);
    WARN_ON_ONCE!(error != 0 && !gfs2_withdrawn(sdp));
    mapping_set_error(metamapping, error);
    if error != 0 { gfs2_io_error(sdp); }
    error
}

unsafe fn rgrp_go_sync(gl: *mut gfs2_glock) -> c_int {
    let sdp = glock_sbd(gl);
    let rgd = gfs2_glock2rgrp(gl);
    if rgd.is_null() || !test_and_clear_bit(GLF_DIRTY, &mut (*gl).gl_flags) { return 0; }
    GLOCK_BUG_ON!(gl, (*gl).gl_state != LM_ST_EXCLUSIVE);
    gfs2_log_flush(sdp, gl, GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_RGRP_GO_SYNC);
    let mut error = gfs2_rgrp_metasync(gl);
    if error == 0 { error = gfs2_ail_empty_gl(gl); }
    gfs2_free_clones(rgd);
    error
}

unsafe fn rgrp_go_inval(gl: *mut gfs2_glock, flags: c_int) {
    let sdp = glock_sbd(gl); let mapping = gfs2_aspace(sdp); let rgd = gfs2_glock2rgrp(gl);
    if rgd.is_null() { return; }
    let bsize = (*sdp).sd_sb.sb_bsize;
    let start = ((*rgd).rd_addr * bsize as u64) & PAGE_MASK as i64;
    let end = page_align(((*rgd).rd_addr + (*rgd).rd_length) * bsize as u64) as i64 - 1;
    gfs2_rgrp_brelse(rgd); WARN_ON_ONCE!((flags & DIO_METADATA) == 0);
    gfs2_assert_withdraw!(sdp, atomic_read(&(*gl).gl_ail_count) == 0);
    truncate_inode_pages_range(mapping, start, end);
}

unsafe fn gfs2_rgrp_go_dump(seq: *mut seq_file, gl: *const gfs2_glock, fs_id_buf: *const c_char) {
    let rgd = (*gl).gl_object as *mut gfs2_rgrpd;
    if !rgd.is_null() { gfs2_rgrp_dump(seq, rgd, fs_id_buf); }
}

unsafe fn gfs2_glock2inode(gl: *mut gfs2_glock) -> *mut gfs2_inode {
    spin_lock(&mut (*gl).gl_lockref.lock); let ip = (*gl).gl_object as *mut gfs2_inode;
    if !ip.is_null() { set_bit(GIF_GLOP_PENDING, &mut (*ip).i_flags); }
    spin_unlock(&mut (*gl).gl_lockref.lock); ip
}

pub unsafe fn gfs2_glock2rgrp(gl: *mut gfs2_glock) -> *mut gfs2_rgrpd {
    spin_lock(&mut (*gl).gl_lockref.lock); let rgd = (*gl).gl_object as *mut gfs2_rgrpd;
    spin_unlock(&mut (*gl).gl_lockref.lock); rgd
}

unsafe fn gfs2_clear_glop_pending(ip: *mut gfs2_inode) { if !ip.is_null() { clear_bit_unlock(GIF_GLOP_PENDING, &mut (*ip).i_flags); wake_up_bit(&mut (*ip).i_flags, GIF_GLOP_PENDING); } }

pub unsafe fn gfs2_inode_metasync(gl: *mut gfs2_glock) -> c_int {
    let mapping = gfs2_glock2aspace(gl); filemap_fdatawrite(mapping); let error = filemap_fdatawait(mapping);
    if error != 0 { gfs2_io_error(glock_sbd(gl)); } error
}

// The remaining glock operation callbacks and operation tables retain the C ABI-facing
// structure and call the corresponding external GFS2 helpers.
unsafe fn inode_go_sync(gl: *mut gfs2_glock) -> c_int { let ip = gfs2_glock2inode(gl); let isreg = !ip.is_null() && S_ISREG((*ip).i_inode.i_mode); let mapping = gfs2_glock2aspace(gl); let mut error = 0; if isreg { if test_and_clear_bit(GIF_SW_PAGED, &mut (*ip).i_flags) { unmap_shared_mapping_range((*ip).i_inode.i_mapping, 0, 0); } inode_dio_wait(&mut (*ip).i_inode); } if !test_and_clear_bit(GLF_DIRTY, &mut (*gl).gl_flags) { gfs2_clear_glop_pending(ip); return 0; } GLOCK_BUG_ON!(gl, (*gl).gl_state != LM_ST_EXCLUSIVE); gfs2_log_flush(glock_sbd(gl), gl, GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_INODE_GO_SYNC); filemap_fdatawrite(mapping); if isreg { filemap_fdatawrite((*ip).i_inode.i_mapping); error = filemap_fdatawait((*ip).i_inode.i_mapping); mapping_set_error((*ip).i_inode.i_mapping, error); } let ret = gfs2_inode_metasync(gl); if error == 0 { error = ret; } let ret = gfs2_ail_empty_gl(gl); if error == 0 { error = ret; } smp_mb__before_atomic(); clear_bit(GLF_DIRTY, &mut (*gl).gl_flags); gfs2_clear_glop_pending(ip); error }

unsafe fn inode_go_inval(gl: *mut gfs2_glock, flags: c_int) { let ip = gfs2_glock2inode(gl); gfs2_assert_withdraw!(glock_sbd(gl), atomic_read(&(*gl).gl_ail_count) == 0); if flags & DIO_METADATA != 0 { let mapping = gfs2_glock2aspace(gl); truncate_inode_pages(mapping, 0); if !ip.is_null() { set_bit(GLF_INSTANTIATE_NEEDED, &mut (*gl).gl_flags); forget_all_cached_acls(&mut (*ip).i_inode); security_inode_invalidate_secctx(&mut (*ip).i_inode); gfs2_dir_hash_inval(ip); } } if ip == GFS2_I((*glock_sbd(gl)).sd_rindex) { gfs2_log_flush(glock_sbd(gl), core::ptr::null_mut(), GFS2_LOG_HEAD_FLUSH_NORMAL | GFS2_LFC_INODE_GO_INVAL); (*glock_sbd(gl)).sd_rindex_uptodate = 0; } if !ip.is_null() && S_ISREG((*ip).i_inode.i_mode) { truncate_inode_pages((*ip).i_inode.i_mapping, 0); } gfs2_clear_glop_pending(ip); }

unsafe fn inode_go_instantiate(gl: *mut gfs2_glock) -> c_int { let ip = (*gl).gl_object as *mut gfs2_inode; if ip.is_null() { return 0; } let error = gfs2_inode_refresh(ip); if error != 0 { return error; } let io_gl = (*ip).i_iopen_gh.gh_gl; (*io_gl).gl_no_formal_ino = (*ip).i_no_formal_ino; 0 }
unsafe fn inode_go_held(gh: *mut gfs2_holder) -> c_int { let gl = (*gh).gh_gl; let ip = (*gl).gl_object as *mut gfs2_inode; if ip.is_null() { return 0; } if (*gh).gh_state != LM_ST_DEFERRED { inode_dio_wait(&mut (*ip).i_inode); } if ((*ip).i_diskflags & GFS2_DIF_TRUNC_IN_PROG) != 0 && (*gl).gl_state == LM_ST_EXCLUSIVE && (*gh).gh_state == LM_ST_EXCLUSIVE { return gfs2_truncatei_resume(ip); } 0 }

unsafe fn gfs2_inode_refresh(ip: *mut gfs2_inode) -> c_int { let mut dibh: *mut buffer_head = core::ptr::null_mut(); let error = gfs2_meta_inode_buffer(ip, &mut dibh); if error != 0 { return error; } let error = gfs2_dinode_in(ip, (*dibh).b_data as *const c_void); brelse(dibh); error }
unsafe fn gfs2_dinode_in(ip: *mut gfs2_inode, buf: *const c_void) -> c_int {
    let sdp = GFS2_SB(&mut (*ip).i_inode); let str_ = buf as *const gfs2_dinode; let inode = &mut (*ip).i_inode;
    let mode = be32_to_cpu((*str_).di_mode); let is_new = inode_state_read_once(inode) & I_NEW != 0;
    if (*ip).i_no_addr != be64_to_cpu((*str_).di_num.no_addr) || (!is_new && inode_wrong_type(inode, mode)) { gfs2_consist_inode(ip); return -EIO; }
    (*ip).i_no_formal_ino = be64_to_cpu((*str_).di_num.no_formal_ino); inode.i_mode = mode; if is_new { inode.i_rdev = 0; if mode & S_IFMT == S_IFBLK || mode & S_IFMT == S_IFCHR { inode.i_rdev = MKDEV(be32_to_cpu((*str_).di_major), be32_to_cpu((*str_).di_minor)); } }
    i_uid_write(inode, be32_to_cpu((*str_).di_uid)); i_gid_write(inode, be32_to_cpu((*str_).di_gid)); set_nlink(inode, be32_to_cpu((*str_).di_nlink)); i_size_write(inode, be64_to_cpu((*str_).di_size)); gfs2_set_inode_blocks(inode, be64_to_cpu((*str_).di_blocks));
    (*ip).i_goal = be64_to_cpu((*str_).di_goal_meta); (*ip).i_generation = be64_to_cpu((*str_).di_generation); (*ip).i_diskflags = be32_to_cpu((*str_).di_flags); (*ip).i_eattr = be64_to_cpu((*str_).di_eattr); gfs2_set_inode_flags(inode);
    let height = be16_to_cpu((*str_).di_height); let depth = be16_to_cpu((*str_).di_depth); if height > (*sdp).sd_max_height || depth > GFS2_DIR_MAX_DEPTH || ((*ip).i_diskflags & GFS2_DIF_EXHASH != 0 && depth < ilog2((*sdp).sd_hash_ptrs)) || (!S_ISDIR(inode.i_mode) && (*ip).i_diskflags & GFS2_DIF_EXHASH != 0) { gfs2_consist_inode(ip); return -EIO; } (*ip).i_height = height as u8; (*ip).i_depth = depth as u8; (*ip).i_entries = be32_to_cpu((*str_).di_entries); if gfs2_is_stuffed(ip) && inode.i_size > gfs2_max_stuffed_size(ip) { gfs2_consist_inode(ip); return -EIO; } if S_ISREG(inode.i_mode) { gfs2_set_aops(inode); } 0
}

unsafe fn freeze_go_callback(gl: *mut gfs2_glock, remote: bool) { let sdp = glock_sbd(gl); let sb = (*sdp).sd_vfs; if !remote || ((*gl).gl_state != LM_ST_SHARED && (*gl).gl_state != LM_ST_UNLOCKED) || (*gl).gl_demote_state != LM_ST_UNLOCKED { return; } if down_read_trylock(&mut (*sb).s_umount) { atomic_inc(&mut (*sb).s_active); up_read(&mut (*sb).s_umount); if !queue_work(gfs2_freeze_wq, &mut (*sdp).sd_freeze_work) { deactivate_super(sb); } } }
unsafe fn iopen_go_callback(gl: *mut gfs2_glock, remote: bool) { let ip = (*gl).gl_object as *mut gfs2_inode; let sdp = glock_sbd(gl); if !remote || test_bit(SDF_KILL, &(*sdp).sd_flags) { return; } if (*gl).gl_demote_state == LM_ST_UNLOCKED && (*gl).gl_state == LM_ST_SHARED && !ip.is_null() { (*gl).gl_lockref.count += 1; if !gfs2_queue_try_to_evict(gl) { (*gl).gl_lockref.count -= 1; } } }

// Operation table definitions mirror the C designated initializers; callback fields and
// constants are provided by the translated GFS2 type definitions.
pub static gfs2_meta_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_META, ..gfs2_glock_operations::ZERO };
pub static gfs2_inode_glops: gfs2_glock_operations = gfs2_glock_operations { go_sync: Some(inode_go_sync), go_inval: Some(inode_go_inval), go_instantiate: Some(inode_go_instantiate), go_held: Some(inode_go_held), go_type: LM_TYPE_INODE, go_flags: GLOF_ASPACE | GLOF_LVB, ..gfs2_glock_operations::ZERO };
pub static gfs2_rgrp_glops: gfs2_glock_operations = gfs2_glock_operations { go_sync: Some(rgrp_go_sync), go_inval: Some(rgrp_go_inval), go_instantiate: Some(gfs2_rgrp_go_instantiate), go_dump: Some(gfs2_rgrp_go_dump), go_type: LM_TYPE_RGRP, go_flags: GLOF_LVB, ..gfs2_glock_operations::ZERO };
pub static gfs2_freeze_glops: gfs2_glock_operations = gfs2_glock_operations { go_callback: Some(freeze_go_callback), go_type: LM_TYPE_NONDISK, ..gfs2_glock_operations::ZERO };
pub static gfs2_iopen_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_IOPEN, go_callback: Some(iopen_go_callback), go_subclass: 1, ..gfs2_glock_operations::ZERO };
pub static gfs2_flock_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_FLOCK, ..gfs2_glock_operations::ZERO };
pub static gfs2_nondisk_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_NONDISK, ..gfs2_glock_operations::ZERO };
pub static gfs2_quota_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_QUOTA, go_flags: GLOF_LVB, ..gfs2_glock_operations::ZERO };
pub static gfs2_journal_glops: gfs2_glock_operations = gfs2_glock_operations { go_type: LM_TYPE_JOURNAL, ..gfs2_glock_operations::ZERO };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
