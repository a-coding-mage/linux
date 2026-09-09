// SPDX-License-Identifier: GPL-2.0
/*
 * bsg.c - block layer implementation of the sg v4 interface
 */

// Kernel dependencies supplied by other translation units are intentionally
// left as external Rust symbols/types.

const BSG_DESCRIPTION: &str = "Block layer SCSI generic (bsg) driver";
const BSG_VERSION: &str = "0.4";

#[repr(C)]
pub struct bsg_device {
    pub queue: *mut request_queue,
    pub device: device,
    pub cdev: cdev,
    pub max_queue: i32,
    pub timeout: u32,
    pub reserved_size: u32,
    pub sg_io_fn: Option<bsg_sg_io_fn>,
    pub uring_cmd_fn: Option<bsg_uring_cmd_fn>,
}

#[inline]
unsafe fn to_bsg_device(inode: *mut inode) -> *mut bsg_device {
    // container_of(inode->i_cdev, struct bsg_device, cdev)
    container_of((*inode).i_cdev, core::mem::offset_of!(bsg_device, cdev))
}

const BSG_DEFAULT_CMDS: i32 = 64;
const BSG_MAX_DEVS: i32 = 1 << MINORBITS;

static mut bsg_minor_ida: ida = ida { _private: [] };
static bsg_class: class = class { _private: [] };
static mut bsg_major: i32 = 0;

unsafe fn bsg_timeout(bd: *mut bsg_device, hdr: *mut sg_io_v4) -> u32 {
    let mut timeout: u32 = BLK_DEFAULT_SG_TIMEOUT;
    if (*hdr).timeout != 0 {
        timeout = msecs_to_jiffies((*hdr).timeout);
    } else if (*bd).timeout != 0 {
        timeout = (*bd).timeout;
    }
    core::cmp::max(timeout, BLK_MIN_SG_TIMEOUT)
}

unsafe fn bsg_sg_io(bd: *mut bsg_device, open_for_write: bool, uarg: *mut core::ffi::c_void) -> i32 {
    let mut hdr: sg_io_v4 = core::mem::zeroed();
    if copy_from_user(&mut hdr as *mut _ as *mut core::ffi::c_void, uarg, core::mem::size_of::<sg_io_v4>()) != 0 { return -EFAULT; }
    if hdr.guard != b'Q' as _ { return -EINVAL; }
    let ret = ((*bd).sg_io_fn.unwrap())((*bd).queue, &mut hdr, open_for_write, bsg_timeout(bd, &mut hdr));
    if ret == 0 && copy_to_user(uarg, &hdr as *const _ as *const core::ffi::c_void, core::mem::size_of::<sg_io_v4>()) != 0 { return -EFAULT; }
    ret
}

unsafe fn bsg_open(inode: *mut inode, _file: *mut file) -> i32 {
    if !blk_get_queue((*to_bsg_device(inode)).queue) { return -ENXIO; }
    0
}

unsafe fn bsg_release(inode: *mut inode, _file: *mut file) -> i32 { blk_put_queue((*to_bsg_device(inode)).queue); 0 }

unsafe fn bsg_get_command_q(bd: *mut bsg_device, uarg: *mut i32) -> i32 { put_user(core::ptr::read_volatile(&(*bd).max_queue), uarg) }

unsafe fn bsg_set_command_q(bd: *mut bsg_device, uarg: *mut i32) -> i32 {
    let mut max_queue = 0;
    if get_user(&mut max_queue, uarg) != 0 { return -EFAULT; }
    if max_queue < 1 { return -EINVAL; }
    core::ptr::write_volatile(&mut (*bd).max_queue, max_queue); 0
}

unsafe fn bsg_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let bd = to_bsg_device(file_inode(file));
    let q = (*bd).queue;
    let uarg = arg as *mut core::ffi::c_void;
    let intp = uarg as *mut i32;
    let mut val = 0i32;
    match cmd {
        SG_GET_COMMAND_Q => bsg_get_command_q(bd, intp),
        SG_SET_COMMAND_Q => bsg_set_command_q(bd, intp),
        SG_GET_VERSION_NUM => put_user(30527, intp),
        SCSI_IOCTL_GET_IDLUN | SCSI_IOCTL_GET_BUS_NUMBER => put_user(0, intp),
        SG_SET_TIMEOUT => { if get_user(&mut val, intp) != 0 { return -EFAULT as isize; } (*bd).timeout = clock_t_to_jiffies(val); 0 },
        SG_GET_TIMEOUT => jiffies_to_clock_t((*bd).timeout),
        SG_GET_RESERVED_SIZE => put_user(core::cmp::min((*bd).reserved_size, queue_max_bytes(q)), intp),
        SG_SET_RESERVED_SIZE => { if get_user(&mut val, intp) != 0 { return -EFAULT as isize; } if val < 0 { return -EINVAL as isize; } (*bd).reserved_size = core::cmp::min(val as u32, queue_max_bytes(q)); 0 },
        SG_EMULATED_HOST => put_user(1, intp),
        SG_IO => bsg_sg_io(bd, (*file).f_mode & FMODE_WRITE != 0, uarg),
        SCSI_IOCTL_SEND_COMMAND => { pr_warn_ratelimited("%s: calling unsupported SCSI_IOCTL_SEND_COMMAND\n", current.comm); -EINVAL },
        _ => -ENOTTY,
    } as isize
}

unsafe fn bsg_check_uring_features(issue_flags: u32) -> i32 {
    if issue_flags & (IO_URING_F_SQE128 | IO_URING_F_CQE32) != (IO_URING_F_SQE128 | IO_URING_F_CQE32) { return -EOPNOTSUPP; }
    0
}

unsafe fn bsg_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    let bd = to_bsg_device(file_inode((*ioucmd).file));
    let open_for_write = (*(*ioucmd).file).f_mode & FMODE_WRITE != 0;
    let ret = bsg_check_uring_features(issue_flags);
    if ret != 0 { return ret; }
    if (*bd).uring_cmd_fn.is_none() { return -EOPNOTSUPP; }
    ((*bd).uring_cmd_fn.unwrap())((*bd).queue, ioucmd, issue_flags, open_for_write)
}

unsafe fn bsg_device_release(dev: *mut device) {
    let bd = container_of(dev, core::mem::offset_of!(bsg_device, device));
    ida_free(&mut bsg_minor_ida, minor((*bd).device.devt));
    kfree(bd as *mut core::ffi::c_void);
}

pub unsafe fn bsg_unregister_queue(bd: *mut bsg_device) {
    let disk = (*(*bd).queue).disk;
    if !disk.is_null() && !(*disk).queue_kobj.sd.is_null() { sysfs_remove_link(&mut (*disk).queue_kobj, "bsg"); }
    cdev_device_del(&mut (*bd).cdev, &mut (*bd).device);
    put_device(&mut (*bd).device);
}

pub unsafe fn bsg_register_queue(q: *mut request_queue, parent: *mut device, name: *const i8, sg_io_fn: Option<bsg_sg_io_fn>, uring_cmd_fn: Option<bsg_uring_cmd_fn>) -> *mut bsg_device {
    let bd = kzalloc_bsg_device();
    if bd.is_null() { return ERR_PTR(-ENOMEM); }
    (*bd).max_queue = BSG_DEFAULT_CMDS; (*bd).reserved_size = i32::MAX as u32; (*bd).queue = q; (*bd).sg_io_fn = sg_io_fn; (*bd).uring_cmd_fn = uring_cmd_fn;
    let ret = ida_alloc_max(&mut bsg_minor_ida, BSG_MAX_DEVS - 1, GFP_KERNEL);
    if ret < 0 { kfree(bd as *mut _); return ERR_PTR(ret); }
    (*bd).device.devt = mkdev(bsg_major, ret); (*bd).device.class = &bsg_class; (*bd).device.parent = parent; (*bd).device.release = Some(bsg_device_release);
    dev_set_name(&mut (*bd).device, b"%s\0".as_ptr() as *const i8, name); device_initialize(&mut (*bd).device);
    cdev_init(&mut (*bd).cdev, &bsg_fops); (*bd).cdev.owner = THIS_MODULE;
    let ret = cdev_device_add(&mut (*bd).cdev, &mut (*bd).device);
    if ret != 0 { put_device(&mut (*bd).device); return ERR_PTR(ret); }
    bd
}

// The remaining module registration metadata and kernel callback wiring are
// represented by the external kernel environment.
extern "C" {
    static bsg_fops: file_operations;
    fn kzalloc_bsg_device() -> *mut bsg_device;
}

unsafe fn bsg_devnode(dev: *const device, _mode: *mut umode_t) -> *mut i8 {
    kasprintf(GFP_KERNEL, b"bsg/%s\0".as_ptr() as *const i8, dev_name(dev))
}

unsafe fn bsg_init() -> i32 {
    let mut devid = 0 as dev_t;
    let ret = class_register(&bsg_class);
    if ret != 0 { return ret; }
    let ret = alloc_chrdev_region(&mut devid, 0, BSG_MAX_DEVS, b"bsg\0".as_ptr() as *const i8);
    if ret != 0 { class_unregister(&bsg_class); return ret; }
    bsg_major = major(devid);
    printk(KERN_INFO, b"%s version %s loaded (major %d)\n\0".as_ptr() as *const i8, BSG_DESCRIPTION.as_ptr(), BSG_VERSION.as_ptr(), bsg_major);
    0
}

// MODULE_AUTHOR("Jens Axboe");
// MODULE_DESCRIPTION(BSG_DESCRIPTION);
// MODULE_LICENSE("GPL");
// device_initcall(bsg_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
