// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation
//
// Author: Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//

// C dependencies translated from:
// <linux/auxiliary_bus.h>, <linux/completion.h>, <linux/debugfs.h>,
// <linux/ktime.h>, <linux/module.h>, <linux/pm_runtime.h>,
// <linux/slab.h>, <linux/uaccess.h>, <sound/sof/header.h>,
// <sound/sof/ipc4/header.h>, and "sof-client.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type umode_t = u16;
type gfp_t = c_uint;

const SOF_IPC_CLIENT_SUSPEND_DELAY_MS: c_int = 3000;

const ENODEV: c_int = 19;
const ENOSPC: c_int = 28;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const GFP_KERNEL: gfp_t = 0;
const SOF_FW_CRASHED: c_int = 0;
const SOF_IPC_TYPE_4: sof_ipc_type = 4;
const SOF_IPC4_MOD_LARGE_CONFIG_GET: u32 = 0;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
}

#[repr(C)]
pub struct auxiliary_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct auxiliary_driver {
    pub probe: Option<
        unsafe extern "C" fn(*mut auxiliary_device, *const auxiliary_device_id) -> c_int,
    >,
    pub remove: Option<unsafe extern "C" fn(*mut auxiliary_device)>,
    pub id_table: *const auxiliary_device_id,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read:
        Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write:
        Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub owner: *mut c_void,
}

pub type sof_ipc_type = c_int;

#[repr(C)]
pub struct sof_client_dev {
    pub auxdev: auxiliary_device,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct sof_msg_inject_priv {
    pub dfs_file: *mut dentry,
    pub max_msg_size: size_t,
    pub ipc_type: sof_ipc_type,

    pub tx_buffer: *mut c_void,
    pub rx_buffer: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_hdr {
    pub size: u32,
}

#[repr(C)]
pub struct sof_ipc_reply {
    pub hdr: sof_ipc_hdr,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub header_u64: u64,
    pub primary: u32,
    pub data_size: size_t,
    pub data_ptr: *mut c_void,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static default_llseek: unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t;

    fn debugfs_file_get(dentry: *mut dentry) -> c_int;
    fn debugfs_file_put(dentry: *mut dentry);
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_uint;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_uint;
    fn simple_write_to_buffer(
        to: *mut c_void,
        available: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        count: size_t,
    ) -> ssize_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);

    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: gfp_t) -> *mut c_void;
    fn devm_kmalloc(dev: *mut device, size: size_t, flags: gfp_t) -> *mut c_void;

    fn auxiliary_dev_to_sof_client_dev(auxdev: *mut auxiliary_device) -> *mut sof_client_dev;
    fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry;
    fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> c_int;
    fn sof_client_get_ipc_type(cdev: *mut sof_client_dev) -> sof_ipc_type;
    fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> size_t;
    fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int;
    fn sof_client_ipc_tx_message(
        cdev: *mut sof_client_dev,
        tx_buffer: *mut c_void,
        rx_buffer: *mut c_void,
        max_msg_size: size_t,
    ) -> c_int;
    fn debugfs_create_file(
        name: *const c_char,
        mode: umode_t,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
}

#[inline]
fn unlikely(x: c_int) -> bool {
    x != 0
}

#[inline]
unsafe fn SOF_IPC4_MSG_IS_MODULE_MSG(_primary: u32) -> bool {
    todo!("external SOF IPC4 macro dependency")
}

#[inline]
unsafe fn SOF_IPC4_MSG_TYPE_GET(_primary: u32) -> u32 {
    todo!("external SOF IPC4 macro dependency")
}

unsafe extern "C" fn sof_msg_inject_dfs_open(inode: *mut inode, file: *mut file) -> c_int {
    let cdev = unsafe { (*inode).i_private as *mut sof_client_dev };
    let mut ret: c_int;

    if unsafe { sof_client_get_fw_state(cdev) } == SOF_FW_CRASHED {
        return -ENODEV;
    }

    ret = unsafe { debugfs_file_get((*file).f_path.dentry) };
    if unlikely(ret) {
        return ret;
    }

    ret = unsafe { simple_open(inode, file) };
    if ret != 0 {
        unsafe { debugfs_file_put((*file).f_path.dentry) };
    }

    ret
}

unsafe extern "C" fn sof_msg_inject_dfs_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
    let rhdr = unsafe { (*priv_).rx_buffer as *mut sof_ipc_reply };

    if unsafe { (*rhdr).hdr.size == 0 || count == 0 || *ppos != 0 } {
        return 0;
    }

    if count > unsafe { (*rhdr).hdr.size as size_t } {
        count = unsafe { (*rhdr).hdr.size as size_t };
    }

    if unsafe { copy_to_user(buffer as *mut c_void, (*priv_).rx_buffer, count) } != 0 {
        return -(EFAULT as ssize_t);
    }

    unsafe {
        *ppos += count as loff_t;
    }
    count as ssize_t
}

unsafe extern "C" fn sof_msg_inject_ipc4_dfs_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
    let ipc4_msg = unsafe { (*priv_).rx_buffer as *mut sof_ipc4_msg };
    let header_size: size_t = size_of::<u64>();
    let mut remaining: size_t;

    if unsafe { (*ipc4_msg).header_u64 == 0 || count == 0 || *ppos != 0 } {
        return 0;
    }

    /* we need space for the header at minimum (u64) */
    if count < header_size {
        return -(ENOSPC as ssize_t);
    }

    remaining = header_size;

    /* Only get large config have payload */
    if unsafe { SOF_IPC4_MSG_IS_MODULE_MSG((*ipc4_msg).primary) }
        && unsafe { SOF_IPC4_MSG_TYPE_GET((*ipc4_msg).primary) == SOF_IPC4_MOD_LARGE_CONFIG_GET }
    {
        remaining += unsafe { (*ipc4_msg).data_size };
    }

    if count > remaining {
        count = remaining;
    } else if count < remaining {
        remaining = count;
    }

    /* copy the header first */
    if unsafe {
        copy_to_user(
            buffer as *mut c_void,
            &(*ipc4_msg).header_u64 as *const u64 as *const c_void,
            header_size,
        )
    } != 0
    {
        return -(EFAULT as ssize_t);
    }

    unsafe {
        *ppos += header_size as loff_t;
    }
    remaining -= header_size;

    if remaining == 0 {
        return count as ssize_t;
    }

    if remaining > unsafe { (*ipc4_msg).data_size } {
        remaining = unsafe { (*ipc4_msg).data_size };
    }

    /* Copy the payload */
    if unsafe {
        copy_to_user(
            buffer.offset(*ppos as isize) as *mut c_void,
            (*ipc4_msg).data_ptr,
            remaining,
        )
    } != 0
    {
        return -(EFAULT as ssize_t);
    }

    unsafe {
        *ppos += remaining as loff_t;
    }
    count as ssize_t
}

unsafe extern "C" fn sof_msg_inject_send_message(cdev: *mut sof_client_dev) -> c_int {
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
    let dev = unsafe { &mut (*cdev).auxdev.dev as *mut device };
    let mut ret: c_int;
    let err: c_int;

    ret = unsafe { pm_runtime_resume_and_get(dev) };
    if ret < 0 && ret != -EACCES {
        unsafe {
            dev_err_ratelimited(
                dev,
                c"debugfs write failed to resume %d\n".as_ptr(),
                ret,
            )
        };
        return ret;
    }

    ret = unsafe { sof_client_boot_dsp(cdev) };
    if ret == 0 {
        /* send the message */
        ret = unsafe {
            sof_client_ipc_tx_message(
                cdev,
                (*priv_).tx_buffer,
                (*priv_).rx_buffer,
                (*priv_).max_msg_size,
            )
        };
        if ret != 0 {
            unsafe { dev_err(dev, c"IPC message send failed: %d\n".as_ptr(), ret) };
        }
    }

    err = unsafe { pm_runtime_put_autosuspend(dev) };
    if err < 0 {
        unsafe {
            dev_err_ratelimited(dev, c"debugfs write failed to idle %d\n".as_ptr(), err)
        };
    }

    ret
}

unsafe extern "C" fn sof_msg_inject_dfs_write(
    file: *mut file,
    buffer: *const c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
    let mut size: ssize_t;
    let ret: c_int;

    if unsafe { *ppos != 0 } {
        return 0;
    }

    size = unsafe {
        simple_write_to_buffer((*priv_).tx_buffer, (*priv_).max_msg_size, ppos, buffer as *const c_void, count)
    };
    if size < 0 {
        return size;
    }
    if size != count as ssize_t {
        return -(EFAULT as ssize_t);
    }

    unsafe {
        memset((*priv_).rx_buffer, 0, (*priv_).max_msg_size);
    }

    ret = unsafe { sof_msg_inject_send_message(cdev) };

    /* return the error code if test failed */
    if ret < 0 {
        size = ret as ssize_t;
    }

    size
}

unsafe extern "C" fn sof_msg_inject_ipc4_dfs_write(
    file: *mut file,
    buffer: *const c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
    let mut ipc4_msg = unsafe { (*priv_).tx_buffer as *mut sof_ipc4_msg };
    let data_size: size_t;
    let ret: c_int;

    if unsafe { *ppos != 0 } {
        return 0;
    }

    if count < size_of::<u64>() {
        return -(EINVAL as ssize_t);
    }

    /* copy the header first */
    if unsafe {
        copy_from_user(
            &mut (*ipc4_msg).header_u64 as *mut u64 as *mut c_void,
            buffer as *const c_void,
            size_of::<u64>(),
        )
    } != 0
    {
        return -(EFAULT as ssize_t);
    }

    data_size = count - size_of::<u64>();
    if data_size > unsafe { (*priv_).max_msg_size } {
        return -(EINVAL as ssize_t);
    }

    /* Copy the payload */
    if unsafe {
        copy_from_user(
            (*ipc4_msg).data_ptr,
            buffer.add(size_of::<u64>()) as *const c_void,
            data_size,
        )
    } != 0
    {
        return -(EFAULT as ssize_t);
    }

    unsafe {
        (*ipc4_msg).data_size = data_size;
    }

    /* Initialize the reply storage */
    ipc4_msg = unsafe { (*priv_).rx_buffer as *mut sof_ipc4_msg };
    unsafe {
        (*ipc4_msg).header_u64 = 0;
        (*ipc4_msg).data_size = (*priv_).max_msg_size;
        memset((*ipc4_msg).data_ptr, 0, (*priv_).max_msg_size);
    }

    ret = unsafe { sof_msg_inject_send_message(cdev) };

    /* return the error code if test failed */
    if ret < 0 {
        return ret as ssize_t;
    }

    count as ssize_t
}

unsafe extern "C" fn sof_msg_inject_dfs_release(
    _inode: *mut inode,
    file: *mut file,
) -> c_int {
    unsafe { debugfs_file_put((*file).f_path.dentry) };

    0
}

static mut sof_msg_inject_fops: file_operations = file_operations {
    open: Some(sof_msg_inject_dfs_open),
    read: Some(sof_msg_inject_dfs_read),
    write: Some(sof_msg_inject_dfs_write),
    llseek: Some(default_llseek),
    release: Some(sof_msg_inject_dfs_release),

    owner: unsafe { THIS_MODULE },
};

static mut sof_msg_inject_ipc4_fops: file_operations = file_operations {
    open: Some(sof_msg_inject_dfs_open),
    read: Some(sof_msg_inject_ipc4_dfs_read),
    write: Some(sof_msg_inject_ipc4_dfs_write),
    llseek: Some(default_llseek),
    release: Some(sof_msg_inject_dfs_release),

    owner: unsafe { THIS_MODULE },
};

unsafe extern "C" fn sof_msg_inject_probe(
    auxdev: *mut auxiliary_device,
    _id: *const auxiliary_device_id,
) -> c_int {
    let cdev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
    let debugfs_root = unsafe { sof_client_get_debugfs_root(cdev) };
    static mut fops: *const file_operations = ptr::null();
    let dev = unsafe { &mut (*auxdev).dev as *mut device };
    let priv_: *mut sof_msg_inject_priv;
    let mut alloc_size: size_t;

    /* allocate memory for client data */
    priv_ = unsafe {
        devm_kzalloc(
            &mut (*auxdev).dev as *mut device,
            size_of::<sof_msg_inject_priv>(),
            GFP_KERNEL,
        ) as *mut sof_msg_inject_priv
    };
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).ipc_type = sof_client_get_ipc_type(cdev);
        (*priv_).max_msg_size = sof_client_get_ipc_max_payload_size(cdev);
        alloc_size = (*priv_).max_msg_size;
    }

    if unsafe { (*priv_).ipc_type == SOF_IPC_TYPE_4 } {
        alloc_size += size_of::<sof_ipc4_msg>();
    }

    unsafe {
        (*priv_).tx_buffer = devm_kmalloc(dev, alloc_size, GFP_KERNEL);
        (*priv_).rx_buffer = devm_kzalloc(dev, alloc_size, GFP_KERNEL);
    }
    if unsafe { (*priv_).tx_buffer.is_null() || (*priv_).rx_buffer.is_null() } {
        return -ENOMEM;
    }

    if unsafe { (*priv_).ipc_type == SOF_IPC_TYPE_4 } {
        let mut ipc4_msg: *mut sof_ipc4_msg;

        ipc4_msg = unsafe { (*priv_).tx_buffer as *mut sof_ipc4_msg };
        unsafe {
            (*ipc4_msg).data_ptr = ((*priv_).tx_buffer as *mut u8).add(size_of::<sof_ipc4_msg>())
                as *mut c_void;
        }

        ipc4_msg = unsafe { (*priv_).rx_buffer as *mut sof_ipc4_msg };
        unsafe {
            (*ipc4_msg).data_ptr = ((*priv_).rx_buffer as *mut u8).add(size_of::<sof_ipc4_msg>())
                as *mut c_void;
        }

        unsafe {
            fops = &raw const sof_msg_inject_ipc4_fops;
        }
    } else {
        unsafe {
            fops = &raw const sof_msg_inject_fops;
        }
    }

    unsafe {
        (*cdev).data = priv_ as *mut c_void;
    }

    unsafe {
        (*priv_).dfs_file = debugfs_create_file(
            c"ipc_msg_inject".as_ptr(),
            0o644,
            debugfs_root,
            cdev as *mut c_void,
            fops,
        );
    }

    /* enable runtime PM */
    unsafe {
        pm_runtime_set_autosuspend_delay(dev, SOF_IPC_CLIENT_SUSPEND_DELAY_MS);
        pm_runtime_use_autosuspend(dev);
        pm_runtime_enable(dev);
        pm_runtime_mark_last_busy(dev);
        pm_runtime_idle(dev);
    }

    0
}

unsafe extern "C" fn sof_msg_inject_remove(auxdev: *mut auxiliary_device) {
    let cdev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
    let priv_ = unsafe { (*cdev).data as *mut sof_msg_inject_priv };

    unsafe {
        pm_runtime_disable(&mut (*auxdev).dev as *mut device);

        debugfs_remove((*priv_).dfs_file);
    }
}

static sof_msg_inject_client_id_table: [auxiliary_device_id; 2] = [
    auxiliary_device_id {
        name: c"snd_sof.msg_injector".as_ptr(),
    },
    auxiliary_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(auxiliary, sof_msg_inject_client_id_table);

/*
 * No need for driver pm_ops as the generic pm callbacks in the auxiliary bus
 * type are enough to ensure that the parent SOF device resumes to bring the DSP
 * back to D0.
 * Driver name will be set based on KBUILD_MODNAME.
 */
static mut sof_msg_inject_client_drv: auxiliary_driver = auxiliary_driver {
    probe: Some(sof_msg_inject_probe),
    remove: Some(sof_msg_inject_remove),

    id_table: sof_msg_inject_client_id_table.as_ptr(),
};

// module_auxiliary_driver(sof_msg_inject_client_drv);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("SOF IPC Message Injector Client Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
