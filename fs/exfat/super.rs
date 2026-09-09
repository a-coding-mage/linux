// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2012-2013 Samsung Electronics Co., Ltd. */

// Linux and exFAT dependencies are supplied by the surrounding translation.

static mut EXFAT_DEFAULT_IOCHARSET: [u8; 5] = *b"utf8\0";
static mut exfat_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn exfat_free_iocharset(sbi: *mut exfat_sb_info) {
    if (*sbi).options.iocharset != EXFAT_DEFAULT_IOCHARSET.as_mut_ptr() as *mut i8 { kfree((*sbi).options.iocharset as *mut _); }
}
unsafe fn exfat_set_iocharset(opts: *mut exfat_mount_options, iocharset: *mut i8) {
    (*opts).iocharset = iocharset;
    (*opts).utf8 = if strcmp(iocharset, b"utf8\0".as_ptr() as *const i8) == 0 { 1 } else { 0 };
}
unsafe fn exfat_put_super(sb: *mut super_block) { let sbi = EXFAT_SB(sb); mutex_lock(&mut (*sbi).s_lock); exfat_clear_volume_dirty(sb); exfat_free_bitmap(sbi); brelse((*sbi).boot_bh); mutex_unlock(&mut (*sbi).s_lock); }
unsafe fn exfat_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 { let sb=(*dentry).d_sb; let sbi=EXFAT_SB(sb); let id=huge_encode_dev((*sb).s_bdev.as_ref().unwrap().bd_dev); (*buf).f_type=(*sb).s_magic; (*buf).f_bsize=(*sbi).cluster_size; (*buf).f_blocks=(*sbi).num_clusters-2; (*buf).f_bfree=(*buf).f_blocks-(*sbi).used_clusters; (*buf).f_bavail=(*buf).f_bfree; (*buf).f_fsid=u64_to_fsid(id); (*buf).f_namelen=EXFAT_MAX_FILE_LEN*NLS_MAX_CHARSET_SIZE; 0 }
unsafe fn exfat_set_vol_flags(sb:*mut super_block, mut new_flags:u16)->i32 { let sbi=EXFAT_SB(sb); let boot=(*sbi).boot_bh; let p_boot=(*boot).b_data as *mut boot_sector; new_flags|=(*sbi).vol_flags_persistent; if (*sbi).vol_flags==new_flags{return 0;} (*sbi).vol_flags=new_flags; if sb_rdonly(sb)!=0{return 0;} (*p_boot).vol_flags=cpu_to_le16(new_flags); set_buffer_uptodate(boot); mark_buffer_dirty(boot); __sync_dirty_buffer(boot,REQ_SYNC|REQ_FUA|REQ_PREFLUSH); 0 }
pub unsafe fn exfat_set_volume_dirty(sb:*mut super_block)->i32 { let sbi=EXFAT_SB(sb); exfat_set_vol_flags(sb,(*sbi).vol_flags|VOLUME_DIRTY) }
pub unsafe fn exfat_clear_volume_dirty(sb:*mut super_block)->i32 { let sbi=EXFAT_SB(sb); exfat_set_vol_flags(sb,(*sbi).vol_flags&!VOLUME_DIRTY) }

unsafe fn exfat_show_options(m:*mut seq_file, root:*mut dentry)->i32 { let sb=(*root).d_sb; let sbi=EXFAT_SB(sb); let o=&mut (*sbi).options; if !uid_eq(o.fs_uid,GLOBAL_ROOT_UID){seq_printf(m,b",uid=%u\0".as_ptr() as *const i8,from_kuid_munged(&init_user_ns,o.fs_uid));} if !gid_eq(o.fs_gid,GLOBAL_ROOT_GID){seq_printf(m,b",gid=%u\0".as_ptr() as *const i8,from_kgid_munged(&init_user_ns,o.fs_gid));} seq_printf(m,b",fmask=%04o,dmask=%04o\0".as_ptr() as *const i8,o.fs_fmask,o.fs_dmask); if o.allow_utime!=0{seq_printf(m,b",allow_utime=%04o\0".as_ptr() as *const i8,o.allow_utime);} if o.utf8!=0{seq_puts(m,b",iocharset=utf8\0".as_ptr() as *const i8);} else if !(*sbi).nls_io.is_null(){seq_printf(m,b",iocharset=%s\0".as_ptr() as *const i8,(*(*sbi).nls_io).charset);} if o.errors==EXFAT_ERRORS_CONT{seq_puts(m,b",errors=continue\0".as_ptr() as *const i8);} else if o.errors==EXFAT_ERRORS_PANIC{seq_puts(m,b",errors=panic\0".as_ptr() as *const i8);} else{seq_puts(m,b",errors=remount-ro\0".as_ptr() as *const i8);} if o.discard!=0{seq_puts(m,b",discard\0".as_ptr() as *const i8);} if o.keep_last_dots!=0{seq_puts(m,b",keep_last_dots\0".as_ptr() as *const i8);} if o.sys_tz!=0{seq_puts(m,b",sys_tz\0".as_ptr() as *const i8);} else if o.time_offset!=0{seq_printf(m,b",time_offset=%d\0".as_ptr() as *const i8,o.time_offset);} if o.zero_size_dir!=0{seq_puts(m,b",zero_size_dir\0".as_ptr() as *const i8);} 0 }

pub unsafe fn exfat_force_shutdown(sb:*mut super_block, flags:u32)->i32 { let sbi=(*sb).s_fs_info as *mut exfat_sb_info; let o=&mut (*sbi).options; if exfat_forced_shutdown(sb)!=0{return 0;} match flags { EXFAT_GOING_DOWN_DEFAULT|EXFAT_GOING_DOWN_FULLSYNC=>{let r=bdev_freeze((*sb).s_bdev);if r!=0{return r;}bdev_thaw((*sb).s_bdev);set_bit(EXFAT_FLAGS_SHUTDOWN,&mut (*sbi).s_exfat_flags);}, EXFAT_GOING_DOWN_NOSYNC=>{set_bit(EXFAT_FLAGS_SHUTDOWN,&mut (*sbi).s_exfat_flags);}, _=>return -EINVAL } o.discard=0; 0 }
unsafe fn exfat_shutdown(sb:*mut super_block){exfat_force_shutdown(sb,EXFAT_GOING_DOWN_NOSYNC);}
unsafe fn exfat_alloc_inode(sb:*mut super_block)->*mut inode { let ei=alloc_inode_sb(sb,exfat_inode_cachep,GFP_NOFS);if ei.is_null(){core::ptr::null_mut()}else{&mut (*ei).vfs_inode} }
unsafe fn exfat_free_inode(i:*mut inode){kmem_cache_free(exfat_inode_cachep,EXFAT_I(i));}

#[repr(C)]
static mut exfat_sops: super_operations = super_operations { alloc_inode:Some(exfat_alloc_inode), free_inode:Some(exfat_free_inode), write_inode:Some(exfat_write_inode), evict_inode:Some(exfat_evict_inode), put_super:Some(exfat_put_super), statfs:Some(exfat_statfs), show_options:Some(exfat_show_options), shutdown:Some(exfat_shutdown) };

#[repr(i32)] enum ExfatOpt { OptUid,OptGid,OptUmask,OptDmask,OptFmask,OptAllowUtime,OptCharset,OptErrors,OptDiscard,OptKeepLastDots,OptSysTz,OptTimeOffset,OptZeroSizeDir,OptUtf8,OptDebug,OptNamecase,OptCodepage }

unsafe fn exfat_parse_param(fc:*mut fs_context,param:*mut fs_parameter)->i32 { let sbi=(*fc).s_fs_info as *mut exfat_sb_info; let o=&mut (*sbi).options; let mut r=core::mem::zeroed(); let opt=fs_parse(fc,exfat_parameters,param,&mut r); if opt<0{return opt;} match opt { x if x==OptUid as i32=>o.fs_uid=r.uid,x if x==OptGid as i32=>o.fs_gid=r.gid,x if x==OptUmask as i32=>{o.fs_fmask=r.uint_32;o.fs_dmask=r.uint_32},x if x==OptDmask as i32=>o.fs_dmask=r.uint_32,x if x==OptFmask as i32=>o.fs_fmask=r.uint_32,x if x==OptAllowUtime as i32=>o.allow_utime=r.uint_32&0o22,x if x==OptCharset as i32=>{exfat_free_iocharset(sbi);exfat_set_iocharset(o,(*param).string);(*param).string=core::ptr::null_mut()},x if x==OptErrors as i32=>o.errors=r.uint_32,x if x==OptDiscard as i32=>o.discard=(!r.negated) as _,x if x==OptKeepLastDots as i32=>o.keep_last_dots=1,x if x==OptSysTz as i32=>o.sys_tz=1,x if x==OptTimeOffset as i32=>{if r.int_32 < -1440 || r.int_32 > 1440{return -EINVAL;}o.time_offset=r.int_32},x if x==OptZeroSizeDir as i32=>o.zero_size_dir=(!r.negated) as _,x if x==OptUtf8 as i32||x==OptDebug as i32||x==OptNamecase as i32||x==OptCodepage as i32=>{},_=>return -EINVAL} 0 }

unsafe fn exfat_hash_init(sb:*mut super_block){let s=EXFAT_SB(sb);spin_lock_init(&mut (*s).inode_hash_lock);for i in 0..EXFAT_HASH_SIZE{INIT_HLIST_HEAD(&mut (*s).inode_hashtable[i]);}}
unsafe fn exfat_read_root(inode:*mut inode,root:*mut exfat_chain)->i32 {let sb=(*inode).i_sb;let s=EXFAT_SB(sb);let e=EXFAT_I(inode);exfat_chain_set(&mut (*e).dir,(*s).root_dir,0,ALLOC_FAT_CHAIN);(*e).entry=-1;(*e).start_clu=(*s).root_dir;(*e).flags=ALLOC_FAT_CHAIN;(*e).type_=TYPE_DIR;(*e).version=0;(*e).hint_bmap.off=EXFAT_EOF_CLUSTER;(*e).hint_stat.eidx=0;(*e).hint_stat.clu=(*s).root_dir;(*e).hint_femp.eidx=EXFAT_HINT_NONE;i_size_write(inode,exfat_cluster_to_bytes(s,(*root).size));let n=exfat_count_dir_entries(sb,root);if n<0{return -EIO;}set_nlink(inode,n+EXFAT_MIN_SUBDIR);(*inode).i_uid=(*s).options.fs_uid;(*inode).i_gid=(*s).options.fs_gid;inode_inc_iversion(inode);(*inode).i_generation=0;(*inode).i_mode=exfat_make_mode(s,EXFAT_ATTR_SUBDIR,0o777);(*inode).i_op=&exfat_dir_inode_operations;(*inode).i_fop=&exfat_dir_operations;(*inode).i_blocks=round_up(i_size_read(inode),(*s).cluster_size)>>9;(*e).i_pos=(((*s).root_dir as i64)<<32)|0xffff_ffff;exfat_save_attr(inode,EXFAT_ATTR_SUBDIR);(*e).i_crtime=simple_inode_init_ts(inode);exfat_truncate_inode_atime(inode);0}

extern "C" { static exfat_parameters: fs_parameter_spec; fn exfat_fill_super(sb:*mut super_block,fc:*mut fs_context)->i32; fn exfat_init_fs_context(fc:*mut fs_context)->i32; fn exfat_kill_sb(sb:*mut super_block); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
