// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright 2019-2025 NXP
//
// Author: Daniel Baluta <daniel.baluta@nxp.com>
//
// Hardware interface for audio DSP on i.MX8

// C dependencies:
// dt-bindings/firmware/imx/rsrc.h
// linux/arm-smccc.h
// linux/firmware/imx/svc/misc.h
// linux/mfd/syscon.h
// linux/reset.h
// imx-common.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

/* imx8/imx8x macros */
const RESET_VECTOR_VADDR: u32 = 0x596f8000;

/* imx8m macros */
const IMX8M_DAP_DEBUG: usize = 0x28800000;
const IMX8M_DAP_DEBUG_SIZE: usize = 64 * 1024;
const IMX8M_DAP_PWRCTL: usize = 0x4000 + 0x3020;
const IMX8M_PWRCTL_CORERESET: u32 = BIT(16);

/* imx8ulp macros */
const FSL_SIP_HIFI_XRDC: u64 = 0xc200000e;
const SYSCTRL0: u32 = 0x8;
const EXECUTE_BIT: u32 = BIT(13);
const RESET_BIT: u32 = BIT(16);
const HIFI4_CLK_BIT: u32 = BIT(17);
const PB_CLK_BIT: u32 = BIT(18);
const PLAT_CLK_BIT: u32 = BIT(19);
const DEBUG_LOGIC_BIT: u32 = BIT(25);

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const IMX_SC_R_DSP: c_uint = 0;
const IMX_SC_C_OFS_SEL: c_uint = 0;
const IMX_SC_C_OFS_AUDIO: c_uint = 0;
const IMX_SC_C_OFS_PERIPH: c_uint = 0;
const IMX_SC_C_OFS_IRQ: c_uint = 0;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut sof_dev_desc,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct imx_common_data {
    pub chip_pdata: *mut c_void,
}

#[repr(C)]
pub struct imx_sc_ipc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arm_smccc_res {
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
}

#[cfg(target_pointer_width = "64")]
type c_ulong = u64;
#[cfg(target_pointer_width = "32")]
type c_ulong = u32;

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub dsp_arch_ops: *const c_void,
    pub debugfs_add_region_item: *const c_void,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,
}

#[repr(C)]
pub struct imx_chip_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub core_kick: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub core_shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub core_reset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct imx_memory_info {
    pub name: *const c_char,
    pub reserved: bool,
}

#[repr(C)]
pub struct imx_ipc_info {
    pub has_panic_code: bool,
    pub boot_mbox_offset: u32,
    pub window_offset: u32,
}

#[repr(C)]
pub struct imx_chip_info {
    pub ipc_info: imx_ipc_info,
    pub has_dma_reserved: bool,
    pub memory: *mut imx_memory_info,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,
    pub ops: *const imx_chip_ops,
}

#[repr(C)]
pub struct snd_sof_of_mach {
    pub compatible: *const c_char,
    pub sof_tplg_filename: *const c_char,
    pub drv_name: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

struct imx8m_chip_data {
    dap: *mut c_void,
    regmap: *mut regmap,
    run_stall: *mut reset_control,
}

extern "C" {
    static mut sof_imx_ops: snd_sof_dsp_ops;
    static sof_xtensa_arch_ops: c_void;
    static snd_sof_debugfs_add_region_item_iomem: c_void;
    static sof_of_pm: dev_pm_ops;

    fn get_chip_pdata(sdev: *mut snd_sof_dev) -> *mut c_void;
    fn get_chip_info(sdev: *mut snd_sof_dev) -> *const imx_chip_info;
    fn imx8_dump(sdev: *mut snd_sof_dev, flags: u32);
    fn imx_sc_pm_cpu_start(ipc: *mut c_void, resource: c_uint, enable: bool, address: u32) -> c_int;
    fn imx_sc_misc_set_control(
        ipc: *mut c_void,
        resource: c_uint,
        ctrl: c_uint,
        value: c_uint,
    ) -> c_int;
    fn imx_scu_get_handle(handle: *mut *mut imx_sc_ipc) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: usize, size: usize) -> *mut c_void;
    fn devm_reset_control_get_exclusive(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn arm_smccc_smc(
        a0: c_ulong,
        a1: c_ulong,
        a2: c_ulong,
        a3: c_ulong,
        a4: c_ulong,
        a5: c_ulong,
        a6: c_ulong,
        a7: c_ulong,
        res: *mut arm_smccc_res,
    );
    fn syscon_regmap_lookup_by_phandle(
        np: *mut device_node,
        property: *const c_char,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    fn sof_of_probe(pdev: *mut platform_device) -> c_int;
    fn sof_of_remove(pdev: *mut platform_device);
    fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe fn ptr_add(ptr: *mut c_void, offset: usize) -> *mut c_void {
    (ptr as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn imx8_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    /*
     * Force the DSP to stall. After the firmware image is loaded,
     * the stall will be removed during run() by a matching
     * imx_sc_pm_cpu_start() call.
     */
    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, false, RESET_VECTOR_VADDR);

    0
}

/*
 * DSP control.
 */
unsafe extern "C" fn imx8x_run(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP, IMX_SC_C_OFS_SEL, 1);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"Error system address offset source select\n".as_ptr(),
        );
        return ret;
    }

    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP, IMX_SC_C_OFS_AUDIO, 0x80);
    if ret < 0 {
        dev_err((*sdev).dev, c"Error system address offset of AUDIO\n".as_ptr());
        return ret;
    }

    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP, IMX_SC_C_OFS_PERIPH, 0x5A);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"Error system address offset of PERIPH %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP, IMX_SC_C_OFS_IRQ, 0x51);
    if ret < 0 {
        dev_err((*sdev).dev, c"Error system address offset of IRQ\n".as_ptr());
        return ret;
    }

    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, true, RESET_VECTOR_VADDR);

    0
}

unsafe extern "C" fn imx8_run(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP, IMX_SC_C_OFS_SEL, 0);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"Error system address offset source select\n".as_ptr(),
        );
        return ret;
    }

    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, true, RESET_VECTOR_VADDR);

    0
}

unsafe extern "C" fn imx8_probe(sdev: *mut snd_sof_dev) -> c_int {
    let mut sc_ipc_handle: *mut imx_sc_ipc = ptr::null_mut();
    let common: *mut imx_common_data;
    let ret: c_int;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    ret = imx_scu_get_handle(&mut sc_ipc_handle);
    if ret < 0 {
        return dev_err_probe(
            (*sdev).dev,
            ret,
            c"failed to fetch SC IPC handle\n".as_ptr(),
        );
    }

    (*common).chip_pdata = sc_ipc_handle as *mut c_void;

    0
}

unsafe extern "C" fn imx8m_reset(sdev: *mut snd_sof_dev) -> c_int {
    let chip: *mut imx8m_chip_data;
    let mut pwrctl: u32;

    chip = get_chip_pdata(sdev) as *mut imx8m_chip_data;

    /* put DSP into reset and stall */
    pwrctl = readl(ptr_add((*chip).dap, IMX8M_DAP_PWRCTL));
    pwrctl |= IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, ptr_add((*chip).dap, IMX8M_DAP_PWRCTL));

    /* keep reset asserted for 10 cycles */
    usleep_range(1, 2);

    reset_control_assert((*chip).run_stall);

    /* take the DSP out of reset and keep stalled for FW loading */
    pwrctl = readl(ptr_add((*chip).dap, IMX8M_DAP_PWRCTL));
    pwrctl &= !IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, ptr_add((*chip).dap, IMX8M_DAP_PWRCTL));

    0
}

unsafe extern "C" fn imx8m_run(sdev: *mut snd_sof_dev) -> c_int {
    let chip: *mut imx8m_chip_data = get_chip_pdata(sdev) as *mut imx8m_chip_data;

    reset_control_deassert((*chip).run_stall)
}

unsafe extern "C" fn imx8m_probe(sdev: *mut snd_sof_dev) -> c_int {
    let common: *mut imx_common_data;
    let chip: *mut imx8m_chip_data;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    chip = devm_kzalloc((*sdev).dev, size_of::<imx8m_chip_data>(), GFP_KERNEL) as *mut imx8m_chip_data;
    if chip.is_null() {
        return -ENOMEM;
    }

    (*chip).dap = devm_ioremap((*sdev).dev, IMX8M_DAP_DEBUG, IMX8M_DAP_DEBUG_SIZE);
    if (*chip).dap.is_null() {
        return dev_err_probe((*sdev).dev, -ENODEV, c"failed to ioremap DAP\n".as_ptr());
    }

    (*chip).run_stall = devm_reset_control_get_exclusive((*sdev).dev, c"runstall".as_ptr());
    if IS_ERR((*chip).run_stall as *const c_void) {
        return dev_err_probe(
            (*sdev).dev,
            PTR_ERR((*chip).run_stall as *const c_void),
            c"failed to get dsp runstall reset control\n".as_ptr(),
        );
    }

    (*common).chip_pdata = chip as *mut c_void;

    0
}

unsafe extern "C" fn imx8ulp_run(sdev: *mut snd_sof_dev) -> c_int {
    let regmap: *mut regmap = get_chip_pdata(sdev) as *mut regmap;

    /* Controls the HiFi4 DSP Reset: 1 in reset, 0 out of reset */
    regmap_update_bits(regmap, SYSCTRL0, RESET_BIT, 0);

    /* Reset HiFi4 DSP Debug logic: 1 debug reset, 0  out of reset*/
    regmap_update_bits(regmap, SYSCTRL0, DEBUG_LOGIC_BIT, 0);

    /* Stall HIFI4 DSP Execution: 1 stall, 0 run */
    regmap_update_bits(regmap, SYSCTRL0, EXECUTE_BIT, 0);

    0
}

unsafe extern "C" fn imx8ulp_reset(sdev: *mut snd_sof_dev) -> c_int {
    let mut smc_res: arm_smccc_res = core::mem::zeroed();
    let regmap: *mut regmap;

    regmap = get_chip_pdata(sdev) as *mut regmap;

    /* HiFi4 Platform Clock Enable: 1 enabled, 0 disabled */
    regmap_update_bits(regmap, SYSCTRL0, PLAT_CLK_BIT, PLAT_CLK_BIT);

    /* HiFi4 PBCLK clock enable: 1 enabled, 0 disabled */
    regmap_update_bits(regmap, SYSCTRL0, PB_CLK_BIT, PB_CLK_BIT);

    /* HiFi4 Clock Enable: 1 enabled, 0 disabled */
    regmap_update_bits(regmap, SYSCTRL0, HIFI4_CLK_BIT, HIFI4_CLK_BIT);

    regmap_update_bits(regmap, SYSCTRL0, RESET_BIT, RESET_BIT);

    usleep_range(1, 2);

    /* Stall HIFI4 DSP Execution: 1 stall, 0 not stall */
    regmap_update_bits(regmap, SYSCTRL0, EXECUTE_BIT, EXECUTE_BIT);
    usleep_range(1, 2);

    arm_smccc_smc(FSL_SIP_HIFI_XRDC, 0, 0, 0, 0, 0, 0, 0, &mut smc_res);

    smc_res.a0 as c_int
}

unsafe extern "C" fn imx8ulp_probe(sdev: *mut snd_sof_dev) -> c_int {
    let common: *mut imx_common_data;
    let regmap: *mut regmap;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    regmap = syscon_regmap_lookup_by_phandle((*(*sdev).dev).of_node, c"fsl,dsp-ctrl".as_ptr());
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(
            (*sdev).dev,
            PTR_ERR(regmap as *const c_void),
            c"failed to fetch dsp ctrl regmap\n".as_ptr(),
        );
    }

    (*common).chip_pdata = regmap as *mut c_void;

    0
}

macro_rules! IMX_SOF_DAI_DRV_ENTRY_BIDIR {
    ($name:expr, $id:expr, $channels:expr) => {
        snd_soc_dai_driver { _private: [] }
    };
}

macro_rules! IMX_SOF_DAI_DRV_ENTRY {
    ($name:expr, $playback:expr, $capture:expr, $id:expr, $channels:expr) => {
        snd_soc_dai_driver { _private: [] }
    };
}

static mut imx8_dai: [snd_soc_dai_driver; 2] = [
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"esai0".as_ptr(), 1, 8),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai1".as_ptr(), 1, 32),
];

static mut imx8m_dai: [snd_soc_dai_driver; 7] = [
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai1".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai2".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai3".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai5".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai6".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai7".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY!(c"micfil".as_ptr(), 0, 0, 1, 8),
];

static mut imx8ulp_dai: [snd_soc_dai_driver; 2] = [
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai5".as_ptr(), 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR!(c"sai6".as_ptr(), 1, 32),
];

static mut sof_imx8_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    dbg_dump: None,
    dsp_arch_ops: ptr::null(),
    debugfs_add_region_item: ptr::null(),
    drv: ptr::null_mut(),
    num_drv: 0,
};

unsafe extern "C" fn imx8_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    /* first copy from template */
    memcpy(
        &mut sof_imx8_ops as *mut _ as *mut c_void,
        &sof_imx_ops as *const _ as *const c_void,
        size_of::<snd_sof_dsp_ops>(),
    );

    /* then set common imx8 ops */
    sof_imx8_ops.dbg_dump = Some(imx8_dump);
    sof_imx8_ops.dsp_arch_ops = &sof_xtensa_arch_ops as *const _ as *const c_void;
    sof_imx8_ops.debugfs_add_region_item =
        &snd_sof_debugfs_add_region_item_iomem as *const _ as *const c_void;

    /* ... and finally set DAI driver */
    sof_imx8_ops.drv = (*get_chip_info(sdev)).drv;
    sof_imx8_ops.num_drv = (*get_chip_info(sdev)).num_drv;

    0
}

static imx8_chip_ops: imx_chip_ops = imx_chip_ops {
    probe: Some(imx8_probe),
    core_kick: Some(imx8_run),
    core_shutdown: Some(imx8_shutdown),
    core_reset: None,
};

static imx8x_chip_ops: imx_chip_ops = imx_chip_ops {
    probe: Some(imx8_probe),
    core_kick: Some(imx8x_run),
    core_shutdown: Some(imx8_shutdown),
    core_reset: None,
};

static imx8m_chip_ops: imx_chip_ops = imx_chip_ops {
    probe: Some(imx8m_probe),
    core_kick: Some(imx8m_run),
    core_shutdown: None,
    core_reset: Some(imx8m_reset),
};

static imx8ulp_chip_ops: imx_chip_ops = imx_chip_ops {
    probe: Some(imx8ulp_probe),
    core_kick: Some(imx8ulp_run),
    core_shutdown: None,
    core_reset: Some(imx8ulp_reset),
};

static mut imx8_memory_regions: [imx_memory_info; 3] = [
    imx_memory_info {
        name: c"iram".as_ptr(),
        reserved: false,
    },
    imx_memory_info {
        name: c"sram".as_ptr(),
        reserved: true,
    },
    imx_memory_info {
        name: ptr::null(),
        reserved: false,
    },
];

static mut imx8m_memory_regions: [imx_memory_info; 3] = [
    imx_memory_info {
        name: c"iram".as_ptr(),
        reserved: false,
    },
    imx_memory_info {
        name: c"sram".as_ptr(),
        reserved: true,
    },
    imx_memory_info {
        name: ptr::null(),
        reserved: false,
    },
];

static mut imx8ulp_memory_regions: [imx_memory_info; 3] = [
    imx_memory_info {
        name: c"iram".as_ptr(),
        reserved: false,
    },
    imx_memory_info {
        name: c"sram".as_ptr(),
        reserved: true,
    },
    imx_memory_info {
        name: ptr::null(),
        reserved: false,
    },
];

static imx8_chip_info: imx_chip_info = imx_chip_info {
    ipc_info: imx_ipc_info {
        has_panic_code: true,
        boot_mbox_offset: 0x800000,
        window_offset: 0x800000,
    },
    has_dma_reserved: false,
    memory: unsafe { imx8_memory_regions.as_mut_ptr() },
    drv: unsafe { imx8_dai.as_mut_ptr() },
    num_drv: 2,
    ops: &imx8_chip_ops,
};

static imx8x_chip_info: imx_chip_info = imx_chip_info {
    ipc_info: imx_ipc_info {
        has_panic_code: true,
        boot_mbox_offset: 0x800000,
        window_offset: 0x800000,
    },
    has_dma_reserved: false,
    memory: unsafe { imx8_memory_regions.as_mut_ptr() },
    drv: unsafe { imx8_dai.as_mut_ptr() },
    num_drv: 2,
    ops: &imx8x_chip_ops,
};

static imx8m_chip_info: imx_chip_info = imx_chip_info {
    ipc_info: imx_ipc_info {
        has_panic_code: true,
        boot_mbox_offset: 0x800000,
        window_offset: 0x800000,
    },
    has_dma_reserved: false,
    memory: unsafe { imx8m_memory_regions.as_mut_ptr() },
    drv: unsafe { imx8m_dai.as_mut_ptr() },
    num_drv: 7,
    ops: &imx8m_chip_ops,
};

static imx8ulp_chip_info: imx_chip_info = imx_chip_info {
    ipc_info: imx_ipc_info {
        has_panic_code: true,
        boot_mbox_offset: 0x800000,
        window_offset: 0x800000,
    },
    has_dma_reserved: true,
    memory: unsafe { imx8ulp_memory_regions.as_mut_ptr() },
    drv: unsafe { imx8ulp_dai.as_mut_ptr() },
    num_drv: 2,
    ops: &imx8ulp_chip_ops,
};

static mut sof_imx8_machs: [snd_sof_of_mach; 10] = [
    snd_sof_of_mach {
        compatible: c"fsl,imx8qxp-mek".as_ptr(),
        sof_tplg_filename: c"sof-imx8-wm8960.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8qxp-mek-wcpu".as_ptr(),
        sof_tplg_filename: c"sof-imx8-wm8962.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8qm-mek".as_ptr(),
        sof_tplg_filename: c"sof-imx8-wm8960.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8qm-mek-revd".as_ptr(),
        sof_tplg_filename: c"sof-imx8-wm8962.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8qxp-mek-bb".as_ptr(),
        sof_tplg_filename: c"sof-imx8-cs42888.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8qm-mek-bb".as_ptr(),
        sof_tplg_filename: c"sof-imx8-cs42888.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8mp-evk".as_ptr(),
        sof_tplg_filename: c"sof-imx8mp-wm8960.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8mp-evk-revb4".as_ptr(),
        sof_tplg_filename: c"sof-imx8mp-wm8962.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: c"fsl,imx8ulp-evk".as_ptr(),
        sof_tplg_filename: c"sof-imx8ulp-btsco.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: ptr::null(),
        sof_tplg_filename: ptr::null(),
        drv_name: ptr::null(),
    },
];

macro_rules! IMX_SOF_DEV_DESC {
    ($name:ident, $machs:ident, $chip_info:expr, $ops:expr, $ops_init:ident) => {
        static $name: sof_dev_desc = sof_dev_desc {
            hw_pdata: $chip_info as *const _ as *mut c_void,
        };
    };
}

macro_rules! IMX_SOF_DEV_DESC_NAME {
    ($name:ident) => {
        $name
    };
}

IMX_SOF_DEV_DESC!(imx8, sof_imx8_machs, &imx8_chip_info, &sof_imx8_ops, imx8_ops_init);
IMX_SOF_DEV_DESC!(imx8x, sof_imx8_machs, &imx8x_chip_info, &sof_imx8_ops, imx8_ops_init);
IMX_SOF_DEV_DESC!(imx8m, sof_imx8_machs, &imx8m_chip_info, &sof_imx8_ops, imx8_ops_init);
IMX_SOF_DEV_DESC!(imx8ulp, sof_imx8_machs, &imx8ulp_chip_info, &sof_imx8_ops, imx8_ops_init);

static sof_of_imx8_ids: [of_device_id; 5] = [
    of_device_id {
        compatible: c"fsl,imx8qxp-dsp".as_ptr(),
        data: &IMX_SOF_DEV_DESC_NAME!(imx8x) as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8qm-dsp".as_ptr(),
        data: &IMX_SOF_DEV_DESC_NAME!(imx8) as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8mp-dsp".as_ptr(),
        data: &IMX_SOF_DEV_DESC_NAME!(imx8m) as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8ulp-dsp".as_ptr(),
        data: &IMX_SOF_DEV_DESC_NAME!(imx8ulp) as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sof_of_imx8_ids);

/* DT driver definition */
static mut snd_sof_of_imx8_driver: platform_driver = platform_driver {
    probe: Some(sof_of_probe),
    remove: Some(sof_of_remove),
    driver: device_driver {
        name: c"sof-audio-of-imx8".as_ptr(),
        pm: unsafe { pm_ptr(&sof_of_pm) },
        of_match_table: sof_of_imx8_ids.as_ptr(),
    },
};
// module_platform_driver(snd_sof_of_imx8_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for IMX8 platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
