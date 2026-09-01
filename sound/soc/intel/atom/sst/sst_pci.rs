// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_pci.c - SST (LPE) driver init file for pci enumeration.
 *
 *  Copyright (C) 2008-14	Intel Corp
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com>
 *		KP Jeeja <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */
/* C includes translated as external dependencies:
 * linux/module.h, linux/pci.h, linux/fs.h, linux/firmware.h,
 * sound/core.h, sound/soc.h, asm/platform_sst_audio.h,
 * ../sst-mfld-platform.h, sst.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const CONFIG_PM: bool = cfg!(CONFIG_PM);

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub device: c_uint,
    pub irq: c_int,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: usize,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct sst_module_info {
    pub mod_base: c_int,
}

#[repr(C)]
pub struct sst_platform_info {
    pub lib_info: *mut sst_module_info,
}

#[repr(C)]
pub struct intel_sst_drv {
    pub pci: *mut pci_dev,
    pub dev_id: c_uint,
    pub pdata: *mut sst_platform_info,
    pub dev: *mut device,
    pub irq_num: c_int,
    pub firmware_name: [c_char; 32],
    pub ddr_base: c_int,
    pub ddr_end: c_int,
    pub ddr: *mut c_void,
    pub shim_phy_add: c_int,
    pub shim: *mut c_void,
    pub mailbox_add: c_int,
    pub mailbox: *mut c_void,
    pub iram_end: c_int,
    pub iram_base: c_int,
    pub iram: *mut c_void,
    pub dram_end: c_int,
    pub dram_base: c_int,
    pub dram: *mut c_void,
}

unsafe extern "C" {
    static SST_DRV_NAME: *const c_char;
    static PCI_DEVICE_ID_INTEL_SST_TNG: c_uint;
    static intel_sst_pm: dev_pm_ops;

    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_int;
    fn pci_resource_end(pci: *mut pci_dev, bar: c_int) -> c_int;
    fn pcim_iomap(pci: *mut pci_dev, bar: c_int, maxlen: usize) -> *mut c_void;
    fn relocate_imr_addr_mrfld(addr: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn sst_alloc_drv_context(
        ctx: *mut *mut intel_sst_drv,
        dev: *mut device,
        dev_id: c_uint,
    ) -> c_int;
    fn sst_context_init(ctx: *mut intel_sst_drv) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_dev_get(pci: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(pci: *mut pci_dev);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn sst_configure_runtime_pm(ctx: *mut intel_sst_drv);
    fn sst_context_cleanup(ctx: *mut intel_sst_drv);
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn module_pci_driver(driver: *mut pci_driver);
}

const FW_SST_PREFIX: &[u8; 8] = b"fw_sst_\0";
const BIN_SUFFIX: &[u8; 5] = b".bin\0";
const FIRMWARE_FMT: &[u8; 8] = b"%s%04x%s\0";
const LIB_INFO_NULL: &[u8; 23] = b"lib_info pointer NULL\n\0";
const FW_DDR_MISMATCH: &[u8; 41] = b"FW LSP DDR BASE does not match with IFWI\n\0";
const DDR_PTR_FMT: &[u8; 17] = b"sst: DDR Ptr %p\n\0";
const SHIM_PTR_FMT: &[u8; 17] = b"SST Shim Ptr %p\n\0";
const SRAM_PTR_FMT: &[u8; 13] = b"SRAM Ptr %p\n\0";
const IRAM_PTR_FMT: &[u8; 13] = b"IRAM Ptr %p\n\0";
const DRAM_PTR_FMT: &[u8; 13] = b"DRAM Ptr %p\n\0";
const PROBE_FMT: &[u8; 18] = b"Probe for DID %x\n\0";
const ENABLE_ERR_FMT: &[u8; 44] = b"device can't be enabled. Returned err: %d\n\0";
const PROBE_FAILED_FMT: &[u8; 22] = b"Probe failed with %d\n\0";

unsafe fn PCI_DEVICE_DATA(_vendor: c_uint, device: c_uint, data: usize) -> pci_device_id {
    pci_device_id {
        vendor: 0,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: data,
    }
}

static mut PCI_DEVICE_ID_INTEL_SST_TNG_ID: c_uint = 0;

unsafe extern "C" fn sst_platform_get_resources(ctx: *mut intel_sst_drv) -> c_int {
    let mut ddr_base: c_int;
    let mut ret: c_int = 0;
    let pci: *mut pci_dev = (*ctx).pci;

    ret = pcim_request_all_regions(pci, SST_DRV_NAME);
    if ret != 0 {
        return ret;
    }

    /* map registers */
    /* DDR base */
    if (*ctx).dev_id == PCI_DEVICE_ID_INTEL_SST_TNG {
        (*ctx).ddr_base = pci_resource_start(pci, 0);
        /* check that the relocated IMR base matches with FW Binary */
        ddr_base = relocate_imr_addr_mrfld((*ctx).ddr_base);
        if (*(*ctx).pdata).lib_info.is_null() {
            dev_err((*ctx).dev, LIB_INFO_NULL.as_ptr() as *const c_char);
            return -EINVAL;
        }
        if ddr_base != (*(*(*ctx).pdata).lib_info).mod_base {
            dev_err((*ctx).dev, FW_DDR_MISMATCH.as_ptr() as *const c_char);
            return -EINVAL;
        }
        (*ctx).ddr_end = pci_resource_end(pci, 0);

        (*ctx).ddr = pcim_iomap(pci, 0, 0);
        if (*ctx).ddr.is_null() {
            return -ENOMEM;
        }

        dev_dbg(
            (*ctx).dev,
            DDR_PTR_FMT.as_ptr() as *const c_char,
            (*ctx).ddr,
        );
    } else {
        (*ctx).ddr = core::ptr::null_mut();
    }
    /* SHIM */
    (*ctx).shim_phy_add = pci_resource_start(pci, 1);
    (*ctx).shim = pcim_iomap(pci, 1, 0);
    if (*ctx).shim.is_null() {
        return -ENOMEM;
    }

    dev_dbg(
        (*ctx).dev,
        SHIM_PTR_FMT.as_ptr() as *const c_char,
        (*ctx).shim,
    );

    /* Shared SRAM */
    (*ctx).mailbox_add = pci_resource_start(pci, 2);
    (*ctx).mailbox = pcim_iomap(pci, 2, 0);
    if (*ctx).mailbox.is_null() {
        return -ENOMEM;
    }

    dev_dbg(
        (*ctx).dev,
        SRAM_PTR_FMT.as_ptr() as *const c_char,
        (*ctx).mailbox,
    );

    /* IRAM */
    (*ctx).iram_end = pci_resource_end(pci, 3);
    (*ctx).iram_base = pci_resource_start(pci, 3);
    (*ctx).iram = pcim_iomap(pci, 3, 0);
    if (*ctx).iram.is_null() {
        return -ENOMEM;
    }

    dev_dbg(
        (*ctx).dev,
        IRAM_PTR_FMT.as_ptr() as *const c_char,
        (*ctx).iram,
    );

    /* DRAM */
    (*ctx).dram_end = pci_resource_end(pci, 4);
    (*ctx).dram_base = pci_resource_start(pci, 4);
    (*ctx).dram = pcim_iomap(pci, 4, 0);
    if (*ctx).dram.is_null() {
        return -ENOMEM;
    }

    dev_dbg(
        (*ctx).dev,
        DRAM_PTR_FMT.as_ptr() as *const c_char,
        (*ctx).dram,
    );
    0
}

/*
 * intel_sst_probe - PCI probe function
 *
 * @pci:	PCI device structure
 * @pci_id: PCI device ID structure
 *
 */
unsafe extern "C" fn intel_sst_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sst_drv_ctx: *mut intel_sst_drv = core::ptr::null_mut();
    let sst_pdata: *mut sst_platform_info = (*pci).dev.platform_data as *mut sst_platform_info;

    dev_dbg(
        &mut (*pci).dev as *mut device,
        PROBE_FMT.as_ptr() as *const c_char,
        (*pci).device,
    );
    ret = sst_alloc_drv_context(
        &mut sst_drv_ctx as *mut *mut intel_sst_drv,
        &mut (*pci).dev as *mut device,
        (*pci).device,
    );
    if ret < 0 {
        return ret;
    }

    (*sst_drv_ctx).pdata = sst_pdata;
    (*sst_drv_ctx).irq_num = (*pci).irq;
    snprintf(
        (*sst_drv_ctx).firmware_name.as_mut_ptr(),
        core::mem::size_of_val(&(*sst_drv_ctx).firmware_name),
        FIRMWARE_FMT.as_ptr() as *const c_char,
        FW_SST_PREFIX.as_ptr() as *const c_char,
        (*sst_drv_ctx).dev_id,
        BIN_SUFFIX.as_ptr() as *const c_char,
    );

    ret = sst_context_init(sst_drv_ctx);
    if ret < 0 {
        return ret;
    }

    /* Init the device */
    ret = pcim_enable_device(pci);
    if ret != 0 {
        dev_err(
            (*sst_drv_ctx).dev,
            ENABLE_ERR_FMT.as_ptr() as *const c_char,
            ret,
        );
        sst_context_cleanup(sst_drv_ctx);
        dev_err(
            (*sst_drv_ctx).dev,
            PROBE_FAILED_FMT.as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }
    (*sst_drv_ctx).pci = pci_dev_get(pci);
    ret = sst_platform_get_resources(sst_drv_ctx);
    if ret < 0 {
        pci_dev_put((*sst_drv_ctx).pci);
        sst_context_cleanup(sst_drv_ctx);
        dev_err(
            (*sst_drv_ctx).dev,
            PROBE_FAILED_FMT.as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    pci_set_drvdata(pci, sst_drv_ctx as *mut c_void);
    sst_configure_runtime_pm(sst_drv_ctx);

    ret
}

/**
 * intel_sst_remove - PCI remove function
 *
 * @pci:	PCI device structure
 *
 * This function is called by OS when a device is unloaded
 * This frees the interrupt etc
 */
unsafe extern "C" fn intel_sst_remove(pci: *mut pci_dev) {
    let sst_drv_ctx: *mut intel_sst_drv = pci_get_drvdata(pci) as *mut intel_sst_drv;

    sst_context_cleanup(sst_drv_ctx);
    pci_dev_put((*sst_drv_ctx).pci);
    pci_set_drvdata(pci, core::ptr::null_mut());
}

/* PCI Routines */
static mut intel_sst_ids: [pci_device_id; 2] = [
    unsafe { PCI_DEVICE_DATA(PCI_DEVICE_ID_INTEL_SST_TNG_ID, PCI_DEVICE_ID_INTEL_SST_TNG_ID, 0) },
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

static mut sst_driver: pci_driver = pci_driver {
    name: core::ptr::null(),
    id_table: unsafe { intel_sst_ids.as_ptr() },
    probe: Some(intel_sst_probe),
    remove: Some(intel_sst_remove),
    /* #ifdef CONFIG_PM
     * .driver = {
     *      .pm = &intel_sst_pm,
     * },
     * #endif
     */
    driver: device_driver {
        pm: if CONFIG_PM {
            unsafe { &intel_sst_pm as *const dev_pm_ops }
        } else {
            core::ptr::null()
        },
    },
};

unsafe fn register_sst_driver() {
    sst_driver.name = SST_DRV_NAME;
    module_pci_driver(&mut sst_driver as *mut pci_driver);
}

/*
 * MODULE_DESCRIPTION("Intel (R) SST(R) Audio Engine PCI Driver");
 * MODULE_AUTHOR("Vinod Koul <vinod.koul@intel.com>");
 * MODULE_AUTHOR("Harsha Priya <priya.harsha@intel.com>");
 * MODULE_AUTHOR("Dharageswari R <dharageswari.r@intel.com>");
 * MODULE_AUTHOR("KP Jeeja <jeeja.kp@intel.com>");
 * MODULE_LICENSE("GPL v2");
 * MODULE_ALIAS("sst");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
