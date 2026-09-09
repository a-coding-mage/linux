/*
 * Copyright (c) 1982, 1986 Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Robert Elz at The University of Melbourne.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding kernel translation are referenced here.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quota_type {
    USRQUOTA = 0,
    GRPQUOTA = 1,
    PRJQUOTA = 2,
}

pub const QTYPE_MASK_USR: u32 = 1 << (USRQUOTA as u32);
pub const QTYPE_MASK_GRP: u32 = 1 << (GRPQUOTA as u32);
pub const QTYPE_MASK_PRJ: u32 = 1 << (PRJQUOTA as u32);

pub type qid_t = __kernel_uid32_t;
pub type qsize_t = i64;

#[repr(C)]
pub union kqid_id {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub projid: kprojid_t,
}

#[repr(C)]
pub struct kqid {
    pub id: kqid_id,
    pub type_: quota_type,
}

extern "C" {
    pub fn qid_eq(left: kqid, right: kqid) -> bool;
    pub fn qid_lt(left: kqid, right: kqid) -> bool;
    pub fn from_kqid(to: *mut user_namespace, qid: kqid) -> qid_t;
    pub fn from_kqid_munged(to: *mut user_namespace, qid: kqid) -> qid_t;
    pub fn qid_valid(qid: kqid) -> bool;
    pub fn make_kuid(from: *mut user_namespace, qid: qid_t) -> kuid_t;
    pub fn make_kgid(from: *mut user_namespace, qid: qid_t) -> kgid_t;
    pub fn make_kprojid(from: *mut user_namespace, qid: qid_t) -> kprojid_t;
    pub fn test_bit(nr: u32, addr: *const c_ulong) -> i32;
    pub fn percpu_counter_inc(counter: *mut percpu_counter);
    pub fn percpu_counter_dec(counter: *mut percpu_counter);
}

#[inline]
pub unsafe fn make_kqid(from: *mut user_namespace, type_: quota_type, qid: qid_t) -> kqid {
    let mut kqid = kqid { id: kqid_id { uid: core::mem::zeroed() }, type_ };
    match type_ {
        quota_type::USRQUOTA => kqid.id.uid = make_kuid(from, qid),
        quota_type::GRPQUOTA => kqid.id.gid = make_kgid(from, qid),
        quota_type::PRJQUOTA => kqid.id.projid = make_kprojid(from, qid),
    }
    kqid
}

#[inline]
pub unsafe fn make_kqid_invalid(type_: quota_type) -> kqid {
    let mut kqid = kqid { id: kqid_id { uid: core::mem::zeroed() }, type_ };
    match type_ {
        quota_type::USRQUOTA => kqid.id.uid = INVALID_UID,
        quota_type::GRPQUOTA => kqid.id.gid = INVALID_GID,
        quota_type::PRJQUOTA => kqid.id.projid = INVALID_PROJID,
    }
    kqid
}

#[inline]
pub fn make_kqid_uid(uid: kuid_t) -> kqid { kqid { id: kqid_id { uid }, type_: quota_type::USRQUOTA } }
#[inline]
pub fn make_kqid_gid(gid: kgid_t) -> kqid { kqid { id: kqid_id { gid }, type_: quota_type::GRPQUOTA } }
#[inline]
pub fn make_kqid_projid(projid: kprojid_t) -> kqid { kqid { id: kqid_id { projid }, type_: quota_type::PRJQUOTA } }

#[inline]
pub unsafe fn qid_has_mapping(ns: *mut user_namespace, qid: kqid) -> bool {
    from_kqid(ns, qid) != (!0 as qid_t)
}

extern "C" { pub static mut dq_data_lock: spinlock_t; }

pub const DQUOT_INIT_ALLOC: u32 = if V1_INIT_ALLOC > V2_INIT_ALLOC { V1_INIT_ALLOC } else { V2_INIT_ALLOC };
pub const DQUOT_INIT_REWRITE: u32 = if V1_INIT_REWRITE > V2_INIT_REWRITE { V1_INIT_REWRITE } else { V2_INIT_REWRITE };
pub const DQUOT_DEL_ALLOC: u32 = if V1_DEL_ALLOC > V2_DEL_ALLOC { V1_DEL_ALLOC } else { V2_DEL_ALLOC };
pub const DQUOT_DEL_REWRITE: u32 = if V1_DEL_REWRITE > V2_DEL_REWRITE { V1_DEL_REWRITE } else { V2_DEL_REWRITE };

#[repr(C)]
pub struct mem_dqblk {
    pub dqb_bhardlimit: qsize_t, pub dqb_bsoftlimit: qsize_t, pub dqb_curspace: qsize_t,
    pub dqb_rsvspace: qsize_t, pub dqb_ihardlimit: qsize_t, pub dqb_isoftlimit: qsize_t,
    pub dqb_curinodes: qsize_t, pub dqb_btime: time64_t, pub dqb_itime: time64_t,
}

#[repr(C)] pub struct quota_format_type;
#[repr(C)] pub struct mem_dqinfo {
    pub dqi_format: *mut quota_format_type, pub dqi_fmt_id: i32, pub dqi_dirty_list: list_head,
    pub dqi_flags: c_ulong, pub dqi_bgrace: c_uint, pub dqi_igrace: c_uint,
    pub dqi_max_spc_limit: qsize_t, pub dqi_max_ino_limit: qsize_t, pub dqi_priv: *mut c_void,
}
#[repr(C)] pub struct super_block;

pub const DQF_GETINFO_MASK: u32 = DQF_ROOT_SQUASH | DQF_SYS_FILE;
pub const DQF_SETINFO_MASK: u32 = DQF_ROOT_SQUASH;
pub const DQF_INFO_DIRTY_B: u32 = DQF_PRIVATE;
pub const DQF_INFO_DIRTY: u32 = 1 << DQF_INFO_DIRTY_B;

extern "C" { pub fn mark_info_dirty(sb: *mut super_block, type_: i32); }
#[inline] pub unsafe fn info_dirty(info: *mut mem_dqinfo) -> i32 { test_bit(DQF_INFO_DIRTY_B, &(*info).dqi_flags) }

pub const DQST_LOOKUPS: usize = 0; pub const DQST_DROPS: usize = 1; pub const DQST_READS: usize = 2;
pub const DQST_WRITES: usize = 3; pub const DQST_CACHE_HITS: usize = 4; pub const DQST_ALLOC_DQUOTS: usize = 5;
pub const DQST_FREE_DQUOTS: usize = 6; pub const DQST_SYNCS: usize = 7; pub const _DQST_DQSTAT_LAST: usize = 8;
#[repr(C)] pub struct dqstats { pub stat: [c_ulong; _DQST_DQSTAT_LAST], pub counter: [percpu_counter; _DQST_DQSTAT_LAST] }
extern "C" { pub static mut dqstats: dqstats; }
#[inline] pub unsafe fn dqstats_inc(type_: c_uint) { percpu_counter_inc(&mut dqstats.counter[type_ as usize]); }
#[inline] pub unsafe fn dqstats_dec(type_: c_uint) { percpu_counter_dec(&mut dqstats.counter[type_ as usize]); }

pub const DQ_MOD_B: u32 = 0; pub const DQ_BLKS_B: u32 = 1; pub const DQ_INODES_B: u32 = 2;
pub const DQ_FAKE_B: u32 = 3; pub const DQ_READ_B: u32 = 4; pub const DQ_ACTIVE_B: u32 = 5;
pub const DQ_RELEASING_B: u32 = 6; pub const DQ_LASTSET_B: u32 = 7;

#[repr(C)] pub struct dquot {
    pub dq_hash: hlist_node, pub dq_inuse: list_head, pub dq_free: list_head, pub dq_dirty: list_head,
    pub dq_lock: mutex, pub dq_dqb_lock: spinlock_t, pub dq_count: atomic_t, pub dq_sb: *mut super_block,
    pub dq_id: kqid, pub dq_off: loff_t, pub dq_flags: c_ulong, pub dq_dqb: mem_dqblk,
}

pub type quota_check_quota_file = unsafe extern "C" fn(*mut super_block, i32) -> i32;
pub type quota_read_file_info = quota_check_quota_file;
pub type quota_write_file_info = quota_check_quota_file;
pub type quota_free_file_info = quota_check_quota_file;
pub type quota_read_dqblk = unsafe extern "C" fn(*mut dquot) -> i32;
pub type quota_get_next_id = unsafe extern "C" fn(*mut super_block, *mut kqid) -> i32;
#[repr(C)] pub struct quota_format_ops {
    pub check_quota_file: Option<quota_check_quota_file>, pub read_file_info: Option<quota_read_file_info>,
    pub write_file_info: Option<quota_write_file_info>, pub free_file_info: Option<quota_free_file_info>,
    pub read_dqblk: Option<quota_read_dqblk>, pub commit_dqblk: Option<quota_read_dqblk>,
    pub release_dqblk: Option<quota_read_dqblk>, pub get_next_id: Option<quota_get_next_id>,
}

#[repr(C)] pub struct dquot_operations {
    pub write_dquot: Option<quota_read_dqblk>, pub alloc_dquot: Option<unsafe extern "C" fn(*mut super_block, i32) -> *mut dquot>,
    pub destroy_dquot: Option<unsafe extern "C" fn(*mut dquot)>, pub acquire_dquot: Option<quota_read_dqblk>,
    pub release_dquot: Option<quota_read_dqblk>, pub mark_dirty: Option<quota_read_dqblk>,
    pub write_info: Option<quota_check_quota_file>, pub get_reserved_space: Option<unsafe extern "C" fn(*mut inode) -> *mut qsize_t>,
    pub get_projid: Option<unsafe extern "C" fn(*mut inode, *mut kprojid_t) -> i32>,
    pub get_inode_usage: Option<unsafe extern "C" fn(*mut inode, *mut qsize_t) -> i32>, pub get_next_id: Option<quota_get_next_id>,
}
#[repr(C)] pub struct path;
#[repr(C)] pub struct qc_dqblk {
    pub d_fieldmask: i32, pub d_spc_hardlimit: u64, pub d_spc_softlimit: u64, pub d_ino_hardlimit: u64,
    pub d_ino_softlimit: u64, pub d_space: u64, pub d_ino_count: u64, pub d_ino_timer: i64, pub d_spc_timer: i64,
    pub d_ino_warns: i32, pub d_spc_warns: i32, pub d_rt_spc_hardlimit: u64, pub d_rt_spc_softlimit: u64,
    pub d_rt_space: u64, pub d_rt_spc_timer: i64, pub d_rt_spc_warns: i32,
}

pub const QC_INO_SOFT: i32=1<<0; pub const QC_INO_HARD: i32=1<<1; pub const QC_SPC_SOFT: i32=1<<2; pub const QC_SPC_HARD: i32=1<<3;
pub const QC_RT_SPC_SOFT: i32=1<<4; pub const QC_RT_SPC_HARD: i32=1<<5; pub const QC_LIMIT_MASK: i32=0x3f;
pub const QC_SPC_TIMER: i32=1<<6; pub const QC_INO_TIMER: i32=1<<7; pub const QC_RT_SPC_TIMER: i32=1<<8; pub const QC_TIMER_MASK: i32=0x1c0;
pub const QC_SPC_WARNS: i32=1<<9; pub const QC_INO_WARNS: i32=1<<10; pub const QC_RT_SPC_WARNS: i32=1<<11; pub const QC_WARNS_MASK: i32=0xe00;
pub const QC_SPACE: i32=1<<12; pub const QC_INO_COUNT: i32=1<<13; pub const QC_RT_SPACE: i32=1<<14; pub const QC_ACCT_MASK: i32=0x7000; pub const QC_FLAGS: i32=1<<15;
pub const QCI_SYSFILE: u32=1<<0; pub const QCI_ROOT_SQUASH: u32=1<<1; pub const QCI_ACCT_ENABLED: u32=1<<2; pub const QCI_LIMITS_ENFORCED: u32=1<<3;

#[repr(C)] pub struct qc_type_state { pub flags: c_uint, pub spc_timelimit: c_uint, pub ino_timelimit: c_uint, pub rt_spc_timelimit: c_uint, pub spc_warnlimit: c_uint, pub ino_warnlimit: c_uint, pub rt_spc_warnlimit: c_uint, pub ino: c_ulonglong, pub blocks: blkcnt_t, pub nextents: blkcnt_t }
#[repr(C)] pub struct qc_state { pub s_incoredqs: c_uint, pub s_state: [qc_type_state; MAXQUOTAS as usize] }
#[repr(C)] pub struct qc_info { pub i_fieldmask: i32, pub i_flags: c_uint, pub i_spc_timelimit: c_uint, pub i_ino_timelimit: c_uint, pub i_rt_spc_timelimit: c_uint, pub i_spc_warnlimit: c_uint, pub i_ino_warnlimit: c_uint, pub i_rt_spc_warnlimit: c_uint }

#[repr(C)] pub struct quotactl_ops {
    pub quota_on: Option<unsafe extern "C" fn(*mut super_block, i32, i32, *const path) -> i32>, pub quota_off: Option<unsafe extern "C" fn(*mut super_block, i32) -> i32>,
    pub quota_enable: Option<unsafe extern "C" fn(*mut super_block, c_uint) -> i32>, pub quota_disable: Option<unsafe extern "C" fn(*mut super_block, c_uint) -> i32>, pub quota_sync: Option<unsafe extern "C" fn(*mut super_block, i32) -> i32>,
    pub set_info: Option<unsafe extern "C" fn(*mut super_block, i32, *mut qc_info) -> i32>, pub get_dqblk: Option<unsafe extern "C" fn(*mut super_block, kqid, *mut qc_dqblk) -> i32>, pub get_nextdqblk: Option<unsafe extern "C" fn(*mut super_block, *mut kqid, *mut qc_dqblk) -> i32>, pub set_dqblk: Option<unsafe extern "C" fn(*mut super_block, kqid, *mut qc_dqblk) -> i32>, pub get_state: Option<unsafe extern "C" fn(*mut super_block, *mut qc_state) -> i32>, pub rm_xquota: Option<unsafe extern "C" fn(*mut super_block, c_uint) -> i32>,
}
#[repr(C)] pub struct module;
#[repr(C)] pub struct quota_format_type { pub qf_fmt_id: i32, pub qf_ops: *const quota_format_ops, pub qf_owner: *mut module, pub qf_next: *mut quota_format_type }

pub const _DQUOT_USAGE_ENABLED: u32=0; pub const _DQUOT_LIMITS_ENABLED: u32=1; pub const _DQUOT_SUSPENDED: u32=2; pub const _DQUOT_STATE_FLAGS: u32=3;
pub const DQUOT_USAGE_ENABLED: u32=1 << (_DQUOT_USAGE_ENABLED * MAXQUOTAS); pub const DQUOT_LIMITS_ENABLED: u32=1 << (_DQUOT_LIMITS_ENABLED * MAXQUOTAS); pub const DQUOT_SUSPENDED: u32=1 << (_DQUOT_SUSPENDED * MAXQUOTAS); pub const DQUOT_STATE_FLAGS: u32=DQUOT_USAGE_ENABLED|DQUOT_LIMITS_ENABLED|DQUOT_SUSPENDED; pub const DQUOT_STATE_LAST: u32=_DQUOT_STATE_FLAGS*MAXQUOTAS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
