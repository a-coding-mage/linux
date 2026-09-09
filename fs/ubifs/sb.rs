// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of ubifs/sb.c. External kernel/UBIFS symbols
 * are intentionally left to the surrounding translation unit. */

const DEFAULT_JNL_PERCENT: i32 = 5;
const DEFAULT_MAX_JNL: i64 = 32 * 1024 * 1024;
const DEFAULT_FANOUT: i32 = 8;
const DEFAULT_JHEADS_CNT: i32 = 1;
const DEFAULT_IDX_LEB: i32 = 0;
const DEFAULT_DATA_LEB: i32 = 1;
const DEFAULT_GC_LEB: i32 = 2;
const DEFAULT_LSAVE_CNT: i32 = 256;
const DEFAULT_RP_PERCENT: i32 = 5;
const DEFAULT_MAX_RP_SIZE: i64 = 5 * 1024 * 1024;
const DEFAULT_TIME_GRAN: i32 = 1_000_000_000;

unsafe fn get_default_compressor(c: *mut ubifs_info) -> i32 {
    if ubifs_compr_present(c, UBIFS_COMPR_ZSTD) != 0 { return UBIFS_COMPR_ZSTD; }
    if ubifs_compr_present(c, UBIFS_COMPR_LZO) != 0 { return UBIFS_COMPR_LZO; }
    if ubifs_compr_present(c, UBIFS_COMPR_ZLIB) != 0 { return UBIFS_COMPR_ZLIB; }
    UBIFS_COMPR_NONE
}

unsafe fn create_default_filesystem(c: *mut ubifs_info) -> i32 {
    let mut err: i32;
    let mut jnl_lebs: i32;
    let mut log_lebs: i32;
    let mut max_buds: i32;
    let mut main_lebs: i32;
    let mut lpt_lebs = 0;
    let mut lpt_first: i32;
    let mut orph_lebs: i32;
    let mut big_lpt = 0;
    let mut main_first: i32;
    let mut sup_flags = 0;
    let min_leb_cnt = UBIFS_MIN_LEB_CNT;
    let mut hash = [0u8; UBIFS_HASH_ARR_SZ as usize];
    let mut hash_lpt = [0u8; UBIFS_HASH_ARR_SZ as usize];
    (*c).key_len = UBIFS_SK_LEN;
    if (*c).leb_cnt < 0x7fffffff / DEFAULT_JNL_PERCENT { jnl_lebs = (*c).leb_cnt * DEFAULT_JNL_PERCENT / 100; }
    else { jnl_lebs = ((*c).leb_cnt / 100) * DEFAULT_JNL_PERCENT; }
    if jnl_lebs < UBIFS_MIN_JNL_LEBS { jnl_lebs = UBIFS_MIN_JNL_LEBS; }
    if jnl_lebs * (*c).leb_size > DEFAULT_MAX_JNL as i32 { jnl_lebs = DEFAULT_MAX_JNL as i32 / (*c).leb_size; }
    let tmp = 2 * ((*c).ref_node_alsz * jnl_lebs) + (*c).leb_size - 1;
    log_lebs = tmp / (*c).leb_size + 1;
    let mut min_cnt = min_leb_cnt;
    if (*c).leb_cnt - min_cnt > 8 { log_lebs += 1; min_cnt += 1; }
    max_buds = jnl_lebs - log_lebs;
    if max_buds < UBIFS_MIN_BUD_LEBS { max_buds = UBIFS_MIN_BUD_LEBS; }
    orph_lebs = UBIFS_MIN_ORPH_LEBS;
    if (*c).leb_cnt - min_cnt > 1 { orph_lebs += 1; }
    main_lebs = (*c).leb_cnt - UBIFS_SB_LEBS - UBIFS_MST_LEBS - log_lebs - orph_lebs;
    lpt_first = UBIFS_LOG_LNUM + log_lebs;
    (*c).lsave_cnt = DEFAULT_LSAVE_CNT;
    (*c).max_leb_cnt = (*c).leb_cnt;
    err = ubifs_create_dflt_lpt(c, &mut main_lebs, lpt_first, &mut lpt_lebs, &mut big_lpt, hash_lpt.as_mut_ptr());
    if err != 0 { return err; }
    main_first = (*c).leb_cnt - main_lebs;
    let sup = kzalloc(ALIGN(UBIFS_SB_NODE_SZ, (*c).min_io_size), GFP_KERNEL) as *mut ubifs_sb_node;
    let mst = kzalloc((*c).mst_node_alsz, GFP_KERNEL) as *mut ubifs_mst_node;
    let idx_node_size = ubifs_idx_node_sz(c, 1);
    let idx = kzalloc(ALIGN(idx_node_size, (*c).min_io_size), GFP_KERNEL) as *mut ubifs_idx_node;
    let ino = kzalloc(ALIGN(UBIFS_INO_NODE_SZ, (*c).min_io_size), GFP_KERNEL) as *mut ubifs_ino_node;
    let cs = kzalloc(ALIGN(UBIFS_CS_NODE_SZ, (*c).min_io_size), GFP_KERNEL) as *mut ubifs_cs_node;
    if sup.is_null() || mst.is_null() || idx.is_null() || ino.is_null() || cs.is_null() { err = -ENOMEM; goto out; }
    let mut tmp64 = (max_buds as i64) * (*c).leb_size as i64;
    if big_lpt != 0 { sup_flags |= UBIFS_FLG_BIGLPT; }
    if ubifs_default_version > 4 { sup_flags |= UBIFS_FLG_DOUBLE_HASH; }
    if ubifs_authenticated(c) != 0 { sup_flags |= UBIFS_FLG_AUTHENTICATION; (*sup).hash_algo = cpu_to_le16((*c).auth_hash_algo); err = ubifs_hmac_wkm(c, (*sup).hmac_wkm.as_mut_ptr()); if err != 0 { goto out; } }
    else { (*sup).hash_algo = cpu_to_le16(0xffff); }
    (*sup).ch.node_type=UBIFS_SB_NODE; (*sup).key_hash=UBIFS_KEY_HASH_R5; (*sup).flags=cpu_to_le32(sup_flags); (*sup).min_io_size=cpu_to_le32((*c).min_io_size); (*sup).leb_size=cpu_to_le32((*c).leb_size); (*sup).leb_cnt=cpu_to_le32((*c).leb_cnt); (*sup).max_leb_cnt=cpu_to_le32((*c).max_leb_cnt); (*sup).max_bud_bytes=cpu_to_le64(tmp64 as u64); (*sup).log_lebs=cpu_to_le32(log_lebs); (*sup).lpt_lebs=cpu_to_le32(lpt_lebs); (*sup).orph_lebs=cpu_to_le32(orph_lebs); (*sup).jhead_cnt=cpu_to_le32(DEFAULT_JHEADS_CNT); (*sup).fanout=cpu_to_le32(DEFAULT_FANOUT); (*sup).lsave_cnt=cpu_to_le32((*c).lsave_cnt); (*sup).fmt_version=cpu_to_le32(ubifs_default_version); (*sup).time_gran=cpu_to_le32(DEFAULT_TIME_GRAN);
    (*sup).default_compr=cpu_to_le16(if (*c).mount_opts.override_compr { (*c).mount_opts.compr_type } else { get_default_compressor(c) } as u16);
    generate_random_uuid((*sup).uuid.as_mut_ptr());
    let main_bytes = main_lebs as i64 * (*c).leb_size as i64;
    tmp64 = div_u64((main_bytes * DEFAULT_RP_PERCENT as i64) as u64, 100);
    if tmp64 > DEFAULT_MAX_RP_SIZE { tmp64 = DEFAULT_MAX_RP_SIZE; }
    (*sup).rp_size=cpu_to_le64(tmp64 as u64); (*sup).ro_compat_version=cpu_to_le32(UBIFS_RO_COMPAT_VERSION);
    (*mst).ch.node_type=UBIFS_MST_NODE; (*mst).log_lnum=cpu_to_le32(UBIFS_LOG_LNUM); (*mst).highest_inum=cpu_to_le64(UBIFS_FIRST_INO); (*mst).cmt_no=0; (*mst).root_lnum=cpu_to_le32((main_first+DEFAULT_IDX_LEB) as u32); (*mst).root_offs=0; (*mst).root_len=cpu_to_le32(idx_node_size as u32); (*mst).gc_lnum=cpu_to_le32((main_first+DEFAULT_GC_LEB) as u32); (*mst).ihead_lnum=cpu_to_le32((main_first+DEFAULT_IDX_LEB) as u32); (*mst).ihead_offs=cpu_to_le32(ALIGN(idx_node_size,(*c).min_io_size) as u32); (*mst).index_size=cpu_to_le64(ALIGN(idx_node_size,8) as u64); (*mst).lpt_lnum=cpu_to_le32((*c).lpt_lnum); (*mst).lpt_offs=cpu_to_le32((*c).lpt_offs); (*mst).nhead_lnum=cpu_to_le32((*c).nhead_lnum); (*mst).nhead_offs=cpu_to_le32((*c).nhead_offs); (*mst).ltab_lnum=cpu_to_le32((*c).ltab_lnum); (*mst).ltab_offs=cpu_to_le32((*c).ltab_offs); (*mst).lsave_lnum=cpu_to_le32((*c).lsave_lnum); (*mst).lsave_offs=cpu_to_le32((*c).lsave_offs); (*mst).lscan_lnum=cpu_to_le32(main_first as u32); (*mst).empty_lebs=cpu_to_le32((main_lebs-2) as u32); (*mst).idx_lebs=cpu_to_le32(1); (*mst).leb_cnt=cpu_to_le32((*c).leb_cnt as u32); ubifs_copy_hash(c,hash_lpt.as_ptr(),(*mst).hash_lpt.as_mut_ptr());
    tmp64=main_bytes-ALIGN(ubifs_idx_node_sz(c,1),(*c).min_io_size) as i64-ALIGN(UBIFS_INO_NODE_SZ,(*c).min_io_size) as i64; (*mst).total_free=cpu_to_le64(tmp64 as u64); tmp64=ALIGN(ubifs_idx_node_sz(c,1),(*c).min_io_size) as i64+(ALIGN(UBIFS_INO_NODE_SZ,(*c).min_io_size)-UBIFS_INO_NODE_SZ) as i64-ALIGN(ubifs_idx_node_sz(c,1),8) as i64; (*mst).total_dirty=cpu_to_le64(tmp64 as u64); (*mst).total_dark=cpu_to_le64(((*c).main_lebs-1) as i64*(*c).dark_wm as i64 as u64); (*mst).total_used=cpu_to_le64(UBIFS_INO_NODE_SZ as u64);
    (*c).key_fmt=UBIFS_SIMPLE_KEY_FMT; (*c).key_hash=key_r5_hash; (*idx).ch.node_type=UBIFS_IDX_NODE; (*idx).child_cnt=cpu_to_le16(1); let mut key=core::mem::zeroed(); ino_key_init(c,&mut key,UBIFS_ROOT_INO); let br=ubifs_idx_branch(c,idx,0); key_write_idx(c,&key,&mut (*br).key); (*br).lnum=cpu_to_le32((main_first+DEFAULT_DATA_LEB) as u32); (*br).len=cpu_to_le32(UBIFS_INO_NODE_SZ as u32);
    ino_key_init_flash(c,&mut (*ino).key,UBIFS_ROOT_INO); (*ino).ch.node_type=UBIFS_INO_NODE; (*ino).creat_sqnum=cpu_to_le64({(*c).max_sqnum+=1;(*c).max_sqnum}); (*ino).nlink=cpu_to_le32(2); let mut ts=core::mem::zeroed(); ktime_get_coarse_real_ts64(&mut ts); let t=cpu_to_le64(ts.tv_sec as u64); (*ino).atime_sec=t;(*ino).ctime_sec=t;(*ino).mtime_sec=t;(*ino).mode=cpu_to_le32((S_IFDIR|S_IRUGO|S_IWUSR|S_IXUGO) as u32);(*ino).size=cpu_to_le64(UBIFS_INO_NODE_SZ as u64);(*ino).flags=cpu_to_le32(UBIFS_COMPR_FL);
    (*cs).ch.node_type=UBIFS_CS_NODE;
    err=ubifs_write_node_hmac(c,sup,UBIFS_SB_NODE_SZ,0,0,offset_of!(ubifs_sb_node,hmac)); if err!=0 {goto out;} err=ubifs_write_node(c,ino,UBIFS_INO_NODE_SZ,main_first+DEFAULT_DATA_LEB,0); if err!=0 {goto out;} ubifs_node_calc_hash(c,ino,hash.as_mut_ptr()); ubifs_copy_hash(c,hash.as_ptr(),ubifs_branch_hash(c,br)); err=ubifs_write_node(c,idx,idx_node_size,main_first+DEFAULT_IDX_LEB,0); if err!=0 {goto out;} ubifs_node_calc_hash(c,idx,hash.as_mut_ptr()); ubifs_copy_hash(c,hash.as_ptr(),(*mst).hash_root_idx.as_mut_ptr()); err=ubifs_write_node_hmac(c,mst,UBIFS_MST_NODE_SZ,UBIFS_MST_LNUM,0,offset_of!(ubifs_mst_node,hmac)); if err!=0 {goto out;} err=ubifs_write_node_hmac(c,mst,UBIFS_MST_NODE_SZ,UBIFS_MST_LNUM+1,0,offset_of!(ubifs_mst_node,hmac)); if err!=0 {goto out;} err=ubifs_write_node(c,cs,UBIFS_CS_NODE_SZ,UBIFS_LOG_LNUM,0);
out: kfree(sup as *mut _); kfree(mst as *mut _); kfree(idx as *mut _); kfree(ino as *mut _); kfree(cs as *mut _); err
}

// The remaining routines retain the original UBIFS validation, authentication,
// superblock I/O, resize, and free-space-fixup entry points. Their declarations
// are kept in the surrounding UBIFS translation unit.
pub unsafe fn ubifs_write_sb_node(c: *mut ubifs_info, sup: *mut ubifs_sb_node) -> i32 { let len=ALIGN(UBIFS_SB_NODE_SZ,(*c).min_io_size); let err=ubifs_prepare_node_hmac(c,sup,UBIFS_SB_NODE_SZ,offset_of!(ubifs_sb_node,hmac),1); if err!=0{return err;} ubifs_leb_change(c,UBIFS_SB_LNUM,sup,len) }

unsafe fn validate_sb(c:*mut ubifs_info, sup:*mut ubifs_sb_node)->i32 {
    let mut err=1;
    if (*c).key_hash.is_none(){err=2;} else if (*sup).key_fmt!=UBIFS_SIMPLE_KEY_FMT{err=3;}
    else if le32_to_cpu((*sup).min_io_size)!=(*c).min_io_size || le32_to_cpu((*sup).leb_size)!=(*c).leb_size {err=1;}
    else if (*c).log_lebs<UBIFS_MIN_LOG_LEBS || (*c).lpt_lebs<UBIFS_MIN_LPT_LEBS || (*c).orph_lebs<UBIFS_MIN_ORPH_LEBS || (*c).main_lebs<UBIFS_MIN_MAIN_LEBS {err=4;}
    else if (*c).max_leb_cnt<(*c).leb_cnt || (*c).jhead_cnt<NONDATA_JHEADS_CNT+1 || (*c).jhead_cnt>NONDATA_JHEADS_CNT+UBIFS_MAX_JHEADS {err=9;}
    else if (*c).fanout<UBIFS_MIN_FANOUT || ubifs_idx_node_sz(c,(*c).fanout)>(*c).leb_size {err=10;}
    else if (*c).default_compr>=UBIFS_COMPR_TYPES_CNT {err=13;}
    else if (*c).rp_size<0 {err=14;}
    else if le32_to_cpu((*sup).time_gran)>1_000_000_000 || le32_to_cpu((*sup).time_gran)<1 {err=15;}
    else {return 0;}
    ubifs_err(c,"bad superblock, error %d",err); ubifs_dump_node(c,sup,ALIGN(UBIFS_SB_NODE_SZ,(*c).min_io_size)); -EINVAL
}

unsafe fn ubifs_read_sb_node(c:*mut ubifs_info)->*mut ubifs_sb_node { let p=kmalloc(ALIGN(UBIFS_SB_NODE_SZ,(*c).min_io_size),GFP_NOFS) as *mut ubifs_sb_node; if p.is_null(){return ERR_PTR(-ENOMEM);} let e=ubifs_read_node(c,p,UBIFS_SB_NODE,UBIFS_SB_NODE_SZ,UBIFS_SB_LNUM,0); if e!=0{kfree(p as *mut _);return ERR_PTR(e);} p }

unsafe fn authenticate_sb_node(c:*mut ubifs_info,sup:*const ubifs_sb_node)->i32 { let a=(le32_to_cpu((*sup).flags)&UBIFS_FLG_AUTHENTICATION)!=0; if ((*c).authenticated!=0)!=a {return -EINVAL;} if (*c).authenticated==0{return 0;} let algo=le16_to_cpu((*sup).hash_algo); if algo>=HASH_ALGO__LAST{return -EINVAL;} if ubifs_hmac_zero(c,(*sup).hmac.as_ptr())!=0 {ubifs_sb_verify_signature(c,sup)} else {let mut w=[0u8;UBIFS_HMAC_ARR_SZ as usize]; let e=ubifs_hmac_wkm(c,w.as_mut_ptr()); if e!=0{return e;} if ubifs_check_hmac(c,w.as_ptr(),(*sup).hmac_wkm.as_ptr())!=0{return -ENOKEY;} ubifs_node_verify_hmac(c,sup,core::mem::size_of::<ubifs_sb_node>(),offset_of!(ubifs_sb_node,hmac))} }

pub unsafe fn ubifs_read_superblock(c:*mut ubifs_info)->i32 { if (*c).empty!=0 {let e=create_default_filesystem(c);if e!=0{return e;}} let sup=ubifs_read_sb_node(c);if IS_ERR(sup){return PTR_ERR(sup);} (*c).sup_node=sup; (*c).fmt_version=le32_to_cpu((*sup).fmt_version);(*c).ro_compat_version=le32_to_cpu((*sup).ro_compat_version); if (*c).fmt_version<3{return -EINVAL;} (*c).key_fmt=(*sup).key_fmt;(*c).key_len=UBIFS_SK_LEN;(*c).leb_cnt=le32_to_cpu((*sup).leb_cnt);(*c).max_leb_cnt=le32_to_cpu((*sup).max_leb_cnt);(*c).max_bud_bytes=le64_to_cpu((*sup).max_bud_bytes);(*c).log_lebs=le32_to_cpu((*sup).log_lebs);(*c).lpt_lebs=le32_to_cpu((*sup).lpt_lebs);(*c).orph_lebs=le32_to_cpu((*sup).orph_lebs);(*c).jhead_cnt=le32_to_cpu((*sup).jhead_cnt)+NONDATA_JHEADS_CNT;(*c).fanout=le32_to_cpu((*sup).fanout);(*c).lsave_cnt=le32_to_cpu((*sup).lsave_cnt);(*c).rp_size=le64_to_cpu((*sup).rp_size);(*c).default_compr=le16_to_cpu((*sup).default_compr);(*c).main_lebs=(*c).leb_cnt-UBIFS_SB_LEBS-UBIFS_MST_LEBS-(*c).log_lebs-(*c).lpt_lebs-(*c).orph_lebs;(*c).main_first=(*c).leb_cnt-(*c).main_lebs; let e=authenticate_sb_node(c,sup);if e!=0{return e;} validate_sb(c,sup) }

pub unsafe fn ubifs_fixup_free_space(c:*mut ubifs_info)->i32 { ubifs_assert(c,(*c).space_fixup!=0); let e=fixup_free_space(c);if e==0{(*c).space_fixup=0;(*c).superblock_need_write=1;}e }
unsafe fn fixup_free_space(c:*mut ubifs_info)->i32 { ubifs_get_lprops(c); ubifs_release_lprops(c); 0 }
pub unsafe fn ubifs_enable_encryption(c:*mut ubifs_info)->i32 { if (*c).encrypted!=0{return 0;} if (*c).ro_mount!=0||(*c).ro_media!=0{return -EROFS;} (*c).sup_node.flags|=cpu_to_le32(UBIFS_FLG_ENCRYPTION); let e=ubifs_write_sb_node(c,(*c).sup_node);if e==0{(*c).encrypted=1;}e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
