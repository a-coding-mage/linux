// SPDX-License-Identifier: GPL-2.0+
/*
 * AMD Yellow Carp ACP PCI Driver
 *
 * Copyright 2021 Advanced Micro Devices, Inc.
 */

// Translated from Linux kernel C source. Includes from linux/*, sound/*, and
// "acp6x.h" are represented here by extern declarations and expected constants.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

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
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const c_char,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_device_info {
    pub parent: *mut device,
    pub fwnode: *mut c_void,
    pub name: *const c_char,
    pub id: c_int,
    pub res: *mut resource,
    pub num_res: c_uint,
    pub data: *const c_void,
    pub size_data: usize,
    pub dma_mask: u64,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pdm_dev_data {
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_driver_inner {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: pci_driver_inner,
}

pub type resource_size_t = u64;
pub type irqreturn_t = c_uint;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_uint = 0x80;

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_ulong = 0x00000200;

const PCI_VENDOR_ID_AMD: c_uint = 0x1022;
const PCI_CLASS_MULTIMEDIA_OTHER: c_uint = 0x0480;

extern "C" {
    static KBUILD_MODNAME: c_char;
    static acp6x_pm: dev_pm_ops;

    static ACP6x_DEVS: c_int;
    static ACP_PGFSM_STATUS: usize;
    static ACP_PGFSM_STATUS_MASK: u32;
    static ACP_POWER_ON_IN_PROGRESS: u32;
    static ACP_PGFSM_CNTL_POWER_ON_MASK: u32;
    static ACP_PGFSM_CONTROL: usize;
    static ACP_SOFT_RESET: usize;
    static ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32;
    static ACP_EXTERNAL_INTR_ENB: usize;
    static ACP_EXT_INTR_STAT_CLEAR_MASK: u32;
    static ACP_EXTERNAL_INTR_STAT: usize;
    static ACP_EXTERNAL_INTR_CNTL: usize;
    static ACP_CONTROL: usize;
    static ACP_CLKMUX_SEL: usize;
    static PDM_DMA_STAT: c_uint;
    static ACP_PIN_CONFIG: usize;
    static ACP_CONFIG_0: c_int;
    static ACP_CONFIG_1: c_int;
    static ACP_CONFIG_2: c_int;
    static ACP_CONFIG_3: c_int;
    static ACP_CONFIG_9: c_int;
    static ACP_CONFIG_10: c_int;
    static ACP_CONFIG_11: c_int;
    static ACP_CONFIG_12: c_int;
    static ACP_CONFIG_13: c_int;
    static ACP_CONFIG_14: c_int;
    static ACP_CONFIG_15: c_int;
    static ACP6x_REG_END: resource_size_t;
    static ACP6x_REG_START: resource_size_t;
    static ACP6x_PDM_MODE: bool;
    static ACP_SUSPEND_DELAY_MS: c_int;
    static ACP_DEVICE_ID: c_uint;

    fn acp6x_readl(addr: *mut c_void) -> u32;
    fn acp6x_writel(val: u32, addr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn cpu_relax();
    fn pr_err(fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_uint;
    fn pci_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_request_regions(pci: *mut pci_dev, res_name: *const c_char) -> c_int;
    fn pci_release_regions(pci: *mut pci_dev);
    fn pci_disable_device(pci: *mut pci_dev);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> resource_size_t;
    fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> resource_size_t;
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut c_void;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_device_register_full(pdevinfo: *const platform_device_info) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_allow(dev: *mut device);
    fn pm_runtime_forbid(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
    fn module_pci_driver(driver: *mut pci_driver);
}

#[repr(C)]
pub struct acp6x_dev_data {
    pub acp6x_base: *mut c_void,
    pub res: *mut resource,
    pub acp6x_audio_mode: bool,
    pub pdev: [*mut platform_device; ACP6X_DEVS_USIZE],
}

const ACP6X_DEVS_USIZE: usize = 3;

#[inline]
const fn BIT(nr: c_uint) -> u32 {
    1u32 << nr
}

unsafe fn acp_addr(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn acp6x_power_on(acp_base: *mut c_void) -> c_int {
    let mut val: u32;
    let mut timeout: c_int;

    val = acp6x_readl(acp_addr(acp_base, ACP_PGFSM_STATUS));

    if val == 0 {
        return val as c_int;
    }

    if (val & ACP_PGFSM_STATUS_MASK) != ACP_POWER_ON_IN_PROGRESS {
        acp6x_writel(
            ACP_PGFSM_CNTL_POWER_ON_MASK,
            acp_addr(acp_base, ACP_PGFSM_CONTROL),
        );
    }
    timeout = 0;
    loop {
        timeout += 1;
        if timeout >= 500 {
            break;
        }
        val = acp6x_readl(acp_addr(acp_base, ACP_PGFSM_STATUS));
        if val == 0 {
            return 0;
        }
        udelay(1);
    }
    -ETIMEDOUT
}

unsafe extern "C" fn acp6x_reset(acp_base: *mut c_void) -> c_int {
    let mut val: u32;
    let mut timeout: c_int;

    acp6x_writel(1, acp_addr(acp_base, ACP_SOFT_RESET));
    timeout = 0;
    loop {
        timeout += 1;
        if timeout >= 500 {
            break;
        }
        val = acp6x_readl(acp_addr(acp_base, ACP_SOFT_RESET));
        if (val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK) != 0 {
            break;
        }
        cpu_relax();
    }
    acp6x_writel(0, acp_addr(acp_base, ACP_SOFT_RESET));
    timeout = 0;
    loop {
        timeout += 1;
        if timeout >= 500 {
            break;
        }
        val = acp6x_readl(acp_addr(acp_base, ACP_SOFT_RESET));
        if val == 0 {
            return 0;
        }
        cpu_relax();
    }
    -ETIMEDOUT
}

unsafe extern "C" fn acp6x_enable_interrupts(acp_base: *mut c_void) {
    acp6x_writel(0x01, acp_addr(acp_base, ACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp6x_disable_interrupts(acp_base: *mut c_void) {
    acp6x_writel(
        ACP_EXT_INTR_STAT_CLEAR_MASK,
        acp_addr(acp_base, ACP_EXTERNAL_INTR_STAT),
    );
    acp6x_writel(0x00, acp_addr(acp_base, ACP_EXTERNAL_INTR_CNTL));
    acp6x_writel(0x00, acp_addr(acp_base, ACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp6x_init(acp_base: *mut c_void) -> c_int {
    let mut ret: c_int;

    /* power on */
    ret = acp6x_power_on(acp_base);
    if ret != 0 {
        pr_err(c"ACP power on failed\n".as_ptr());
        return ret;
    }
    acp6x_writel(0x01, acp_addr(acp_base, ACP_CONTROL));
    /* Reset */
    ret = acp6x_reset(acp_base);
    if ret != 0 {
        pr_err(c"ACP reset failed\n".as_ptr());
        return ret;
    }
    acp6x_writel(0x03, acp_addr(acp_base, ACP_CLKMUX_SEL));
    acp6x_enable_interrupts(acp_base);
    0
}

unsafe extern "C" fn acp6x_deinit(acp_base: *mut c_void) -> c_int {
    let mut ret: c_int;

    acp6x_disable_interrupts(acp_base);
    /* Reset */
    ret = acp6x_reset(acp_base);
    if ret != 0 {
        pr_err(c"ACP reset failed\n".as_ptr());
        return ret;
    }
    acp6x_writel(0x00, acp_addr(acp_base, ACP_CLKMUX_SEL));
    acp6x_writel(0x00, acp_addr(acp_base, ACP_CONTROL));
    0
}

unsafe extern "C" fn acp6x_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mut adata: *mut acp6x_dev_data;
    let mut yc_pdm_data: *mut pdm_dev_data;
    let val: u32;

    adata = dev_id as *mut acp6x_dev_data;
    if adata.is_null() {
        return IRQ_NONE;
    }

    val = acp6x_readl(acp_addr((*adata).acp6x_base, ACP_EXTERNAL_INTR_STAT));
    if (val & BIT(PDM_DMA_STAT)) != 0 {
        yc_pdm_data = dev_get_drvdata(addr_of_mut!((*(*adata).pdev[0]).dev)) as *mut pdm_dev_data;
        acp6x_writel(
            BIT(PDM_DMA_STAT),
            acp_addr((*adata).acp6x_base, ACP_EXTERNAL_INTR_STAT),
        );
        if !(*yc_pdm_data).capture_stream.is_null() {
            snd_pcm_period_elapsed((*yc_pdm_data).capture_stream);
        }
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

unsafe extern "C" fn snd_acp6x_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    let mut adata: *mut acp6x_dev_data;
    let mut pdevinfo: [platform_device_info; ACP6X_DEVS_USIZE] = zeroed();
    let mut index: c_int = 0;
    let mut val: c_int = 0x00;
    let addr: resource_size_t;
    let irqflags: c_uint;
    let flag: c_uint;
    let mut ret: c_int;

    irqflags = IRQF_SHARED;

    /* Return if acp config flag is defined */
    flag = snd_amd_acp_find_config(pci);
    if flag != 0 {
        return -ENODEV;
    }

    /* Yellow Carp device check */
    match (*pci).revision {
        0x60 | 0x6f | 0x62 => {
            /* RPL */
        }
        _ => {
            dev_dbg(addr_of_mut!((*pci).dev), c"acp6x pci device not found\n".as_ptr());
            return -ENODEV;
        }
    }
    if pci_enable_device(pci) != 0 {
        dev_err(addr_of_mut!((*pci).dev), c"pci_enable_device failed\n".as_ptr());
        return -ENODEV;
    }

    ret = pci_request_regions(pci, c"AMD ACP3x audio".as_ptr());
    if ret < 0 {
        dev_err(addr_of_mut!((*pci).dev), c"pci_request_regions failed\n".as_ptr());
        goto_disable_pci(pci);
        return ret;
    }

    adata = devm_kzalloc(
        addr_of_mut!((*pci).dev),
        size_of::<acp6x_dev_data>(),
        GFP_KERNEL,
    ) as *mut acp6x_dev_data;
    if adata.is_null() {
        ret = -ENOMEM;
        goto_release_regions(pci);
        return ret;
    }

    addr = pci_resource_start(pci, 0);
    (*adata).acp6x_base = devm_ioremap(
        addr_of_mut!((*pci).dev),
        addr,
        pci_resource_len(pci, 0),
    );
    if (*adata).acp6x_base.is_null() {
        ret = -ENOMEM;
        goto_release_regions(pci);
        return ret;
    }
    pci_set_master(pci);
    pci_set_drvdata(pci, adata as *mut c_void);
    ret = acp6x_init((*adata).acp6x_base);
    if ret != 0 {
        goto_release_regions(pci);
        return ret;
    }
    val = acp6x_readl(acp_addr((*adata).acp6x_base, ACP_PIN_CONFIG)) as c_int;
    if val == ACP_CONFIG_0
        || val == ACP_CONFIG_1
        || val == ACP_CONFIG_2
        || val == ACP_CONFIG_3
        || val == ACP_CONFIG_9
        || val == ACP_CONFIG_15
    {
        dev_info(addr_of_mut!((*pci).dev), c"Audio Mode %d\n".as_ptr(), val);
    } else if val == ACP_CONFIG_10
        || val == ACP_CONFIG_11
        || val == ACP_CONFIG_12
        || val == ACP_CONFIG_13
        || val == ACP_CONFIG_14
    {
        /* PIN 10 to 14 is reserve for RPL */
        if (*pci).revision == 0x62 {
            dev_info(addr_of_mut!((*pci).dev), c"RPL Audio Mode %d\n".as_ptr(), val);
        } else {
            ret = snd_acp6x_probe_register_devs(pci, adata, &mut pdevinfo, &mut index, addr);
            if ret != 0 {
                return ret;
            }
        }
    } else {
        ret = snd_acp6x_probe_register_devs(pci, adata, &mut pdevinfo, &mut index, addr);
        if ret != 0 {
            return ret;
        }
    }
    ret = devm_request_irq(
        addr_of_mut!((*pci).dev),
        (*pci).irq,
        acp6x_irq_handler,
        irqflags,
        c"ACP_PCI_IRQ".as_ptr(),
        adata as *mut c_void,
    );
    if ret != 0 {
        dev_err(addr_of_mut!((*pci).dev), c"ACP PCI IRQ request failed\n".as_ptr());
        goto_unregister_devs(pci, adata, index);
        return ret;
    }
    pm_runtime_set_autosuspend_delay(addr_of_mut!((*pci).dev), ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(addr_of_mut!((*pci).dev));
    pm_runtime_put_noidle(addr_of_mut!((*pci).dev));
    pm_runtime_allow(addr_of_mut!((*pci).dev));

    0
}

unsafe fn snd_acp6x_probe_register_devs(
    pci: *mut pci_dev,
    adata: *mut acp6x_dev_data,
    pdevinfo: &mut [platform_device_info; ACP6X_DEVS_USIZE],
    index: &mut c_int,
    addr: resource_size_t,
) -> c_int {
    let mut ret: c_int;

    (*adata).res = devm_kzalloc(
        addr_of_mut!((*pci).dev),
        size_of::<resource>(),
        GFP_KERNEL,
    ) as *mut resource;
    if (*adata).res.is_null() {
        ret = -ENOMEM;
        goto_de_init(pci, adata);
        return ret;
    }

    (*(*adata).res).name = c"acp_iomem".as_ptr();
    (*(*adata).res).flags = IORESOURCE_MEM;
    (*(*adata).res).start = addr;
    (*(*adata).res).end = addr + (ACP6x_REG_END - ACP6x_REG_START);

    (*adata).acp6x_audio_mode = ACP6x_PDM_MODE;

    *pdevinfo = zeroed();
    pdevinfo[0].name = c"acp_yc_pdm_dma".as_ptr();
    pdevinfo[0].id = 0;
    pdevinfo[0].parent = addr_of_mut!((*pci).dev);
    pdevinfo[0].num_res = 1;
    pdevinfo[0].res = (*adata).res;

    pdevinfo[1].name = c"dmic-codec".as_ptr();
    pdevinfo[1].id = 0;
    pdevinfo[1].parent = addr_of_mut!((*pci).dev);

    pdevinfo[2].name = c"acp_yc_mach".as_ptr();
    pdevinfo[2].id = 0;
    pdevinfo[2].parent = addr_of_mut!((*pci).dev);

    *index = 0;
    while *index < ACP6X_DEVS_USIZE as c_int {
        (*adata).pdev[*index as usize] =
            platform_device_register_full(&pdevinfo[*index as usize]);
        if IS_ERR((*adata).pdev[*index as usize] as *const c_void) {
            dev_err(
                addr_of_mut!((*pci).dev),
                c"cannot register %s device\n".as_ptr(),
                pdevinfo[*index as usize].name,
            );
            ret = PTR_ERR((*adata).pdev[*index as usize] as *const c_void);
            goto_unregister_devs(pci, adata, *index);
            return ret;
        }
        *index += 1;
    }
    0
}

unsafe fn goto_unregister_devs(pci: *mut pci_dev, adata: *mut acp6x_dev_data, mut index: c_int) {
    index -= 1;
    while index >= 0 {
        platform_device_unregister((*adata).pdev[index as usize]);
        index -= 1;
    }
    goto_de_init(pci, adata);
}

unsafe fn goto_de_init(pci: *mut pci_dev, adata: *mut acp6x_dev_data) {
    if acp6x_deinit((*adata).acp6x_base) != 0 {
        dev_err(addr_of_mut!((*pci).dev), c"ACP de-init failed\n".as_ptr());
    }
    goto_release_regions(pci);
}

unsafe fn goto_release_regions(pci: *mut pci_dev) {
    pci_release_regions(pci);
    goto_disable_pci(pci);
}

unsafe fn goto_disable_pci(pci: *mut pci_dev) {
    pci_disable_device(pci);
}

unsafe extern "C" fn snd_acp6x_suspend(dev: *mut device) -> c_int {
    let adata: *mut acp6x_dev_data;
    let ret: c_int;

    adata = dev_get_drvdata(dev) as *mut acp6x_dev_data;
    ret = acp6x_deinit((*adata).acp6x_base);
    if ret != 0 {
        dev_err(dev, c"ACP de-init failed\n".as_ptr());
    }
    ret
}

unsafe extern "C" fn snd_acp6x_resume(dev: *mut device) -> c_int {
    let adata: *mut acp6x_dev_data;
    let ret: c_int;

    adata = dev_get_drvdata(dev) as *mut acp6x_dev_data;
    ret = acp6x_init((*adata).acp6x_base);
    if ret != 0 {
        dev_err(dev, c"ACP init failed\n".as_ptr());
    }
    ret
}

// static const struct dev_pm_ops acp6x_pm = {
//     RUNTIME_PM_OPS(snd_acp6x_suspend, snd_acp6x_resume, NULL)
//     SYSTEM_SLEEP_PM_OPS(snd_acp6x_suspend, snd_acp6x_resume)
// };
// The RUNTIME_PM_OPS and SYSTEM_SLEEP_PM_OPS macro expansion is supplied by
// Linux PM headers and is preserved here as the external acp6x_pm declaration.

unsafe extern "C" fn snd_acp6x_remove(pci: *mut pci_dev) {
    let adata: *mut acp6x_dev_data;
    let ret: c_int;
    let mut index: c_int;

    adata = pci_get_drvdata(pci) as *mut acp6x_dev_data;
    if (*adata).acp6x_audio_mode == ACP6x_PDM_MODE {
        index = 0;
        while index < ACP6X_DEVS_USIZE as c_int {
            platform_device_unregister((*adata).pdev[index as usize]);
            index += 1;
        }
    }
    ret = acp6x_deinit((*adata).acp6x_base);
    if ret != 0 {
        dev_err(addr_of_mut!((*pci).dev), c"ACP de-init failed\n".as_ptr());
    }
    pm_runtime_forbid(addr_of_mut!((*pci).dev));
    pm_runtime_get_noresume(addr_of_mut!((*pci).dev));
    pci_release_regions(pci);
    pci_disable_device(pci);
}

static snd_acp6x_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_AMD,
        device: unsafe { ACP_DEVICE_ID },
        subvendor: !0,
        subdevice: !0,
        class: unsafe { ACP_CLASS_MULTIMEDIA_OTHER_SHIFTED },
        class_mask: 0xffffff,
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

static mut yc_acp6x_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const c_char },
    id_table: snd_acp6x_ids.as_ptr(),
    probe: Some(snd_acp6x_probe),
    remove: Some(snd_acp6x_remove),
    driver: pci_driver_inner {
        pm: unsafe { pm_ptr(&acp6x_pm as *const dev_pm_ops) },
    },
};

const ACP_CLASS_MULTIMEDIA_OTHER_SHIFTED: c_uint = PCI_CLASS_MULTIMEDIA_OTHER << 8;

// MODULE_DEVICE_TABLE(pci, snd_acp6x_ids);
// module_pci_driver(yc_acp6x_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP Yellow Carp PCI driver");
// MODULE_LICENSE("GPL v2");
//
// Module metadata and registration macros are build-system/module declarations
// in C. The driver object and registration dependency are preserved above.

#[allow(dead_code)]
unsafe fn register_yc_acp6x_driver() {
    module_pci_driver(addr_of_mut!(yc_acp6x_driver));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
