// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel/quota implementation.

const QUOTABLOCK_BITS: u32 = 10;
const QUOTABLOCK_SIZE: qsize_t = 1 << QUOTABLOCK_BITS;

#[inline]
unsafe fn v1_stoqb(space: qsize_t) -> qsize_t {
    (space.wrapping_add(QUOTABLOCK_SIZE).wrapping_sub(1)) >> QUOTABLOCK_BITS
}

#[inline]
unsafe fn v1_qbtos(blocks: qsize_t) -> qsize_t {
    blocks << QUOTABLOCK_BITS
}

unsafe fn v1_disk2mem_dqblk(m: *mut mem_dqblk, d: *mut v1_disk_dqblk) {
    (*m).dqb_ihardlimit = (*d).dqb_ihardlimit;
    (*m).dqb_isoftlimit = (*d).dqb_isoftlimit;
    (*m).dqb_curinodes = (*d).dqb_curinodes;
    (*m).dqb_bhardlimit = v1_qbtos((*d).dqb_bhardlimit);
    (*m).dqb_bsoftlimit = v1_qbtos((*d).dqb_bsoftlimit);
    (*m).dqb_curspace = v1_qbtos((*d).dqb_curblocks);
    (*m).dqb_itime = (*d).dqb_itime;
    (*m).dqb_btime = (*d).dqb_btime;
}

unsafe fn v1_mem2disk_dqblk(d: *mut v1_disk_dqblk, m: *mut mem_dqblk) {
    (*d).dqb_ihardlimit = (*m).dqb_ihardlimit;
    (*d).dqb_isoftlimit = (*m).dqb_isoftlimit;
    (*d).dqb_curinodes = (*m).dqb_curinodes;
    (*d).dqb_bhardlimit = v1_stoqb((*m).dqb_bhardlimit);
    (*d).dqb_bsoftlimit = v1_stoqb((*m).dqb_bsoftlimit);
    (*d).dqb_curblocks = v1_stoqb((*m).dqb_curspace);
    (*d).dqb_itime = (*m).dqb_itime;
    (*d).dqb_btime = (*m).dqb_btime;
}

unsafe fn v1_read_dqblk(dquot: *mut dquot) -> c_int {
    let type_ = (*dquot).dq_id.type_;
    let mut dqblk: v1_disk_dqblk = core::mem::zeroed();
    let dqopt = sb_dqopt((*dquot).dq_sb);
    if (*dqopt).files[type_ as usize].is_null() { return -EINVAL; }
    (*dquot).dq_sb.as_ref().unwrap().s_op.as_ref().unwrap().quota_read.unwrap()(
        (*dquot).dq_sb, type_, &mut dqblk as *mut _ as *mut c_char,
        core::mem::size_of::<v1_disk_dqblk>(),
        v1_dqoff(from_kqid(&init_user_ns, (*dquot).dq_id)));
    v1_disk2mem_dqblk(&mut (*dquot).dq_dqb, &mut dqblk);
    if (*dquot).dq_dqb.dqb_bhardlimit == 0 && (*dquot).dq_dqb.dqb_bsoftlimit == 0 &&
       (*dquot).dq_dqb.dqb_ihardlimit == 0 && (*dquot).dq_dqb.dqb_isoftlimit == 0 {
        set_bit(DQ_FAKE_B, &mut (*dquot).dq_flags);
    }
    dqstats_inc(DQST_READS);
    0
}

unsafe fn v1_commit_dqblk(dquot: *mut dquot) -> ssize_t {
    let type_: i16 = (*dquot).dq_id.type_;
    let mut dqblk: v1_disk_dqblk = core::mem::zeroed();
    v1_mem2disk_dqblk(&mut dqblk, &mut (*dquot).dq_dqb);
    if ((type_ == USRQUOTA) && uid_eq((*dquot).dq_id.uid, GLOBAL_ROOT_UID)) ||
       ((type_ == GRPQUOTA) && gid_eq((*dquot).dq_id.gid, GLOBAL_ROOT_GID)) {
        dqblk.dqb_btime = (*sb_dqopt((*dquot).dq_sb)).info[type_ as usize].dqi_bgrace;
        dqblk.dqb_itime = (*sb_dqopt((*dquot).dq_sb)).info[type_ as usize].dqi_igrace;
    }
    let mut ret: ssize_t = 0;
    if !(*sb_dqopt((*dquot).dq_sb)).files[type_ as usize].is_null() {
        ret = (*dquot).dq_sb.as_ref().unwrap().s_op.as_ref().unwrap().quota_write.unwrap()(
            (*dquot).dq_sb, type_, &mut dqblk as *mut _ as *mut c_char,
            core::mem::size_of::<v1_disk_dqblk>(),
            v1_dqoff(from_kqid(&init_user_ns, (*dquot).dq_id)));
    }
    if ret != core::mem::size_of::<v1_disk_dqblk>() as ssize_t {
        quota_error((*dquot).dq_sb, c"dquota write failed".as_ptr());
        if ret >= 0 { ret = -EIO as ssize_t; }
    } else { ret = 0; }
    dqstats_inc(DQST_WRITES);
    ret
}

const V2_INITQMAGICS: [u32; 2] = [0xd9c01f11, 0xd9c01927];

#[repr(C)]
struct v2_disk_dqheader { dqh_magic: __le32, dqh_version: __le32 }

unsafe fn v1_check_quota_file(sb: *mut super_block, type_: c_int) -> c_int {
    let inode = (*sb_dqopt(sb)).files[type_ as usize];
    let isize = i_size_read(inode);
    if isize == 0 { return 0; }
    let blocks = (isize >> BLOCK_SIZE_BITS) as ulong;
    let off = (isize & (BLOCK_SIZE - 1)) as size_t;
    if (blocks % core::mem::size_of::<v1_disk_dqblk>() as ulong * BLOCK_SIZE + off) %
       core::mem::size_of::<v1_disk_dqblk>() != 0 { return 0; }
    let mut dqhead: v2_disk_dqheader = core::mem::zeroed();
    let size = (*sb).s_op.as_ref().unwrap().quota_read.unwrap()(
        sb, type_, &mut dqhead as *mut _ as *mut c_char,
        core::mem::size_of::<v2_disk_dqheader>(), 0);
    if size != core::mem::size_of::<v2_disk_dqheader>() as ssize_t { return 1; }
    if le32_to_cpu(dqhead.dqh_magic) != V2_INITQMAGICS[type_ as usize] { return 1; }
    printk(KERN_INFO, c"VFS: %s: Refusing to turn on old quota format on given file. It probably contains newer quota format.\n".as_ptr(), (*sb).s_id);
    0
}

unsafe fn v1_read_file_info(sb: *mut super_block, type_: c_int) -> c_int {
    let dqopt = sb_dqopt(sb); let mut dqblk: v1_disk_dqblk = core::mem::zeroed();
    down_read(&mut (*dqopt).dqio_sem); let memalloc = memalloc_nofs_save();
    let mut ret = (*sb).s_op.as_ref().unwrap().quota_read.unwrap()(sb, type_, &mut dqblk as *mut _ as *mut c_char, core::mem::size_of::<v1_disk_dqblk>(), v1_dqoff(0));
    if ret != core::mem::size_of::<v1_disk_dqblk>() as ssize_t { if ret >= 0 { ret = -EIO as ssize_t; } } else {
        (*dqopt).info[type_ as usize].dqi_max_spc_limit = 0xffffffffu64 << QUOTABLOCK_BITS;
        (*dqopt).info[type_ as usize].dqi_max_ino_limit = 0xffffffff;
        (*dqopt).info[type_ as usize].dqi_igrace = if dqblk.dqb_itime != 0 { dqblk.dqb_itime } else { MAX_IQ_TIME };
        (*dqopt).info[type_ as usize].dqi_bgrace = if dqblk.dqb_btime != 0 { dqblk.dqb_btime } else { MAX_DQ_TIME };
        ret = 0;
    }
    memalloc_nofs_restore(memalloc); up_read(&mut (*dqopt).dqio_sem); ret as c_int
}

unsafe fn v1_write_file_info(sb: *mut super_block, type_: c_int) -> c_int {
    let dqopt = sb_dqopt(sb); let mut dqblk: v1_disk_dqblk = core::mem::zeroed();
    down_write(&mut (*dqopt).dqio_sem); let memalloc = memalloc_nofs_save();
    let mut ret = (*sb).s_op.as_ref().unwrap().quota_read.unwrap()(sb, type_, &mut dqblk as *mut _ as *mut c_char, core::mem::size_of::<v1_disk_dqblk>(), v1_dqoff(0));
    if ret == core::mem::size_of::<v1_disk_dqblk>() as ssize_t {
        spin_lock(&mut dq_data_lock); (*dqopt).info[type_ as usize].dqi_flags &= !DQF_INFO_DIRTY;
        dqblk.dqb_itime = (*dqopt).info[type_ as usize].dqi_igrace; dqblk.dqb_btime = (*dqopt).info[type_ as usize].dqi_bgrace; spin_unlock(&mut dq_data_lock);
        ret = (*sb).s_op.as_ref().unwrap().quota_write.unwrap()(sb, type_, &mut dqblk as *mut _ as *mut c_char, core::mem::size_of::<v1_disk_dqblk>(), v1_dqoff(0));
        if ret == core::mem::size_of::<v1_disk_dqblk>() as ssize_t { ret = 0; } else if ret >= 0 { ret = -EIO as ssize_t; }
    } else if ret >= 0 { ret = -EIO as ssize_t; }
    memalloc_nofs_restore(memalloc); up_write(&mut (*dqopt).dqio_sem); ret as c_int
}

static v1_format_ops: quota_format_ops = quota_format_ops {
    check_quota_file: Some(v1_check_quota_file), read_file_info: Some(v1_read_file_info),
    write_file_info: Some(v1_write_file_info), read_dqblk: Some(v1_read_dqblk),
    commit_dqblk: Some(v1_commit_dqblk),
};

static mut v1_quota_format: quota_format_type = quota_format_type {
    qf_fmt_id: QFMT_VFS_OLD, qf_ops: &v1_format_ops, qf_owner: THIS_MODULE,
};

unsafe fn init_v1_quota_format() -> c_int {
    register_quota_format(&mut v1_quota_format); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
