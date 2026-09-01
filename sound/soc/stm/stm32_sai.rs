// SPDX-License-Identifier: GPL-2.0-only
/*
 * STM32 ALSA SoC Digital Audio Interface (SAI) driver.
 *
 * Copyright (C) 2016, STMicroelectronics - All Rights Reserved
 * Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type u32 = u32;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_sai_conf {
    pub version: u32,
    pub fifo_size: u32,
    pub has_spdif_pdm: bool,
    pub get_sai_ck_parent: Option<unsafe extern "C" fn(*mut stm32_sai_data) -> c_int>,
    pub no_dma_burst: bool,
}

#[repr(C)]
pub struct stm32_sai_data {
    pub pdev: *mut platform_device,
    pub base: *mut u8,
    pub conf: stm32_sai_conf,
    pub pclk: *mut clk,
    pub clk_x8k: *mut clk,
    pub clk_x11k: *mut clk,
    pub irq: c_int,
    pub gcr: u32,
    pub set_sync: Option<
        unsafe extern "C" fn(
            *mut stm32_sai_data,
            *mut device_node,
            c_int,
            c_int,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    static SAI_GCR_SYNCIN_MASK: u32;
    static SAI_GCR_SYNCOUT_MASK: u32;
    static SAI_IDR_ID_MASK: u32;
    static SAI_HWCFGR_FIFO_SIZE: u32;
    static SAI_HWCFGR_SPDIF_PDM: u32;
    static SAI_VERR_MAJ_MASK: u32;
    static SAI_VERR_MIN_MASK: u32;
    static STM_SAI_IDR: usize;
    static STM_SAI_HWCFGR: usize;
    static STM_SAI_VERR: usize;
    static SAI_IPIDR_NUMBER: u32;
    static STM_SAI_STM32F4: u32;
    static STM_SAI_STM32H7: u32;
    static STM_SAI_SYNC_OUT_NONE: u32;
    static STM_SAI_SYNC_OUT_A: c_int;
    static GFP_KERNEL: u32;

    fn FIELD_PREP(mask: u32, val: c_int) -> u32;
    fn FIELD_GET(mask: u32, val: u32) -> u32;
    fn STM_SAI_IS_F4(sai: *mut stm32_sai_data) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_int) -> *mut u8;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn writel_relaxed(val: u32, addr: *mut u8);
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn put_device(dev: *mut device);
    fn platform_get_irq(pdev: *mut platform_device, num: c_int) -> c_int;
    fn devm_reset_control_get_optional_exclusive(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn reset_control_assert(rst: *mut reset_control);
    fn reset_control_deassert(rst: *mut reset_control);
    fn udelay(usecs: c_ulong);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_of_platform_populate(dev: *mut device) -> c_int;
    fn pinctrl_pm_select_sleep_state(dev: *mut device) -> c_int;
    fn pinctrl_pm_select_default_state(dev: *mut device) -> c_int;
    fn module_platform_driver(driver: *mut platform_driver);
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

unsafe extern "C" fn stm32_sai_get_parent_clk(sai: *mut stm32_sai_data) -> c_int {
    let dev = unsafe { &mut (*(*sai).pdev).dev as *mut device };

    unsafe {
        (*sai).clk_x8k = devm_clk_get(dev, c"x8k".as_ptr());
        if IS_ERR((*sai).clk_x8k as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*sai).clk_x8k as *const c_void),
                c"missing x8k parent clock\n".as_ptr(),
            );
        }

        (*sai).clk_x11k = devm_clk_get(dev, c"x11k".as_ptr());
        if IS_ERR((*sai).clk_x11k as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*sai).clk_x11k as *const c_void),
                c"missing x11k parent clock\n".as_ptr(),
            );
        }
    }

    0
}

static stm32_sai_conf_f4: stm32_sai_conf = stm32_sai_conf {
    version: unsafe { STM_SAI_STM32F4 },
    fifo_size: 8,
    has_spdif_pdm: false,
    get_sai_ck_parent: Some(stm32_sai_get_parent_clk),
    no_dma_burst: false,
};

/*
 * Default settings for STM32H7x socs and STM32MP1x.
 * These default settings will be overridden if the soc provides
 * support of hardware configuration registers.
 * - STM32H7: rely on default settings
 * - STM32MP1: retrieve settings from registers
 */
static stm32_sai_conf_h7: stm32_sai_conf = stm32_sai_conf {
    version: unsafe { STM_SAI_STM32H7 },
    fifo_size: 8,
    has_spdif_pdm: true,
    get_sai_ck_parent: Some(stm32_sai_get_parent_clk),
    no_dma_burst: false,
};

/*
 * STM32MP2x:
 * - do not use SAI parent clock source selection
 * - do not use DMA burst mode
 */
static stm32_sai_conf_mp25: stm32_sai_conf = stm32_sai_conf {
    version: 0,
    fifo_size: 0,
    has_spdif_pdm: false,
    get_sai_ck_parent: None,
    no_dma_burst: true,
};

static stm32_sai_ids: [of_device_id; 4] = [
    of_device_id {
        compatible: c"st,stm32f4-sai".as_ptr(),
        data: &stm32_sai_conf_f4 as *const stm32_sai_conf as *const c_void,
    },
    of_device_id {
        compatible: c"st,stm32h7-sai".as_ptr(),
        data: &stm32_sai_conf_h7 as *const stm32_sai_conf as *const c_void,
    },
    of_device_id {
        compatible: c"st,stm32mp25-sai".as_ptr(),
        data: &stm32_sai_conf_mp25 as *const stm32_sai_conf as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

unsafe extern "C" fn stm32_sai_pclk_disable(dev: *mut device) -> c_int {
    let sai = unsafe { dev_get_drvdata(dev) as *mut stm32_sai_data };

    unsafe {
        clk_disable_unprepare((*sai).pclk);
    }

    0
}

unsafe extern "C" fn stm32_sai_pclk_enable(dev: *mut device) -> c_int {
    let sai = unsafe { dev_get_drvdata(dev) as *mut stm32_sai_data };
    let ret: c_int;

    unsafe {
        ret = clk_prepare_enable((*sai).pclk);
        if ret != 0 {
            dev_err(
                &mut (*(*sai).pdev).dev,
                c"failed to enable clock: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }
    }

    0
}

unsafe extern "C" fn stm32_sai_sync_conf_client(
    sai: *mut stm32_sai_data,
    synci: c_int,
) -> c_int {
    let ret: c_int;

    /* Enable peripheral clock to allow GCR register access */
    unsafe {
        ret = stm32_sai_pclk_enable(&mut (*(*sai).pdev).dev);
        if ret != 0 {
            return ret;
        }

        writel_relaxed(FIELD_PREP(SAI_GCR_SYNCIN_MASK, synci - 1), (*sai).base);

        stm32_sai_pclk_disable(&mut (*(*sai).pdev).dev);
    }

    0
}

unsafe extern "C" fn stm32_sai_sync_conf_provider(
    sai: *mut stm32_sai_data,
    synco: c_int,
) -> c_int {
    let prev_synco: u32;
    let ret: c_int;

    /* Enable peripheral clock to allow GCR register access */
    unsafe {
        ret = stm32_sai_pclk_enable(&mut (*(*sai).pdev).dev);
        if ret != 0 {
            return ret;
        }

        dev_dbg(
            &mut (*(*sai).pdev).dev,
            c"Set %pOFn%s as synchro provider\n".as_ptr(),
            (*(*sai).pdev).dev.of_node,
            if synco == STM_SAI_SYNC_OUT_A {
                c"A".as_ptr()
            } else {
                c"B".as_ptr()
            },
        );

        prev_synco = FIELD_GET(SAI_GCR_SYNCOUT_MASK, readl_relaxed((*sai).base));
        if prev_synco != STM_SAI_SYNC_OUT_NONE && synco as u32 != prev_synco {
            dev_err(
                &mut (*(*sai).pdev).dev,
                c"%pOFn%s already set as sync provider\n".as_ptr(),
                (*(*sai).pdev).dev.of_node,
                if prev_synco == STM_SAI_SYNC_OUT_A as u32 {
                    c"A".as_ptr()
                } else {
                    c"B".as_ptr()
                },
            );
            stm32_sai_pclk_disable(&mut (*(*sai).pdev).dev);
            return -EINVAL;
        }

        writel_relaxed(FIELD_PREP(SAI_GCR_SYNCOUT_MASK, synco), (*sai).base);

        stm32_sai_pclk_disable(&mut (*(*sai).pdev).dev);
    }

    0
}

unsafe extern "C" fn stm32_sai_set_sync(
    sai_client: *mut stm32_sai_data,
    np_provider: *mut device_node,
    synco: c_int,
    synci: c_int,
) -> c_int {
    let pdev = unsafe { of_find_device_by_node(np_provider) };
    let sai_provider: *mut stm32_sai_data;
    let ret: c_int;

    unsafe {
        if pdev.is_null() {
            dev_err(
                &mut (*(*sai_client).pdev).dev,
                c"Device not found for node %pOFn\n".as_ptr(),
                np_provider,
            );
            return -ENODEV;
        }

        sai_provider = platform_get_drvdata(pdev) as *mut stm32_sai_data;
        put_device(&mut (*pdev).dev);
        if sai_provider.is_null() {
            dev_err(
                &mut (*(*sai_client).pdev).dev,
                c"SAI sync provider data not found\n".as_ptr(),
            );
            return -EINVAL;
        }

        /* Configure sync client */
        ret = stm32_sai_sync_conf_client(sai_client, synci);
        if ret < 0 {
            return ret;
        }

        /* Configure sync provider */
        stm32_sai_sync_conf_provider(sai_provider, synco)
    }
}

unsafe extern "C" fn stm32_sai_probe(pdev: *mut platform_device) -> c_int {
    let sai: *mut stm32_sai_data;
    let conf: *const stm32_sai_conf;
    let rst: *mut reset_control;
    let mut val: u32;
    let ret: c_int;

    unsafe {
        sai = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<stm32_sai_data>(),
            GFP_KERNEL,
        ) as *mut stm32_sai_data;
        if sai.is_null() {
            return -ENOMEM;
        }

        (*sai).pdev = pdev;

        (*sai).base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*sai).base as *const c_void) {
            return PTR_ERR((*sai).base as *const c_void);
        }

        conf = device_get_match_data(&mut (*pdev).dev) as *const stm32_sai_conf;
        if !conf.is_null() {
            ptr::copy_nonoverlapping(conf, &mut (*sai).conf, 1);
        } else {
            return -EINVAL;
        }

        if !STM_SAI_IS_F4(sai) {
            (*sai).pclk = devm_clk_get(&mut (*pdev).dev, c"pclk".as_ptr());
            if IS_ERR((*sai).pclk as *const c_void) {
                return dev_err_probe(
                    &mut (*pdev).dev,
                    PTR_ERR((*sai).pclk as *const c_void),
                    c"missing bus clock pclk\n".as_ptr(),
                );
            }
        }

        if let Some(get_sai_ck_parent) = (*sai).conf.get_sai_ck_parent {
            let parent_ret = get_sai_ck_parent(sai);
            if parent_ret != 0 {
                return parent_ret;
            }
        }

        /* init irqs */
        (*sai).irq = platform_get_irq(pdev, 0);
        if (*sai).irq < 0 {
            return (*sai).irq;
        }

        /* reset */
        rst = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, ptr::null());
        if IS_ERR(rst as *const c_void) {
            return dev_err_probe(
                &mut (*pdev).dev,
                PTR_ERR(rst as *const c_void),
                c"Reset controller error\n".as_ptr(),
            );
        }

        reset_control_assert(rst);
        udelay(2);
        reset_control_deassert(rst);

        /* Enable peripheral clock to allow register access */
        ret = clk_prepare_enable((*sai).pclk);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                c"failed to enable clock: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }

        val = FIELD_GET(
            SAI_IDR_ID_MASK,
            readl_relaxed((*sai).base.add(STM_SAI_IDR)),
        );
        if val == SAI_IPIDR_NUMBER {
            val = readl_relaxed((*sai).base.add(STM_SAI_HWCFGR));
            (*sai).conf.fifo_size = FIELD_GET(SAI_HWCFGR_FIFO_SIZE, val);
            (*sai).conf.has_spdif_pdm = FIELD_GET(SAI_HWCFGR_SPDIF_PDM, val) != 0;

            val = readl_relaxed((*sai).base.add(STM_SAI_VERR));
            (*sai).conf.version = val;

            dev_dbg(
                &mut (*pdev).dev,
                c"SAI version: %lu.%lu registered\n".as_ptr(),
                FIELD_GET(SAI_VERR_MAJ_MASK, val) as c_ulong,
                FIELD_GET(SAI_VERR_MIN_MASK, val) as c_ulong,
            );
        }
        clk_disable_unprepare((*sai).pclk);

        (*sai).set_sync = Some(stm32_sai_set_sync);
        platform_set_drvdata(pdev, sai as *mut c_void);

        devm_of_platform_populate(&mut (*pdev).dev)
    }
}

/*
 * When pins are shared by two sai sub instances, pins have to be defined
 * in sai parent node. In this case, pins state is not managed by alsa fw.
 * These pins are managed in suspend/resume callbacks.
 */
unsafe extern "C" fn stm32_sai_suspend(dev: *mut device) -> c_int {
    let sai = unsafe { dev_get_drvdata(dev) as *mut stm32_sai_data };
    let ret: c_int;

    unsafe {
        ret = stm32_sai_pclk_enable(dev);
        if ret != 0 {
            return ret;
        }

        (*sai).gcr = readl_relaxed((*sai).base);
        stm32_sai_pclk_disable(dev);

        pinctrl_pm_select_sleep_state(dev)
    }
}

unsafe extern "C" fn stm32_sai_resume(dev: *mut device) -> c_int {
    let sai = unsafe { dev_get_drvdata(dev) as *mut stm32_sai_data };
    let ret: c_int;

    unsafe {
        ret = stm32_sai_pclk_enable(dev);
        if ret != 0 {
            return ret;
        }

        writel_relaxed((*sai).gcr, (*sai).base);
        stm32_sai_pclk_disable(dev);

        pinctrl_pm_select_default_state(dev)
    }
}

static stm32_sai_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(stm32_sai_suspend),
    resume: Some(stm32_sai_resume),
};

/* MODULE_DEVICE_TABLE(of, stm32_sai_ids); */

static mut stm32_sai_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"st,stm32-sai".as_ptr(),
        of_match_table: stm32_sai_ids.as_ptr(),
        pm: unsafe { pm_ptr(&stm32_sai_pm_ops) },
    },
    probe: Some(stm32_sai_probe),
};

unsafe extern "C" fn __register_stm32_sai_driver() {
    unsafe {
        module_platform_driver(&mut stm32_sai_driver);
    }
}

/* module_platform_driver(stm32_sai_driver); */

/* MODULE_DESCRIPTION("STM32 Soc SAI Interface"); */
/* MODULE_AUTHOR("Olivier Moysan <olivier.moysan@st.com>"); */
/* MODULE_ALIAS("platform:st,stm32-sai"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
