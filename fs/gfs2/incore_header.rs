/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of incore.h; kernel-provided types remain external. */

pub const DIO_WAIT: u32 = 0x00000010;
pub const DIO_METADATA: u32 = 0x00000020;
pub const GBF_FULL: u32 = 1;
pub const GDLM_STRNAME_BYTES: usize = 25;
pub const GDLM_LVB_SIZE: usize = 32;
pub const GFS2_MAXQUOTAS: usize = 2;
pub const GFS2_FSNAME_LEN: usize = 256;
pub const GFS2_BAD_INO: u64 = 1;

pub type Gfs2GlopBhT = unsafe extern "C" fn(*mut gfs2_glock, u32);

#[repr(C)] pub struct gfs2_log_operations {
    pub lo_before_commit: Option<unsafe extern "C" fn(*mut gfs2_sbd, *mut gfs2_trans)>,
    pub lo_after_commit: Option<unsafe extern "C" fn(*mut gfs2_sbd, *mut gfs2_trans)>,
    pub lo_before_scan: Option<unsafe extern "C" fn(*mut gfs2_jdesc, *mut gfs2_log_header_host, i32)>,
    pub lo_scan_elements: Option<unsafe extern "C" fn(*mut gfs2_jdesc, u32, *mut gfs2_log_descriptor, *mut u64, i32) -> i32>,
    pub lo_after_scan: Option<unsafe extern "C" fn(*mut gfs2_jdesc, i32, i32)>,
    pub lo_name: *const i8,
}

#[repr(C)] pub struct gfs2_log_header_host { pub lh_sequence:u64, pub lh_flags:u32, pub lh_tail:u32, pub lh_blkno:u32, pub lh_local_total:i64, pub lh_local_free:i64, pub lh_local_dinodes:i64 }
#[repr(C)] pub struct gfs2_bitmap { pub bi_bh:*mut buffer_head, pub bi_clone:*mut i8, pub bi_flags:usize, pub bi_offset:u32, pub bi_start:u32, pub bi_bytes:u32, pub bi_blocks:u32 }
#[repr(C)] pub struct gfs2_rgrpd { pub rd_node:rb_node, pub rd_gl:*mut gfs2_glock, pub rd_addr:u64, pub rd_data0:u64, pub rd_length:u32, pub rd_data:u32, pub rd_bitbytes:u32, pub rd_free:u32, pub rd_requested:u32, pub rd_reserved:u32, pub rd_free_clone:u32, pub rd_dinodes:u32, pub rd_igeneration:u64, pub rd_bits:*mut gfs2_bitmap, pub rd_sbd:*mut gfs2_sbd, pub rd_rgl:*mut gfs2_rgrp_lvb, pub rd_last_alloc:u32, pub rd_flags:u32, pub rd_extfail_pt:u32, pub rd_rsspin:spinlock_t, pub rd_mutex:mutex, pub rd_rstree:rb_root }
pub const GFS2_RDF_CHECK:u32=0x10000000; pub const GFS2_RDF_ERROR:u32=0x40000000; pub const GFS2_RDF_PREFERRED:u32=0x80000000; pub const GFS2_RDF_MASK:u32=0xf0000000;

#[repr(C)] pub struct gfs2_bufdata { pub bd_bh:*mut buffer_head, pub bd_gl:*mut gfs2_glock, pub bd_blkno:u64, pub bd_list:list_head, pub bd_tr:*mut gfs2_trans, pub bd_ail_st_list:list_head, pub bd_ail_gl_list:list_head }
#[repr(C)] pub struct lm_lockname { pub ln_number:u64, pub ln_sbd:*mut gfs2_sbd, pub ln_type:u32 }
#[repr(C)] pub struct gfs2_glock_operations { pub go_sync:Option<unsafe extern "C" fn(*mut gfs2_glock)->i32>, pub go_xmote_bh:Option<unsafe extern "C" fn(*mut gfs2_glock)->i32>, pub go_inval:Option<unsafe extern "C" fn(*mut gfs2_glock,i32)>, pub go_instantiate:Option<unsafe extern "C" fn(*mut gfs2_glock)->i32>, pub go_held:Option<unsafe extern "C" fn(*mut gfs2_holder)->i32>, pub go_dump:Option<unsafe extern "C" fn(*mut seq_file,*const gfs2_glock,*const i8)>, pub go_callback:Option<unsafe extern "C" fn(*mut gfs2_glock,bool)>, pub go_subclass:i32, pub go_type:i32, pub go_flags:usize }
pub const GLOF_ASPACE:usize=1; pub const GLOF_LVB:usize=2;

#[repr(C)] pub struct gfs2_lkstats { pub stats:[u64;7] }
#[repr(C)] pub struct gfs2_holder { pub gh_list:list_head, pub gh_gl:*mut gfs2_glock, pub gh_owner_pid:*mut pid, pub gh_flags:u16, pub gh_state:u16, pub gh_error:i32, pub gh_iflags:usize, pub gh_ip:usize }
#[repr(C)] pub struct gfs2_qadata { pub qa_qd:[*mut gfs2_quota_data;4], pub qa_qd_ghs:[gfs2_holder;4], pub qa_qd_num:u32, pub qa_ref:i32 }
#[repr(C)] pub struct gfs2_blkreserv { pub rs_node:rb_node, pub rs_rgd:*mut gfs2_rgrpd, pub rs_start:u64, pub rs_requested:u32, pub rs_reserved:u32 }
#[repr(C)] pub struct gfs2_alloc_parms { pub target:u64, pub min_target:u32, pub aflags:u32, pub allowed:u64 }

#[repr(C)] pub struct gfs2_glock { pub gl_flags:usize, pub gl_name:lm_lockname, pub gl_lockref:lockref, pub gl_state:u8, pub gl_target:u8, pub gl_demote_state:u8, pub gl_req:u8, pub gl_reply:u8, pub gl_demote_time:usize, pub gl_hold_time:isize, pub gl_holders:list_head, pub gl_ops:*const gfs2_glock_operations, pub gl_dstamp:ktime_t, pub gl_stats:gfs2_lkstats, pub gl_lksb:dlm_lksb, pub gl_tchange:usize, pub gl_object:*mut core::ffi::c_void, pub gl_dead:list_head, pub gl_ail_list:list_head, pub gl_ail_count:atomic_t, pub gl_revokes:atomic_t, pub gl_work:delayed_work, pub gl_delete:delayed_work, pub gl_no_formal_ino:u64, pub gl_rcu:rcu_head, pub gl_node:rhash_head }
#[inline] pub unsafe fn glock_type(gl:*const gfs2_glock)->u32 { (*gl).gl_name.ln_type }
#[inline] pub unsafe fn glock_number(gl:*const gfs2_glock)->u64 { (*gl).gl_name.ln_number }

#[repr(C)] pub struct gfs2_inode { pub i_inode:inode, pub i_no_addr:u64, pub i_no_formal_ino:u64, pub i_generation:u64, pub i_eattr:u64, pub i_flags:usize, pub i_gl:*mut gfs2_glock, pub i_iopen_gh:gfs2_holder, pub i_qadata:*mut gfs2_qadata, pub i_rgd_gh:gfs2_holder, pub i_res:gfs2_blkreserv, pub i_goal:u64, pub i_sizehint:atomic_t, pub i_rw_mutex:rw_semaphore, pub i_ordered:list_head, pub i_hash_cache:*mut u64, pub i_entries:u32, pub i_diskflags:u32, pub i_height:u8, pub i_depth:u8, pub i_rahead:u16 }
#[inline] pub unsafe fn GFS2_I(inode:*mut inode)->*mut gfs2_inode { inode as *mut gfs2_inode }
#[inline] pub unsafe fn GFS2_SB(inode:*const inode)->*mut gfs2_sbd { (*(*inode).i_sb).s_fs_info as *mut gfs2_sbd }
#[repr(C)] pub struct gfs2_file { pub f_fl_mutex:mutex, pub f_fl_gh:gfs2_holder }
#[repr(C)] pub struct gfs2_revoke_replay { pub rr_list:list_head, pub rr_blkno:u64, pub rr_where:u32 }
#[repr(C)] pub struct gfs2_quota_data { pub qd_hlist:hlist_bl_node, pub qd_list:list_head, pub qd_id:kqid, pub qd_sbd:*mut gfs2_sbd, pub qd_lockref:lockref, pub qd_lru:list_head, pub qd_hash:u32, pub qd_flags:usize, pub qd_change:i64, pub qd_change_sync:i64, pub qd_slot:u32, pub qd_slot_ref:u32, pub qd_bh:*mut buffer_head, pub qd_bh_qc:*mut gfs2_quota_change, pub qd_bh_count:u32, pub qd_gl:*mut gfs2_glock, pub qd_qb:gfs2_quota_lvb, pub qd_sync_gen:u64, pub qd_last_warn:usize, pub qd_rcu:rcu_head }
#[repr(C)] pub struct gfs2_trans { pub tr_ip:usize, pub tr_blocks:u32, pub tr_revokes:u32, pub tr_reserved:u32, pub tr_flags:usize, pub tr_num_buf_new:u32, pub tr_num_databuf_new:u32, pub tr_num_buf_rm:u32, pub tr_num_databuf_rm:u32, pub tr_num_revoke:u32, pub tr_list:list_head, pub tr_databuf:list_head, pub tr_buf:list_head, pub tr_first:u32, pub tr_ail1_list:list_head, pub tr_ail2_list:list_head }
#[repr(C)] pub struct gfs2_journal_extent { pub list:list_head, pub lblock:u32, pub dblock:u64, pub blocks:u64 }
#[repr(C)] pub struct gfs2_jdesc { pub jd_list:list_head, pub extent_list:list_head, pub nr_extents:u32, pub jd_work:work_struct, pub jd_inode:*mut inode, pub jd_log_bio:*mut bio, pub jd_flags:usize, pub jd_jid:u32, pub jd_blocks:u32, pub jd_recover_error:i32, pub jd_found_blocks:u32, pub jd_found_revokes:u32, pub jd_replayed_blocks:u32, pub jd_revoke_list:list_head, pub jd_replay_tail:u32 }
#[repr(C)] pub struct gfs2_statfs_change_host { pub sc_total:i64, pub sc_free:i64, pub sc_dinodes:i64 }

pub const GFS2_QUOTA_DEFAULT:u32=0; pub const GFS2_QUOTA_OFF:u32=0; pub const GFS2_QUOTA_ACCOUNT:u32=1; pub const GFS2_QUOTA_ON:u32=2; pub const GFS2_QUOTA_QUIET:u32=3; pub const GFS2_DATA_DEFAULT:u32=2; pub const GFS2_DATA_WRITEBACK:u32=1; pub const GFS2_DATA_ORDERED:u32=2; pub const GFS2_ERRORS_DEFAULT:u32=0; pub const GFS2_ERRORS_WITHDRAW:u32=0; pub const GFS2_ERRORS_DEACTIVATE:u32=1; pub const GFS2_ERRORS_PANIC:u32=3;
#[repr(C)] pub struct gfs2_args { pub ar_lockproto:[i8;0], pub ar_locktable:[i8;0], pub ar_hostdata:[i8;0], pub ar_bits:u32, pub ar_commit:i32, pub ar_statfs_quantum:i32, pub ar_quota_quantum:i32, pub ar_statfs_percent:i32 }
#[repr(C)] pub struct gfs2_tune { pub gt_spin:spinlock_t, pub gt_logd_secs:u32, pub gt_quota_warn_period:u32, pub gt_quota_scale_num:u32, pub gt_quota_scale_den:u32, pub gt_quota_quantum:u32, pub gt_new_files_jdata:u32, pub gt_max_readahead:u32, pub gt_complain_secs:u32, pub gt_statfs_quantum:u32, pub gt_statfs_slow:u32, pub gt_withdraw_helper_timeout:u32 }
#[repr(C)] pub struct gfs2_inum_host { pub no_formal_ino:u64, pub no_addr:u64 }
#[repr(C)] pub struct gfs2_sb_host { pub sb_magic:u32, pub sb_type:u32, pub sb_fs_format:u32, pub sb_multihost_format:u32, pub sb_bsize:u32, pub sb_bsize_shift:u32, pub sb_master_dir:gfs2_inum_host, pub sb_root_dir:gfs2_inum_host, pub sb_lockproto:[i8;0], pub sb_locktable:[i8;0] }
#[repr(C)] pub struct lm_lockstruct { pub ls_jid:i32, pub ls_first:u32, pub ls_ops:*const lm_lockops, pub ls_dlm:*mut dlm_lockspace_t, pub ls_recover_jid_done:i32, pub ls_recover_jid_status:i32, pub ls_mounted_lksb:dlm_lksb, pub ls_control_lksb:dlm_lksb, pub ls_control_lvb:[i8;32], pub ls_sync_wait:completion, pub ls_lvb_bits:*mut i8, pub ls_sem:rw_semaphore, pub ls_recover_spin:spinlock_t, pub ls_recover_flags:usize, pub ls_recover_mount:u32, pub ls_recover_start:u32, pub ls_recover_block:u32, pub ls_recover_size:u32, pub ls_recover_submit:*mut u32, pub ls_recover_result:*mut u32 }
#[repr(C)] pub struct gfs2_pcpu_lkstats { pub lkstats:[gfs2_lkstats;10] }
#[repr(C)] pub struct local_statfs_inode { pub si_list:list_head, pub si_sc_inode:*mut inode, pub si_jid:u32 }

// The complete superblock object contains kernel-private members and is retained as an opaque dependency.
#[repr(C)] pub struct gfs2_sbd { pub sd_vfs:*mut super_block, pub sd_lkstats:*mut gfs2_pcpu_lkstats, pub sd_flags:usize, pub sd_sb:gfs2_sb_host, pub sd_args:gfs2_args, pub sd_tune:gfs2_tune, pub sd_inode:*mut inode, pub sd_log_tr:*mut gfs2_trans, pub sd_log_sequence:u64, pub sd_fsname:[i8;262], pub sd_table_name:[i8;256], pub sd_proto_name:[i8;256] }

#[inline] pub unsafe fn glock_sbd(gl:*const gfs2_glock)->*mut gfs2_sbd { (*gl).gl_name.ln_sbd }
#[inline] pub unsafe fn gfs2_aspace(sdp:*mut gfs2_sbd)->*mut address_space { (*sdp).sd_inode as *mut address_space }
#[inline] pub unsafe fn gfs2_glstats_inc(gl:*mut gfs2_glock, which:usize) { (*gl).gl_stats.stats[which] = (*gl).gl_stats.stats[which].wrapping_add(1); }
pub unsafe extern "C" { pub fn gfs2_glock2rgrp(gl:*mut gfs2_glock)->*mut gfs2_rgrpd; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
