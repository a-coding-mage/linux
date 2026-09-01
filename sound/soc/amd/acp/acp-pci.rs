// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Generic PCI interface for ACP device
 */

// Dependencies from:
// <linux/delay.h>
// <linux/interrupt.h>
// <linux/pci.h>
// <linux/platform_device.h>
// <linux/module.h>
// <linux/pm_runtime.h>
// "amd.h"
// "../mach-config.h"

const DRV_NAME: *const core::ffi::c_char = b"acp_pci\0".as_ptr() as *const core::ffi::c_char;

const ACP3x_REG_START: u32 = 0x1240000;
const ACP3x_REG_END: u32 = 0x125C000;

type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type c_void = core::ffi::c_void;
type size_t = usize;
type u32 = core::ffi::c_uint;
type irqreturn_t = c_uint;

const IRQ_NONE: irqreturn_t = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const IORESOURCE_MEM: c_uint = 0x00000200;
const IRQF_SHARED: c_uint = 0x00000080;
const PLATFORM_DEVID_NONE: c_int = -1;
const FLAG_AMD_LEGACY: c_uint = 0;
const FLAG_AMD_LEGACY_ONLY_DMIC: c_uint = 0;
const PCI_VENDOR_ID_AMD: c_uint = 0x1022;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub revision: c_uint,
    pub irq: c_uint,
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
pub struct resource {
    pub start: u32,
    pub end: u32,
    pub flags: c_uint,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_device_info {
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub name: *mut core::ffi::c_char,
    pub id: c_uint,
    pub res: *const resource,
    pub num_res: c_uint,
    pub data: *const c_void,
    pub size_data: size_t,
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
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct acp_hw_ops {
    pub irq: Option<unsafe extern "C" fn(c_int, *mut acp_chip_info) -> irqreturn_t>,
}

#[repr(C)]
pub struct acp_chip_info {
    pub acp_hw_ops: *mut acp_hw_ops,
    pub is_i2s_config: bool,
    pub is_pdm_dev: bool,
    pub is_pdm_config: bool,
    pub res: *mut resource,
    pub name: *mut core::ffi::c_char,
    pub acp_plat_dev: *mut platform_device,
    pub dmic_codec_dev: *mut platform_device,
    pub mach_dev: *mut platform_device,
    pub chip_pdev: *mut platform_device,
    pub dev: *mut device,
    pub acp_rev: c_uint,
    pub rsrc: *const c_void,
    pub acp_hw_ops_init: Option<unsafe extern "C" fn(*mut acp_chip_info)>,
    pub machines: *const c_void,
    pub flag: c_uint,
    pub base: *mut c_void,
    pub addr: c_uint,
    pub stream_list: c_void,
    pub acp_lock: c_void,
}

unsafe extern "C" {
    static rn_rsrc: c_void;
    static rmb_rsrc: c_void;
    static acp63_rsrc: c_void;
    static acp70_rsrc: c_void;
    static snd_soc_acpi_amd_acp_machines: c_void;
    static snd_soc_acpi_amd_rmb_acp_machines: c_void;
    static snd_soc_acpi_amd_acp63_acp_machines: c_void;
    static snd_soc_acpi_amd_acp70_acp_machines: c_void;
    static ACP_PCI_DEV_ID: c_uint;
    static KBUILD_MODNAME: core::ffi::c_char;

    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn platform_device_register_full(
        pdevinfo: *const platform_device_info,
    ) -> *mut platform_device;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const core::ffi::c_char,
        id: c_int,
        data: *const c_void,
        size: size_t,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...) -> c_int;
    fn dev_err_probe(
        dev: *mut device,
        err: c_int,
        fmt: *const core::ffi::c_char,
        ...
    ) -> c_int;
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_uint;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_uint;
    fn pcim_iomap_region(
        pci: *mut pci_dev,
        bar: c_int,
        name: *const core::ffi::c_char,
    ) -> *mut c_void;
    fn acp31_hw_ops_init(chip: *mut acp_chip_info);
    fn acp6x_hw_ops_init(chip: *mut acp_chip_info);
    fn acp63_hw_ops_init(chip: *mut acp_chip_info);
    fn acp70_hw_ops_init(chip: *mut acp_chip_info);
    fn acp_hw_init(chip: *mut acp_chip_info) -> c_int;
    fn acp_hw_deinit(chip: *mut acp_chip_info) -> c_int;
    fn acp_hw_en_interrupts(chip: *mut acp_chip_info) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const core::ffi::c_char,
        data: *mut c_void,
    ) -> c_int;
    fn check_acp_config(pci: *mut pci_dev, chip: *mut acp_chip_info);
    fn acp_machine_select(chip: *mut acp_chip_info);
    fn INIT_LIST_HEAD(list: *mut c_void);
    fn spin_lock_init(lock: *mut c_void);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_allow(dev: *mut device);
    fn pm_runtime_forbid(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let chip: *mut acp_chip_info = data as *mut acp_chip_info;

    if !chip.is_null()
        && !(*chip).acp_hw_ops.is_null()
        && (*(*chip).acp_hw_ops).irq.is_some()
    {
        return ((*(*chip).acp_hw_ops).irq.unwrap())(irq, chip);
    }

    IRQ_NONE
}

unsafe extern "C" fn acp_fill_platform_dev_info(
    pdevinfo: *mut platform_device_info,
    parent: *mut device,
    fw_node: *mut fwnode_handle,
    name: *mut core::ffi::c_char,
    id: c_uint,
    res: *const resource,
    num_res: c_uint,
    data: *const c_void,
    size_data: size_t,
) {
    (*pdevinfo).name = name;
    (*pdevinfo).id = id;
    (*pdevinfo).parent = parent;
    (*pdevinfo).num_res = num_res;
    (*pdevinfo).res = res;
    (*pdevinfo).data = data;
    (*pdevinfo).size_data = size_data;
    (*pdevinfo).fwnode = fw_node;
}

unsafe extern "C" fn create_acp_platform_devs(
    pci: *mut pci_dev,
    chip: *mut acp_chip_info,
    addr: u32,
) -> c_int {
    let mut pdevinfo: platform_device_info = core::mem::zeroed();
    let parent: *mut device;
    let mut ret: c_int;

    parent = &mut (*pci).dev;

    if (*chip).is_i2s_config || (*chip).is_pdm_dev {
        (*chip).res = devm_kzalloc(
            &mut (*pci).dev,
            core::mem::size_of::<resource>(),
            GFP_KERNEL,
        ) as *mut resource;
        if (*chip).res.is_null() {
            ret = -ENOMEM;
            return ret;
        }
        (*(*chip).res).flags = IORESOURCE_MEM;
        (*(*chip).res).start = addr;
        (*(*chip).res).end = addr + (ACP3x_REG_END - ACP3x_REG_START);
        memset(
            &mut pdevinfo as *mut platform_device_info as *mut c_void,
            0,
            core::mem::size_of_val(&pdevinfo),
        );
    }

    memset(
        &mut pdevinfo as *mut platform_device_info as *mut c_void,
        0,
        core::mem::size_of_val(&pdevinfo),
    );
    acp_fill_platform_dev_info(
        &mut pdevinfo,
        parent,
        core::ptr::null_mut(),
        (*chip).name,
        0,
        (*chip).res,
        1,
        chip as *const c_void,
        core::mem::size_of_val(&*chip),
    );

    (*chip).acp_plat_dev = platform_device_register_full(&pdevinfo);
    if IS_ERR((*chip).acp_plat_dev as *const c_void) {
        dev_err(
            &mut (*pci).dev,
            b"cannot register %s device\n\0".as_ptr() as *const core::ffi::c_char,
            pdevinfo.name,
        );
        ret = PTR_ERR((*chip).acp_plat_dev as *const c_void);
        return ret;
    }
    if (*chip).is_pdm_dev && (*chip).is_pdm_config {
        (*chip).dmic_codec_dev = platform_device_register_data(
            &mut (*pci).dev,
            b"dmic-codec\0".as_ptr() as *const core::ffi::c_char,
            PLATFORM_DEVID_NONE,
            core::ptr::null(),
            0,
        );
        if IS_ERR((*chip).dmic_codec_dev as *const c_void) {
            dev_err(
                &mut (*pci).dev,
                b"failed to create DMIC device\n\0".as_ptr() as *const core::ffi::c_char,
            );
            ret = PTR_ERR((*chip).dmic_codec_dev as *const c_void);
            platform_device_unregister((*chip).acp_plat_dev);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn acp_pci_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let dev: *mut device = &mut (*pci).dev;
    let chip: *mut acp_chip_info;
    let flag: c_uint;
    let addr: c_uint;
    let mut ret: c_int;

    let _ = pci_id;

    flag = snd_amd_acp_find_config(pci);
    if flag != FLAG_AMD_LEGACY && flag != FLAG_AMD_LEGACY_ONLY_DMIC {
        return -ENODEV;
    }

    chip = devm_kzalloc(
        &mut (*pci).dev,
        core::mem::size_of::<acp_chip_info>(),
        GFP_KERNEL,
    ) as *mut acp_chip_info;
    if chip.is_null() {
        return -ENOMEM;
    }

    if pcim_enable_device(pci) != 0 {
        return dev_err_probe(
            &mut (*pci).dev,
            -ENODEV,
            b"pci_enable_device failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    pci_set_master(pci);

    (*chip).acp_rev = (*pci).revision;
    match (*pci).revision {
        0x01 => {
            (*chip).name = b"acp_asoc_renoir\0".as_ptr() as *mut core::ffi::c_char;
            (*chip).rsrc = &rn_rsrc as *const c_void;
            (*chip).acp_hw_ops_init = Some(acp31_hw_ops_init);
            (*chip).machines = &snd_soc_acpi_amd_acp_machines as *const c_void;
        }
        0x6f => {
            (*chip).name = b"acp_asoc_rembrandt\0".as_ptr() as *mut core::ffi::c_char;
            (*chip).rsrc = &rmb_rsrc as *const c_void;
            (*chip).acp_hw_ops_init = Some(acp6x_hw_ops_init);
            (*chip).machines = &snd_soc_acpi_amd_rmb_acp_machines as *const c_void;
        }
        0x63 => {
            (*chip).name = b"acp_asoc_acp63\0".as_ptr() as *mut core::ffi::c_char;
            (*chip).rsrc = &acp63_rsrc as *const c_void;
            (*chip).acp_hw_ops_init = Some(acp63_hw_ops_init);
            (*chip).machines = &snd_soc_acpi_amd_acp63_acp_machines as *const c_void;
        }
        0x70 | 0x71 | 0x72 => {
            (*chip).name = b"acp_asoc_acp70\0".as_ptr() as *mut core::ffi::c_char;
            (*chip).rsrc = &acp70_rsrc as *const c_void;
            (*chip).acp_hw_ops_init = Some(acp70_hw_ops_init);
            (*chip).machines = &snd_soc_acpi_amd_acp70_acp_machines as *const c_void;
        }
        _ => {
            dev_err(
                dev,
                b"Unsupported device revision:0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                (*pci).revision,
            );
            return -EINVAL;
        }
    }
    (*chip).flag = flag;

    addr = pci_resource_start(pci, 0);
    (*chip).base = pcim_iomap_region(
        pci,
        0,
        b"AMD ACP3x audio\0".as_ptr() as *const core::ffi::c_char,
    );
    if IS_ERR((*chip).base as *const c_void) {
        return PTR_ERR((*chip).base as *const c_void);
    }

    (*chip).addr = addr;

    ((*chip).acp_hw_ops_init.unwrap())(chip);
    ret = acp_hw_init(chip);
    if ret != 0 {
        acp_hw_deinit(chip);
        return ret;
    }

    ret = devm_request_irq(
        dev,
        (*pci).irq,
        irq_handler,
        IRQF_SHARED,
        b"ACP_I2S_IRQ\0".as_ptr() as *const core::ffi::c_char,
        chip as *mut c_void,
    );
    if ret != 0 {
        dev_err(
            &mut (*pci).dev,
            b"ACP I2S IRQ request failed %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        acp_hw_deinit(chip);
        return ret;
    }

    check_acp_config(pci, chip);
    if !(*chip).is_pdm_dev && !(*chip).is_i2s_config {
        dev_set_drvdata(&mut (*pci).dev, chip as *mut c_void);
        pm_runtime_set_autosuspend_delay(&mut (*pci).dev, 2000);
        pm_runtime_use_autosuspend(&mut (*pci).dev);
        pm_runtime_put_noidle(&mut (*pci).dev);
        pm_runtime_allow(&mut (*pci).dev);
        return ret;
    }

    ret = create_acp_platform_devs(pci, chip, addr);
    if ret < 0 {
        dev_err(
            &mut (*pci).dev,
            b"ACP platform devices creation failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
        acp_hw_deinit(chip);
        return ret;
    }

    (*chip).chip_pdev = (*chip).acp_plat_dev;
    (*chip).dev = &mut (*(*chip).acp_plat_dev).dev;

    acp_machine_select(chip);

    INIT_LIST_HEAD(&mut (*chip).stream_list as *mut c_void);
    spin_lock_init(&mut (*chip).acp_lock as *mut c_void);

    dev_set_drvdata(&mut (*pci).dev, chip as *mut c_void);
    pm_runtime_set_autosuspend_delay(&mut (*pci).dev, 2000);
    pm_runtime_use_autosuspend(&mut (*pci).dev);
    pm_runtime_put_noidle(&mut (*pci).dev);
    pm_runtime_allow(&mut (*pci).dev);
    ret
}

unsafe extern "C" fn snd_acp_suspend(dev: *mut device) -> c_int {
    let chip: *mut acp_chip_info;
    let ret: c_int;

    chip = dev_get_drvdata(dev) as *mut acp_chip_info;
    ret = acp_hw_deinit(chip);
    if ret != 0 {
        dev_err(
            dev,
            b"ACP de-init failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }
    ret
}

unsafe extern "C" fn snd_acp_resume(dev: *mut device) -> c_int {
    let chip: *mut acp_chip_info;
    let mut ret: c_int;

    chip = dev_get_drvdata(dev) as *mut acp_chip_info;
    ret = acp_hw_init(chip);
    if ret != 0 {
        dev_err(
            dev,
            b"ACP init failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    ret = acp_hw_en_interrupts(chip);
    if ret != 0 {
        dev_err(
            dev,
            b"ACP en-interrupts failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    ret
}

// static const struct dev_pm_ops acp_pm_ops = {
//      RUNTIME_PM_OPS(snd_acp_suspend, snd_acp_resume, NULL)
//      SYSTEM_SLEEP_PM_OPS(snd_acp_suspend, snd_acp_resume)
// };
// The Linux PM helper macros are preserved here as conditional/dependency intent.
static acp_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn acp_pci_remove(pci: *mut pci_dev) {
    let chip: *mut acp_chip_info;
    let ret: c_int;

    chip = pci_get_drvdata(pci) as *mut acp_chip_info;
    pm_runtime_forbid(&mut (*pci).dev);
    pm_runtime_get_noresume(&mut (*pci).dev);
    if !(*chip).dmic_codec_dev.is_null() {
        platform_device_unregister((*chip).dmic_codec_dev);
    }
    if !(*chip).acp_plat_dev.is_null() {
        platform_device_unregister((*chip).acp_plat_dev);
    }
    if !(*chip).mach_dev.is_null() {
        platform_device_unregister((*chip).mach_dev);
    }

    ret = acp_hw_deinit(chip);
    if ret != 0 {
        dev_err(
            &mut (*pci).dev,
            b"ACP de-init failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }
}

/* PCI IDs */
// static const struct pci_device_id acp_pci_ids[] = {
//      { PCI_DEVICE(PCI_VENDOR_ID_AMD, ACP_PCI_DEV_ID)},
//      { 0, }
// };
static acp_pci_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_AMD,
        device: unsafe { ACP_PCI_DEV_ID },
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
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
// MODULE_DEVICE_TABLE(pci, acp_pci_ids);

/* pci_driver definition */
static mut snd_amd_acp_pci_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const core::ffi::c_char },
    id_table: acp_pci_ids.as_ptr(),
    probe: Some(acp_pci_probe),
    remove: Some(acp_pci_remove),
    driver: device_driver {
        pm: unsafe { pm_ptr(&acp_pm_ops) },
    },
};
// module_pci_driver(snd_amd_acp_pci_driver);

// MODULE_DESCRIPTION("AMD ACP common PCI support");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_IMPORT_NS("SND_SOC_ACP_COMMON");
// MODULE_ALIAS(DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
