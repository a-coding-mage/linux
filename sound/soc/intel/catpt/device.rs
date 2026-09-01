// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//
// Special thanks to:
//    Marcin Barlik <marcin.barlik@intel.com>
//    Piotr Papierkowski <piotr.papierkowski@intel.com>
//
// for sharing LPT-LP and WTP-LP AudioDSP architecture expertise and
// helping backtrack its historical background
//

// C dependencies:
// linux/acpi.h, linux/dma-mapping.h, linux/interrupt.h, linux/module.h,
// linux/pci.h, linux/platform_device.h, linux/pm_runtime.h,
// sound/intel-dsp-config.h, sound/soc.h, sound/soc-acpi.h,
// core.h, registers.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);

    fn catpt_ipc_enter_dxstate(
        cdev: *mut catpt_dev,
        state: c_uint,
        dx_ctx: *mut catpt_dx_context,
    ) -> c_int;
    fn catpt_store_firmware_context(cdev: *mut catpt_dev) -> c_int;
    fn catpt_dsp_power_down(cdev: *mut catpt_dev) -> c_int;
    fn catpt_dsp_power_up(cdev: *mut catpt_dev) -> c_int;
    fn catpt_boot_firmware(cdev: *mut catpt_dev, restore: bool) -> c_int;
    fn catpt_ipc_set_device_format(
        cdev: *mut catpt_dev,
        devfmt: *mut catpt_ssp_device_format,
    ) -> c_int;
    fn catpt_dmac_probe(cdev: *mut catpt_dev) -> c_int;
    fn catpt_first_boot_firmware(cdev: *mut catpt_dev) -> c_int;
    fn catpt_register_plat_component(cdev: *mut catpt_dev) -> c_int;
    fn catpt_dmac_remove(cdev: *mut catpt_dev);
    fn catpt_sram_free(sram: *mut catpt_sram);
    fn catpt_dram_size(cdev: *mut catpt_dev) -> resource_size_t;
    fn catpt_iram_size(cdev: *mut catpt_dev) -> resource_size_t;
    fn catpt_ipc_init(ipc: *mut catpt_ipc, dev: *mut device);
    fn catpt_dsp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn catpt_dsp_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;

    fn snd_soc_acpi_find_machine(
        machines: *mut snd_soc_acpi_mach,
    ) -> *mut snd_soc_acpi_mach;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);

    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);

    fn init_completion(x: *mut completion);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mutex_init(lock: *mut mutex);
    fn resource_set_range(res: *mut resource, start: resource_size_t, size: resource_size_t);

    fn acpi_match_device(
        ids: *const acpi_device_id,
        dev: *mut device,
    ) -> *const acpi_device_id;
    fn snd_intel_acpi_dsp_driver_probe(dev: *mut device, id: *const c_char) -> c_int;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn dmam_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flag: c_uint,
    ) -> *mut c_void;
    fn resource_size(res: *const resource) -> resource_size_t;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;

    fn lpt_dsp_pll_shutdown(cdev: *mut catpt_dev);
    fn wpt_dsp_pll_shutdown(cdev: *mut catpt_dev);

    static catpt_attr_groups: *const attribute_group;
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub owner: *mut module,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver_embedded,
}

#[repr(C)]
pub struct device_driver_embedded {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
    pub dev_groups: *const attribute_group,
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub type resource_size_t = usize;
pub type dma_addr_t = u64;
pub type irqreturn_t = c_uint;

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct catpt_sram {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_ipc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_dx_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_ssp_device_format {
    pub iface: c_uint,
}

#[repr(C)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub spec: *const catpt_spec,
    pub fw_ready: completion,
    pub stream_list: list_head,
    pub stream_mutex: mutex,
    pub clk_mutex: mutex,
    pub devfmt: [catpt_ssp_device_format; CATPT_SSP_COUNT as usize],
    pub dram: catpt_sram,
    pub iram: catpt_sram,
    pub ipc: catpt_ipc,
    pub dx_ctx: catpt_dx_context,
    pub lpe_ba: *mut c_void,
    pub lpe_base: resource_size_t,
    pub pci_ba: *mut c_void,
    pub dxbuf_vaddr: *mut c_void,
    pub dxbuf_paddr: dma_addr_t,
    pub irq: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct catpt_spec {
    pub machines: *mut snd_soc_acpi_mach,
    pub core_id: c_uint,
    pub fw_name: *const c_char,
    pub host_dram_offset: resource_size_t,
    pub host_iram_offset: resource_size_t,
    pub host_shim_offset: resource_size_t,
    pub host_dma_offset: [resource_size_t; 2],
    pub host_ssp_offset: [resource_size_t; 2],
    pub dram_mask: c_uint,
    pub iram_mask: c_uint,
    pub d3srampgd_bit: c_uint,
    pub d3pgd_bit: c_uint,
    pub pll_shutdown: Option<unsafe extern "C" fn(*mut catpt_dev)>,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

const UINT_MAX: c_uint = c_uint::MAX;
const CATPT_DX_STATE_D3: c_uint = 3;
const CATPT_SSP_COUNT: c_int = 2;
const CATPT_SSP_IFACE_0: usize = 0;
const CATPT_SSP_IFACE_1: usize = 1;
const PLATFORM_DEVID_NONE: c_int = -1;
const GFP_KERNEL: c_uint = 0;
const IRQF_SHARED: c_ulong = 0x00000080;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const SND_INTEL_DSP_DRIVER_ANY: c_int = 0;
const SND_INTEL_DSP_DRIVER_SST: c_int = 2;

const LPT_VDRTCTL0_DSRAMPGE_MASK: c_uint = 0;
const LPT_VDRTCTL0_ISRAMPGE_MASK: c_uint = 0;
const LPT_VDRTCTL0_D3SRAMPGD: c_uint = 0;
const LPT_VDRTCTL0_D3PGD: c_uint = 0;
const WPT_VDRTCTL0_DSRAMPGE_MASK: c_uint = 0;
const WPT_VDRTCTL0_ISRAMPGE_MASK: c_uint = 0;
const WPT_VDRTCTL0_D3SRAMPGD: c_uint = 0;
const WPT_VDRTCTL0_D3PGD: c_uint = 0;

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0u64
    } else {
        (1u64 << n) - 1
    }
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) >= -4095isize
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as c_long
}

fn CATPT_IPC_RET(ret: c_int) -> c_int {
    ret
}

unsafe extern "C" fn catpt_do_suspend(dev: *mut device) -> c_int {
    let cdev = dev_get_drvdata(dev) as *mut catpt_dev;
    let mut ret: c_int;

    memset(
        &mut (*cdev).dx_ctx as *mut catpt_dx_context as *mut c_void,
        0,
        size_of::<catpt_dx_context>(),
    );
    ret = catpt_ipc_enter_dxstate(cdev, CATPT_DX_STATE_D3, &mut (*cdev).dx_ctx);
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    ret = catpt_store_firmware_context(cdev);
    if ret != 0 {
        return ret;
    }

    catpt_dsp_power_down(cdev)
}

/* Do not block the system from suspending, recover on resume() if needed. */
unsafe extern "C" fn catpt_suspend(dev: *mut device) -> c_int {
    catpt_do_suspend(dev);
    0
}

unsafe extern "C" fn catpt_resume(dev: *mut device) -> c_int {
    let cdev = dev_get_drvdata(dev) as *mut catpt_dev;
    let mut ret: c_int;
    let mut i: c_int;

    ret = catpt_dsp_power_up(cdev);
    if ret != 0 {
        return ret;
    }

    if !try_module_get((*(*dev).driver).owner) {
        dev_info(dev, c"module unloading, skipping fw boot\n".as_ptr());
        return 0;
    }
    module_put((*(*dev).driver).owner);

    ret = catpt_boot_firmware(cdev, true);
    if ret != 0 {
        dev_err(
            (*cdev).dev,
            c"boot firmware failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    /* reconfigure SSP devices after Dx transition */
    i = 0;
    while i < CATPT_SSP_COUNT {
        if (*cdev).devfmt[i as usize].iface == UINT_MAX {
            i += 1;
            continue;
        }

        ret = catpt_ipc_set_device_format(cdev, &mut (*cdev).devfmt[i as usize]);
        if ret != 0 {
            return CATPT_IPC_RET(ret);
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn catpt_runtime_suspend(dev: *mut device) -> c_int {
    if !try_module_get((*(*dev).driver).owner) {
        dev_info(dev, c"module unloading, skipping suspend\n".as_ptr());
        return 0;
    }
    module_put((*(*dev).driver).owner);

    catpt_do_suspend(dev)
}

unsafe extern "C" fn catpt_runtime_resume(dev: *mut device) -> c_int {
    catpt_resume(dev)
}

// Equivalent to:
// static const struct dev_pm_ops catpt_dev_pm = {
//     SYSTEM_SLEEP_PM_OPS(catpt_suspend, catpt_resume)
//     RUNTIME_PM_OPS(catpt_runtime_suspend, catpt_runtime_resume, NULL)
// };
static catpt_dev_pm: dev_pm_ops = dev_pm_ops { _private: [] };

/* machine board owned by CATPT is removed with this hook */
unsafe extern "C" fn board_pdev_unregister(data: *mut c_void) {
    platform_device_unregister(data as *mut platform_device);
}

unsafe extern "C" fn catpt_register_board(cdev: *mut catpt_dev) -> c_int {
    let spec = (*cdev).spec;
    let mut mach: *mut snd_soc_acpi_mach;
    let board: *mut platform_device;

    mach = snd_soc_acpi_find_machine((*spec).machines);
    if mach.is_null() {
        dev_info((*cdev).dev, c"no machines present\n".as_ptr());
        return 0;
    }

    (*mach).mach_params.platform = c"catpt-platform".as_ptr();
    board = platform_device_register_data(
        ptr::null_mut(),
        (*mach).drv_name,
        PLATFORM_DEVID_NONE,
        mach as *const c_void,
        size_of::<snd_soc_acpi_mach>(),
    );
    if IS_ERR(board as *const c_void) {
        dev_err(
            (*cdev).dev,
            c"register board failed: %ld\n".as_ptr(),
            PTR_ERR(board),
        );
        return PTR_ERR(board) as c_int;
    }

    devm_add_action_or_reset(
        (*cdev).dev,
        Some(board_pdev_unregister),
        board as *mut c_void,
    )
}

unsafe extern "C" fn catpt_probe_components(cdev: *mut catpt_dev) -> c_int {
    let mut ret: c_int;

    ret = catpt_dsp_power_up(cdev);
    if ret != 0 {
        return ret;
    }

    ret = catpt_dmac_probe(cdev);
    if ret != 0 {
        dev_err((*cdev).dev, c"DMAC probe failed: %d\n".as_ptr(), ret);
        catpt_dsp_power_down(cdev);
        return ret;
    }

    ret = catpt_first_boot_firmware(cdev);
    if ret != 0 {
        dev_err((*cdev).dev, c"first fw boot failed: %d\n".as_ptr(), ret);
        catpt_dmac_remove(cdev);
        catpt_dsp_power_down(cdev);
        return ret;
    }

    ret = catpt_register_plat_component(cdev);
    if ret != 0 {
        dev_err(
            (*cdev).dev,
            c"register plat comp failed: %d\n".as_ptr(),
            ret,
        );
        catpt_dmac_remove(cdev);
        catpt_dsp_power_down(cdev);
        return ret;
    }

    /* reflect actual ADSP state in pm_runtime */
    pm_runtime_set_active((*cdev).dev);

    pm_runtime_set_autosuspend_delay((*cdev).dev, 2000);
    pm_runtime_use_autosuspend((*cdev).dev);
    pm_runtime_mark_last_busy((*cdev).dev);
    /* Enable PM before spawning child device. See catpt_dai_pcm_new(). */
    pm_runtime_enable((*cdev).dev);

    ret = catpt_register_board(cdev);
    if ret != 0 {
        dev_err((*cdev).dev, c"register board failed: %d\n".as_ptr(), ret);
        pm_runtime_disable((*cdev).dev);
        snd_soc_unregister_component((*cdev).dev);
        catpt_dmac_remove(cdev);
        catpt_dsp_power_down(cdev);
        return ret;
    }

    0
}

unsafe extern "C" fn catpt_dev_init(
    cdev: *mut catpt_dev,
    dev: *mut device,
    spec: *const catpt_spec,
) {
    (*cdev).dev = dev;
    (*cdev).spec = spec;
    init_completion(&mut (*cdev).fw_ready);
    INIT_LIST_HEAD(&mut (*cdev).stream_list);
    mutex_init(&mut (*cdev).stream_mutex);
    mutex_init(&mut (*cdev).clk_mutex);

    /*
     * Mark both device formats as uninitialized. Once corresponding
     * cpu_dai's pcm is created, proper values are assigned.
     */
    (*cdev).devfmt[CATPT_SSP_IFACE_0].iface = UINT_MAX;
    (*cdev).devfmt[CATPT_SSP_IFACE_1].iface = UINT_MAX;

    resource_set_range(
        &mut (*cdev).dram as *mut catpt_sram as *mut resource,
        (*spec).host_dram_offset,
        catpt_dram_size(cdev),
    );
    resource_set_range(
        &mut (*cdev).iram as *mut catpt_sram as *mut resource,
        (*spec).host_iram_offset,
        catpt_iram_size(cdev),
    );
    catpt_ipc_init(&mut (*cdev).ipc, dev);
}

unsafe extern "C" fn catpt_acpi_probe(pdev: *mut platform_device) -> c_int {
    let mut spec: *const catpt_spec;
    let mut cdev: *mut catpt_dev;
    let dev: *mut device = &mut (*pdev).dev;
    let mut id: *const acpi_device_id;
    let mut res: *mut resource = ptr::null_mut();
    let mut ret: c_int;

    id = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
    if id.is_null() {
        return -ENODEV;
    }

    ret = snd_intel_acpi_dsp_driver_probe(dev, (*id).id.as_ptr());
    if ret != SND_INTEL_DSP_DRIVER_ANY && ret != SND_INTEL_DSP_DRIVER_SST {
        dev_dbg(
            dev,
            c"CATPT ACPI driver not selected, aborting probe\n".as_ptr(),
        );
        return -ENODEV;
    }

    cdev = devm_kzalloc(dev, size_of::<catpt_dev>(), GFP_KERNEL) as *mut catpt_dev;
    if cdev.is_null() {
        return -ENOMEM;
    }

    spec = (*id).driver_data as *const catpt_spec;
    catpt_dev_init(cdev, dev, spec);

    /* map DSP bar address */
    (*cdev).lpe_ba = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*cdev).lpe_ba) {
        return PTR_ERR((*cdev).lpe_ba) as c_int;
    }
    (*cdev).lpe_base = (*res).start;

    /* map PCI bar address */
    (*cdev).pci_ba = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR((*cdev).pci_ba) {
        return PTR_ERR((*cdev).pci_ba) as c_int;
    }

    /*
     * As per design HOST is responsible for preserving firmware's runtime
     * context during D0 -> D3 -> D0 transitions.  Addresses used for DMA
     * to/from HOST memory shall be outside the reserved range of 0xFFFxxxxx.
     */
    ret = dma_coerce_mask_and_coherent((*cdev).dev, DMA_BIT_MASK(31));
    if ret != 0 {
        return ret;
    }

    (*cdev).dxbuf_vaddr = dmam_alloc_coherent(
        dev,
        resource_size(&mut (*cdev).dram as *mut catpt_sram as *const resource),
        &mut (*cdev).dxbuf_paddr,
        GFP_KERNEL,
    );
    if (*cdev).dxbuf_vaddr.is_null() {
        return -ENOMEM;
    }

    ret = platform_get_irq(pdev, 0);
    if ret < 0 {
        return ret;
    }
    (*cdev).irq = ret as c_uint;

    platform_set_drvdata(pdev, cdev as *mut c_void);

    ret = devm_request_threaded_irq(
        dev,
        (*cdev).irq,
        Some(catpt_dsp_irq_handler),
        Some(catpt_dsp_irq_thread),
        IRQF_SHARED,
        c"AudioDSP".as_ptr(),
        cdev as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    catpt_probe_components(cdev)
}

unsafe extern "C" fn catpt_acpi_remove(pdev: *mut platform_device) {
    let cdev = platform_get_drvdata(pdev) as *mut catpt_dev;

    pm_runtime_disable((*cdev).dev);

    snd_soc_unregister_component((*cdev).dev);
    catpt_dmac_remove(cdev);
    catpt_dsp_power_down(cdev);

    catpt_sram_free(&mut (*cdev).iram);
    catpt_sram_free(&mut (*cdev).dram);
}

static mut lpt_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: c"INT33CA".as_ptr(),
        drv_name: c"hsw_rt5640".as_ptr(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
    snd_soc_acpi_mach {
        id: ptr::null(),
        drv_name: ptr::null(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
];

static mut wpt_machines: [snd_soc_acpi_mach; 5] = [
    snd_soc_acpi_mach {
        id: c"INT33CA".as_ptr(),
        drv_name: c"hsw_rt5640".as_ptr(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
    snd_soc_acpi_mach {
        id: c"INT343A".as_ptr(),
        drv_name: c"bdw_rt286".as_ptr(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
    snd_soc_acpi_mach {
        id: c"10EC5650".as_ptr(),
        drv_name: c"bdw-rt5650".as_ptr(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
    snd_soc_acpi_mach {
        id: c"RT5677CE".as_ptr(),
        drv_name: c"bdw-rt5677".as_ptr(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
    snd_soc_acpi_mach {
        id: ptr::null(),
        drv_name: ptr::null(),
        mach_params: snd_soc_acpi_mach_params {
            platform: ptr::null(),
        },
    },
];

static mut lpt_desc: catpt_spec = catpt_spec {
    machines: unsafe { lpt_machines.as_mut_ptr() },
    core_id: 0x01,
    fw_name: c"intel/IntcSST1.bin".as_ptr(),
    host_dram_offset: 0x000000,
    host_iram_offset: 0x080000,
    host_shim_offset: 0x0E7000,
    host_dma_offset: [0x0F0000, 0x0F8000],
    host_ssp_offset: [0x0E8000, 0x0E9000],
    dram_mask: LPT_VDRTCTL0_DSRAMPGE_MASK,
    iram_mask: LPT_VDRTCTL0_ISRAMPGE_MASK,
    d3srampgd_bit: LPT_VDRTCTL0_D3SRAMPGD,
    d3pgd_bit: LPT_VDRTCTL0_D3PGD,
    pll_shutdown: Some(lpt_dsp_pll_shutdown),
};

static mut wpt_desc: catpt_spec = catpt_spec {
    machines: unsafe { wpt_machines.as_mut_ptr() },
    core_id: 0x02,
    fw_name: c"intel/IntcSST2.bin".as_ptr(),
    host_dram_offset: 0x000000,
    host_iram_offset: 0x0A0000,
    host_shim_offset: 0x0FB000,
    host_dma_offset: [0x0FE000, 0x0FF000],
    host_ssp_offset: [0x0FC000, 0x0FD000],
    dram_mask: WPT_VDRTCTL0_DSRAMPGE_MASK,
    iram_mask: WPT_VDRTCTL0_ISRAMPGE_MASK,
    d3srampgd_bit: WPT_VDRTCTL0_D3SRAMPGD,
    d3pgd_bit: WPT_VDRTCTL0_D3PGD,
    pll_shutdown: Some(wpt_dsp_pll_shutdown),
};

static catpt_ids: [acpi_device_id; 3] = [
    acpi_device_id {
        id: [
            b'I' as c_char,
            b'N' as c_char,
            b'T' as c_char,
            b'3' as c_char,
            b'3' as c_char,
            b'C' as c_char,
            b'8' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: unsafe { &lpt_desc as *const catpt_spec as c_ulong },
    },
    acpi_device_id {
        id: [
            b'I' as c_char,
            b'N' as c_char,
            b'T' as c_char,
            b'3' as c_char,
            b'4' as c_char,
            b'3' as c_char,
            b'8' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: unsafe { &wpt_desc as *const catpt_spec as c_ulong },
    },
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, catpt_ids);

static mut catpt_acpi_driver: platform_driver = platform_driver {
    probe: Some(catpt_acpi_probe),
    remove: Some(catpt_acpi_remove),
    driver: device_driver_embedded {
        name: c"intel_catpt".as_ptr(),
        acpi_match_table: catpt_ids.as_ptr(),
        pm: &catpt_dev_pm,
        dev_groups: unsafe { catpt_attr_groups },
    },
};
// module_platform_driver(catpt_acpi_driver);

// MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
// MODULE_DESCRIPTION("Intel LPT/WPT AudioDSP driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
