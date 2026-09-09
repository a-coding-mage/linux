// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of jfs/super.c.  Kernel declarations are
 * supplied by the surrounding JFS/Linux compatibility environment. */

const MAX_COMMIT_THREADS: usize = 64;
static mut jfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut commit_threads: i32 = 0;
static mut jfsCommitThread: [*mut task_struct; MAX_COMMIT_THREADS] = [core::ptr::null_mut(); MAX_COMMIT_THREADS];
pub static mut jfsIOthread: *mut task_struct = core::ptr::null_mut();
pub static mut jfsSyncThread: *mut task_struct = core::ptr::null_mut();

#[cfg(CONFIG_JFS_DEBUG)]
pub static mut jfsloglevel: i32 = JFS_LOGLEVEL_WARN;

unsafe fn jfs_handle_error(sb: *mut super_block) {
    let sbi = JFS_SBI(sb);
    if sb_rdonly(sb) { return; }
    updateSuper(sb, FM_DIRTY);
    if (*sbi).flag & JFS_ERR_PANIC != 0 {
        panic!("JFS (device %s): panic forced after error\\n", (*sb).s_id);
    } else if (*sbi).flag & JFS_ERR_REMOUNT_RO != 0 {
        jfs_err!("ERROR: (device %s): remounting filesystem as read-only", (*sb).s_id);
        (*sb).s_flags |= SB_RDONLY;
    }
}

pub unsafe fn jfs_error(sb: *mut super_block, fmt: *const core::ffi::c_char, mut args: ...) {
    let mut vaf: va_format = core::mem::zeroed();
    vaf.fmt = fmt; vaf.va = &mut args;
    pr_err!("ERROR: (device %s): %ps: %pV\\n", (*sb).s_id, builtin_return_address(0), &vaf);
    jfs_handle_error(sb);
}

unsafe fn jfs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let p = alloc_inode_sb(sb, jfs_inode_cachep, GFP_NOFS);
    if p.is_null() { return core::ptr::null_mut(); }
    #[cfg(CONFIG_QUOTA)] { core::ptr::write_bytes((*p).i_dquot.as_mut_ptr(), 0, (*p).i_dquot.len()); }
    &mut (*p).vfs_inode
}
unsafe fn jfs_free_inode(inode: *mut inode) { kmem_cache_free(jfs_inode_cachep, JFS_IP(inode)); }

unsafe fn jfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let sbi = JFS_SBI((*dentry).d_sb); let imap = JFS_IP((*sbi).ipimap).i_imap;
    jfs_info!("In jfs_statfs");
    (*buf).f_type=JFS_SUPER_MAGIC; (*buf).f_bsize=(*sbi).bsize;
    (*buf).f_blocks=(*sbi).bmap.db_mapsize; (*buf).f_bfree=(*sbi).bmap.db_nfree;
    (*buf).f_bavail=(*sbi).bmap.db_nfree;
    let maxinodes = core::cmp::min(atomic_read(&(*imap).im_numinos) as i64 +
        (((*sbi).bmap.db_nfree >> (*imap).im_l2nbperiext) << L2INOSPEREXT) as i64, 0xffff_ffff);
    (*buf).f_files=maxinodes; (*buf).f_ffree=maxinodes-(atomic_read(&(*imap).im_numinos)-atomic_read(&(*imap).im_numfree)) as i64;
    (*buf).f_fsid.val[0]=crc32_le(0, &(*sbi).uuid as *const _ as *const i8, core::mem::size_of_val(&(*sbi).uuid)/2);
    (*buf).f_fsid.val[1]=crc32_le(0, (&(*sbi).uuid as *const _ as *const u8).add(core::mem::size_of_val(&(*sbi).uuid)/2) as *const i8, core::mem::size_of_val(&(*sbi).uuid)/2);
    (*buf).f_namelen=JFS_NAME_MAX; 0
}

#[repr(C)] pub struct jfs_context { pub flag:i32, pub uid:kuid_t, pub gid:kgid_t, pub umask:u32, pub minblks_trim:u32, pub nls_map:*mut core::ffi::c_void, pub resize:bool, pub newLVSize:i64 }

unsafe fn jfs_put_super(sb:*mut super_block) { let sbi=JFS_SBI(sb); jfs_info!("In jfs_put_super"); jfs_quota_off_umount(sb); let rc=jfs_umount(sb); if rc!=0 { jfs_err!("jfs_umount failed with return code %d",rc); } unload_nls((*sbi).nls_tab); truncate_inode_pages((*sbi).direct_inode.i_mapping,0); iput((*sbi).direct_inode); kfree(sbi); }

enum { Opt_integrity, Opt_nointegrity, Opt_iocharset, Opt_resize, Opt_resize_nosize, Opt_errors, Opt_ignore, Opt_err, Opt_quota, Opt_usrquota, Opt_grpquota, Opt_uid, Opt_gid, Opt_umask, Opt_discard, Opt_nodiscard, Opt_discard_minblk }

unsafe fn jfs_quota_off_umount(sb:*mut super_block) { for t in 0..MAXQUOTAS { jfs_quota_off(sb,t); } }
unsafe fn jfs_quota_off(sb:*mut super_block, typ:i32)->i32 { dquot_quota_off(sb,typ) }
unsafe fn jfs_quota_on(sb:*mut super_block, typ:i32, format_id:i32, path:*const path)->i32 { dquot_quota_on(sb,typ,format_id,path) }

unsafe fn jfs_parse_param(fc:*mut fs_context, param:*mut fs_parameter)->i32 {
    let ctx=(*fc).fs_private as *mut jfs_context; let result=fs_parse(fc,param); if result.opt<0{return result.opt;}
    match result.opt { Opt_integrity=>if result.negated{(*ctx).flag|=JFS_NOINTEGRITY}else{(*ctx).flag&=!JFS_NOINTEGRITY}, Opt_ignore=>{}, Opt_resize|Opt_resize_nosize=>{if (*fc).purpose!=FS_CONTEXT_FOR_RECONFIGURE{return -EINVAL;} (*ctx).resize=true; (*ctx).newLVSize=result.uint_64;}, Opt_errors=>{(*ctx).flag=((*ctx).flag&!JFS_ERR_MASK)|result.uint_32 as i32}, Opt_uid=>(*ctx).uid=result.uid, Opt_gid=>(*ctx).gid=result.gid, Opt_umask=>{if result.uint_32&!0o777!=0{return -EINVAL;}(*ctx).umask=result.uint_32}, Opt_discard=>{(*ctx).minblks_trim=64;(*ctx).flag|=JFS_DISCARD}, Opt_nodiscard=>(*ctx).flag&=!JFS_DISCARD, Opt_discard_minblk=>{(*ctx).minblks_trim=result.uint_32;(*ctx).flag|=JFS_DISCARD}, _=>return -EINVAL } 0
}
unsafe fn jfs_reconfigure(fc:*mut fs_context)->i32 { let ctx=(*fc).fs_private as *mut jfs_context; let sb=(*(*fc).root).d_sb; sync_filesystem(sb); let sbi=JFS_SBI(sb); (*sbi).flag=(*ctx).flag; (*sbi).uid=(*ctx).uid; (*sbi).gid=(*ctx).gid; (*sbi).umask=(*ctx).umask; if (*ctx).resize { if sb_rdonly(sb){return -EROFS;} return jfs_extendfs(sb,(*ctx).newLVSize,0); } 0 }
unsafe fn jfs_show_options(seq:*mut seq_file, root:*mut dentry)->i32 { let sbi=JFS_SBI((*root).d_sb); if uid_valid((*sbi).uid){seq_printf(seq,cstr!(",uid=%d"),from_kuid(&init_user_ns,(*sbi).uid));} if gid_valid((*sbi).gid){seq_printf(seq,cstr!(",gid=%d"),from_kgid(&init_user_ns,(*sbi).gid));} if (*sbi).flag&JFS_NOINTEGRITY!=0{seq_puts(seq,cstr!(",nointegrity"));} if (*sbi).flag&JFS_DISCARD!=0{seq_printf(seq,cstr!(",discard=%u"),(*sbi).minblks_trim);} 0 }
unsafe fn jfs_init_options(fc:*mut fs_context,ctx:*mut jfs_context){(*ctx).flag=JFS_ERR_REMOUNT_RO;(*ctx).uid=INVALID_UID;(*ctx).gid=INVALID_GID;(*ctx).umask=!0;(*ctx).nls_map=(-1isize) as *mut _; if (*fc).purpose==FS_CONTEXT_FOR_RECONFIGURE{let s=JFS_SBI((*(*fc).root).d_sb);(*ctx).flag=(*s).flag;(*ctx).uid=(*s).uid;(*ctx).gid=(*s).gid;(*ctx).umask=(*s).umask;}}
unsafe fn jfs_free_fc(fc:*mut fs_context){let ctx=(*fc).fs_private as *mut jfs_context;if (*ctx).nls_map!=(-1isize as *mut _){unload_nls((*ctx).nls_map);}kfree(ctx);}
unsafe fn jfs_init_fs_context(fc:*mut fs_context)->i32{let ctx=kzalloc_obj::<jfs_context>();if ctx.is_null(){return -ENOMEM;}jfs_init_options(fc,ctx);(*fc).fs_private=ctx as *mut _;0}

unsafe fn jfs_freeze(sb:*mut super_block)->i32 { let sbi=JFS_SBI(sb); if !sb_rdonly(sb) { txQuiesce(sb); let rc=lmLogShutdown((*sbi).log); if rc!=0 { jfs_error(sb,cstr!("lmLogShutdown failed\\n")); txResume(sb); return rc; } let _=updateSuper(sb,FM_CLEAN); } 0 }
unsafe fn jfs_unfreeze(sb:*mut super_block)->i32 { let sbi=JFS_SBI(sb); if !sb_rdonly(sb) { let mut rc=updateSuper(sb,FM_MOUNT); if rc==0 { rc=lmLogInit((*sbi).log); } if rc!=0 { jfs_error(sb,cstr!("updateSuper failed\\n")); } txResume(sb); return rc; } 0 }
unsafe fn jfs_get_tree(fc:*mut fs_context)->i32 { get_tree_bdev(fc,jfs_fill_super) }
unsafe fn jfs_sync_fs(sb:*mut super_block, wait:i32)->i32 { let log=JFS_SBI(sb).log; if !log.is_null() { dquot_writeback_dquots(sb,-1); jfs_flush_journal(log,wait); jfs_syncpt(log,0); } 0 }

unsafe fn jfs_fill_super(sb:*mut super_block, fc:*mut fs_context)->i32 { let ctx=(*fc).fs_private as *mut jfs_context; let sbi=kzalloc_obj::<jfs_sb_info>(); if sbi.is_null(){return -ENOMEM;} (*sb).s_fs_info=sbi; (*sb).s_max_links=JFS_LINK_MAX; (*sbi).sb=sb; (*sbi).flag=(*ctx).flag; (*sbi).uid=(*ctx).uid; (*sbi).gid=(*ctx).gid; (*sbi).umask=(*ctx).umask; (*ctx).nls_map=core::ptr::null_mut(); if !sb_set_blocksize(sb,PSIZE){kfree(sbi);return -EINVAL;} (*sb).s_op=&jfs_super_operations; (*sb).s_export_op=&jfs_export_operations; let inode=new_inode(sb); if inode.is_null(){kfree(sbi);return -ENOMEM;} (*sbi).direct_inode=inode; let rc=jfs_mount(sb); if rc!=0 {iput(inode);kfree(sbi);return rc;} (*sb).s_magic=JFS_SUPER_MAGIC; let root=jfs_iget(sb,ROOT_I); if IS_ERR(root){jfs_umount(sb);iput(inode);kfree(sbi);return PTR_ERR(root);} (*sb).s_root=d_make_root(root); if (*sb).s_root.is_null(){jfs_umount(sb);iput(inode);kfree(sbi);return -EINVAL;} (*sb).s_maxbytes=core::cmp::min(((*sb).s_blocksize as i64)<<40,MAX_LFS_FILESIZE); (*sb).s_time_gran=1; 0 }

static mut jfs_super_operations: super_operations = super_operations { alloc_inode:Some(jfs_alloc_inode), free_inode:Some(jfs_free_inode), put_super:Some(jfs_put_super), sync_fs:Some(jfs_sync_fs), freeze_fs:Some(jfs_freeze), unfreeze_fs:Some(jfs_unfreeze), statfs:Some(jfs_statfs), ..super_operations::EMPTY };
static mut jfs_export_operations: export_operations = export_operations::EMPTY;

unsafe fn init_once(foo:*mut core::ffi::c_void) { let p=foo as *mut jfs_inode_info; core::ptr::write_bytes(p,0,1); INIT_LIST_HEAD(&mut (*p).anon_inode_list); init_rwsem(&mut (*p).rdwrlock); mutex_init(&mut (*p).commit_mutex); init_rwsem(&mut (*p).xattr_sem); spin_lock_init(&mut (*p).ag_lock); (*p).active_ag=-1; inode_init_once(&mut (*p).vfs_inode); }
unsafe fn init_jfs_fs()->i32 { let mut rc=metapage_init(); if rc!=0{return rc;} rc=txInit(); if rc!=0{metapage_exit();return rc;} jfsIOthread=kthread_run(jfsIOWait,core::ptr::null_mut(),cstr!("jfsIO")); if IS_ERR(jfsIOthread){rc=PTR_ERR(jfsIOthread);txExit();metapage_exit();return rc;} if commit_threads<1{commit_threads=num_online_cpus();} if commit_threads>MAX_COMMIT_THREADS as i32{commit_threads=MAX_COMMIT_THREADS as i32;} for i in 0..commit_threads as usize {jfsCommitThread[i]=kthread_run(jfs_lazycommit,core::ptr::null_mut(),cstr!("jfsCommit"));} jfsSyncThread=kthread_run(jfs_sync,core::ptr::null_mut(),cstr!("jfsSync")); rc=register_filesystem(&mut jfs_fs_type); if rc!=0{txExit();metapage_exit();} rc }
unsafe fn exit_jfs_fs(){ txExit(); metapage_exit(); kthread_stop(jfsIOthread); for i in 0..commit_threads as usize{kthread_stop(jfsCommitThread[i]);} kthread_stop(jfsSyncThread); unregister_filesystem(&mut jfs_fs_type); rcu_barrier(); kmem_cache_destroy(jfs_inode_cachep); }
static mut jfs_fs_type:file_system_type=file_system_type::EMPTY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
