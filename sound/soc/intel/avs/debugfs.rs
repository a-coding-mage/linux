// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type u32 = u32;

const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: usize = 4096;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const TASK_INTERRUPTIBLE: c_int = 1;
const AVS_IPC_NOT_SUPPORTED: c_int = 0;
const AVS_LOG_ENABLE: c_uint = 1;
const AVS_LOG_DISABLE: c_uint = 0;
const AVS_FW_REGS_SIZE: usize = 0;
const AVS_FW_REGS_WINDOW: c_uint = 0;
const AVS_DEBUG_WINDOW: c_uint = 0;
const AVS_WINDOW_CHUNK_SIZE: usize = 0;
const DISABLE_TIMERS: c_uint = c_uint::MAX;

#[repr(C)]
pub struct __kfifo {
    pub in_: c_uint,
    pub out: c_uint,
    pub mask: c_uint,
    pub esize: c_uint,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct kfifo {
    pub kfifo: __kfifo,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

#[repr(C)]
pub struct avs_dev {
    pub trace_fifo: kfifo,
    pub trace_waitq: wait_queue_head_t,
    pub trace_lock: spinlock_t,
    pub logged_resources: c_ulong,
    pub hw_cfg: avs_hw_cfg,
    pub dev: *mut device,
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub dsp_cores: u32,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
}

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct dentry;

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct avs_probe_point_desc {
    pub id: avs_probe_point_id,
    pub purpose: u32,
    pub node_id: avs_node_id,
}

#[repr(C)]
pub union avs_probe_point_id {
    pub value: u32,
}

#[repr(C)]
pub union avs_node_id {
    pub val: u32,
}

#[repr(C)]
pub union avs_notify_msg {
    pub log: avs_log_buffer_status_msg,
}

#[repr(C)]
pub struct avs_log_buffer_status_msg {
    pub core: c_ulong,
}

unsafe extern "C" {
    static mut snd_soc_debugfs_root: *mut dentry;
    static mut current: *mut task_struct;

    fn kfifo_avail(fifo: *mut kfifo) -> c_uint;
    fn kfifo_size(fifo: *mut kfifo) -> c_uint;
    fn kfifo_initialized(fifo: *mut kfifo) -> bool;
    fn kfifo_is_empty(fifo: *mut kfifo) -> bool;
    fn kfifo_to_user(fifo: *mut kfifo, to: *mut c_char, len: size_t, copied: *mut c_uint) -> c_int;
    fn kfifo_alloc(fifo: *mut kfifo, size: c_uint, gfp_mask: c_uint) -> c_int;
    fn kfifo_free(fifo: *mut kfifo);

    fn memcpy_fromio(to: *mut c_void, from: *const c_void, len: size_t);
    fn smp_mb();
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn avs_sram_addr(adev: *mut avs_dev, window: c_uint) -> *const c_void;
    fn avs_ipc_probe_get_points(
        adev: *mut avs_dev,
        desc: *mut *mut avs_probe_point_desc,
        num_desc: *mut size_t,
    ) -> c_int;
    fn avs_ipc_probe_connect_points(
        adev: *mut avs_dev,
        desc: *mut avs_probe_point_desc,
        num_desc: size_t,
    ) -> c_int;
    fn avs_ipc_probe_disconnect_points(
        adev: *mut avs_dev,
        id: *mut avs_probe_point_id,
        num_desc: size_t,
    ) -> c_int;
    fn AVS_IPC_RET(ret: c_int) -> c_int;
    fn parse_int_array_user(from: *const c_char, count: size_t, array: *mut *mut c_int) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn wake_up(wq: *mut wait_queue_head_t);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn prepare_to_wait(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t, state: c_int);
    fn finish_wait(wq: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn signal_pending(p: *mut task_struct) -> c_int;
    fn schedule();
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn avs_dsp_disable_d0ix(adev: *mut avs_dev) -> c_int;
    fn avs_dsp_enable_d0ix(adev: *mut avs_dev) -> c_int;
    fn avs_ipc_set_system_time(adev: *mut avs_dev) -> c_int;
    fn avs_dsp_op(adev: *mut avs_dev, op: avs_dsp_op_id, ...) -> c_int;
    fn AVS_NOTIFICATION(kind: avs_notify_kind) -> avs_notify_msg;
    fn hweight_long(w: c_ulong) -> c_uint;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_create_u32(name: *const c_char, mode: c_uint, parent: *mut dentry, value: *mut u32);
    fn debugfs_remove_recursive(dentry: *mut dentry);
}

#[repr(C)]
pub enum avs_dsp_op_id {
    log_buffer_status,
    enable_logs,
}

#[repr(C)]
pub enum avs_notify_kind {
    LOG_BUFFER_STATUS,
}

unsafe fn __kfifo_fromio(fifo: *mut kfifo, src: *const c_void, mut len: c_uint) -> c_uint {
    let __fifo: *mut __kfifo = ptr::addr_of_mut!((*fifo).kfifo);
    let l: c_uint;
    let off: c_uint;

    len = core::cmp::min(len, kfifo_avail(fifo));
    off = (*__fifo).in_ & (*__fifo).mask;
    l = core::cmp::min(len, kfifo_size(fifo) - off);

    memcpy_fromio(((*__fifo).data as *mut u8).add(off as usize) as *mut c_void, src, l as size_t);
    memcpy_fromio((*__fifo).data, (src as *const u8).add(l as usize) as *const c_void, (len - l) as size_t);
    /* Make sure data copied from SRAM is visible to all CPUs. */
    smp_mb();
    (*__fifo).in_ = (*__fifo).in_.wrapping_add(len);

    len
}

pub unsafe extern "C" fn avs_logging_fw(adev: *mut avs_dev) -> bool {
    kfifo_initialized(ptr::addr_of_mut!((*adev).trace_fifo))
}

pub unsafe extern "C" fn avs_dump_fw_log(adev: *mut avs_dev, src: *const c_void, len: c_uint) {
    __kfifo_fromio(ptr::addr_of_mut!((*adev).trace_fifo), src, len);
}

pub unsafe extern "C" fn avs_dump_fw_log_wakeup(adev: *mut avs_dev, src: *const c_void, len: c_uint) {
    avs_dump_fw_log(adev, src, len);
    wake_up(ptr::addr_of_mut!((*adev).trace_waitq));
}

unsafe extern "C" fn fw_regs_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let buf: *mut c_char;
    let ret: c_int;

    buf = kzalloc(AVS_FW_REGS_SIZE, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    memcpy_fromio(buf as *mut c_void, avs_sram_addr(adev, AVS_FW_REGS_WINDOW), AVS_FW_REGS_SIZE);

    ret = simple_read_from_buffer(to, count, ppos, buf as *const c_void, AVS_FW_REGS_SIZE) as c_int;
    kfree(buf as *mut c_void);
    ret as ssize_t
}

static fw_regs_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(fw_regs_read),
    write: None,
    llseek: None,
    release: None,
};

unsafe extern "C" fn debug_window_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let size: size_t;
    let buf: *mut c_char;
    let ret: c_int;

    size = ((*adev).hw_cfg.dsp_cores as size_t) * AVS_WINDOW_CHUNK_SIZE;
    buf = kzalloc(size, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    memcpy_fromio(buf as *mut c_void, avs_sram_addr(adev, AVS_DEBUG_WINDOW), size);

    ret = simple_read_from_buffer(to, count, ppos, buf as *const c_void, size) as c_int;
    kfree(buf as *mut c_void);
    ret as ssize_t
}

static debug_window_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(debug_window_read),
    write: None,
    llseek: None,
    release: None,
};

unsafe extern "C" fn probe_points_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let mut desc: *mut avs_probe_point_desc = ptr::null_mut();
    let mut num_desc: size_t = 0;
    let mut len: size_t = 0;
    let buf: *mut c_char;
    let mut i: c_int;
    let mut ret: c_int;

    /* Prevent chaining, send and dump IPC value just once. */
    if *ppos != 0 {
        return 0;
    }

    buf = kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    ret = avs_ipc_probe_get_points(adev, &mut desc, &mut num_desc);
    if ret != 0 {
        ret = AVS_IPC_RET(ret);
        kfree(buf as *mut c_void);
        return ret as ssize_t;
    }

    i = 0;
    while (i as size_t) < num_desc {
        ret = scnprintf(
            buf.add(len),
            PAGE_SIZE - len,
            b"Id: %#010x  Purpose: %d  Node id: %#x\n\0".as_ptr() as *const c_char,
            (*desc.add(i as usize)).id.value,
            (*desc.add(i as usize)).purpose,
            (*desc.add(i as usize)).node_id.val,
        );
        len += ret as size_t;
        i += 1;
    }

    ret = simple_read_from_buffer(to, count, ppos, buf as *const c_void, len) as c_int;
    kfree(desc as *mut c_void);
    kfree(buf as *mut c_void);
    ret as ssize_t
}

unsafe extern "C" fn probe_points_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let desc: *mut avs_probe_point_desc;
    let mut array: *mut u32 = ptr::null_mut();
    let num_elems: u32;
    let bytes: size_t;
    let mut ret: c_int;

    ret = parse_int_array_user(from, count, &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret != 0 {
        return ret as ssize_t;
    }

    num_elems = *array;
    bytes = mem::size_of::<u32>() * num_elems as size_t;
    if bytes % mem::size_of::<avs_probe_point_desc>() != 0 {
        ret = -EINVAL;
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    desc = array.add(1) as *mut avs_probe_point_desc;
    ret = avs_ipc_probe_connect_points(adev, desc, bytes / mem::size_of::<avs_probe_point_desc>());
    if ret != 0 {
        ret = AVS_IPC_RET(ret);
    } else {
        ret = count as c_int;
    }
    kfree(array as *mut c_void);
    ret as ssize_t
}

static probe_points_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(probe_points_read),
    write: Some(probe_points_write),
    llseek: None,
    release: None,
};

unsafe extern "C" fn probe_points_disconnect_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let id: *mut avs_probe_point_id;
    let mut array: *mut u32 = ptr::null_mut();
    let num_elems: u32;
    let bytes: size_t;
    let mut ret: c_int;

    ret = parse_int_array_user(from, count, &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret != 0 {
        return ret as ssize_t;
    }

    num_elems = *array;
    bytes = mem::size_of::<u32>() * num_elems as size_t;
    if bytes % mem::size_of::<avs_probe_point_id>() != 0 {
        ret = -EINVAL;
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    id = array.add(1) as *mut avs_probe_point_id;
    ret = avs_ipc_probe_disconnect_points(adev, id, bytes / mem::size_of::<avs_probe_point_id>());
    if ret != 0 {
        ret = AVS_IPC_RET(ret);
    } else {
        ret = count as c_int;
    }
    kfree(array as *mut c_void);
    ret as ssize_t
}

static probe_points_disconnect_fops: file_operations = file_operations {
    open: Some(simple_open),
    write: Some(probe_points_disconnect_write),
    llseek: Some(default_llseek),
    read: None,
    release: None,
};

unsafe extern "C" fn strace_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let fifo: *mut kfifo = ptr::addr_of_mut!((*adev).trace_fifo);
    let mut copied: c_uint = 0;

    if kfifo_is_empty(fifo) {
        let mut wait = MaybeUninit::<wait_queue_entry_t>::zeroed();

        prepare_to_wait(ptr::addr_of_mut!((*adev).trace_waitq), wait.as_mut_ptr(), TASK_INTERRUPTIBLE);
        if signal_pending(current) == 0 {
            schedule();
        }
        finish_wait(ptr::addr_of_mut!((*adev).trace_waitq), wait.as_mut_ptr());
    }

    if kfifo_to_user(fifo, to, count, &mut copied) != 0 {
        return -(EFAULT as ssize_t);
    }
    *ppos += copied as loff_t;
    copied as ssize_t
}

unsafe extern "C" fn strace_open(inode: *mut inode, file: *mut file) -> c_int {
    let adev: *mut avs_dev = (*inode).i_private as *mut avs_dev;
    let ret: c_int;

    if !try_module_get((*(*adev).dev).driver.as_ref().unwrap().owner) {
        return -ENODEV;
    }

    if kfifo_initialized(ptr::addr_of_mut!((*adev).trace_fifo)) {
        return -EBUSY;
    }

    ret = kfifo_alloc(ptr::addr_of_mut!((*adev).trace_fifo), PAGE_SIZE as c_uint, GFP_KERNEL);
    if ret < 0 {
        return ret;
    }

    (*file).private_data = adev as *mut c_void;
    0
}

unsafe extern "C" fn strace_release(_inode: *mut inode, file: *mut file) -> c_int {
    let mut msg: avs_notify_msg = AVS_NOTIFICATION(avs_notify_kind::LOG_BUFFER_STATUS);
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let resource_mask: c_ulong;
    let mut i: c_ulong;
    let num_cores: u32;

    resource_mask = (*adev).logged_resources;
    num_cores = (*adev).hw_cfg.dsp_cores;

    let flags = spin_lock_irqsave(ptr::addr_of_mut!((*adev).trace_lock));
    /* Gather any remaining logs. */
    i = 0;
    while i < num_cores as c_ulong {
        if (resource_mask & (1 as c_ulong).wrapping_shl(i as u32)) != 0 {
            msg.log.core = i;
            avs_dsp_op(adev, avs_dsp_op_id::log_buffer_status, &mut msg);
        }
        i += 1;
    }

    kfifo_free(ptr::addr_of_mut!((*adev).trace_fifo));
    spin_unlock_irqrestore(ptr::addr_of_mut!((*adev).trace_lock), flags);

    module_put((*(*adev).dev).driver.as_ref().unwrap().owner);
    0
}

static strace_fops: file_operations = file_operations {
    llseek: Some(default_llseek),
    read: Some(strace_read),
    open: Some(strace_open),
    release: Some(strace_release),
    write: None,
};

unsafe fn enable_logs(adev: *mut avs_dev, resource_mask: u32, priorities: *mut u32) -> c_int {
    let mut ret: c_int;

    /* Logging demands D0i0 state from DSP. */
    if (*adev).logged_resources == 0 {
        pm_runtime_get_sync((*adev).dev);

        ret = avs_dsp_disable_d0ix(adev);
        if ret != 0 {
            pm_runtime_put_autosuspend((*adev).dev);
            return ret;
        }
    }

    ret = avs_ipc_set_system_time(adev);
    if ret != 0 && ret != AVS_IPC_NOT_SUPPORTED {
        ret = AVS_IPC_RET(ret);
        if (*adev).logged_resources == 0 {
            avs_dsp_enable_d0ix(adev);
            pm_runtime_put_autosuspend((*adev).dev);
        }
        return ret;
    }

    ret = avs_dsp_op(
        adev,
        avs_dsp_op_id::enable_logs,
        AVS_LOG_ENABLE,
        (*adev).aging_timer_period,
        (*adev).fifo_full_timer_period,
        resource_mask,
        priorities,
    );
    if ret != 0 {
        if (*adev).logged_resources == 0 {
            avs_dsp_enable_d0ix(adev);
            pm_runtime_put_autosuspend((*adev).dev);
        }
        return ret;
    }

    (*adev).logged_resources |= resource_mask as c_ulong;
    0
}

unsafe fn disable_logs(adev: *mut avs_dev, resource_mask: u32) -> c_int {
    let ret: c_int;

    /* Check if there's anything to do. */
    if (*adev).logged_resources == 0 {
        return 0;
    }

    ret = avs_dsp_op(
        adev,
        avs_dsp_op_id::enable_logs,
        AVS_LOG_DISABLE,
        DISABLE_TIMERS,
        DISABLE_TIMERS,
        resource_mask,
        ptr::null_mut::<c_void>(),
    );

    /*
     * If IPC fails causing recovery, logged_resources is already zero
     * so unsetting bits is still safe.
     */
    (*adev).logged_resources &= !(resource_mask as c_ulong);

    /* If that's the last resource, allow for D3. */
    if (*adev).logged_resources == 0 {
        avs_dsp_enable_d0ix(adev);
        pm_runtime_put_autosuspend((*adev).dev);
    }

    ret
}

unsafe extern "C" fn trace_control_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let mut buf = [0 as c_char; 64];
    let len: c_int;

    len = snprintf(
        buf.as_mut_ptr(),
        mem::size_of_val(&buf),
        b"0x%08x\n\0".as_ptr() as *const c_char,
        (*adev).logged_resources as u32,
    );

    simple_read_from_buffer(to, count, ppos, buf.as_ptr() as *const c_void, len as size_t)
}

unsafe extern "C" fn trace_control_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let adev: *mut avs_dev = (*file).private_data as *mut avs_dev;
    let mut array: *mut u32 = ptr::null_mut();
    let num_elems: u32;
    let resource_mask: u32;
    let mut ret: c_int;

    ret = parse_int_array_user(from, count, &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret != 0 {
        return ret as ssize_t;
    }

    num_elems = *array;
    if num_elems == 0 {
        ret = -EINVAL;
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    /*
     * Disable if just resource mask is provided - no log priority flags.
     *
     * Enable input format:   mask, prio1, .., prioN
     * Where 'N' equals number of bits set in the 'mask'.
     */
    resource_mask = *array.add(1);
    if num_elems == 1 {
        ret = disable_logs(adev, resource_mask);
    } else {
        if num_elems != hweight_long(resource_mask as c_ulong) + 1 {
            ret = -EINVAL;
            kfree(array as *mut c_void);
            return ret as ssize_t;
        }

        ret = enable_logs(adev, resource_mask, array.add(2));
    }

    if ret == 0 {
        ret = count as c_int;
    }
    kfree(array as *mut c_void);
    ret as ssize_t
}

static trace_control_fops: file_operations = file_operations {
    llseek: Some(default_llseek),
    read: Some(trace_control_read),
    write: Some(trace_control_write),
    open: Some(simple_open),
    release: None,
};

pub unsafe extern "C" fn avs_debugfs_init(adev: *mut avs_dev) {
    init_waitqueue_head(ptr::addr_of_mut!((*adev).trace_waitq));
    spin_lock_init(ptr::addr_of_mut!((*adev).trace_lock));

    (*adev).debugfs_root = debugfs_create_dir(b"avs\0".as_ptr() as *const c_char, snd_soc_debugfs_root);

    /* Initialize timer periods with recommended defaults. */
    (*adev).aging_timer_period = 10;
    (*adev).fifo_full_timer_period = 10;

    debugfs_create_file(
        b"strace\0".as_ptr() as *const c_char,
        0o444,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &strace_fops,
    );
    debugfs_create_file(
        b"trace_control\0".as_ptr() as *const c_char,
        0o644,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &trace_control_fops,
    );
    debugfs_create_file(
        b"fw_regs\0".as_ptr() as *const c_char,
        0o444,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &fw_regs_fops,
    );
    debugfs_create_file(
        b"debug_window\0".as_ptr() as *const c_char,
        0o444,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &debug_window_fops,
    );

    debugfs_create_u32(
        b"trace_aging_period\0".as_ptr() as *const c_char,
        0o644,
        (*adev).debugfs_root,
        ptr::addr_of_mut!((*adev).aging_timer_period),
    );
    debugfs_create_u32(
        b"trace_fifo_full_period\0".as_ptr() as *const c_char,
        0o644,
        (*adev).debugfs_root,
        ptr::addr_of_mut!((*adev).fifo_full_timer_period),
    );

    debugfs_create_file(
        b"probe_points\0".as_ptr() as *const c_char,
        0o644,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &probe_points_fops,
    );
    debugfs_create_file(
        b"probe_points_disconnect\0".as_ptr() as *const c_char,
        0o200,
        (*adev).debugfs_root,
        adev as *mut c_void,
        &probe_points_disconnect_fops,
    );
}

pub unsafe extern "C" fn avs_debugfs_exit(adev: *mut avs_dev) {
    debugfs_remove_recursive((*adev).debugfs_root);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
