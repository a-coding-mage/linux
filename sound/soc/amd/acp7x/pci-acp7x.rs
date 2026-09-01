// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD common ACP PCI driver for ACP7.x variants
 * which includes ACP7.D/7.E/7.F and future variants
 * with same register layout.
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// C dependencies:
// linux/errno.h, linux/io.h, linux/module.h, linux/pci.h,
// linux/pm_runtime.h, linux/slab.h, linux/types.h, "acp7x.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_int = 0;

const ACP7D_PCI_REV: u8 = 0; // supplied by acp7x.h
const ACP7E_PCI_REV: u8 = 0; // supplied by acp7x.h
const ACP7F_PCI_REV: u8 = 0; // supplied by acp7x.h
const ACP7X_REG_END: u32 = 0; // supplied by acp7x.h
const ACP7X_REG_START: u32 = 0; // supplied by acp7x.h
const ACP_SUSPEND_DELAY_MS: c_int = 0; // supplied by acp7x.h
const PCI_VENDOR_ID_AMD: u32 = 0; // supplied by linux/pci.h
const ACP_DEVICE_ID: u32 = 0; // supplied by acp7x.h
const PCI_CLASS_MULTIMEDIA_OTHER: u32 = 0; // supplied by linux/pci.h
const KBUILD_MODNAME: *const c_char = ptr::null(); // supplied by linux/module.h

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
	pub dev: device,
	pub revision: u8,
}

#[repr(C)]
pub struct pci_device_id {
	pub vendor: u32,
	pub device: u32,
	pub subvendor: u32,
	pub subdevice: u32,
	pub class: u32,
	pub class_mask: u32,
	pub driver_data: usize,
}

#[repr(C)]
pub struct acp_hw_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct acp7x_dev_data {
	pub hw_ops: *mut acp_hw_ops,
	pub acp7x_base: *mut c_void,
	pub addr: u32,
	pub reg_range: u32,
	pub acp_rev: u8,
}

#[repr(C)]
pub struct dev_pm_ops {
	pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct pci_driver_inner_driver {
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
	pub name: *const c_char,
	pub id_table: *const pci_device_id,
	pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
	pub driver: pci_driver_inner_driver,
}

unsafe extern "C" {
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
	fn acp7x_hw_init_ops(ops: *mut acp_hw_ops);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn snd_amd_acp_find_config(pci: *mut pci_dev) -> u32;
	fn pci_enable_device(pci: *mut pci_dev) -> c_int;
	fn pci_request_regions(pci: *mut pci_dev, res_name: *const c_char) -> c_int;
	fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> u32;
	fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> usize;
	fn devm_ioremap(dev: *mut device, offset: u32, size: usize) -> *mut c_void;
	fn pci_set_master(pci: *mut pci_dev);
	fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
	fn acp_hw_init(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int;
	fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
	fn pm_runtime_use_autosuspend(dev: *mut device);
	fn pm_runtime_put_noidle(dev: *mut device);
	fn pm_runtime_allow(dev: *mut device);
	fn pci_release_regions(pci: *mut pci_dev);
	fn pci_disable_device(pci: *mut pci_dev);
	fn acp_hw_suspend(dev: *mut device) -> c_int;
	fn acp_hw_runtime_resume(dev: *mut device) -> c_int;
	fn acp_hw_resume(dev: *mut device) -> c_int;
	fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
	fn acp_hw_deinit(adata: *mut acp7x_dev_data, dev: *mut device) -> c_int;
	fn pm_runtime_forbid(dev: *mut device);
	fn pm_runtime_get_noresume(dev: *mut device);
}

const fn PCI_DEVICE(vendor: u32, device: u32) -> pci_device_id {
	pci_device_id {
		vendor,
		device,
		subvendor: 0,
		subdevice: 0,
		class: 0,
		class_mask: 0,
		driver_data: 0,
	}
}

unsafe extern "C" fn acp_hw_init_ops(adata: *mut acp7x_dev_data, pci: *mut pci_dev) -> c_int {
	unsafe {
		(*adata).hw_ops = devm_kzalloc(
			&mut (*pci).dev,
			size_of::<acp_hw_ops>(),
			GFP_KERNEL,
		) as *mut acp_hw_ops;
		if (*adata).hw_ops.is_null() {
			return -ENOMEM;
		}

		match (*adata).acp_rev {
			ACP7D_PCI_REV | ACP7E_PCI_REV | ACP7F_PCI_REV => {
				acp7x_hw_init_ops((*adata).hw_ops);
			}
			_ => {
				dev_err(&mut (*pci).dev, c"ACP device not found\n".as_ptr());
				return -ENODEV;
			}
		}
		0
	}
}

unsafe extern "C" fn snd_acp7x_probe(
	pci: *mut pci_dev,
	pci_id: *const pci_device_id,
) -> c_int {
	unsafe {
		let mut adata: *mut acp7x_dev_data;
		let addr: u32;
		let flag: u32;
		let mut ret: c_int;

		let _ = pci_id;

		flag = snd_amd_acp_find_config(pci);
		if flag != 0 {
			return -ENODEV;
		}
		/* ACP PCI revision id check for ACP7.x platforms */
		match (*pci).revision {
			ACP7D_PCI_REV | ACP7E_PCI_REV | ACP7F_PCI_REV => {}
			_ => {
				return -ENODEV;
			}
		}
		if pci_enable_device(pci) != 0 {
			dev_err(&mut (*pci).dev, c"pci_enable_device failed\n".as_ptr());
			return -ENODEV;
		}

		ret = pci_request_regions(pci, c"AMD ACP7.x audio".as_ptr());
		if ret < 0 {
			dev_err(&mut (*pci).dev, c"pci_request_regions failed\n".as_ptr());
			pci_disable_device(pci);
			return ret;
		}
		adata = devm_kzalloc(
			&mut (*pci).dev,
			size_of::<acp7x_dev_data>(),
			GFP_KERNEL,
		) as *mut acp7x_dev_data;
		if adata.is_null() {
			ret = -ENOMEM;
			pci_release_regions(pci);
			pci_disable_device(pci);
			return ret;
		}
		addr = pci_resource_start(pci, 0);
		(*adata).acp7x_base = devm_ioremap(&mut (*pci).dev, addr, pci_resource_len(pci, 0));
		if (*adata).acp7x_base.is_null() {
			ret = -ENOMEM;
			pci_release_regions(pci);
			pci_disable_device(pci);
			return ret;
		}
		(*adata).addr = addr;
		(*adata).reg_range = ACP7X_REG_END.wrapping_sub(ACP7X_REG_START);
		(*adata).acp_rev = (*pci).revision;
		pci_set_master(pci);
		pci_set_drvdata(pci, adata as *mut c_void);
		ret = acp_hw_init_ops(adata, pci);
		if ret != 0 {
			dev_err(&mut (*pci).dev, c"ACP hw ops init failed\n".as_ptr());
			pci_release_regions(pci);
			pci_disable_device(pci);
			return ret;
		}
		ret = acp_hw_init(adata, &mut (*pci).dev);
		if ret != 0 {
			pci_release_regions(pci);
			pci_disable_device(pci);
			return ret;
		}

		pm_runtime_set_autosuspend_delay(&mut (*pci).dev, ACP_SUSPEND_DELAY_MS);
		pm_runtime_use_autosuspend(&mut (*pci).dev);
		pm_runtime_put_noidle(&mut (*pci).dev);
		pm_runtime_allow(&mut (*pci).dev);
		0
	}
}

// __maybe_unused
unsafe extern "C" fn snd_acp_suspend(dev: *mut device) -> c_int {
	unsafe { acp_hw_suspend(dev) }
}

// __maybe_unused
unsafe extern "C" fn snd_acp_runtime_resume(dev: *mut device) -> c_int {
	unsafe { acp_hw_runtime_resume(dev) }
}

// __maybe_unused
unsafe extern "C" fn snd_acp_resume(dev: *mut device) -> c_int {
	unsafe { acp_hw_resume(dev) }
}

// SET_RUNTIME_PM_OPS(snd_acp_suspend, snd_acp_runtime_resume, NULL)
// SET_SYSTEM_SLEEP_PM_OPS(snd_acp_suspend, snd_acp_resume)
static acp7x_pm_ops: dev_pm_ops = dev_pm_ops {
	runtime_suspend: Some(snd_acp_suspend),
	runtime_resume: Some(snd_acp_runtime_resume),
	runtime_idle: None,
	suspend: Some(snd_acp_suspend),
	resume: Some(snd_acp_resume),
};

unsafe extern "C" fn snd_acp7x_remove(pci: *mut pci_dev) {
	unsafe {
		let adata: *mut acp7x_dev_data;
		let ret: c_int;

		adata = pci_get_drvdata(pci) as *mut acp7x_dev_data;
		ret = acp_hw_deinit(adata, &mut (*pci).dev);
		if ret != 0 {
			dev_err(&mut (*pci).dev, c"ACP de-init failed\n".as_ptr());
		}
		pm_runtime_forbid(&mut (*pci).dev);
		pm_runtime_get_noresume(&mut (*pci).dev);
		pci_release_regions(pci);
		pci_disable_device(pci);
	}
}

static snd_acp7x_ids: [pci_device_id; 2] = [
	pci_device_id {
		class: PCI_CLASS_MULTIMEDIA_OTHER << 8,
		class_mask: 0xffffff,
		..PCI_DEVICE(PCI_VENDOR_ID_AMD, ACP_DEVICE_ID)
	},
	pci_device_id {
		vendor: 0,
		device: 0,
		subvendor: 0,
		subdevice: 0,
		class: 0,
		class_mask: 0,
		driver_data: 0,
	},
];
// MODULE_DEVICE_TABLE(pci, snd_acp7x_ids);

static mut acp7x_pci_driver: pci_driver = pci_driver {
	name: KBUILD_MODNAME,
	id_table: snd_acp7x_ids.as_ptr(),
	probe: Some(snd_acp7x_probe),
	remove: Some(snd_acp7x_remove),
	driver: pci_driver_inner_driver {
		pm: &acp7x_pm_ops,
	},
};

// module_pci_driver(acp7x_pci_driver);

// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP PCI driver for ACP7.X");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
