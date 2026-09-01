// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2023 Google Inc
//
// Author: Curtis Malainey <cujomalainey@chromium.org>
//

// C dependencies:
// <linux/auxiliary_bus.h>
// <linux/debugfs.h>
// <linux/module.h>
// <linux/pm_runtime.h>
// <sound/sof/header.h>
// "sof-client.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type loff_t = i64;
pub type umode_t = u16;

pub const SOF_IPC_CLIENT_SUSPEND_DELAY_MS: c_int = 3000;
pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const EFAULT: c_int = 14;
pub const EACCES: c_int = 13;

#[repr(C)]
pub struct dentry {
	_private: [u8; 0],
}

#[repr(C)]
pub struct inode {
	_private: [u8; 0],
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
pub struct module {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device {
	_private: [u8; 0],
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
pub struct sof_ipc_cmd_hdr {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_client_dev {
	pub auxdev: auxiliary_device,
	pub data: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
	pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
	pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
	pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
	pub owner: *mut module,
}

#[repr(C)]
pub struct auxiliary_driver {
	pub probe: Option<unsafe extern "C" fn(*mut auxiliary_device, *const auxiliary_device_id) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut auxiliary_device)>,
	pub id_table: *const auxiliary_device_id,
}

#[repr(C)]
pub struct sof_msg_inject_priv {
	pub kernel_dfs_file: *mut dentry,
	pub max_msg_size: size_t,

	pub kernel_buffer: *mut c_void,
}

unsafe extern "C" {
	static mut THIS_MODULE: *mut module;

	fn debugfs_file_get(dentry: *mut dentry) -> c_int;
	fn debugfs_file_put(dentry: *mut dentry);
	fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
	fn simple_write_to_buffer(to: *mut c_void, available: size_t, ppos: *mut loff_t,
				  from: *const c_char, count: size_t) -> ssize_t;
	fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
	fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
	fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int;
	fn sof_client_ipc_rx_message(cdev: *mut sof_client_dev, hdr: *mut sof_ipc_cmd_hdr,
				     msg_buf: *mut c_void);
	fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
	fn auxiliary_dev_to_sof_client_dev(auxdev: *mut auxiliary_device) -> *mut sof_client_dev;
	fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry;
	fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
	fn sof_client_get_ipc_max_payload_size(cdev: *mut sof_client_dev) -> size_t;
	fn devm_kmalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
	fn debugfs_create_file(name: *const c_char, mode: umode_t, parent: *mut dentry,
			       data: *mut c_void, fops: *const file_operations) -> *mut dentry;
	fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
	fn pm_runtime_use_autosuspend(dev: *mut device);
	fn pm_runtime_set_active(dev: *mut device);
	fn pm_runtime_enable(dev: *mut device);
	fn pm_runtime_mark_last_busy(dev: *mut device);
	fn pm_runtime_idle(dev: *mut device) -> c_int;
	fn pm_runtime_disable(dev: *mut device);
	fn debugfs_remove(dentry: *mut dentry);
}

unsafe extern "C" fn sof_msg_inject_dfs_open(inode: *mut inode, file: *mut file) -> c_int {
	let mut ret: c_int = unsafe { debugfs_file_get((*file).f_path.dentry) };

	if ret != 0 {
		return ret;
	}

	ret = unsafe { simple_open(inode, file) };
	if ret != 0 {
		unsafe { debugfs_file_put((*file).f_path.dentry) };
	}

	ret
}

unsafe extern "C" fn sof_kernel_msg_inject_dfs_write(
	file: *mut file,
	buffer: *const c_char,
	count: size_t,
	ppos: *mut loff_t,
) -> ssize_t {
	let cdev: *mut sof_client_dev = unsafe { (*file).private_data as *mut sof_client_dev };
	let priv_: *mut sof_msg_inject_priv = unsafe { (*cdev).data as *mut sof_msg_inject_priv };
	let hdr: *mut sof_ipc_cmd_hdr = unsafe { (*priv_).kernel_buffer as *mut sof_ipc_cmd_hdr };
	let dev: *mut device = unsafe { &mut (*cdev).auxdev.dev };
	let mut size: ssize_t;
	let mut ret: c_int;

	if unsafe { *ppos } != 0 {
		return 0;
	}

	size = unsafe {
		simple_write_to_buffer((*priv_).kernel_buffer, (*priv_).max_msg_size, ppos, buffer, count)
	};
	if size < 0 {
		return size;
	}
	if size != count as ssize_t {
		return -(EFAULT as ssize_t);
	}

	ret = unsafe { pm_runtime_resume_and_get(dev) };
	if ret < 0 && ret != -EACCES {
		unsafe {
			dev_err_ratelimited(dev, c"debugfs write failed to resume %d\n".as_ptr(), ret);
		}
		return ret as ssize_t;
	}

	ret = unsafe { sof_client_boot_dsp(cdev) };
	if ret == 0 {
		unsafe { sof_client_ipc_rx_message(cdev, hdr, (*priv_).kernel_buffer) };
	}

	ret = unsafe { pm_runtime_put_autosuspend(dev) };
	if ret < 0 {
		unsafe {
			dev_err_ratelimited(dev, c"debugfs write failed to idle %d\n".as_ptr(), ret);
		}
	}

	count as ssize_t
}

unsafe extern "C" fn sof_msg_inject_dfs_release(_inode: *mut inode, file: *mut file) -> c_int {
	unsafe { debugfs_file_put((*file).f_path.dentry) };

	0
}

static mut sof_kernel_msg_inject_fops: file_operations = file_operations {
	open: Some(sof_msg_inject_dfs_open),
	write: Some(sof_kernel_msg_inject_dfs_write),
	release: Some(sof_msg_inject_dfs_release),

	owner: unsafe { THIS_MODULE },
};

unsafe extern "C" fn sof_msg_inject_probe(
	auxdev: *mut auxiliary_device,
	_id: *const auxiliary_device_id,
) -> c_int {
	let cdev: *mut sof_client_dev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
	let debugfs_root: *mut dentry = unsafe { sof_client_get_debugfs_root(cdev) };
	let dev: *mut device = unsafe { &mut (*auxdev).dev };
	let priv_: *mut sof_msg_inject_priv;
	let alloc_size: size_t;

	/* allocate memory for client data */
	priv_ = unsafe {
		devm_kzalloc(&mut (*auxdev).dev, core::mem::size_of::<sof_msg_inject_priv>(), GFP_KERNEL)
			as *mut sof_msg_inject_priv
	};
	if priv_.is_null() {
		return -ENOMEM;
	}

	unsafe {
		(*priv_).max_msg_size = sof_client_get_ipc_max_payload_size(cdev);
	}
	alloc_size = unsafe { (*priv_).max_msg_size };
	unsafe {
		(*priv_).kernel_buffer = devm_kmalloc(dev, alloc_size, GFP_KERNEL);
	}

	if unsafe { (*priv_).kernel_buffer }.is_null() {
		return -ENOMEM;
	}

	unsafe {
		(*cdev).data = priv_ as *mut c_void;
	}

	unsafe {
		(*priv_).kernel_dfs_file = debugfs_create_file(
			c"kernel_ipc_msg_inject".as_ptr(),
			0o644,
			debugfs_root,
			cdev as *mut c_void,
			&raw const sof_kernel_msg_inject_fops,
		);
	}

	/* enable runtime PM */
	unsafe {
		pm_runtime_set_autosuspend_delay(dev, SOF_IPC_CLIENT_SUSPEND_DELAY_MS);
		pm_runtime_use_autosuspend(dev);
		pm_runtime_set_active(dev);
		pm_runtime_enable(dev);
		pm_runtime_mark_last_busy(dev);
		pm_runtime_idle(dev);
	}

	0
}

unsafe extern "C" fn sof_msg_inject_remove(auxdev: *mut auxiliary_device) {
	let cdev: *mut sof_client_dev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
	let priv_: *mut sof_msg_inject_priv = unsafe { (*cdev).data as *mut sof_msg_inject_priv };

	unsafe { pm_runtime_disable(&mut (*auxdev).dev) };

	unsafe { debugfs_remove((*priv_).kernel_dfs_file) };
}

static sof_msg_inject_client_id_table: [auxiliary_device_id; 2] = [
	auxiliary_device_id {
		name: c"snd_sof.kernel_injector".as_ptr(),
	},
	auxiliary_device_id {
		name: core::ptr::null(),
	},
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
// MODULE_DESCRIPTION("SOF IPC Kernel Injector Client Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
