// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies are supplied by other translation units.

pub static mut root_mountflags: i32 = MS_RDONLY | MS_SILENT;
static mut saved_root_name: [u8; 64] = [0; 64];
static mut root_wait: i32 = 0;
pub static mut ROOT_DEV: dev_t = 0;

unsafe fn readonly(str_: *mut i8) -> i32 {
    if *str_ != 0 { return 0; }
    root_mountflags |= MS_RDONLY;
    1
}

unsafe fn readwrite(str_: *mut i8) -> i32 {
    if *str_ != 0 { return 0; }
    root_mountflags &= !MS_RDONLY;
    1
}

unsafe fn root_dev_setup(line: *mut i8) -> i32 {
    strscpy(saved_root_name.as_mut_ptr(), line, saved_root_name.len());
    1
}

unsafe fn rootwait_setup(str_: *mut i8) -> i32 {
    if *str_ != 0 { return 0; }
    root_wait = -1;
    1
}

unsafe fn rootwait_timeout_setup(str_: *mut i8) -> i32 {
    let mut sec: i32 = 0;
    if kstrtoint(str_, 0, &mut sec) != 0 || sec < 0 {
        pr_warn(c"ignoring invalid rootwait value\n");
        root_wait = -1;
        return 1;
    }
    if check_mul_overflow(sec, MSEC_PER_SEC, &mut root_wait) != 0 {
        pr_warn(c"ignoring excessive rootwait value\n");
        root_wait = -1;
        return 1;
    }
    1
}

static mut root_mount_data: *mut i8 = core::ptr::null_mut();
unsafe fn root_data_setup(str_: *mut i8) -> i32 { root_mount_data = str_; 1 }
static mut root_fs_names: *mut i8 = core::ptr::null_mut();
unsafe fn fs_names_setup(str_: *mut i8) -> i32 { root_fs_names = str_; 1 }
static mut root_delay: u32 = 0;
unsafe fn root_delay_setup(str_: *mut i8) -> i32 {
    if kstrtouint(str_, 0, &mut root_delay) != 0 { return 0; }
    1
}

unsafe fn split_fs_names(page: *mut i8, size: usize) -> i32 {
    let mut count = 1;
    strscpy(page, root_fs_names, size);
    let mut p = page;
    while *p != 0 {
        p = p.add(1);
        if *p.offset(-1) == b',' as i8 { *p.offset(-1) = 0; count += 1; }
    }
    count
}

unsafe fn do_mount_root(name: *const i8, fs: *const i8, flags: i32, data: *const core::ffi::c_void) -> i32 {
    let mut data_page: *mut i8 = core::ptr::null_mut();
    if !data.is_null() {
        data_page = kmalloc(PAGE_SIZE, GFP_KERNEL);
        if data_page.is_null() { return -ENOMEM; }
        strscpy_pad(data_page, data, PAGE_SIZE);
    }
    let ret = init_mount(name, c"/root".as_ptr(), fs, flags, data_page);
    if ret == 0 {
        init_chdir(c"/root".as_ptr());
        let s = (*(*current).fs).pwd.dentry.dereference().d_sb;
        ROOT_DEV = (*s).s_dev;
        printk(KERN_INFO c"VFS: Mounted root (%s filesystem)%s on device %u:%u.\n",
            (*(*s).s_type).name, if sb_rdonly(s) { c" readonly".as_ptr() } else { c"".as_ptr() },
            MAJOR(ROOT_DEV), MINOR(ROOT_DEV));
    }
    kfree(data_page as *mut core::ffi::c_void);
    ret
}

pub unsafe fn mount_root_generic(name: *mut i8, pretty_name: *mut i8, mut flags: i32) {
    let fs_names = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if fs_names.is_null() { panic(c"VFS: Unable to mount root fs: not enough memory"); }
    let mut b = [0i8; BDEVNAME_SIZE];
    scnprintf(b.as_mut_ptr(), BDEVNAME_SIZE, c"unknown-block(%u,%u)", MAJOR(ROOT_DEV), MINOR(ROOT_DEV));
    let mut num_fs = if !root_fs_names.is_null() { split_fs_names(fs_names, PAGE_SIZE) } else { list_bdev_fs_names(fs_names, PAGE_SIZE) };
    'retry: loop {
        let mut p = fs_names;
        let mut i = 0;
        while i < num_fs {
            let err;
            if *p == 0 { p = p.add(strlen(p) + 1); i += 1; continue; }
            err = do_mount_root(name, p, flags, root_mount_data as *const _);
            match err {
                0 => break 'retry,
                -EACCES | -EINVAL => { #[cfg(CONFIG_BLOCK)] init_flush_fput(); p = p.add(strlen(p) + 1); i += 1; continue; }
                _ => {}
            }
            printk(c"VFS: Cannot open root device \"%s\" or %s: error %d\n", pretty_name, b.as_ptr(), err);
            printk(c"Please append a correct \"root=\" boot option; here are the available partitions:\n");
            printk_all_partitions();
            if !root_fs_names.is_null() { num_fs = list_bdev_fs_names(fs_names, PAGE_SIZE); }
            if num_fs == 0 { pr_err(c"Can't find any bdev filesystem to be used for mount!\n"); }
            else { pr_err(c"List of all bdev filesystems:\n"); let mut q = fs_names; for _ in 0..num_fs { pr_err(c" %s", q); q = q.add(strlen(q) + 1); } pr_err(c"\n"); }
            panic(c"VFS: Unable to mount root fs on %s", b.as_ptr());
        }
        if flags & SB_RDONLY == 0 { flags |= SB_RDONLY; continue; }
        printk(c"List of all partitions:\n"); printk_all_partitions(); printk(c"No filesystem could mount root, tried: ");
        let mut q = fs_names; for _ in 0..num_fs { printk(c" %s", q); q = q.add(strlen(q) + 1); } printk(c"\n");
        panic(c"VFS: Unable to mount root fs on \"%s\" or %s", pretty_name, b.as_ptr());
    }
    kfree(fs_names as *mut _);
}

#[cfg(CONFIG_ROOT_NFS)]
unsafe fn mount_nfs_root() {
    let (mut root_dev, mut root_data) = (core::ptr::null_mut(), core::ptr::null_mut());
    if nfs_root_data(&mut root_dev, &mut root_data) != 0 { pr_err(c"VFS: Unable to mount root fs via NFS.\n"); return; }
    let mut timeout = 5u32;
    for attempt in 1.. { if do_mount_root(root_dev, c"nfs".as_ptr(), root_mountflags, root_data) == 0 { return; } if attempt > 5 { break; } ssleep(timeout); timeout <<= 1; if timeout > 30 { timeout = 30; } }
    pr_err(c"VFS: Unable to mount root fs via NFS.\n");
}
#[cfg(not(CONFIG_ROOT_NFS))] unsafe fn mount_nfs_root() {}

#[cfg(CONFIG_CIFS_ROOT)]
unsafe fn mount_cifs_root() {
    let (mut root_dev, mut root_data) = (core::ptr::null_mut(), core::ptr::null_mut());
    if cifs_root_data(&mut root_dev, &mut root_data) != 0 { pr_err(c"VFS: Unable to mount root fs via SMB.\n"); return; }
    let mut timeout = 5u32;
    for attempt in 1.. { if do_mount_root(root_dev, c"cifs".as_ptr(), root_mountflags, root_data) == 0 { return; } if attempt > 5 { break; } ssleep(timeout); timeout <<= 1; if timeout > 30 { timeout = 30; } }
    pr_err(c"VFS: Unable to mount root fs via SMB.\n");
}
#[cfg(not(CONFIG_CIFS_ROOT))] unsafe fn mount_cifs_root() {}

unsafe fn fs_is_nodev(fstype: *mut i8) -> bool {
    let fs = get_fs_type(fstype);
    if fs.is_null() { return false; }
    let ret = (*fs).fs_flags & FS_REQUIRES_DEV == 0; put_filesystem(fs); ret
}

unsafe fn mount_nodev_root(root_device_name: *mut i8) -> i32 {
    let fs_names = kmalloc(PAGE_SIZE, GFP_KERNEL); if fs_names.is_null() { return -EINVAL; }
    let num_fs = split_fs_names(fs_names, PAGE_SIZE); let mut fstype = fs_names; let mut err = -EINVAL;
    for _ in 0..num_fs { if *fstype != 0 && fs_is_nodev(fstype) { err = do_mount_root(root_device_name, fstype, root_mountflags, root_mount_data as *const _); if err == 0 { break; } } fstype = fstype.add(strlen(fstype) + 1); }
    kfree(fs_names as *mut _); err
}

#[cfg(CONFIG_BLOCK)] unsafe fn mount_block_root(root_device_name: *mut i8) { let err = create_dev(c"/dev/root".as_ptr(), ROOT_DEV); if err < 0 { pr_emerg(c"Failed to create /dev/root: %d\n", err); } mount_root_generic(c"/dev/root".as_ptr() as *mut _, root_device_name, root_mountflags); }
#[cfg(not(CONFIG_BLOCK))] unsafe fn mount_block_root(_: *mut i8) {}

pub unsafe fn mount_root(root_device_name: *mut i8) {
    match ROOT_DEV { Root_NFS => mount_nfs_root(), Root_CIFS => mount_cifs_root(), Root_Generic => mount_root_generic(root_device_name, root_device_name, root_mountflags), 0 => { if !root_device_name.is_null() && !root_fs_names.is_null() && mount_nodev_root(root_device_name) == 0 { return; } mount_block_root(root_device_name); }, _ => mount_block_root(root_device_name) }
}

unsafe fn wait_for_root(root_device_name: *mut i8) {
    if ROOT_DEV != 0 { return; }
    pr_info(c"Waiting for root device %s...\n", root_device_name);
    let end = ktime_add_ms(ktime_get_raw(), root_wait as i64);
    while !driver_probe_done() || early_lookup_bdev(root_device_name, &mut ROOT_DEV) < 0 { msleep(5); if root_wait > 0 && ktime_after(ktime_get_raw(), end) { break; } }
    async_synchronize_full();
}

unsafe fn parse_root_device(root_device_name: *mut i8) -> dev_t {
    if strncmp(root_device_name, c"mtd".as_ptr(), 3) == 0 || strncmp(root_device_name, c"ubi".as_ptr(), 3) == 0 { return Root_Generic; }
    if strcmp(root_device_name, c"/dev/nfs".as_ptr()) == 0 { return Root_NFS; }
    if strcmp(root_device_name, c"/dev/cifs".as_ptr()) == 0 { return Root_CIFS; }
    if strcmp(root_device_name, c"/dev/ram".as_ptr()) == 0 { return Root_RAM0; }
    let mut dev = 0; let error = early_lookup_bdev(root_device_name, &mut dev);
    if error != 0 { if error == -EINVAL && root_wait != 0 { pr_err(c"Disabling rootwait; root= is invalid.\n"); root_wait = 0; } return 0; } dev
}

pub unsafe fn prepare_namespace() {
    if root_delay != 0 { printk(KERN_INFO c"Waiting %d sec before mounting root device...\n", root_delay); ssleep(root_delay); }
    wait_for_device_probe(); md_run_setup();
    if saved_root_name[0] != 0 { ROOT_DEV = parse_root_device(saved_root_name.as_mut_ptr()); }
    initrd_load(); if root_wait != 0 { wait_for_root(saved_root_name.as_mut_ptr()); } mount_root(saved_root_name.as_mut_ptr()); devtmpfs_mount();
    if init_pivot_root(c".".as_ptr(), c".".as_ptr()) != 0 { pr_err(c"VFS: Failed to pivot into new rootfs\n"); return; }
    if init_umount(c".".as_ptr(), MNT_DETACH) != 0 { pr_err(c"VFS: Failed to unmount old rootfs\n"); return; }
    pr_info(c"VFS: Pivoted into new rootfs\n");
}

static mut is_tmpfs: bool = false;
unsafe fn rootfs_init_fs_context(fc: *mut fs_context) -> i32 { if IS_ENABLED(CONFIG_TMPFS) && is_tmpfs { return shmem_init_fs_context(fc); } ramfs_init_fs_context(fc) }
pub static mut rootfs_fs_type: file_system_type = file_system_type { name: c"rootfs".as_ptr(), init_fs_context: Some(rootfs_init_fs_context), kill_sb: Some(kill_anon_super) };
pub unsafe fn init_rootfs() { if IS_ENABLED(CONFIG_TMPFS) { if saved_root_name[0] == 0 && root_fs_names.is_null() { is_tmpfs = true; } else if !root_fs_names.is_null() && !strstr(root_fs_names, c"tmpfs".as_ptr()).is_null() { is_tmpfs = true; } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
