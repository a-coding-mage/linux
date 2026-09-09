// SPDX-License-Identifier: GPL-2.0-only
/*
 * CUSE: Character device in Userspace
 *
 * Copyright (C) 2008-2009  SUSE Linux Products GmbH
 * Copyright (C) 2008-2009  Tejun Heo <tj@kernel.org>
 *
 * CUSE enables character devices to be implemented from userland much
 * like FUSE allows filesystems.  On initialization /dev/cuse is
 * created.  By opening the file and replying to the CUSE_INIT request
 * userland CUSE server can create a character device.  After that the
 * operation is very similar to FUSE.
 *
 * This is a source-level Rust translation of the Linux kernel implementation.
 * Kernel and FUSE dependencies are supplied by other translation units.
 */

// C headers and preprocessor configuration are supplied by the kernel build.
const CUSE_CONNTBL_LEN: usize = 64;

#[repr(C)]
struct CuseConn {
    list: ListHead,
    fm: FuseMount,
    fc: FuseConn,
    cdev: *mut Cdev,
    dev: *mut Device,
    unrestricted_ioctl: bool,
}

static mut CUSE_LOCK: Mutex = Mutex { _private: [] };
static mut CUSE_CONNTBL: [ListHead; CUSE_CONNTBL_LEN] = [ListHead { _private: [] }; CUSE_CONNTBL_LEN];
static mut CUSE_CLASS: *mut Class = core::ptr::null_mut();

unsafe fn fc_to_cc(fc: *mut FuseConn) -> *mut CuseConn {
    container_of!(fc, CuseConn, fc)
}

unsafe fn cuse_conntbl_head(devt: DevT) -> *mut ListHead {
    CUSE_CONNTBL.as_mut_ptr().add(((major(devt) + minor(devt)) % CUSE_CONNTBL_LEN as u64) as usize)
}

unsafe extern "C" fn cuse_read_iter(kiocb: *mut Kiocb, to: *mut IovIter) -> Isize {
    let mut io = FUSE_IO_PRIV_SYNC!(kiocb);
    let mut pos: LoFF = 0;
    fuse_direct_io(&mut io, to, &mut pos, FUSE_DIO_CUSE)
}

unsafe extern "C" fn cuse_write_iter(kiocb: *mut Kiocb, from: *mut IovIter) -> Isize {
    let mut io = FUSE_IO_PRIV_SYNC!(kiocb);
    let mut pos: LoFF = 0;
    // No locking or generic_write_checks(), the server is responsible for locking and sanity checks.
    fuse_direct_io(&mut io, from, &mut pos, FUSE_DIO_WRITE | FUSE_DIO_CUSE)
}

unsafe extern "C" fn cuse_open(inode: *mut Inode, file: *mut File) -> Int {
    let devt = (*(*inode).i_cdev).dev;
    let mut cc: *mut CuseConn = core::ptr::null_mut();
    mutex_lock(&mut CUSE_LOCK);
    let mut pos: *mut CuseConn;
    list_for_each_entry!(pos, cuse_conntbl_head(devt), list, {
        if (*(*pos).dev).devt == devt {
            fuse_conn_get(&mut (*pos).fc);
            cc = pos;
            break;
        }
    });
    mutex_unlock(&mut CUSE_LOCK);
    if cc.is_null() { return -ENODEV; }
    let rc = fuse_do_open(&mut (*cc).fm, 0, file, 0);
    if rc != 0 { fuse_conn_put(&mut (*cc).fc); }
    rc
}

unsafe extern "C" fn cuse_release(_inode: *mut Inode, file: *mut File) -> Int {
    let ff = (*file).private_data as *mut FuseFile;
    let fm = (*ff).fm;
    fuse_sync_release(core::ptr::null_mut(), ff, (*file).f_flags);
    fuse_conn_put((*fm).fc);
    0
}

unsafe extern "C" fn cuse_file_ioctl(file: *mut File, cmd: CUInt, arg: CULong) -> Long {
    let ff = (*file).private_data as *mut FuseFile;
    let cc = fc_to_cc((*(*ff).fm).fc);
    let mut flags = 0;
    if (*cc).unrestricted_ioctl { flags |= FUSE_IOCTL_UNRESTRICTED; }
    fuse_do_ioctl(file, cmd, arg, flags)
}

unsafe extern "C" fn cuse_file_compat_ioctl(file: *mut File, cmd: CUInt, arg: CULong) -> Long {
    let ff = (*file).private_data as *mut FuseFile;
    let cc = fc_to_cc((*(*ff).fm).fc);
    let mut flags = FUSE_IOCTL_COMPAT;
    if (*cc).unrestricted_ioctl { flags |= FUSE_IOCTL_UNRESTRICTED; }
    fuse_do_ioctl(file, cmd, arg, flags)
}

static mut CUSE_FRONTEND_FOPS: FileOperations = FileOperations {
    owner: THIS_MODULE, read_iter: Some(cuse_read_iter), write_iter: Some(cuse_write_iter),
    open: Some(cuse_open), release: Some(cuse_release), unlocked_ioctl: Some(cuse_file_ioctl),
    compat_ioctl: Some(cuse_file_compat_ioctl), poll: Some(fuse_file_poll), llseek: Some(noop_llseek),
};

#[repr(C)] struct CuseDevinfo { name: *const CChar }

unsafe fn cuse_parse_one(pp: *mut *mut CChar, end: *mut CChar, keyp: *mut *mut CChar, valp: *mut *mut CChar) -> Int {
    let mut p = *pp;
    while p < end && *p == 0 { p = p.add(1); }
    if p == end { return 0; }
    if *end.sub(1) != 0 { pr_err!("info not properly terminated\n"); return -EINVAL; }
    let mut key = p;
    let mut val = p;
    p = p.add(strlen(p));
    if !valp.is_null() { val = strsep(&mut val, b"=".as_ptr() as *mut CChar); if val.is_null() { val = key.add(strlen(key)); } val = strstrip(val); }
    key = strstrip(key);
    if strlen(key) == 0 { pr_err!("zero length info key specified\n"); return -EINVAL; }
    *pp = p; *keyp = key; if !valp.is_null() { *valp = val; } 1
}

unsafe fn cuse_parse_devinfo(mut p: *mut CChar, len: usize, devinfo: *mut CuseDevinfo) -> Int {
    let end = p.add(len); let mut key = core::ptr::null_mut(); let mut val = core::ptr::null_mut();
    loop { let rc = cuse_parse_one(&mut p, end, &mut key, &mut val); if rc < 0 { return rc; } if rc == 0 { break; }
        if strcmp(key, b"DEVNAME\0".as_ptr() as *const CChar) == 0 { (*devinfo).name = val; } else { pr_warn!("unknown device info \"%s\"\n", key); }
    }
    if (*devinfo).name.is_null() || strlen((*devinfo).name) == 0 { pr_err!("DEVNAME unspecified\n"); return -EINVAL; } 0
}

unsafe extern "C" fn cuse_gendev_release(dev: *mut Device) { kfree(dev as *mut core::ffi::c_void); }

#[repr(C)] struct CuseInitArgs { ap: FuseArgsPages, input: CuseInitIn, output: CuseInitOut, folio: *mut Folio, desc: FuseFolioDesc, fc: *mut FuseConn }

unsafe extern "C" fn cuse_process_init_reply(args: *mut FuseArgs, error: Int) {
    let ia = container_of!(args, CuseInitArgs, ap.args); let fc = (*ia).fc; let ap = &mut (*ia).ap;
    let cc = fc_to_cc(fc); let arg = &mut (*ia).output; let folio = (*ap).folios[0]; let mut devinfo = CuseDevinfo { name: core::ptr::null() };
    let mut dev: *mut Device; let mut cdev: *mut Cdev; let mut devt: DevT; let mut rc: Int; let mut i: Int;
    if error != 0 || (*arg).major != FUSE_KERNEL_VERSION || (*arg).minor < 11 { goto!(err); }
    (*fc).minor = (*arg).minor; (*fc).max_read = max_t!((*arg).max_read, 4096); (*fc).max_write = max_t!((*arg).max_write, 4096);
    (*cc).unrestricted_ioctl = ((*arg).flags & CUSE_UNRESTRICTED_IOCTL) != 0;
    rc = cuse_parse_devinfo(folio_address(folio), (*ap).args.out_args[1].size, &mut devinfo); if rc != 0 { goto!(err); }
    devt = mkdev((*arg).dev_major, (*arg).dev_minor);
    rc = if major(devt) == 0 { alloc_chrdev_region(&mut devt, minor(devt), 1, devinfo.name) } else { register_chrdev_region(devt, 1, devinfo.name) };
    if rc != 0 { pr_err!("failed to register chrdev region\n"); goto!(err); }
    dev = kzalloc_device(); if dev.is_null() { rc = -ENOMEM; goto!(err_region); }
    device_initialize(dev); (*dev).class = CUSE_CLASS; (*dev).devt = devt; (*dev).release = Some(cuse_gendev_release); dev_set_drvdata(dev, cc); dev_set_name(dev, b"%s\0".as_ptr() as *const CChar, devinfo.name);
    mutex_lock(&mut CUSE_LOCK);
    for i in 0..CUSE_CONNTBL_LEN { let mut pos: *mut CuseConn; list_for_each_entry!(pos, &mut CUSE_CONNTBL[i], list, { if strcmp(dev_name((*pos).dev), dev_name(dev)) == 0 { goto!(err_unlock); } }); }
    rc = device_add(dev); if rc != 0 { goto!(err_unlock); }
    cdev = cdev_alloc(); if cdev.is_null() { rc = -ENOMEM; goto!(err_dev); }
    (*cdev).owner = THIS_MODULE; (*cdev).ops = &CUSE_FRONTEND_FOPS; rc = cdev_add(cdev, devt, 1); if rc != 0 { goto!(err_cdev); }
    (*cc).dev = dev; (*cc).cdev = cdev; list_add(&mut (*cc).list, cuse_conntbl_head(devt)); mutex_unlock(&mut CUSE_LOCK); device_uevent_add(dev);
out: kfree(ia as *mut core::ffi::c_void); folio_put(folio); return;
err_cdev: cdev_del(cdev); err_dev: device_del(dev); err_unlock: mutex_unlock(&mut CUSE_LOCK); put_device(dev); err_region: unregister_chrdev_region(devt, 1); err: fuse_chan_abort((*fc).chan, false); goto!(out);
}

unsafe extern "C" fn cuse_fc_release(fc: *mut FuseConn) { kfree(fc_to_cc(fc) as *mut core::ffi::c_void); }

unsafe extern "C" fn cuse_send_init(cc: *mut CuseConn) -> Int {
    build_bug_on!(CUSE_INIT_INFO_MAX > PAGE_SIZE);
    let folio = folio_alloc(GFP_KERNEL | __GFP_ZERO, 0); if folio.is_null() { return -ENOMEM; }
    let ia = kzalloc_cuse_init_args(); if ia.is_null() { folio_put(folio); return -ENOMEM; }
    (*ia).input.major = FUSE_KERNEL_VERSION; (*ia).input.minor = FUSE_KERNEL_MINOR_VERSION; (*ia).input.flags |= CUSE_UNRESTRICTED_IOCTL;
    (*ia).ap.args.opcode = CUSE_INIT; (*ia).ap.args.in_numargs = 1; (*ia).ap.args.in_args[0] = FuseArg { size: size_of::<CuseInitIn>(), value: &mut (*ia).input as *mut _ as *mut core::ffi::c_void };
    (*ia).ap.args.out_numargs = 2; (*ia).ap.args.out_args[0] = FuseArg { size: size_of::<CuseInitOut>(), value: &mut (*ia).output as *mut _ as *mut core::ffi::c_void };
    (*ia).ap.args.out_args[1].size = CUSE_INIT_INFO_MAX; (*ia).ap.args.out_argvar = true; (*ia).ap.args.out_pages = true; (*ia).ap.num_folios = 1; (*ia).ap.folios = &mut (*ia).folio; (*ia).ap.descs = &mut (*ia).desc; (*ia).folio = folio; (*ia).desc.length = CUSE_INIT_INFO_MAX; (*ia).fc = &mut (*cc).fc; (*ia).ap.args.end = Some(cuse_process_init_reply);
    let rc = fuse_simple_background(&mut (*cc).fm, &mut (*ia).ap.args, GFP_KERNEL);
    if rc != 0 { kfree(ia as *mut _); folio_put(folio); } rc
}

unsafe extern "C" fn cuse_channel_open(_inode: *mut Inode, file: *mut File) -> Int {
    let fch = fuse_dev_chan_new(); if fch.is_null() { return -ENOMEM; }
    let cc = kzalloc_cuse_conn(); if cc.is_null() { fuse_chan_free(fch); return -ENOMEM; }
    fuse_conn_init(&mut (*cc).fc, &mut (*cc).fm, (*(*file).f_cred).user_ns, fch); (*cc).fc.release = Some(cuse_fc_release);
    let fud = fuse_dev_alloc_install((*cc).fc.chan); fuse_conn_put(&mut (*cc).fc); if fud.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD!(&mut (*cc).list); smp_store_release!(&mut (*(*cc).fc.chan).initialized, 1);
    let rc = cuse_send_init(cc); if rc != 0 { fuse_dev_put(fud); return rc; } (*file).private_data = fud as *mut _; 0
}

unsafe extern "C" fn cuse_channel_release(inode: *mut Inode, file: *mut File) -> Int {
    let fud = fuse_get_dev(file); let cc = fc_to_cc((*(*fud).chan).conn); mutex_lock(&mut CUSE_LOCK); list_del_init(&mut (*cc).list); mutex_unlock(&mut CUSE_LOCK);
    if !(*cc).dev.is_null() { device_unregister((*cc).dev); } if !(*cc).cdev.is_null() { unregister_chrdev_region((*(*cc).cdev).dev, 1); cdev_del((*cc).cdev); } fuse_dev_release(inode, file)
}

unsafe extern "C" fn cuse_init() -> Int {
    for i in 0..CUSE_CONNTBL_LEN { INIT_LIST_HEAD!(&mut CUSE_CONNTBL[i]); }
    CUSE_CLASS = class_create(b"cuse\0".as_ptr() as *const CChar); if is_err(CUSE_CLASS) { return ptr_err(CUSE_CLASS); }
    let rc = misc_register(CUSE_MINOR, b"cuse\0".as_ptr() as *const CChar); if rc != 0 { class_destroy(CUSE_CLASS); return rc; } 0
}
unsafe extern "C" fn cuse_exit() { misc_deregister(CUSE_MINOR); class_destroy(CUSE_CLASS); rcu_barrier(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
