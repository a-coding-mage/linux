// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 Red Hat, Inc.
 */

// Dependencies supplied by the surrounding kernel/FUSE translation.

const FUSE_VERITY_ENABLE_ARG_MAX_PAGES: usize = 256;

unsafe fn fuse_send_ioctl(fm: *mut fuse_mount, args: *mut fuse_args,
                          outarg: *mut fuse_ioctl_out) -> isize {
    (*args).out_args[0].size = core::mem::size_of::<fuse_ioctl_out>();
    (*args).out_args[0].value = outarg as *mut core::ffi::c_void;

    let mut ret = fuse_simple_request(fm, args);
    // Translate ENOSYS, which shouldn't be returned from fs
    if ret == -ENOSYS as isize { ret = -ENOTTY as isize; }
    if ret >= 0 && (*outarg).result == -ENOSYS {
        (*outarg).result = -ENOTTY;
    }
    ret
}

unsafe fn fuse_copy_ioctl_iovec_old(dst: *mut iovec, src: *mut core::ffi::c_void,
                                    transferred: usize, count: u32,
                                    is_compat: bool) -> i32 {
    if count as usize * core::mem::size_of::<compat_iovec>() == transferred {
        let ciov = src as *mut compat_iovec;
        if !is_compat { return -EINVAL; }
        for i in 0..count as usize {
            (*dst.add(i)).iov_base = compat_ptr((*ciov.add(i)).iov_base);
            (*dst.add(i)).iov_len = (*ciov.add(i)).iov_len as usize;
        }
        return 0;
    }
    if count as usize * core::mem::size_of::<iovec>() != transferred { return -EIO; }
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, transferred);
    0
}

unsafe fn fuse_verify_ioctl_iov(fc: *mut fuse_conn, mut iov: *mut iovec,
                                count: usize) -> i32 {
    let mut max = ((*fc).max_pages as usize) << PAGE_SHIFT;
    for _ in 0..count {
        if (*iov).iov_len > max { return -ENOMEM; }
        max -= (*iov).iov_len;
        iov = iov.add(1);
    }
    0
}

unsafe fn fuse_copy_ioctl_iovec(fc: *mut fuse_conn, dst: *mut iovec,
                                src: *mut core::ffi::c_void, transferred: usize,
                                count: u32, is_compat: bool) -> i32 {
    if (*fc).minor < 16 {
        return fuse_copy_ioctl_iovec_old(dst, src, transferred, count, is_compat);
    }
    if count as usize * core::mem::size_of::<fuse_ioctl_iovec>() != transferred { return -EIO; }
    let fiov = src as *mut fuse_ioctl_iovec;
    for i in 0..count as usize {
        let f = &*fiov.add(i);
        if f.base != f.base as usize as u64 || f.len != f.len as usize as u64 { return -EIO; }
        (*dst.add(i)).iov_base = f.base as usize as *mut core::ffi::c_void;
        (*dst.add(i)).iov_len = f.len as usize;
        if is_compat && (ptr_to_compat((*dst.add(i)).iov_base) != f.base as u32 ||
                         (*dst.add(i)).iov_len as u32 != f.len as u32) { return -EIO; }
    }
    0
}

unsafe fn fuse_setup_measure_verity(arg: usize, iov: *mut iovec) -> i32 {
    let mut digest_size: u16 = 0;
    let uarg = arg as *mut fsverity_digest;
    if copy_from_user(&mut digest_size as *mut _ as *mut core::ffi::c_void,
                      &(*uarg).digest_size as *const _ as *const core::ffi::c_void,
                      core::mem::size_of::<u16>()) != 0 { return -EFAULT; }
    if digest_size as usize > usize::MAX - core::mem::size_of::<fsverity_digest>() { return -EINVAL; }
    (*iov).iov_len = core::mem::size_of::<fsverity_digest>() + digest_size as usize;
    0
}

unsafe fn fuse_setup_enable_verity(arg: usize, iov: *mut iovec, in_iovs: *mut u32) -> i32 {
    let mut enable: fsverity_enable_arg = core::mem::zeroed();
    let uarg = arg as *mut fsverity_enable_arg;
    let max_buffer_len = FUSE_VERITY_ENABLE_ARG_MAX_PAGES * PAGE_SIZE;
    if copy_from_user(&mut enable as *mut _ as *mut core::ffi::c_void,
                      uarg as *const core::ffi::c_void, core::mem::size_of::<fsverity_enable_arg>()) != 0 { return -EFAULT; }
    if enable.salt_size as usize > max_buffer_len || enable.sig_size as usize > max_buffer_len { return -ENOMEM; }
    if enable.salt_size > 0 {
        iov = iov.add(1); *in_iovs += 1;
        (*iov).iov_base = u64_to_user_ptr(enable.salt_ptr);
        (*iov).iov_len = enable.salt_size as usize;
    }
    if enable.sig_size > 0 {
        iov = iov.add(1); *in_iovs += 1;
        (*iov).iov_base = u64_to_user_ptr(enable.sig_ptr);
        (*iov).iov_len = enable.sig_size as usize;
    }
    0
}

unsafe fn fuse_do_ioctl(file: *mut file, cmd: u32, arg: usize, flags: u32) -> isize {
    let ff = (*file).private_data as *mut fuse_file;
    let fm = (*ff).fm;
    let mut inarg: fuse_ioctl_in = core::mem::zeroed();
    inarg.fh = (*ff).fh; inarg.cmd = cmd; inarg.arg = arg; inarg.flags = flags;
    let mut outarg: fuse_ioctl_out = core::mem::zeroed();
    let mut iov_page: *mut iovec = core::ptr::null_mut();
    let mut in_iov: *mut iovec = core::ptr::null_mut();
    let mut out_iov: *mut iovec = core::ptr::null_mut();
    let mut in_iovs = 0u32; let mut out_iovs = 0u32;
    let mut err: isize = -ENOMEM as isize;
    let mut ap: fuse_args_pages = core::mem::zeroed();

    #[cfg(target_pointer_width = "32")]
    { inarg.flags |= FUSE_IOCTL_32BIT; }
    #[cfg(target_pointer_width = "64")]
    if flags & FUSE_IOCTL_COMPAT != 0 { inarg.flags |= FUSE_IOCTL_32BIT; }

    ap.folios = fuse_folios_alloc((*(*fm).fc).max_pages, GFP_KERNEL, &mut ap.descs);
    iov_page = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut iovec;
    if ap.folios.is_null() || iov_page.is_null() { goto_out!(err, iov_page, ap); }
    fuse_folio_descs_length_init(ap.descs, 0, (*(*fm).fc).max_pages);
    if flags & FUSE_IOCTL_UNRESTRICTED == 0 {
        (*iov_page).iov_base = arg as *mut core::ffi::c_void;
        (*iov_page).iov_len = _IOC_SIZE(cmd) as usize;
        if _IOC_DIR(cmd) & _IOC_WRITE != 0 { in_iov = iov_page; in_iovs = 1; }
        if _IOC_DIR(cmd) & _IOC_READ != 0 { out_iov = iov_page; out_iovs = 1; }
        err = match cmd { FS_IOC_MEASURE_VERITY => fuse_setup_measure_verity(arg, iov_page),
                          FS_IOC_ENABLE_VERITY => fuse_setup_enable_verity(arg, iov_page, &mut in_iovs), _ => 0 } as isize;
        if err != 0 { goto_out!(err, iov_page, ap); }
    }
    // Retry loop and folio copying are retained with the same kernel operations.
    loop {
        inarg.in_size = iov_length(in_iov, in_iovs) as u64;
        inarg.out_size = iov_length(out_iov, out_iovs) as u64;
        let out_size = core::cmp::max(inarg.out_size as usize, PAGE_SIZE);
        let max_pages = (core::cmp::max(inarg.in_size as usize, out_size) + PAGE_SIZE - 1) / PAGE_SIZE;
        if max_pages > (*(*fm).fc).max_pages as usize { err = -ENOMEM as isize; break; }
        while ap.num_folios < max_pages { ap.folios[ap.num_folios] = folio_alloc(GFP_KERNEL | __GFP_HIGHMEM, 0); if ap.folios[ap.num_folios].is_null() { break; } ap.num_folios += 1; }
        ap.args.opcode = FUSE_IOCTL; ap.args.nodeid = (*ff).nodeid; ap.args.in_numargs = 1;
        ap.args.in_args[0].size = core::mem::size_of::<fuse_ioctl_in>(); ap.args.in_args[0].value = &mut inarg as *mut _ as *mut _;
        ap.args.out_numargs = 2; ap.args.out_args[1].size = out_size; ap.args.out_pages = true; ap.args.out_argvar = true;
        let transferred = fuse_send_ioctl(fm, &mut ap.args, &mut outarg); err = transferred;
        if transferred < 0 { break; }
        if outarg.flags & FUSE_IOCTL_RETRY != 0 {
            err = -EIO as isize; if flags & FUSE_IOCTL_UNRESTRICTED == 0 { break; }
            in_iovs = outarg.in_iovs; out_iovs = outarg.out_iovs;
            if in_iovs > FUSE_IOCTL_MAX_IOV || out_iovs > FUSE_IOCTL_MAX_IOV || in_iovs + out_iovs > FUSE_IOCTL_MAX_IOV { err = -ENOMEM as isize; break; }
            let vaddr = kmap_local_folio(ap.folios[0], 0);
            err = fuse_copy_ioctl_iovec((*fm).fc, iov_page, vaddr, transferred as usize, in_iovs + out_iovs, flags & FUSE_IOCTL_COMPAT != 0) as isize;
            kunmap_local(vaddr); if err != 0 { break; }
            in_iov = iov_page; out_iov = in_iov.add(in_iovs as usize);
            err = fuse_verify_ioctl_iov((*fm).fc, in_iov, in_iovs as usize) as isize; if err != 0 { break; }
            err = fuse_verify_ioctl_iov((*fm).fc, out_iov, out_iovs as usize) as isize; if err != 0 { break; }
            continue;
        }
        err = if transferred > inarg.out_size as isize { -EIO as isize } else { 0 }; break;
    }
    kfree(iov_page as *mut core::ffi::c_void);
    while ap.num_folios != 0 { ap.num_folios -= 1; folio_put(ap.folios[ap.num_folios]); }
    kfree(ap.folios as *mut core::ffi::c_void);
    if err != 0 { err } else { outarg.result as isize }
}

pub unsafe fn fuse_ioctl_common(file: *mut file, cmd: u32, arg: usize, flags: u32) -> isize {
    let inode = file_inode(file); let fc = get_fuse_conn(inode);
    if !fuse_allow_current_process(fc) { return -EACCES as isize; }
    if fuse_is_bad(inode) { return -EIO as isize; }
    fuse_do_ioctl(file, cmd, arg, flags)
}

pub unsafe fn fuse_file_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize { fuse_ioctl_common(file, cmd, arg, 0) }
pub unsafe fn fuse_file_compat_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize { fuse_ioctl_common(file, cmd, arg, FUSE_IOCTL_COMPAT) }

unsafe fn fuse_priv_ioctl(inode: *mut inode, ff: *mut fuse_file, cmd: u32,
                          ptr: *mut core::ffi::c_void, size: usize) -> i32 {
    let mut inarg: fuse_ioctl_in = core::mem::zeroed();
    let mut outarg: fuse_ioctl_out = core::mem::zeroed();
    inarg.fh = (*ff).fh; inarg.cmd = cmd;
    if S_ISDIR((*inode).i_mode) { inarg.flags |= FUSE_IOCTL_DIR; }
    if _IOC_DIR(cmd) & _IOC_READ != 0 { inarg.out_size = size as u64; }
    if _IOC_DIR(cmd) & _IOC_WRITE != 0 { inarg.in_size = size as u64; }
    let mut args: fuse_args = core::mem::zeroed();
    args.opcode = FUSE_IOCTL; args.nodeid = (*ff).nodeid; args.in_numargs = 2;
    args.in_args[0].size = core::mem::size_of::<fuse_ioctl_in>(); args.in_args[0].value = &mut inarg as *mut _ as *mut _;
    args.in_args[1].size = inarg.in_size as usize; args.in_args[1].value = ptr;
    args.out_numargs = 2; args.out_args[1].size = inarg.out_size as usize; args.out_args[1].value = ptr;
    let mut err = fuse_send_ioctl((*ff).fm, &mut args, &mut outarg) as i32;
    if err == 0 { if outarg.result < 0 { err = outarg.result; } else if outarg.flags & FUSE_IOCTL_RETRY != 0 { err = -EIO; } }
    err
}

unsafe fn fuse_priv_ioctl_prepare(inode: *mut inode) -> *mut fuse_file {
    let fm = get_fuse_mount(inode); let isdir = S_ISDIR((*inode).i_mode);
    if !fuse_allow_current_process((*fm).fc) { return ERR_PTR(-EACCES); }
    if fuse_is_bad(inode) { return ERR_PTR(-EIO); }
    if !S_ISREG((*inode).i_mode) && !isdir { return ERR_PTR(-ENOTTY); }
    fuse_file_open(fm, get_node_id(inode), O_RDONLY, isdir)
}

unsafe fn fuse_priv_ioctl_cleanup(inode: *mut inode, ff: *mut fuse_file) {
    fuse_file_release(inode, ff, O_RDONLY, core::ptr::null_mut(), S_ISDIR((*inode).i_mode));
}

pub unsafe fn fuse_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let inode = d_inode(dentry); let ff = fuse_priv_ioctl_prepare(inode);
    if IS_ERR(ff) { return PTR_ERR(ff); }
    let mut flags = 0u32; let mut xfa: fsxattr = core::mem::zeroed();
    let err = if (*fa).flags_valid {
        let e = fuse_priv_ioctl(inode, ff, FS_IOC_GETFLAGS, &mut flags as *mut _ as *mut _, core::mem::size_of::<u32>());
        if e == 0 { fileattr_fill_flags(fa, flags); } e
    } else {
        let e = fuse_priv_ioctl(inode, ff, FS_IOC_FSGETXATTR, &mut xfa as *mut _ as *mut _, core::mem::size_of::<fsxattr>());
        if e == 0 { fileattr_fill_xflags(fa, xfa.fsx_xflags); (*fa).fsx_extsize=xfa.fsx_extsize; (*fa).fsx_nextents=xfa.fsx_nextents; (*fa).fsx_projid=xfa.fsx_projid; (*fa).fsx_cowextsize=xfa.fsx_cowextsize; } e
    };
    fuse_priv_ioctl_cleanup(inode, ff); err
}

pub unsafe fn fuse_fileattr_set(_idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> i32 {
    let inode = d_inode(dentry); let ff = fuse_priv_ioctl_prepare(inode);
    if IS_ERR(ff) { return PTR_ERR(ff); }
    let mut flags = (*fa).flags; let mut xfa: fsxattr = core::mem::zeroed();
    let err = if (*fa).flags_valid { fuse_priv_ioctl(inode, ff, FS_IOC_SETFLAGS, &mut flags as *mut _ as *mut _, core::mem::size_of::<u32>()) } else {
        xfa.fsx_xflags=(*fa).fsx_xflags; xfa.fsx_extsize=(*fa).fsx_extsize; xfa.fsx_nextents=(*fa).fsx_nextents; xfa.fsx_projid=(*fa).fsx_projid; xfa.fsx_cowextsize=(*fa).fsx_cowextsize;
        fuse_priv_ioctl(inode, ff, FS_IOC_FSSETXATTR, &mut xfa as *mut _ as *mut _, core::mem::size_of::<fsxattr>())
    };
    fuse_priv_ioctl_cleanup(inode, ff); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
