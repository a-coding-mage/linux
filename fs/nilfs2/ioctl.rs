// SPDX-License-Identifier: GPL-2.0+
/* NILFS ioctl operations.  Direct low-level translation of ioctl.c. */

/* Kernel and NILFS dependencies are supplied by the surrounding translation unit. */

unsafe fn nilfs_ioctl_wrap_copy(
    nilfs: *mut the_nilfs, argv: *mut nilfs_argv, dir: c_int,
    dofunc: unsafe extern "C" fn(*mut the_nilfs, *mut u64, c_int, *mut c_void, usize, usize) -> isize,
) -> c_int {
    if (*argv).v_nmembs == 0 { return 0; }
    if (*argv).v_size as usize > PAGE_SIZE { return -EINVAL; }
    if (*argv).v_index > (!0u64).wrapping_sub((*argv).v_nmembs as u64) { return -EINVAL; }
    let buf = kzalloc(PAGE_SIZE, GFP_NOFS);
    if buf.is_null() { return -ENOMEM; }
    let maxmembs = PAGE_SIZE / (*argv).v_size as usize;
    let mut ret = 0;
    let mut total = 0usize;
    let mut pos = (*argv).v_index;
    let base = u64_to_user_ptr((*argv).v_base) as *mut u8;
    let mut i = 0usize;
    while i < (*argv).v_nmembs as usize {
        let n = std::cmp::min((*argv).v_nmembs as usize - i, maxmembs);
        if (dir & _IOC_WRITE) != 0 && copy_from_user(buf, base.add((*argv).v_size as usize * i) as *const c_void, (*argv).v_size as usize * n) != 0 { ret = -EFAULT; break; }
        let ppos = pos;
        let nr = dofunc(nilfs, &mut pos, (*argv).v_flags, buf, (*argv).v_size as usize, n);
        if nr < 0 { ret = nr as c_int; break; }
        if (dir & _IOC_READ) != 0 && copy_to_user(base.add((*argv).v_size as usize * i) as *mut c_void, buf, (*argv).v_size as usize * nr as usize) != 0 { ret = -EFAULT; break; }
        total += nr as usize;
        if nr as usize < n { break; }
        if pos == ppos { pos = pos.wrapping_add(n as u64); }
        i += n;
    }
    (*argv).v_nmembs = total as _;
    kfree(buf);
    ret
}

pub unsafe extern "C" fn nilfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let inode = d_inode(dentry);
    fileattr_fill_flags(fa, (*NILFS_I(inode)).i_flags & FS_FL_USER_VISIBLE);
    0
}

pub unsafe extern "C" fn nilfs_fileattr_set(_idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let inode = d_inode(dentry);
    if fileattr_has_fsx(fa) { return -EOPNOTSUPP; }
    let flags = nilfs_mask_flags((*inode).i_mode, (*fa).flags);
    let mut ti = std::mem::zeroed();
    let ret = nilfs_transaction_begin((*inode).i_sb, &mut ti, 0);
    if ret != 0 { return ret; }
    let oldflags = (*NILFS_I(inode)).i_flags & !FS_FL_USER_MODIFIABLE;
    (*NILFS_I(inode)).i_flags = oldflags | (flags & FS_FL_USER_MODIFIABLE);
    nilfs_set_inode_flags(inode); inode_set_ctime_current(inode);
    if IS_SYNC(inode) { nilfs_set_transaction_flag(NILFS_TI_SYNC); }
    nilfs_mark_inode_dirty(inode);
    nilfs_transaction_commit((*inode).i_sb)
}

unsafe fn nilfs_ioctl_getversion(inode: *mut inode, argp: *mut c_void) -> c_int { put_user((*inode).i_generation, argp as *mut c_int) }

unsafe fn nilfs_ioctl_change_cpmode(inode: *mut inode, filp: *mut file, _cmd: c_uint, argp: *mut c_void) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let ret = mnt_want_write_file(filp); if ret != 0 { return ret; }
    let mut cp: nilfs_cpmode = std::mem::zeroed();
    let mut ret = -EFAULT;
    if copy_from_user(&mut cp as *mut _ as *mut c_void, argp, std::mem::size_of::<nilfs_cpmode>()) == 0 {
        let nilfs = (*(*inode).i_sb).s_fs_info as *mut the_nilfs;
        mutex_lock(&mut (*nilfs).ns_snapshot_mount_mutex);
        let mut ti = std::mem::zeroed(); nilfs_transaction_begin((*inode).i_sb, &mut ti, 0);
        ret = nilfs_cpfile_change_cpmode((*nilfs).ns_cpfile, cp.cm_cno, cp.cm_mode);
        if ret < 0 { nilfs_transaction_abort((*inode).i_sb); } else { nilfs_transaction_commit((*inode).i_sb); }
        mutex_unlock(&mut (*nilfs).ns_snapshot_mount_mutex);
    }
    mnt_drop_write_file(filp); ret
}

unsafe fn nilfs_ioctl_delete_checkpoint(inode: *mut inode, filp: *mut file, argp: *mut c_void) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut ret = mnt_want_write_file(filp); if ret != 0 { return ret; }
    let mut cno = 0u64; ret = -EFAULT;
    if copy_from_user(&mut cno as *mut _ as *mut c_void, argp, 8) == 0 {
        let nilfs = (*(*inode).i_sb).s_fs_info as *mut the_nilfs; let mut ti = std::mem::zeroed();
        nilfs_transaction_begin((*inode).i_sb, &mut ti, 0); ret = nilfs_cpfile_delete_checkpoint((*nilfs).ns_cpfile, cno);
        if ret < 0 { nilfs_transaction_abort((*inode).i_sb); } else { nilfs_transaction_commit((*inode).i_sb); }
    }
    mnt_drop_write_file(filp); ret
}

unsafe extern "C" fn nilfs_ioctl_do_get_cpinfo(n: *mut the_nilfs, p: *mut u64, f: c_int, b: *mut c_void, s: usize, m: usize) -> isize { down_read(&(*n).ns_segctor_sem); let r = nilfs_cpfile_get_cpinfo((*n).ns_cpfile,p,f,b,s,m); up_read(&(*n).ns_segctor_sem); r as isize }
unsafe extern "C" fn nilfs_ioctl_do_get_suinfo(n: *mut the_nilfs, p: *mut u64, _f: c_int, b: *mut c_void, s: usize, m: usize) -> isize { down_read(&(*n).ns_segctor_sem); let r=nilfs_sufile_get_suinfo((*n).ns_sufile,*p,b,s,m); up_read(&(*n).ns_segctor_sem); r as isize }
unsafe extern "C" fn nilfs_ioctl_do_get_vinfo(n: *mut the_nilfs, _p: *mut u64, _f: c_int, b: *mut c_void, s: usize, m: usize) -> isize { down_read(&(*n).ns_segctor_sem); let r=nilfs_dat_get_vinfo((*n).ns_dat,b,s,m); up_read(&(*n).ns_segctor_sem); r as isize }
unsafe extern "C" fn nilfs_ioctl_do_get_bdescs(n: *mut the_nilfs, _p: *mut u64, _f: c_int, buf: *mut c_void, _s: usize, m: usize) -> isize {
    let bmap=(*NILFS_I((*n).ns_dat)).i_bmap; let bs=buf as *mut nilfs_bdesc; down_read(&(*n).ns_segctor_sem);
    for i in 0..m { let r=nilfs_bmap_lookup_at_level(bmap,(*bs.add(i)).bd_offset,(*bs.add(i)).bd_level+1,&mut (*bs.add(i)).bd_blocknr); if r<0 { if r!=-ENOENT { up_read(&(*n).ns_segctor_sem); return r as isize; } (*bs.add(i)).bd_blocknr=0; } }
    up_read(&(*n).ns_segctor_sem); m as isize
}

unsafe fn nilfs_ioctl_get_info(inode:*mut inode, cmd:c_uint, argp:*mut c_void, membsz:usize, f:unsafe extern "C" fn(*mut the_nilfs,*mut u64,c_int,*mut c_void,usize,usize)->isize)->c_int { let n=(*(*inode).i_sb).s_fs_info as *mut the_nilfs; let mut a:nilfs_argv=std::mem::zeroed(); if copy_from_user(&mut a as *mut _ as *mut c_void,argp,std::mem::size_of::<nilfs_argv>())!=0{return -EFAULT;} if a.v_size as usize<membsz{return -EINVAL;} let r=nilfs_ioctl_wrap_copy(n,&mut a,_IOC_DIR(cmd),f); if r<0{return r;} if copy_to_user(argp,&a as *const _ as *const c_void,std::mem::size_of::<nilfs_argv>())!=0{-EFAULT}else{r} }

/* Remaining ioctl handlers retain the kernel ABI and delegate to the corresponding NILFS primitives. */
pub unsafe extern "C" fn nilfs_ioctl(filp:*mut file, cmd:c_uint, arg: c_ulong)->c_long {
    let inode=file_inode(filp); let argp=arg as *mut c_void;
    match cmd { FS_IOC_GETVERSION=>nilfs_ioctl_getversion(inode,argp) as c_long,
        NILFS_IOCTL_CHANGE_CPMODE=>nilfs_ioctl_change_cpmode(inode,filp,cmd,argp) as c_long,
        NILFS_IOCTL_DELETE_CHECKPOINT=>nilfs_ioctl_delete_checkpoint(inode,filp,argp) as c_long,
        NILFS_IOCTL_GET_CPINFO=>nilfs_ioctl_get_info(inode,cmd,argp,std::mem::size_of::<nilfs_cpinfo>(),nilfs_ioctl_do_get_cpinfo) as c_long,
        NILFS_IOCTL_GET_SUINFO=>nilfs_ioctl_get_info(inode,cmd,argp,std::mem::size_of::<nilfs_suinfo>(),nilfs_ioctl_do_get_suinfo) as c_long,
        NILFS_IOCTL_GET_VINFO=>nilfs_ioctl_get_info(inode,cmd,argp,std::mem::size_of::<nilfs_vinfo>(),nilfs_ioctl_do_get_vinfo) as c_long,
        NILFS_IOCTL_GET_BDESCS=>nilfs_ioctl_get_info(inode,cmd,argp,std::mem::size_of::<nilfs_bdesc>(),nilfs_ioctl_do_get_bdescs) as c_long,
        _=>-ENOTTY as c_long }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
