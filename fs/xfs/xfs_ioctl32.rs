// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2004-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Kernel dependencies supplied by the surrounding XFS translation.

#[inline]
unsafe fn _native_ioc(cmd: u32, size: usize) -> u32 {
    _IOC(_IOC_DIR(cmd), _IOC_TYPE(cmd), _IOC_NR(cmd), size)
}

#[cfg(BROKEN_X86_ALIGNMENT)]
unsafe fn xfs_compat_ioc_fsgeometry_v1(
    mp: *mut xfs_mount,
    arg32: *mut compat_xfs_fsop_geom_v1_t,
) -> i32 {
    let mut fsgeo: xfs_fsop_geom = core::mem::zeroed();
    xfs_fs_geometry(mp, &mut fsgeo, 3);
    if copy_to_user(arg32 as *mut _, &fsgeo as *const _, core::mem::size_of::<compat_xfs_fsop_geom_v1_t>()) != 0 { return -EFAULT; }
    0
}

#[cfg(BROKEN_X86_ALIGNMENT)]
unsafe fn xfs_compat_growfs_data_copyin(in_: *mut xfs_growfs_data, arg32: *mut compat_xfs_growfs_data_t) -> i32 {
    if get_user(&mut (*in_).newblocks, &(*arg32).newblocks) != 0 || get_user(&mut (*in_).imaxpct, &(*arg32).imaxpct) != 0 { return -EFAULT; }
    0
}

#[cfg(BROKEN_X86_ALIGNMENT)]
unsafe fn xfs_compat_growfs_rt_copyin(in_: *mut xfs_growfs_rt, arg32: *mut compat_xfs_growfs_rt_t) -> i32 {
    if get_user(&mut (*in_).newblocks, &(*arg32).newblocks) != 0 || get_user(&mut (*in_).extsize, &(*arg32).extsize) != 0 { return -EFAULT; }
    0
}

#[cfg(BROKEN_X86_ALIGNMENT)]
unsafe fn xfs_fsinumbers_fmt_compat(breq: *mut xfs_ibulk, ig: *const xfs_inumbers) -> i32 {
    let mut ig1: xfs_inogrp = core::mem::zeroed();
    xfs_inumbers_to_inogrp(&mut ig1, ig);
    let p32 = (*breq).ubuffer as *mut compat_xfs_inogrp;
    if put_user(ig1.xi_startino, &mut (*p32).xi_startino) != 0 || put_user(ig1.xi_alloccount, &mut (*p32).xi_alloccount) != 0 || put_user(ig1.xi_allocmask, &mut (*p32).xi_allocmask) != 0 { return -EFAULT; }
    xfs_ibulk_advance(breq, core::mem::size_of::<compat_xfs_inogrp>())
}

#[cfg(not(BROKEN_X86_ALIGNMENT))]
use xfs_fsinumbers_fmt as xfs_fsinumbers_fmt_compat;

unsafe fn xfs_ioctl32_bstime_copyin(bstime: *mut xfs_bstime_t, bstime32: *mut compat_xfs_bstime_t) -> i32 {
    let mut sec32: old_time32_t = 0;
    if get_user(&mut sec32, &(*bstime32).tv_sec) != 0 || get_user(&mut (*bstime).tv_nsec, &(*bstime32).tv_nsec) != 0 { return -EFAULT; }
    (*bstime).tv_sec = sec32;
    0
}

unsafe fn xfs_ioctl32_bstat_copyin(bstat: *mut xfs_bstat, bstat32: *mut compat_xfs_bstat) -> i32 {
    macro_rules! gu { ($a:expr, $b:expr) => { if get_user($a, $b) != 0 { return -EFAULT; } }; }
    gu!(&mut (*bstat).bs_ino, &(*bstat32).bs_ino); gu!(&mut (*bstat).bs_mode, &(*bstat32).bs_mode); gu!(&mut (*bstat).bs_nlink, &(*bstat32).bs_nlink);
    gu!(&mut (*bstat).bs_uid, &(*bstat32).bs_uid); gu!(&mut (*bstat).bs_gid, &(*bstat32).bs_gid); gu!(&mut (*bstat).bs_rdev, &(*bstat32).bs_rdev);
    gu!(&mut (*bstat).bs_blksize, &(*bstat32).bs_blksize); gu!(&mut (*bstat).bs_size, &(*bstat32).bs_size);
    if xfs_ioctl32_bstime_copyin(&mut (*bstat).bs_atime, &mut (*bstat32).bs_atime) != 0 || xfs_ioctl32_bstime_copyin(&mut (*bstat).bs_mtime, &mut (*bstat32).bs_mtime) != 0 || xfs_ioctl32_bstime_copyin(&mut (*bstat).bs_ctime, &mut (*bstat32).bs_ctime) != 0 { return -EFAULT; }
    gu!(&mut (*bstat).bs_blocks, &(*bstat32).bs_size); gu!(&mut (*bstat).bs_xflags, &(*bstat32).bs_size); gu!(&mut (*bstat).bs_extsize, &(*bstat32).bs_extsize); gu!(&mut (*bstat).bs_extents, &(*bstat32).bs_extents); gu!(&mut (*bstat).bs_gen, &(*bstat32).bs_gen); gu!(&mut (*bstat).bs_projid_lo, &(*bstat32).bs_projid_lo); gu!(&mut (*bstat).bs_projid_hi, &(*bstat32).bs_projid_hi); gu!(&mut (*bstat).bs_forkoff, &(*bstat32).bs_forkoff); gu!(&mut (*bstat).bs_dmevmask, &(*bstat32).bs_dmevmask); gu!(&mut (*bstat).bs_dmstate, &(*bstat32).bs_dmstate); gu!(&mut (*bstat).bs_aextents, &(*bstat32).bs_aextents);
    0
}

unsafe fn xfs_bstime_store_compat(p32: *mut compat_xfs_bstime_t, p: *const xfs_bstime_t) -> i32 {
    let sec32: __s32 = (*p).tv_sec as __s32;
    if put_user(sec32, &mut (*p32).tv_sec) != 0 || put_user((*p).tv_nsec, &mut (*p32).tv_nsec) != 0 { return -EFAULT; }
    0
}

unsafe fn xfs_fsbulkstat_one_fmt_compat(breq: *mut xfs_ibulk, bstat: *const xfs_bulkstat) -> i32 {
    let mut bs1: xfs_bstat = core::mem::zeroed();
    xfs_bulkstat_to_bstat((*breq).mp, &mut bs1, bstat);
    let p32 = (*breq).ubuffer as *mut compat_xfs_bstat;
    macro_rules! pu { ($a:expr, $b:expr) => { if put_user($a, $b) != 0 { return -EFAULT; } }; }
    pu!(bs1.bs_ino, &mut (*p32).bs_ino); pu!(bs1.bs_mode, &mut (*p32).bs_mode); pu!(bs1.bs_nlink, &mut (*p32).bs_nlink); pu!(bs1.bs_uid, &mut (*p32).bs_uid); pu!(bs1.bs_gid, &mut (*p32).bs_gid); pu!(bs1.bs_rdev, &mut (*p32).bs_rdev); pu!(bs1.bs_blksize, &mut (*p32).bs_blksize); pu!(bs1.bs_size, &mut (*p32).bs_size);
    if xfs_bstime_store_compat(&mut (*p32).bs_atime, &bs1.bs_atime) != 0 || xfs_bstime_store_compat(&mut (*p32).bs_mtime, &bs1.bs_mtime) != 0 || xfs_bstime_store_compat(&mut (*p32).bs_ctime, &bs1.bs_ctime) != 0 { return -EFAULT; }
    pu!(bs1.bs_blocks, &mut (*p32).bs_blocks); pu!(bs1.bs_xflags, &mut (*p32).bs_xflags); pu!(bs1.bs_extsize, &mut (*p32).bs_extsize); pu!(bs1.bs_extents, &mut (*p32).bs_extents); pu!(bs1.bs_gen, &mut (*p32).bs_gen); pu!(bs1.bs_projid, &mut (*p32).bs_projid); pu!(bs1.bs_projid_hi, &mut (*p32).bs_projid_hi); pu!(bs1.bs_forkoff, &mut (*p32).bs_forkoff); pu!(bs1.bs_dmevmask, &mut (*p32).bs_dmevmask); pu!(bs1.bs_dmstate, &mut (*p32).bs_dmstate); pu!(bs1.bs_aextents, &mut (*p32).bs_aextents);
    xfs_ibulk_advance(breq, core::mem::size_of::<compat_xfs_bstat>())
}

unsafe fn xfs_compat_ioc_fsbulkstat(filp: *mut file, cmd: u32, p32: *mut compat_xfs_fsop_bulkreq) -> i32 {
    let mp = XFS_I(file_inode(filp)).i_mount;
    let mut addr: u32 = 0;
    let mut bulkreq: xfs_fsop_bulkreq = core::mem::zeroed();
    let mut breq: xfs_ibulk = core::mem::zeroed();
    breq.mp=mp; breq.idmap=file_mnt_idmap(filp); breq.ocount=0;
    let mut lastino: xfs_ino_t = 0;
    let inumbers_func = xfs_fsinumbers_fmt_compat;
    let bs_one_func = xfs_fsbulkstat_one_fmt_compat;
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    if xfs_is_shutdown(mp) { return -EIO; }
    if get_user(&mut addr,&(*p32).lastip)!=0{return -EFAULT;} bulkreq.lastip=compat_ptr(addr);
    if get_user(&mut bulkreq.icount,&(*p32).icount)!=0 || get_user(&mut addr,&(*p32).ubuffer)!=0{return -EFAULT;} bulkreq.ubuffer=compat_ptr(addr);
    if get_user(&mut addr,&(*p32).ocount)!=0{return -EFAULT;} bulkreq.ocount=compat_ptr(addr);
    if copy_from_user(&mut lastino,bulkreq.lastip,core::mem::size_of::<__s64>())!=0{return -EFAULT;}
    if bulkreq.icount<=0 || bulkreq.ubuffer.is_null(){return -EINVAL;}
    breq.ubuffer=bulkreq.ubuffer; breq.icount=bulkreq.icount;
    let error = if cmd==XFS_IOC_FSINUMBERS_32 { breq.startino=if lastino!=0{lastino+1}else{0}; let e=xfs_inumbers(&mut breq,inumbers_func); lastino=breq.startino-1;e }
    else if cmd==XFS_IOC_FSBULKSTAT_SINGLE_32 { breq.startino=lastino;breq.icount=1;let e=xfs_bulkstat_one(&mut breq,bs_one_func);lastino=breq.startino;e }
    else if cmd==XFS_IOC_FSBULKSTAT_32 {breq.startino=if lastino!=0{lastino+1}else{0};let e=xfs_bulkstat(&mut breq,bs_one_func);lastino=breq.startino-1;e}
    else {-EINVAL};
    if error!=0{return error;}
    if !bulkreq.lastip.is_null() && copy_to_user(bulkreq.lastip,&lastino,core::mem::size_of::<xfs_ino_t>())!=0{return -EFAULT;}
    if !bulkreq.ocount.is_null() && copy_to_user(bulkreq.ocount,&breq.ocount,core::mem::size_of::<__s32>())!=0{return -EFAULT;}
    0
}

// The remaining ioctl dispatcher and compatibility helpers retain the C ABI and
// are expressed with raw pointers; dependent XFS and kernel symbols are external.
unsafe fn xfs_compat_handlereq_copyin(hreq: *mut xfs_fsop_handlereq_t, arg32: *mut compat_xfs_fsop_handlereq_t) -> i32 {
    let mut h: compat_xfs_fsop_handlereq_t = core::mem::zeroed();
    if copy_from_user(&mut h, arg32, core::mem::size_of::<compat_xfs_fsop_handlereq_t>()) != 0 { return -EFAULT; }
    (*hreq).fd=h.fd; (*hreq).path=compat_ptr(h.path); (*hreq).oflags=h.oflags; (*hreq).ihandle=compat_ptr(h.ihandle); (*hreq).ihandlen=h.ihandlen; (*hreq).ohandle=compat_ptr(h.ohandle); (*hreq).ohandlen=compat_ptr(h.ohandlen); 0
}

unsafe fn xfs_compat_handlereq_to_dentry(parfilp: *mut file, hreq: *mut compat_xfs_fsop_handlereq_t) -> *mut dentry { xfs_handle_to_dentry(parfilp, compat_ptr((*hreq).ihandle), (*hreq).ihandlen) }

unsafe fn xfs_compat_attrlist_by_handle(parfilp: *mut file, p: *mut compat_xfs_fsop_attrlist_handlereq_t) -> i32 {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut h = core::mem::zeroed(); if copy_from_user(&mut h, p, core::mem::size_of_val(&h)) != 0 { return -EFAULT; }
    let d = xfs_compat_handlereq_to_dentry(parfilp, &mut h.hreq); if IS_ERR(d) { return PTR_ERR(d); }
    let e=xfs_ioc_attr_list(XFS_I(d_inode(d)), compat_ptr(h.buffer), h.buflen, h.flags, &mut (*p).pos); dput(d); e
}

unsafe fn xfs_compat_attrmulti_by_handle(parfilp: *mut file, arg: *mut core::ffi::c_void) -> i32 {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut h: compat_xfs_fsop_attrmulti_handlereq_t=core::mem::zeroed(); if copy_from_user(&mut h,arg,core::mem::size_of_val(&h))!=0{return -EFAULT;}
    if h.opcount >= INT_MAX / core::mem::size_of::<compat_xfs_attr_multiop_t>() { return -E2BIG; }
    let d=xfs_compat_handlereq_to_dentry(parfilp,&mut h.hreq); if IS_ERR(d){return PTR_ERR(d);}
    let size=h.opcount*core::mem::size_of::<compat_xfs_attr_multiop_t>(); if size==0 || size>16*PAGE_SIZE { dput(d); return -E2BIG; }
    let ops=memdup_user(compat_ptr(h.ops),size); if IS_ERR(ops){let e=PTR_ERR(ops); dput(d); return e;}
    for i in 0..h.opcount { (*ops.add(i)).am_error=xfs_ioc_attrmulti_one(parfilp,d_inode(d),(*ops.add(i)).am_opcode,compat_ptr((*ops.add(i)).am_attrname),compat_ptr((*ops.add(i)).am_attrvalue),&mut (*ops.add(i)).am_length,(*ops.add(i)).am_flags); }
    let mut e=0; if copy_to_user(compat_ptr(h.ops),ops,size)!=0{e=-EFAULT;} kfree(ops); dput(d); e
}

pub unsafe extern "C" fn xfs_file_compat_ioctl(filp: *mut file, mut cmd: u32, p: c_ulong) -> c_long {
    let inode=file_inode(filp); let ip=XFS_I(inode); let arg=compat_ptr(p); let mut error: i32;
    trace_xfs_file_compat_ioctl(ip);
    match cmd {
        XFS_IOC_GETVERSION_32 => { cmd=_native_ioc(cmd,core::mem::size_of::<c_long>()); xfs_file_ioctl(filp,cmd,p) }
        XFS_IOC_SWAPEXT_32 => { let mut sxp: xfs_swapext=core::mem::zeroed(); let sxu=arg as *mut compat_xfs_swapext; if copy_from_user(&mut sxp,sxu,core::mem::offset_of!(xfs_swapext,sx_stat))!=0 || xfs_ioctl32_bstat_copyin(&mut sxp.sx_stat,&mut (*sxu).sx_stat)!=0{return -EFAULT as c_long;} error=mnt_want_write_file(filp); if error!=0{return error as c_long;} error=xfs_ioc_swapext(&mut sxp); mnt_drop_write_file(filp); error as c_long }
        XFS_IOC_FSBULKSTAT_32 | XFS_IOC_FSBULKSTAT_SINGLE_32 | XFS_IOC_FSINUMBERS_32 => xfs_compat_ioc_fsbulkstat(filp,cmd,arg) as c_long,
        XFS_IOC_FD_TO_HANDLE_32 | XFS_IOC_PATH_TO_HANDLE_32 | XFS_IOC_PATH_TO_FSHANDLE_32 => { let mut h=core::mem::zeroed(); if xfs_compat_handlereq_copyin(&mut h,arg)!=0{return -EFAULT as c_long;} cmd=_native_ioc(cmd,core::mem::size_of::<xfs_fsop_handlereq_t>()); xfs_find_handle(cmd,&mut h) as c_long }
        XFS_IOC_OPEN_BY_HANDLE_32 => { let mut h=core::mem::zeroed(); if xfs_compat_handlereq_copyin(&mut h,arg)!=0{return -EFAULT as c_long;} xfs_open_by_handle(filp,&mut h) as c_long }
        XFS_IOC_READLINK_BY_HANDLE_32 => { let mut h=core::mem::zeroed(); if xfs_compat_handlereq_copyin(&mut h,arg)!=0{return -EFAULT as c_long;} xfs_readlink_by_handle(filp,&mut h) as c_long }
        XFS_IOC_ATTRLIST_BY_HANDLE_32 => xfs_compat_attrlist_by_handle(filp,arg as *mut _) as c_long,
        XFS_IOC_ATTRMULTI_BY_HANDLE_32 => xfs_compat_attrmulti_by_handle(filp,arg) as c_long,
        _ => xfs_file_ioctl(filp,cmd,arg as c_ulong),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
