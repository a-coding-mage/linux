// SPDX-License-Identifier: GPL-2.0+
//
// AMD ACP PCI Driver
//
// Copyright 2016 Advanced Micro Devices, Inc.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

// Dependencies from:
// <linux/pci.h>
// <linux/module.h>
// <linux/io.h>
// <linux/platform_device.h>
// <linux/interrupt.h>
// <linux/pm_runtime.h>
// <linux/delay.h>
// "acp3x.h"

type bool_t = bool;
type u32 = u32;
type resource_size_t = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_uint,
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
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const c_char,
    pub flags: c_uint,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device_info {
    pub parent: *mut device,
    pub fwnode: *mut c_void,
    pub name: *const c_char,
    pub id: c_int,
    pub res: *const resource,
    pub num_res: c_uint,
    pub data: *const c_void,
    pub size_data: usize,
    pub dma_mask: u64,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
pub struct acp3x_dev_data {
    pub acp3x_base: *mut c_void,
    pub acp3x_audio_mode: bool_t,
    pub res: *mut resource,
    pub pdev: [*mut platform_device; ACP3x_DEVS as usize],
    pub pme_en: u32,
}

extern "C" {
    static KBUILD_MODNAME: c_char;

    static ACP3x_DEVS: c_int;
    static ACP_PGFSM_STATUS_MASK: u32;
    static ACP_POWER_ON_IN_PROGRESS: u32;
    static ACP_PGFSM_CNTL_POWER_ON_MASK: u32;
    static ACP3x_SOFT_RESET__SoftResetAudDone_MASK: u32;
    static ACP_EXT_INTR_STAT_CLEAR_MASK: u32;
    static ACP3x_REG_END: u32;
    static ACP3x_REG_START: u32;
    static ACP3x_I2STDM_REG_START: u32;
    static ACP3x_I2STDM_REG_END: u32;
    static ACP3x_BT_TDM_REG_START: u32;
    static ACP3x_BT_TDM_REG_END: u32;
    static ACP3x_I2S_MODE: bool_t;
    static I2S_MODE: u32;

    static mmACP_PGFSM_STATUS: usize;
    static mmACP_PGFSM_CONTROL: usize;
    static mmACP_PME_EN: usize;
    static mmACP_SOFT_RESET: usize;
    static mmACP_EXTERNAL_INTR_ENB: usize;
    static mmACP_EXTERNAL_INTR_STAT: usize;
    static mmACP_EXTERNAL_INTR_CNTL: usize;
    static mmACP_I2S_PIN_CONFIG: usize;

    static GFP_KERNEL: c_uint;
    static IRQF_SHARED: c_uint;
    static IORESOURCE_MEM: c_uint;
    static IORESOURCE_IRQ: c_uint;
    static PCI_VENDOR_ID_AMD: u32;
    static PCI_CLASS_MULTIMEDIA_OTHER: u32;

    static ENODEV: c_int;
    static ENOMEM: c_int;
    static ETIMEDOUT: c_int;

    fn rv_readl(addr: *mut c_void) -> u32;
    fn rv_writel(val: u32, addr: *mut c_void);
    fn udelay(usecs: c_uint);
    fn cpu_relax();

    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    fn pci_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_disable_device(pci: *mut pci_dev);
    fn pci_request_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_release_regions(pci: *mut pci_dev);
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> u32;
    fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> u32;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: u32, size: u32) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;

    fn platform_device_register_full(
        pdevinfo: *const platform_device_info,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_allow(dev: *mut device);
    fn pm_runtime_forbid(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
}

unsafe fn acp_add(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn acp3x_power_on(adata: *mut acp3x_dev_data) -> c_int {
    let acp3x_base = (*adata).acp3x_base;
    let mut val: u32;
    let mut timeout: c_int;

    val = rv_readl(acp_add(acp3x_base, mmACP_PGFSM_STATUS));

    if val == 0 {
        return val as c_int;
    }

    if !((val & ACP_PGFSM_STATUS_MASK) == ACP_POWER_ON_IN_PROGRESS) {
        rv_writel(
            ACP_PGFSM_CNTL_POWER_ON_MASK,
            acp_add(acp3x_base, mmACP_PGFSM_CONTROL),
        );
    }
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = rv_readl(acp_add(acp3x_base, mmACP_PGFSM_STATUS));
        if val == 0 {
            /* ACP power On clears PME_EN.
             * Restore the value to its prior state
             */
            rv_writel((*adata).pme_en, acp_add(acp3x_base, mmACP_PME_EN));
            return 0;
        }
        udelay(1);
    }
    -ETIMEDOUT
}

unsafe extern "C" fn acp3x_reset(acp3x_base: *mut c_void) -> c_int {
    let mut val: u32;
    let mut timeout: c_int;

    rv_writel(1, acp_add(acp3x_base, mmACP_SOFT_RESET));
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = rv_readl(acp_add(acp3x_base, mmACP_SOFT_RESET));
        if (val & ACP3x_SOFT_RESET__SoftResetAudDone_MASK) != 0 {
            break;
        }
        cpu_relax();
    }
    rv_writel(0, acp_add(acp3x_base, mmACP_SOFT_RESET));
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = rv_readl(acp_add(acp3x_base, mmACP_SOFT_RESET));
        if val == 0 {
            return 0;
        }
        cpu_relax();
    }
    -ETIMEDOUT
}

unsafe extern "C" fn acp3x_enable_interrupts(acp_base: *mut c_void) {
    rv_writel(0x01, acp_add(acp_base, mmACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp3x_disable_interrupts(acp_base: *mut c_void) {
    rv_writel(
        ACP_EXT_INTR_STAT_CLEAR_MASK,
        acp_add(acp_base, mmACP_EXTERNAL_INTR_STAT),
    );
    rv_writel(0x00, acp_add(acp_base, mmACP_EXTERNAL_INTR_CNTL));
    rv_writel(0x00, acp_add(acp_base, mmACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp3x_init(adata: *mut acp3x_dev_data) -> c_int {
    let acp3x_base = (*adata).acp3x_base;
    let mut ret: c_int;

    /* power on */
    ret = acp3x_power_on(adata);
    if ret != 0 {
        pr_err(b"ACP3x power on failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    /* Reset */
    ret = acp3x_reset(acp3x_base);
    if ret != 0 {
        pr_err(b"ACP3x reset failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    acp3x_enable_interrupts(acp3x_base);
    0
}

unsafe extern "C" fn acp3x_deinit(acp3x_base: *mut c_void) -> c_int {
    let mut ret: c_int;

    acp3x_disable_interrupts(acp3x_base);
    /* Reset */
    ret = acp3x_reset(acp3x_base);
    if ret != 0 {
        pr_err(b"ACP3x reset failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

unsafe extern "C" fn snd_acp3x_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    let mut adata: *mut acp3x_dev_data;
    let mut pdevinfo: [platform_device_info; ACP3x_DEVS as usize] = zeroed();
    let irqflags: c_uint;
    let mut ret: c_int;
    let mut i: c_int;
    let addr: u32;
    let mut val: u32;

    /* Raven device detection */
    if (*pci).revision != 0x00 {
        return -ENODEV;
    }

    if pci_enable_device(pci) != 0 {
        dev_err(
            &mut (*pci).dev,
            b"pci_enable_device failed\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    ret = pci_request_regions(pci, b"AMD ACP3x audio\0".as_ptr() as *const c_char);
    if ret < 0 {
        dev_err(
            &mut (*pci).dev,
            b"pci_request_regions failed\n\0".as_ptr() as *const c_char,
        );
        pci_disable_device(pci);
        return ret;
    }

    adata = devm_kzalloc(
        &mut (*pci).dev,
        size_of::<acp3x_dev_data>(),
        GFP_KERNEL,
    ) as *mut acp3x_dev_data;
    if adata.is_null() {
        ret = -ENOMEM;
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }

    irqflags = IRQF_SHARED;

    addr = pci_resource_start(pci, 0);
    (*adata).acp3x_base = devm_ioremap(&mut (*pci).dev, addr, pci_resource_len(pci, 0));
    if (*adata).acp3x_base.is_null() {
        ret = -ENOMEM;
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }
    pci_set_master(pci);
    pci_set_drvdata(pci, adata as *mut c_void);
    /* Save ACP_PME_EN state */
    (*adata).pme_en = rv_readl(acp_add((*adata).acp3x_base, mmACP_PME_EN));
    ret = acp3x_init(adata);
    if ret != 0 {
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }

    val = rv_readl(acp_add((*adata).acp3x_base, mmACP_I2S_PIN_CONFIG));
    match val {
        x if x == I2S_MODE => {
            (*adata).res = devm_kzalloc(
                &mut (*pci).dev,
                size_of::<resource>() * 4,
                GFP_KERNEL,
            ) as *mut resource;
            if (*adata).res.is_null() {
                ret = -ENOMEM;
                if acp3x_deinit((*adata).acp3x_base) != 0 {
                    dev_err(
                        &mut (*pci).dev,
                        b"ACP de-init failed\n\0".as_ptr() as *const c_char,
                    );
                }
                pci_release_regions(pci);
                pci_disable_device(pci);
                return ret;
            }

            (*adata).res.add(0).as_mut().unwrap().name =
                b"acp3x_i2s_iomem\0".as_ptr() as *const c_char;
            (*adata).res.add(0).as_mut().unwrap().flags = IORESOURCE_MEM;
            (*adata).res.add(0).as_mut().unwrap().start = addr;
            (*adata).res.add(0).as_mut().unwrap().end =
                addr.wrapping_add(ACP3x_REG_END.wrapping_sub(ACP3x_REG_START));

            (*adata).res.add(1).as_mut().unwrap().name =
                b"acp3x_i2s_sp\0".as_ptr() as *const c_char;
            (*adata).res.add(1).as_mut().unwrap().flags = IORESOURCE_MEM;
            (*adata).res.add(1).as_mut().unwrap().start =
                addr.wrapping_add(ACP3x_I2STDM_REG_START);
            (*adata).res.add(1).as_mut().unwrap().end = addr.wrapping_add(ACP3x_I2STDM_REG_END);

            (*adata).res.add(2).as_mut().unwrap().name =
                b"acp3x_i2s_bt\0".as_ptr() as *const c_char;
            (*adata).res.add(2).as_mut().unwrap().flags = IORESOURCE_MEM;
            (*adata).res.add(2).as_mut().unwrap().start =
                addr.wrapping_add(ACP3x_BT_TDM_REG_START);
            (*adata).res.add(2).as_mut().unwrap().end = addr.wrapping_add(ACP3x_BT_TDM_REG_END);

            (*adata).res.add(3).as_mut().unwrap().name =
                b"acp3x_i2s_irq\0".as_ptr() as *const c_char;
            (*adata).res.add(3).as_mut().unwrap().flags = IORESOURCE_IRQ;
            (*adata).res.add(3).as_mut().unwrap().start = (*pci).irq;
            (*adata).res.add(3).as_mut().unwrap().end = (*adata).res.add(3).as_ref().unwrap().start;

            (*adata).acp3x_audio_mode = ACP3x_I2S_MODE;

            ptr::write_bytes(
                pdevinfo.as_mut_ptr() as *mut u8,
                0,
                size_of::<[platform_device_info; ACP3x_DEVS as usize]>(),
            );
            pdevinfo[0].name = b"acp3x_rv_i2s_dma\0".as_ptr() as *const c_char;
            pdevinfo[0].id = 0;
            pdevinfo[0].parent = &mut (*pci).dev;
            pdevinfo[0].num_res = 4;
            pdevinfo[0].res = (*adata).res.add(0);
            pdevinfo[0].data = &irqflags as *const c_uint as *const c_void;
            pdevinfo[0].size_data = size_of::<c_uint>();

            pdevinfo[1].name = b"acp3x_i2s_playcap\0".as_ptr() as *const c_char;
            pdevinfo[1].id = 0;
            pdevinfo[1].parent = &mut (*pci).dev;
            pdevinfo[1].num_res = 1;
            pdevinfo[1].res = (*adata).res.add(1);

            pdevinfo[2].name = b"acp3x_i2s_playcap\0".as_ptr() as *const c_char;
            pdevinfo[2].id = 1;
            pdevinfo[2].parent = &mut (*pci).dev;
            pdevinfo[2].num_res = 1;
            pdevinfo[2].res = (*adata).res.add(1);

            pdevinfo[3].name = b"acp3x_i2s_playcap\0".as_ptr() as *const c_char;
            pdevinfo[3].id = 2;
            pdevinfo[3].parent = &mut (*pci).dev;
            pdevinfo[3].num_res = 1;
            pdevinfo[3].res = (*adata).res.add(2);
            i = 0;
            while i < ACP3x_DEVS {
                (*adata).pdev[i as usize] = platform_device_register_full(&pdevinfo[i as usize]);
                if IS_ERR((*adata).pdev[i as usize] as *const c_void) {
                    dev_err(
                        &mut (*pci).dev,
                        b"cannot register %s device\n\0".as_ptr() as *const c_char,
                        pdevinfo[i as usize].name,
                    );
                    ret = PTR_ERR((*adata).pdev[i as usize] as *const c_void);
                    if val == I2S_MODE {
                        i = 0;
                        while i < ACP3x_DEVS {
                            platform_device_unregister((*adata).pdev[i as usize]);
                            i += 1;
                        }
                    }
                    if acp3x_deinit((*adata).acp3x_base) != 0 {
                        dev_err(
                            &mut (*pci).dev,
                            b"ACP de-init failed\n\0".as_ptr() as *const c_char,
                        );
                    }
                    pci_release_regions(pci);
                    pci_disable_device(pci);
                    return ret;
                }
                i += 1;
            }
        }
        _ => {
            dev_info(
                &mut (*pci).dev,
                b"ACP audio mode : %d\n\0".as_ptr() as *const c_char,
                val,
            );
        }
    }
    pm_runtime_set_autosuspend_delay(&mut (*pci).dev, 2000);
    pm_runtime_use_autosuspend(&mut (*pci).dev);
    pm_runtime_put_noidle(&mut (*pci).dev);
    pm_runtime_allow(&mut (*pci).dev);
    0
}

unsafe extern "C" fn snd_acp3x_suspend(dev: *mut device) -> c_int {
    let mut ret: c_int;
    let adata: *mut acp3x_dev_data;

    adata = dev_get_drvdata(dev) as *mut acp3x_dev_data;
    ret = acp3x_deinit((*adata).acp3x_base);
    if ret != 0 {
        dev_err(dev, b"ACP de-init failed\n\0".as_ptr() as *const c_char);
    } else {
        dev_dbg(dev, b"ACP de-initialized\n\0".as_ptr() as *const c_char);
    }

    0
}

unsafe extern "C" fn snd_acp3x_resume(dev: *mut device) -> c_int {
    let mut ret: c_int;
    let adata: *mut acp3x_dev_data;

    adata = dev_get_drvdata(dev) as *mut acp3x_dev_data;
    ret = acp3x_init(adata);
    if ret != 0 {
        dev_err(dev, b"ACP init failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

static acp3x_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(snd_acp3x_suspend),
    runtime_resume: Some(snd_acp3x_resume),
    resume: Some(snd_acp3x_resume),
};

unsafe extern "C" fn snd_acp3x_remove(pci: *mut pci_dev) {
    let adata: *mut acp3x_dev_data;
    let mut i: c_int;
    let ret: c_int;

    adata = pci_get_drvdata(pci) as *mut acp3x_dev_data;
    if (*adata).acp3x_audio_mode == ACP3x_I2S_MODE {
        i = 0;
        while i < ACP3x_DEVS {
            platform_device_unregister((*adata).pdev[i as usize]);
            i += 1;
        }
    }
    ret = acp3x_deinit((*adata).acp3x_base);
    if ret != 0 {
        dev_err(
            &mut (*pci).dev,
            b"ACP de-init failed\n\0".as_ptr() as *const c_char,
        );
    }
    pm_runtime_forbid(&mut (*pci).dev);
    pm_runtime_get_noresume(&mut (*pci).dev);
    pci_release_regions(pci);
    pci_disable_device(pci);
}

const fn PCI_DEVICE(vend: u32, dev: u32) -> pci_device_id {
    pci_device_id {
        vendor: vend,
        device: dev,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

static snd_acp3x_ids: [pci_device_id; 2] = [
    pci_device_id {
        class: PCI_CLASS_MULTIMEDIA_OTHER << 8,
        class_mask: 0xffffff,
        ..PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x15e2)
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
// MODULE_DEVICE_TABLE(pci, snd_acp3x_ids);

static mut acp3x_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const c_char },
    id_table: snd_acp3x_ids.as_ptr(),
    probe: Some(snd_acp3x_probe),
    remove: Some(snd_acp3x_remove),
    driver: device_driver { pm: &acp3x_pm },
};

// module_pci_driver(acp3x_driver);
//
// MODULE_AUTHOR("Vishnuvardhanrao.Ravulapati@amd.com");
// MODULE_AUTHOR("Maruthi.Bayyavarapu@amd.com");
// MODULE_DESCRIPTION("AMD ACP3x PCI driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
