// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
 */
// pr_fmt(fmt) = "fwctl: " fmt
// C dependencies: linux/fwctl.h, linux/container_of.h, linux/fs.h,
// linux/module.h, linux/sizes.h, linux/slab.h, and uapi/fwctl/fwctl.h.

const FWCTL_MAX_DEVICES: usize = 4096;
const MAX_RPC_LEN: usize = 2 * 1024 * 1024;

static_assert!(FWCTL_MAX_DEVICES < (1usize << MINORBITS));

static mut fwctl_dev: dev_t = 0;
static mut fwctl_ida: ida = ida::default();
static mut fwctl_tainted: c_ulong = 0;

#[repr(C)]
struct fwctl_ucmd {
    uctx: *mut fwctl_uctx,
    ubuffer: *mut c_void,
    cmd: *mut c_void,
    user_size: u32,
}

unsafe fn ucmd_respond(ucmd: *mut fwctl_ucmd, cmd_len: usize) -> c_int {
    if copy_to_user((*ucmd).ubuffer, (*ucmd).cmd,
                    core::cmp::min((*ucmd).user_size as usize, cmd_len)) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn copy_to_user_zero_pad(to: *mut c_void, from: *const c_void,
                                from_len: usize, user_len: usize) -> c_int {
    let copy_len = core::cmp::min(from_len, user_len);
    if copy_to_user(to, from, copy_len) != 0 {
        return -EFAULT;
    }
    if copy_len < user_len && clear_user((to as *mut u8).add(copy_len), user_len - copy_len) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn fwctl_cmd_info(ucmd: *mut fwctl_ucmd) -> c_int {
    let fwctl = (*(*ucmd).uctx).fwctl;
    let cmd = (*ucmd).cmd as *mut fwctl_info;
    let mut driver_info_len: usize = 0;

    if (*cmd).flags != 0 { return -EOPNOTSUPP; }
    if (*(*fwctl).ops).info.is_none() && (*cmd).device_data_len != 0 {
        if clear_user(u64_to_user_ptr((*cmd).out_device_data), (*cmd).device_data_len as usize) != 0 { return -EFAULT; }
    } else if (*cmd).device_data_len != 0 {
        let driver_info = ((*(*fwctl).ops).info.unwrap())((*ucmd).uctx, &mut driver_info_len);
        if IS_ERR(driver_info) { return PTR_ERR(driver_info); }
        if copy_to_user_zero_pad(u64_to_user_ptr((*cmd).out_device_data), driver_info, driver_info_len, (*cmd).device_data_len as usize) != 0 { return -EFAULT; }
    }
    (*cmd).out_device_type = (*(*fwctl).ops).device_type;
    (*cmd).device_data_len = driver_info_len as u32;
    ucmd_respond(ucmd, core::mem::size_of::<fwctl_info>())
}

unsafe fn fwctl_cmd_rpc(ucmd: *mut fwctl_ucmd) -> c_int {
    let fwctl = (*(*ucmd).uctx).fwctl;
    let cmd = (*ucmd).cmd as *mut fwctl_rpc;
    if (*cmd).in_len as usize > MAX_RPC_LEN || (*cmd).out_len as usize > MAX_RPC_LEN { return -EMSGSIZE; }
    match (*cmd).scope {
        FWCTL_RPC_CONFIGURATION | FWCTL_RPC_DEBUG_READ_ONLY => {}
        FWCTL_RPC_DEBUG_WRITE_FULL => {
            if !capable(CAP_SYS_RAWIO) { return -EPERM; }
            if !test_and_set_bit(0, &mut fwctl_tainted) {
                dev_warn(&(*fwctl).dev, "%s(%d): has requested full access to the physical device", current.comm, task_pid_nr(current));
                add_taint(TAINT_FWCTL, LOCKDEP_STILL_OK);
            }
        }
        FWCTL_RPC_DEBUG_WRITE => {
            if !test_and_set_bit(0, &mut fwctl_tainted) {
                dev_warn(&(*fwctl).dev, "%s(%d): has requested full access to the physical device", current.comm, task_pid_nr(current));
                add_taint(TAINT_FWCTL, LOCKDEP_STILL_OK);
            }
        }
        _ => return -EOPNOTSUPP,
    }
    let inbuf = kvzalloc((*cmd).in_len as usize, GFP_KERNEL_ACCOUNT);
    if inbuf.is_null() { return -ENOMEM; }
    if copy_from_user(inbuf, u64_to_user_ptr((*cmd).in), (*cmd).in_len as usize) != 0 { return -EFAULT; }
    let mut out_len = (*cmd).out_len as usize;
    let outbuf = ((*(*fwctl).ops).fw_rpc.unwrap())((*ucmd).uctx, (*cmd).scope, inbuf, (*cmd).in_len as usize, &mut out_len);
    if IS_ERR(outbuf) { return PTR_ERR(outbuf); }
    if outbuf == inbuf { /* The driver can re-use inbuf as outbuf */ }
    if copy_to_user(u64_to_user_ptr((*cmd).out), outbuf, core::cmp::min((*cmd).out_len as usize, out_len)) != 0 { return -EFAULT; }
    (*cmd).out_len = out_len as u32;
    ucmd_respond(ucmd, core::mem::size_of::<fwctl_rpc>())
}

#[repr(C)]
union fwctl_ucmd_buffer { info: fwctl_info, rpc: fwctl_rpc }

#[repr(C)]
struct fwctl_ioctl_op {
    size: c_uint,
    min_size: c_uint,
    ioctl_num: c_uint,
    execute: unsafe fn(*mut fwctl_ucmd) -> c_int,
}

// IOCTL_OP entries, preserving C sizeof/offsetofend and ioctl dispatch layout.
static fwctl_ioctl_ops: [fwctl_ioctl_op; 2] = [
    fwctl_ioctl_op { size: core::mem::size_of::<fwctl_info>() as c_uint, min_size: offsetofend_fwctl_info_out_device_data(), ioctl_num: FWCTL_INFO, execute: fwctl_cmd_info },
    fwctl_ioctl_op { size: core::mem::size_of::<fwctl_rpc>() as c_uint, min_size: offsetofend_fwctl_rpc_out(), ioctl_num: FWCTL_RPC, execute: fwctl_cmd_rpc },
];

unsafe fn fwctl_fops_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let uctx = (*filp).private_data as *mut fwctl_uctx;
    let nr = _IOC_NR(cmd);
    if (nr - FWCTL_CMD_BASE) as usize >= fwctl_ioctl_ops.len() { return -ENOIOCTLCMD as c_long; }
    let op = &fwctl_ioctl_ops[(nr - FWCTL_CMD_BASE) as usize];
    if op.ioctl_num != cmd { return -ENOIOCTLCMD as c_long; }
    let mut ucmd = fwctl_ucmd { uctx, cmd: core::ptr::null_mut(), ubuffer: arg as *mut c_void, user_size: 0 };
    let mut buf = fwctl_ucmd_buffer { info: core::mem::zeroed() };
    ucmd.cmd = &mut buf as *mut _ as *mut c_void;
    let ret = get_user(&mut ucmd.user_size, ucmd.ubuffer as *const u32);
    if ret != 0 { return ret as c_long; }
    if ucmd.user_size < op.min_size { return -EINVAL as c_long; }
    let ret = copy_struct_from_user(ucmd.cmd, op.size as usize, ucmd.ubuffer, ucmd.user_size as usize);
    if ret != 0 { return ret as c_long; }
    if (*uctx).fwctl.ops.is_null() { return -ENODEV as c_long; }
    (op.execute)(&mut ucmd) as c_long
}

unsafe fn fwctl_fops_open(inode: *mut inode, filp: *mut file) -> c_int {
    let fwctl = container_of((*inode).i_cdev, fwctl_device, cdev);
    if (*fwctl).ops.is_null() { return -ENODEV; }
    let uctx = kzalloc((*(*fwctl).ops).uctx_size, GFP_KERNEL_ACCOUNT) as *mut fwctl_uctx;
    if uctx.is_null() { return -ENOMEM; }
    (*uctx).fwctl = fwctl;
    let ret = ((*(*fwctl).ops).open_uctx.unwrap())(uctx);
    if ret != 0 { return ret; }
    list_add_tail(&mut (*uctx).uctx_list_entry, &mut (*fwctl).uctx_list);
    get_device(&mut (*fwctl).dev);
    (*filp).private_data = uctx as *mut c_void;
    0
}

unsafe fn fwctl_destroy_uctx(uctx: *mut fwctl_uctx) {
    list_del(&mut (*uctx).uctx_list_entry);
    ((*(*(*uctx).fwctl).ops).close_uctx.unwrap())(uctx);
}

unsafe fn fwctl_fops_release(_inode: *mut inode, filp: *mut file) -> c_int {
    let uctx = (*filp).private_data as *mut fwctl_uctx;
    let fwctl = (*uctx).fwctl;
    if !(*fwctl).ops.is_null() { fwctl_destroy_uctx(uctx); }
    kfree(uctx as *mut c_void);
    fwctl_put(fwctl);
    0
}

static fwctl_fops: file_operations = file_operations { owner: THIS_MODULE, open: Some(fwctl_fops_open), release: Some(fwctl_fops_release), unlocked_ioctl: Some(fwctl_fops_ioctl) };

unsafe fn fwctl_device_release(device: *mut device) {
    let fwctl = container_of(device, fwctl_device, dev);
    ida_free(&mut fwctl_ida, (*fwctl).dev.devt - fwctl_dev);
    mutex_destroy(&mut (*fwctl).uctx_list_lock);
    kfree(fwctl as *mut c_void);
}

unsafe fn fwctl_devnode(dev: *const device, mode: *mut umode_t) -> *mut c_char { kasprintf(GFP_KERNEL, "fwctl/%s", dev_name(dev)) }

static mut fwctl_class: class = class { name: "fwctl", dev_release: Some(fwctl_device_release), devnode: Some(fwctl_devnode) };

unsafe fn _alloc_device(parent: *mut device, ops: *const fwctl_ops, size: usize) -> *mut fwctl_device {
    let fwctl = kzalloc(size, GFP_KERNEL) as *mut fwctl_device;
    if fwctl.is_null() { return core::ptr::null_mut(); }
    let devnum = ida_alloc_max(&mut fwctl_ida, FWCTL_MAX_DEVICES - 1, GFP_KERNEL);
    if devnum < 0 { return core::ptr::null_mut(); }
    (*fwctl).dev.devt = fwctl_dev + devnum as dev_t;
    (*fwctl).dev.class = &mut fwctl_class;
    (*fwctl).dev.parent = parent;
    init_rwsem(&mut (*fwctl).registration_lock);
    mutex_init(&mut (*fwctl).uctx_list_lock);
    INIT_LIST_HEAD(&mut (*fwctl).uctx_list);
    device_initialize(&mut (*fwctl).dev);
    fwctl
}

#[no_mangle]
pub unsafe extern "C" fn _fwctl_alloc_device(parent: *mut device, ops: *const fwctl_ops, size: usize) -> *mut fwctl_device {
    let fwctl = _alloc_device(parent, ops, size);
    if fwctl.is_null() { return core::ptr::null_mut(); }
    cdev_init(&mut (*fwctl).cdev, &fwctl_fops);
    (*fwctl).cdev.owner = THIS_MODULE;
    if dev_set_name(&mut (*fwctl).dev, "fwctl%d", (*fwctl).dev.devt - fwctl_dev) != 0 { return core::ptr::null_mut(); }
    (*fwctl).ops = ops;
    fwctl
}

pub unsafe extern "C" fn fwctl_register(fwctl: *mut fwctl_device) -> c_int { cdev_device_add(&mut (*fwctl).cdev, &mut (*fwctl).dev) }

pub unsafe extern "C" fn fwctl_unregister(fwctl: *mut fwctl_device) {
    cdev_device_del(&mut (*fwctl).cdev, &mut (*fwctl).dev);
    while !(*fwctl).uctx_list.next.is_null() {
        let uctx = list_first_entry_or_null(&mut (*fwctl).uctx_list, fwctl_uctx, uctx_list_entry);
        if uctx.is_null() { break; }
        fwctl_destroy_uctx(uctx);
    }
    (*fwctl).ops = core::ptr::null();
}

unsafe fn fwctl_init() -> c_int {
    let ret = alloc_chrdev_region(&mut fwctl_dev, 0, FWCTL_MAX_DEVICES as c_uint, "fwctl");
    if ret != 0 { return ret; }
    let ret = class_register(&mut fwctl_class);
    if ret != 0 { unregister_chrdev_region(fwctl_dev, FWCTL_MAX_DEVICES as c_uint); }
    ret
}

unsafe fn fwctl_exit() {
    class_unregister(&mut fwctl_class);
    unregister_chrdev_region(fwctl_dev, FWCTL_MAX_DEVICES as c_uint);
}

// subsys_initcall(fwctl_init);
// module_exit(fwctl_exit);
// MODULE_DESCRIPTION("fwctl device firmware access framework");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
