// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2021 Mediatek Inc. All rights reserved.
//
// Author: YC Hung <yc.hung@mediatek.com>
//

/*
 * Hardware interface for audio DSP on mt8195
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = c_uint;
type phys_addr_t = usize;
type resource_size_t = usize;

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dsp_box {
    pub offset: c_int,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub bar: [*mut c_void; 8],
    pub mmio_bar: c_int,
    pub mailbox_bar: c_int,
    pub dsp_box: snd_sof_dsp_box,
}

#[repr(C)]
pub struct mtk_adsp_chip_info {
    pub pa_dram: phys_addr_t,
    pub dramsize: u32,
    pub va_cfgreg: *mut c_void,
    pub pa_cfgreg: phys_addr_t,
    pub cfgregsize: u32,
    pub pa_sram: phys_addr_t,
    pub sramsize: u32,
    pub dram_offset: c_int,
    pub va_sram: *mut c_void,
    pub va_dram: *mut c_void,
}

#[repr(C)]
pub struct adsp_priv {
    pub dev: *mut device,
    pub sdev: *mut snd_sof_dev,
    pub adsp: *mut mtk_adsp_chip_info,
    pub ipc_dev: *mut platform_device,
    pub dsp_ipc: *mut mtk_adsp_ipc,
}

#[repr(C)]
pub struct mtk_adsp_ipc {
    pub ops: *const mtk_adsp_ipc_ops,
}

#[repr(C)]
pub struct mtk_adsp_ipc_ops {
    pub handle_reply: Option<unsafe extern "C" fn(*mut c_void)>,
    pub handle_request: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub block_read: *const c_void,
    pub block_write: *const c_void,
    pub mailbox_read: *const c_void,
    pub mailbox_write: *const c_void,
    pub write: *const c_void,
    pub read: *const c_void,
    pub write64: *const c_void,
    pub read64: *const c_void,
    pub send_msg: *const c_void,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub ipc_msg_data: *const c_void,
    pub set_stream_data_offset: *const c_void,
    pub get_bar_index: *const c_void,
    pub pcm_open: *const c_void,
    pub pcm_hw_params: *const c_void,
    pub pcm_pointer: *const c_void,
    pub pcm_close: *const c_void,
    pub load_firmware: *const c_void,
    pub dsp_arch_ops: *const c_void,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub debugfs_add_region_item: *const c_void,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: usize,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub hw_info: u32,
}

#[repr(C)]
pub struct snd_sof_of_mach {
    pub compatible: *const c_char,
    pub sof_tplg_filename: *const c_char,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub of_machines: *mut snd_sof_of_mach,
    pub ipc_supported_mask: u32,
    pub ipc_default: c_int,
    pub default_fw_path: [*const c_char; 4],
    pub default_tplg_path: [*const c_char; 4],
    pub default_fw_filename: [*const c_char; 4],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const snd_sof_dsp_ops,
    pub ipc_timeout: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: *const c_void,
    pub remove: *const c_void,
    pub shutdown: *const c_void,
    pub driver: platform_driver_inner,
}

const MBOX_OFFSET: c_int = 0;
const IORESOURCE_MEM: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const PLATFORM_DEVID_NONE: c_int = -1;
const SOF_FW_BLK_TYPE_IRAM: usize = 0;
const SOF_FW_BLK_TYPE_SRAM: usize = 1;
const DSP_REG_BAR: usize = 2;
const SOF_IPC_TYPE_3: usize = 3;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EPROBE_DEFER: c_int = 517;
const DRAM_REMAP_MASK: phys_addr_t = 0xfff;
const TOTAL_SIZE_SHARED_DRAM_FROM_TAIL: u32 = 0;
const ADSP_SRAM_POOL_CON: phys_addr_t = 0;
const DSP_SRAM_POOL_PD_MASK: u32 = 0;
const DSP_EMI_MAP_ADDR: phys_addr_t = 0;
const DRAM_PHYS_BASE_FROM_DSP_VIEW: phys_addr_t = 0;
const DRAM_REMAP_SHIFT: c_int = 12;
const SRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0;
const DSP_RESET_SW: u32 = 0;
const ADSP_PWAIT: u32 = 0;
const SUSPEND_DSP_IDLE_POLL_INTERVAL_US: u32 = 0;
const SUSPEND_DSP_IDLE_TIMEOUT_US: u32 = 0;
const DSP_PDEBUGPC: u32 = 0;
const DSP_PDEBUGDATA: u32 = 0;
const DSP_PDEBUGBUS0: u32 = 0;
const DSP_PDEBUGBUS1: u32 = 0;
const DSP_PDEBUGINST: u32 = 0;
const DSP_PDEBUGLS0STAT: u32 = 0;
const DSP_PDEBUGLS1STAT: u32 = 0;
const DSP_PFAULTBUS: u32 = 0;
const DSP_PFAULTINFO: u32 = 0;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32 = 1 << 4;

const fn BIT(n: usize) -> u32 {
    1u32 << n
}

unsafe extern "C" {
    static sof_xtensa_arch_ops: c_void;
    static sof_of_pm: c_void;
    static sof_block_read: c_void;
    static sof_block_write: c_void;
    static sof_mailbox_read: c_void;
    static sof_mailbox_write: c_void;
    static sof_io_write: c_void;
    static sof_io_read: c_void;
    static sof_io_write64: c_void;
    static sof_io_read64: c_void;
    static mtk_adsp_send_msg: c_void;
    static sof_ipc_msg_data: c_void;
    static sof_set_stream_data_offset: c_void;
    static mtk_adsp_get_bar_index: c_void;
    static sof_stream_pcm_open: c_void;
    static mtk_adsp_stream_pcm_hw_params: c_void;
    static mtk_adsp_stream_pcm_pointer: c_void;
    static sof_stream_pcm_close: c_void;
    static snd_sof_load_firmware_memcpy: c_void;
    static snd_sof_debugfs_add_region_item_iomem: c_void;
    static sof_of_probe: c_void;
    static sof_of_remove: c_void;
    static sof_of_shutdown: c_void;

    fn mtk_adsp_handle_reply(data: *mut c_void);
    fn mtk_adsp_handle_request(data: *mut c_void);
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn of_reserved_mem_region_to_resource(node: *mut c_void, index: c_int, res: *mut resource) -> c_int;
    fn resource_size(res: *const resource) -> u32;
    fn platform_get_resource_byname(
        pdev: *mut platform_device,
        ty: c_uint,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: phys_addr_t, size: usize) -> *mut c_void;
    fn ioremap(offset: phys_addr_t, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn mtk_adsp_ipc_set_data(ipc: *mut mtk_adsp_ipc, data: *mut adsp_priv);
    fn mt8195_adsp_init_clock(sdev: *mut snd_sof_dev) -> c_int;
    fn adsp_clock_on(sdev: *mut snd_sof_dev) -> c_int;
    fn adsp_clock_off(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
    fn sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: usize, offset: u32) -> u32;
    fn mtk_adsp_dump(sdev: *mut snd_sof_dev, flags: u32);
}

macro_rules! dev_err {
    ($($arg:tt)*) => {};
}
macro_rules! dev_dbg {
    ($($arg:tt)*) => {};
}
macro_rules! dev_warn {
    ($($arg:tt)*) => {};
}
macro_rules! dev_info {
    ($($arg:tt)*) => {};
}

unsafe fn IS_ERR(ptr: *mut c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn to_platform_device(dev: *mut device) -> *mut platform_device {
    dev as *mut platform_device
}

unsafe fn snd_sof_dsp_read_poll_timeout(
    sdev: *mut snd_sof_dev,
    bar: usize,
    offset: u32,
    out: *mut u32,
    interval_us: u32,
    timeout_us: u32,
) -> c_int {
    let _ = interval_us;
    let _ = timeout_us;
    *out = snd_sof_dsp_read(sdev, bar, offset);
    if (*out & ADSP_PWAIT) == ADSP_PWAIT {
        0
    } else {
        -EIO
    }
}

unsafe extern "C" fn mt8195_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    MBOX_OFFSET
}

unsafe extern "C" fn mt8195_get_window_offset(_sdev: *mut snd_sof_dev, _id: u32) -> c_int {
    MBOX_OFFSET
}

static dsp_ops: mtk_adsp_ipc_ops = mtk_adsp_ipc_ops {
    handle_reply: Some(mtk_adsp_handle_reply),
    handle_request: Some(mtk_adsp_handle_request),
};

unsafe extern "C" fn platform_parse_resource(pdev: *mut platform_device, data: *mut c_void) -> c_int {
    let mut mmio: *mut resource;
    let mut res = resource { start: 0 };
    let dev = &mut (*pdev).dev as *mut device;
    let adsp = data as *mut mtk_adsp_chip_info;
    let mut ret: c_int;

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_err!(dev, "of_reserved_mem_device_init failed\n");
        return ret;
    }

    ret = of_reserved_mem_region_to_resource((*dev).of_node, 1, &mut res);
    if ret != 0 {
        dev_err!(dev, "of_address_to_resource sysmem failed\n");
        return ret;
    }

    (*adsp).pa_dram = res.start as phys_addr_t;
    (*adsp).dramsize = resource_size(&res);
    if ((*adsp).pa_dram & DRAM_REMAP_MASK) != 0 {
        dev_err!(dev, "adsp memory(%#x) is not 4K-aligned\n", (*adsp).pa_dram as u32);
        return -EINVAL;
    }

    if (*adsp).dramsize < TOTAL_SIZE_SHARED_DRAM_FROM_TAIL {
        dev_err!(dev, "adsp memory(%#x) is not enough for share\n", (*adsp).dramsize);
        return -EINVAL;
    }

    dev_dbg!(dev, "dram pbase=%pa, dramsize=%#x\n", &mut (*adsp).pa_dram, (*adsp).dramsize);

    /* Parse CFG base */
    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"cfg\0".as_ptr() as *const c_char);
    if mmio.is_null() {
        dev_err!(dev, "no ADSP-CFG register resource\n");
        return -ENXIO;
    }
    /* remap for DSP register accessing */
    (*adsp).va_cfgreg = devm_ioremap_resource(dev, mmio);
    if IS_ERR((*adsp).va_cfgreg) {
        return PTR_ERR((*adsp).va_cfgreg);
    }

    (*adsp).pa_cfgreg = (*mmio).start as phys_addr_t;
    (*adsp).cfgregsize = resource_size(mmio);

    dev_dbg!(dev, "cfgreg-vbase=%p, cfgregsize=%#x\n", (*adsp).va_cfgreg, (*adsp).cfgregsize);

    /* Parse SRAM */
    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"sram\0".as_ptr() as *const c_char);
    if mmio.is_null() {
        dev_err!(dev, "no SRAM resource\n");
        return -ENXIO;
    }

    (*adsp).pa_sram = (*mmio).start as phys_addr_t;
    (*adsp).sramsize = resource_size(mmio);

    dev_dbg!(dev, "sram pbase=%pa,%#x\n", &mut (*adsp).pa_sram, (*adsp).sramsize);

    ret
}

unsafe extern "C" fn adsp_sram_power_on(dev: *mut device, on: bool) -> c_int {
    let va_dspsysreg: *mut c_void;
    let srampool_con: u32;

    va_dspsysreg = ioremap(ADSP_SRAM_POOL_CON, 0x4);
    if va_dspsysreg.is_null() {
        dev_err!(dev, "failed to ioremap sram pool base %#x\n", ADSP_SRAM_POOL_CON);
        return -ENOMEM;
    }

    srampool_con = readl(va_dspsysreg);
    if on {
        writel(srampool_con & !DSP_SRAM_POOL_PD_MASK, va_dspsysreg);
    } else {
        writel(srampool_con | DSP_SRAM_POOL_PD_MASK, va_dspsysreg);
    }

    iounmap(va_dspsysreg);
    0
}

/*  Init the basic DSP DRAM address */
unsafe extern "C" fn adsp_memory_remap_init(dev: *mut device, adsp: *mut mtk_adsp_chip_info) -> c_int {
    let vaddr_emi_map: *mut c_void;
    let mut offset: c_int;

    if adsp.is_null() {
        return -ENXIO;
    }

    vaddr_emi_map = devm_ioremap(dev, DSP_EMI_MAP_ADDR, 0x4);
    if vaddr_emi_map.is_null() {
        dev_err!(dev, "failed to ioremap emi map base %#x\n", DSP_EMI_MAP_ADDR);
        return -ENOMEM;
    }

    offset = ((*adsp).pa_dram as isize - DRAM_PHYS_BASE_FROM_DSP_VIEW as isize) as c_int;
    (*adsp).dram_offset = offset;
    offset >>= DRAM_REMAP_SHIFT;
    dev_dbg!(dev, "adsp->pa_dram %pa, offset %#x\n", &mut (*adsp).pa_dram, offset);
    writel(offset as u32, vaddr_emi_map);
    if offset as u32 != readl(vaddr_emi_map) {
        dev_err!(dev, "write emi map fail : %#x\n", readl(vaddr_emi_map));
        return -EIO;
    }

    0
}

unsafe extern "C" fn mt8195_run(sdev: *mut snd_sof_dev) -> c_int {
    let adsp_bootup_addr: u32;

    adsp_bootup_addr = SRAM_PHYS_BASE_FROM_DSP_VIEW;
    dev_dbg!((*sdev).dev, "HIFIxDSP boot from base : 0x%08X\n", adsp_bootup_addr);
    sof_hifixdsp_boot_sequence(sdev, adsp_bootup_addr);

    0
}

unsafe extern "C" fn mt8195_dsp_probe(sdev: *mut snd_sof_dev) -> c_int {
    let pdev = to_platform_device((*sdev).dev);
    let priv_: *mut adsp_priv;
    let mut ret: c_int;

    priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<adsp_priv>(), GFP_KERNEL) as *mut adsp_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*(*sdev).pdata).hw_pdata = priv_ as *mut c_void;
    (*priv_).dev = (*sdev).dev;
    (*priv_).sdev = sdev;

    (*priv_).adsp = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<mtk_adsp_chip_info>(),
        GFP_KERNEL,
    ) as *mut mtk_adsp_chip_info;
    if (*priv_).adsp.is_null() {
        return -ENOMEM;
    }

    ret = platform_parse_resource(pdev, (*priv_).adsp as *mut c_void);
    if ret != 0 {
        return ret;
    }

    ret = mt8195_adsp_init_clock(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "mt8195_adsp_init_clock failed\n");
        return -EINVAL;
    }

    ret = adsp_clock_on(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_clock_on fail!\n");
        return -EINVAL;
    }

    ret = adsp_sram_power_on((*sdev).dev, true);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_sram_power_on fail!\n");
        adsp_clock_off(sdev);
        return ret;
    }

    ret = adsp_memory_remap_init(&mut (*pdev).dev, (*priv_).adsp);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_memory_remap_init fail!\n");
        adsp_sram_power_on(&mut (*pdev).dev, false);
        adsp_clock_off(sdev);
        return ret;
    }

    (*sdev).bar[SOF_FW_BLK_TYPE_IRAM] = devm_ioremap(
        (*sdev).dev,
        (*(*priv_).adsp).pa_sram,
        (*(*priv_).adsp).sramsize as usize,
    );
    if (*sdev).bar[SOF_FW_BLK_TYPE_IRAM].is_null() {
        dev_err!(
            (*sdev).dev,
            "failed to ioremap base %pa size %#x\n",
            &mut (*(*priv_).adsp).pa_sram,
            (*(*priv_).adsp).sramsize
        );
        ret = -EINVAL;
        adsp_sram_power_on(&mut (*pdev).dev, false);
        adsp_clock_off(sdev);
        return ret;
    }

    (*(*priv_).adsp).va_sram = (*sdev).bar[SOF_FW_BLK_TYPE_IRAM];

    (*sdev).bar[SOF_FW_BLK_TYPE_SRAM] = devm_ioremap(
        (*sdev).dev,
        (*(*priv_).adsp).pa_dram,
        (*(*priv_).adsp).dramsize as usize,
    );
    if (*sdev).bar[SOF_FW_BLK_TYPE_SRAM].is_null() {
        dev_err!(
            (*sdev).dev,
            "failed to ioremap base %pa size %#x\n",
            &mut (*(*priv_).adsp).pa_dram,
            (*(*priv_).adsp).dramsize
        );
        ret = -EINVAL;
        adsp_sram_power_on(&mut (*pdev).dev, false);
        adsp_clock_off(sdev);
        return ret;
    }
    (*(*priv_).adsp).va_dram = (*sdev).bar[SOF_FW_BLK_TYPE_SRAM];

    (*sdev).bar[DSP_REG_BAR] = (*(*priv_).adsp).va_cfgreg;

    (*sdev).mmio_bar = SOF_FW_BLK_TYPE_SRAM as c_int;
    (*sdev).mailbox_bar = SOF_FW_BLK_TYPE_SRAM as c_int;

    /* set default mailbox offset for FW ready message */
    (*sdev).dsp_box.offset = mt8195_get_mailbox_offset(sdev);

    (*priv_).ipc_dev = platform_device_register_data(
        &mut (*pdev).dev,
        b"mtk-adsp-ipc\0".as_ptr() as *const c_char,
        PLATFORM_DEVID_NONE,
        pdev as *const c_void,
        core::mem::size_of::<platform_device>(),
    );
    if IS_ERR((*priv_).ipc_dev as *mut c_void) {
        ret = PTR_ERR((*priv_).ipc_dev);
        dev_err!((*sdev).dev, "failed to register mtk-adsp-ipc device\n");
        adsp_sram_power_on(&mut (*pdev).dev, false);
        adsp_clock_off(sdev);
        return ret;
    }

    (*priv_).dsp_ipc = dev_get_drvdata(&mut (*(*priv_).ipc_dev).dev) as *mut mtk_adsp_ipc;
    if (*priv_).dsp_ipc.is_null() {
        ret = -EPROBE_DEFER;
        dev_err!((*sdev).dev, "failed to get drvdata\n");
        platform_device_unregister((*priv_).ipc_dev);
        adsp_sram_power_on(&mut (*pdev).dev, false);
        adsp_clock_off(sdev);
        return ret;
    }

    mtk_adsp_ipc_set_data((*priv_).dsp_ipc, priv_);
    (*(*priv_).dsp_ipc).ops = &dsp_ops;

    0
}

unsafe extern "C" fn mt8195_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    snd_sof_suspend((*sdev).dev)
}

unsafe extern "C" fn mt8195_dsp_remove(sdev: *mut snd_sof_dev) {
    let pdev = to_platform_device((*sdev).dev);
    let priv_ = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;

    platform_device_unregister((*priv_).ipc_dev);
    adsp_sram_power_on(&mut (*pdev).dev, false);
    adsp_clock_off(sdev);
}

unsafe extern "C" fn mt8195_dsp_suspend(sdev: *mut snd_sof_dev, _target_state: u32) -> c_int {
    let pdev = to_platform_device((*sdev).dev);
    let mut ret: c_int;
    let mut reset_sw: u32 = 0;
    let dbg_pc: u32;

    /* wait dsp enter idle, timeout is 1 second */
    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        DSP_REG_BAR,
        DSP_RESET_SW,
        &mut reset_sw,
        SUSPEND_DSP_IDLE_POLL_INTERVAL_US,
        SUSPEND_DSP_IDLE_TIMEOUT_US,
    );
    if ret < 0 {
        dbg_pc = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGPC);
        dev_warn!(
            (*sdev).dev,
            "dsp not idle, powering off anyway : swrest %#x, pc %#x, ret %d\n",
            reset_sw,
            dbg_pc,
            ret
        );
    }

    /* stall and reset dsp */
    sof_hifixdsp_shutdown(sdev);

    /* power down adsp sram */
    ret = adsp_sram_power_on(&mut (*pdev).dev, false);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_sram_power_off fail!\n");
        return ret;
    }

    /* turn off adsp clock */
    adsp_clock_off(sdev)
}

unsafe extern "C" fn mt8195_dsp_resume(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    /* turn on adsp clock */
    ret = adsp_clock_on(sdev);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_clock_on fail!\n");
        return ret;
    }

    /* power on adsp sram */
    ret = adsp_sram_power_on((*sdev).dev, true);
    if ret != 0 {
        dev_err!((*sdev).dev, "adsp_sram_power_on fail!\n");
    }

    ret
}

unsafe extern "C" fn mt8195_adsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let dbg_pc: u32;
    let dbg_data: u32;
    let dbg_bus0: u32;
    let dbg_bus1: u32;
    let dbg_inst: u32;
    let dbg_ls0stat: u32;
    let dbg_ls1stat: u32;
    let faultbus: u32;
    let faultinfo: u32;
    let swrest: u32;

    /* dump debug registers */
    dbg_pc = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGPC);
    dbg_data = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGDATA);
    dbg_bus0 = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGBUS0);
    dbg_bus1 = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGBUS1);
    dbg_inst = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGINST);
    dbg_ls0stat = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGLS0STAT);
    dbg_ls1stat = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGLS1STAT);
    faultbus = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PFAULTBUS);
    faultinfo = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PFAULTINFO);
    swrest = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_RESET_SW);

    dev_info!(
        (*sdev).dev,
        "adsp dump : pc %#x, data %#x, bus0 %#x, bus1 %#x, swrest %#x",
        dbg_pc,
        dbg_data,
        dbg_bus0,
        dbg_bus1,
        swrest
    );
    dev_info!(
        (*sdev).dev,
        "dbg_inst %#x, ls0stat %#x, ls1stat %#x, faultbus %#x, faultinfo %#x",
        dbg_inst,
        dbg_ls0stat,
        dbg_ls1stat,
        faultbus,
        faultinfo
    );

    mtk_adsp_dump(sdev, flags);
}

static mut mt8195_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: b"SOF_DL2\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 0,
            channels_max: 0,
        },
    },
    snd_soc_dai_driver {
        name: b"SOF_DL3\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 0,
            channels_max: 0,
        },
    },
    snd_soc_dai_driver {
        name: b"SOF_UL4\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            channels_min: 0,
            channels_max: 0,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
        },
    },
    snd_soc_dai_driver {
        name: b"SOF_UL5\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            channels_min: 0,
            channels_max: 0,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
        },
    },
];

/* mt8195 ops */
static sof_mt8195_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* probe and remove */
    probe: Some(mt8195_dsp_probe),
    remove: Some(mt8195_dsp_remove),
    shutdown: Some(mt8195_dsp_shutdown),

    /* DSP core boot */
    run: Some(mt8195_run),

    /* Block IO */
    block_read: unsafe { &sof_block_read as *const c_void },
    block_write: unsafe { &sof_block_write as *const c_void },

    /* Mailbox IO */
    mailbox_read: unsafe { &sof_mailbox_read as *const c_void },
    mailbox_write: unsafe { &sof_mailbox_write as *const c_void },

    /* Register IO */
    write: unsafe { &sof_io_write as *const c_void },
    read: unsafe { &sof_io_read as *const c_void },
    write64: unsafe { &sof_io_write64 as *const c_void },
    read64: unsafe { &sof_io_read64 as *const c_void },

    /* ipc */
    send_msg: unsafe { &mtk_adsp_send_msg as *const c_void },
    get_mailbox_offset: Some(mt8195_get_mailbox_offset),
    get_window_offset: Some(mt8195_get_window_offset),
    ipc_msg_data: unsafe { &sof_ipc_msg_data as *const c_void },
    set_stream_data_offset: unsafe { &sof_set_stream_data_offset as *const c_void },

    /* misc */
    get_bar_index: unsafe { &mtk_adsp_get_bar_index as *const c_void },

    /* stream callbacks */
    pcm_open: unsafe { &sof_stream_pcm_open as *const c_void },
    pcm_hw_params: unsafe { &mtk_adsp_stream_pcm_hw_params as *const c_void },
    pcm_pointer: unsafe { &mtk_adsp_stream_pcm_pointer as *const c_void },
    pcm_close: unsafe { &sof_stream_pcm_close as *const c_void },

    /* firmware loading */
    load_firmware: unsafe { &snd_sof_load_firmware_memcpy as *const c_void },

    /* Firmware ops */
    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops as *const c_void },

    /* Debug information */
    dbg_dump: Some(mt8195_adsp_dump),
    debugfs_add_region_item: unsafe { &snd_sof_debugfs_add_region_item_iomem as *const c_void },

    /* DAI drivers */
    drv: unsafe { mt8195_dai.as_ptr() as *mut snd_soc_dai_driver },
    num_drv: 4,

    /* PM */
    suspend: Some(mt8195_dsp_suspend),
    resume: Some(mt8195_dsp_resume),

    /* ALSA HW info flags */
    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
};

static mut sof_mt8195_machs: [snd_sof_of_mach; 4] = [
    snd_sof_of_mach {
        compatible: b"google,tomato\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-mt8195-mt6359-rt1019-rt5682.tplg\0".as_ptr() as *const c_char,
    },
    snd_sof_of_mach {
        compatible: b"google,dojo\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-mt8195-mt6359-max98390-rt5682.tplg\0".as_ptr() as *const c_char,
    },
    snd_sof_of_mach {
        compatible: b"mediatek,mt8195\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-mt8195.tplg\0".as_ptr() as *const c_char,
    },
    snd_sof_of_mach {
        /* sentinel */
        compatible: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

static sof_of_mt8195_desc: sof_dev_desc = sof_dev_desc {
    of_machines: unsafe { sof_mt8195_machs.as_ptr() as *mut snd_sof_of_mach },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3 as c_int,
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"mediatek/sof\0".as_ptr() as *const c_char,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"mediatek/sof-tplg\0".as_ptr() as *const c_char,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"sof-mt8195.ri\0".as_ptr() as *const c_char,
    ],
    nocodec_tplg_filename: b"sof-mt8195-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: &sof_mt8195_ops,
    ipc_timeout: 1000,
};

static sof_of_mt8195_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8195-dsp\0".as_ptr() as *const c_char,
        data: &sof_of_mt8195_desc as *const sof_dev_desc as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, sof_of_mt8195_ids); */

/* DT driver definition */
static snd_sof_of_mt8195_driver: platform_driver = platform_driver {
    probe: unsafe { &sof_of_probe as *const c_void },
    remove: unsafe { &sof_of_remove as *const c_void },
    shutdown: unsafe { &sof_of_shutdown as *const c_void },
    driver: platform_driver_inner {
        name: b"sof-audio-of-mt8195\0".as_ptr() as *const c_char,
        pm: unsafe { &sof_of_pm as *const c_void },
        of_match_table: sof_of_mt8195_ids.as_ptr(),
    },
};

/* module_platform_driver(snd_sof_of_mt8195_driver); */

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF support for MTL 8195 platforms"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_MTK_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
