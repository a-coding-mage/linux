// SPDX-License-Identifier: GPL-2.0
/*
 * Implements pstore backend driver that write to block (or non-block) storage
 * devices, using the pstore/zone API.
 */

// Linux kernel includes and build-time configuration are supplied by the
// surrounding translation unit.

extern "C" {
    static mut kmsg_size: c_long;
    static mut max_reason: c_int;
    static mut pmsg_size: c_long;
    static mut console_size: c_long;
    static mut ftrace_size: c_long;
    static mut best_effort: bool;
    static mut blkdev: [c_char; 80];
    static mut pstore_blk_lock: mutex;
    static mut psblk_file: *mut file;
    static mut pstore_device_info: *mut pstore_device_info;
}

static mut KMSG_SIZE: c_long = CONFIG_PSTORE_BLK_KMSG_SIZE as c_long;
static mut MAX_REASON: c_int = CONFIG_PSTORE_BLK_MAX_REASON as c_int;
#[cfg(CONFIG_PSTORE_PMSG)]
static mut PMSG_SIZE: c_long = CONFIG_PSTORE_BLK_PMSG_SIZE as c_long;
#[cfg(not(CONFIG_PSTORE_PMSG))]
static mut PMSG_SIZE: c_long = -1;
#[cfg(CONFIG_PSTORE_CONSOLE)]
static mut CONSOLE_SIZE: c_long = CONFIG_PSTORE_BLK_CONSOLE_SIZE as c_long;
#[cfg(not(CONFIG_PSTORE_CONSOLE))]
static mut CONSOLE_SIZE: c_long = -1;
#[cfg(CONFIG_PSTORE_FTRACE)]
static mut FTRACE_SIZE: c_long = CONFIG_PSTORE_BLK_FTRACE_SIZE as c_long;
#[cfg(not(CONFIG_PSTORE_FTRACE))]
static mut FTRACE_SIZE: c_long = -1;
static mut BEST_EFFORT: bool = false;
static mut BLKDEV: [c_char; 80] = CONFIG_PSTORE_BLK_BLKDEV;

// Module parameters:
// kmsg_size (long, 0400), max_reason (int, 0400), pmsg_size (long, 0400),
// console_size (long, 0400), ftrace_size (long, 0400), best_effort (bool, 0400),
// and blkdev (string, 80, 0400).

unsafe fn check_size(name: c_long, alignsize: c_long) -> c_long {
    let mut value = if name <= 0 { 0 } else { name.wrapping_mul(1024) };
    if value & (alignsize - 1) != 0 {
        pr_info!("size must align to %d\n", alignsize);
        value = ALIGN(name, alignsize);
    }
    value
}

unsafe fn __register_pstore_device(dev: *mut pstore_device_info) -> c_int {
    let mut ret: c_int;
    lockdep_assert_held!(&raw mut pstore_blk_lock);
    if dev.is_null() { pr_err!("NULL device info\n"); return -EINVAL; }
    if (*dev).zone.total_size == 0 { pr_err!("zero sized device\n"); return -EINVAL; }
    if (*dev).zone.read.is_none() { pr_err!("no read handler for device\n"); return -EINVAL; }
    if (*dev).zone.write.is_none() { pr_err!("no write handler for device\n"); return -EINVAL; }
    if !pstore_device_info.is_null() { return -EBUSY; }
    if (*dev).flags == 0 { (*dev).flags = UINT_MAX; }
    let enabled = (*dev).flags;
    KMSG_SIZE = if enabled & PSTORE_FLAGS_DMESG != 0 { check_size(KMSG_SIZE, 4096) / 1024 } else { 0 };
    PMSG_SIZE = if enabled & PSTORE_FLAGS_PMSG != 0 { check_size(PMSG_SIZE, 4096) / 1024 } else { 0 };
    CONSOLE_SIZE = if enabled & PSTORE_FLAGS_CONSOLE != 0 { check_size(CONSOLE_SIZE, 4096) / 1024 } else { 0 };
    FTRACE_SIZE = if enabled & PSTORE_FLAGS_FTRACE != 0 { check_size(FTRACE_SIZE, 4096) / 1024 } else { 0 };
    (*dev).zone.name = KBUILD_MODNAME;
    (*dev).zone.max_reason = MAX_REASON;
    (*dev).zone.owner = THIS_MODULE;
    ret = register_pstore_zone(&mut (*dev).zone);
    if ret == 0 { pstore_device_info = dev; }
    ret
}

pub unsafe fn register_pstore_device(dev: *mut pstore_device_info) -> c_int {
    mutex_lock(&mut pstore_blk_lock);
    let ret = __register_pstore_device(dev);
    mutex_unlock(&mut pstore_blk_lock);
    ret
}

unsafe fn __unregister_pstore_device(dev: *mut pstore_device_info) {
    lockdep_assert_held!(&raw mut pstore_blk_lock);
    if !pstore_device_info.is_null() && pstore_device_info == dev {
        unregister_pstore_zone(&mut (*dev).zone);
        pstore_device_info = core::ptr::null_mut();
    }
}

pub unsafe fn unregister_pstore_device(dev: *mut pstore_device_info) {
    mutex_lock(&mut pstore_blk_lock);
    __unregister_pstore_device(dev);
    mutex_unlock(&mut pstore_blk_lock);
}

unsafe extern "C" fn psblk_generic_blk_read(buf: *mut c_char, bytes: usize, mut pos: loff_t) -> ssize_t {
    kernel_read(psblk_file, buf, bytes, &mut pos)
}

unsafe extern "C" fn psblk_generic_blk_write(buf: *const c_char, bytes: usize, mut pos: loff_t) -> ssize_t {
    if in_interrupt() || irqs_disabled() { return -EBUSY as ssize_t; }
    kernel_write(psblk_file, buf, bytes, &mut pos)
}

unsafe fn __register_pstore_blk(dev: *mut pstore_device_info, devpath: *const c_char) -> c_int {
    let mut ret = -ENODEV;
    lockdep_assert_held!(&raw mut pstore_blk_lock);
    psblk_file = filp_open(devpath, O_RDWR | O_DSYNC | O_NOATIME | O_EXCL, 0);
    if IS_ERR(psblk_file) { ret = PTR_ERR(psblk_file); pr_err!("failed to open '%s': %d!\n", devpath, ret); return ret; }
    if !S_ISBLK((*file_inode(psblk_file)).i_mode) { pr_err!("'%s' is not block device!\n", devpath); fput(psblk_file); psblk_file = core::ptr::null_mut(); return ret; }
    (*dev).zone.total_size = bdev_nr_bytes(I_BDEV((*psblk_file).f_mapping.host));
    ret = __register_pstore_device(dev);
    if ret != 0 { fput(psblk_file); psblk_file = core::ptr::null_mut(); }
    ret
}

pub unsafe fn pstore_blk_get_config(info: *mut pstore_blk_config) -> c_int {
    strscpy((*info).device.as_mut_ptr(), BLKDEV.as_ptr());
    (*info).max_reason = MAX_REASON;
    (*info).kmsg_size = check_size(KMSG_SIZE, 4096);
    (*info).pmsg_size = check_size(PMSG_SIZE, 4096);
    (*info).ftrace_size = check_size(FTRACE_SIZE, 4096);
    (*info).console_size = check_size(CONSOLE_SIZE, 4096);
    0
}

#[cfg(not(MODULE))]
static DEVNAME: &[u8] = b"/dev/pstore-blk\0";

#[cfg(not(MODULE))]
unsafe fn early_boot_devpath(initial_devname: *const c_char) -> *const c_char {
    let mut dev: dev_t = 0;
    if early_lookup_bdev(initial_devname, &mut dev) != 0 { pr_err!("failed to resolve '%s'!\n", initial_devname); return initial_devname; }
    init_unlink(DEVNAME.as_ptr() as *const c_char);
    init_mknod(DEVNAME.as_ptr() as *const c_char, S_IFBLK | 0o600, new_encode_dev(dev));
    DEVNAME.as_ptr() as *const c_char
}

#[cfg(MODULE)]
unsafe fn early_boot_devpath(initial_devname: *const c_char) -> *const c_char { initial_devname }

unsafe fn __best_effort_init() -> c_int {
    if !BEST_EFFORT { return 0; }
    if BLKDEV[0] == 0 { pr_err!("blkdev empty with best_effort=Y\n"); return -EINVAL; }
    let dev = kzalloc::<pstore_device_info>();
    if dev.is_null() { return -ENOMEM; }
    (*dev).zone.read = Some(psblk_generic_blk_read);
    (*dev).zone.write = Some(psblk_generic_blk_write);
    let ret = __register_pstore_blk(dev, early_boot_devpath(BLKDEV.as_ptr()));
    if ret != 0 { kfree(dev); } else { pr_info!("attached %s (%lu) (no dedicated panic_write!)\n", BLKDEV.as_ptr(), (*dev).zone.total_size); }
    ret
}

unsafe fn __best_effort_exit() {
    if !psblk_file.is_null() {
        let dev = pstore_device_info;
        __unregister_pstore_device(dev);
        kfree(dev);
        fput(psblk_file);
        psblk_file = core::ptr::null_mut();
    }
}

unsafe fn pstore_blk_init() -> c_int {
    mutex_lock(&mut pstore_blk_lock);
    let ret = __best_effort_init();
    mutex_unlock(&mut pstore_blk_lock);
    ret
}

unsafe fn pstore_blk_exit() {
    mutex_lock(&mut pstore_blk_lock);
    __best_effort_exit();
    __unregister_pstore_device(pstore_device_info);
    mutex_unlock(&mut pstore_blk_lock);
}

// late_initcall(pstore_blk_init); module_exit(pstore_blk_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("WeiXiong Liao <liaoweixiong@allwinnertech.com>");
// MODULE_AUTHOR("Kees Cook <keescook@chromium.org>");
// MODULE_DESCRIPTION("pstore backend for block devices");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
