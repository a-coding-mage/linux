// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2022 Mediatek Inc. All rights reserved.
//
// Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
//         Tinghan Shen <tinghan.shen@mediatek.com>

/*
 * Hardware interface for audio DSP on mt8186
 */

// C dependencies:
// linux/delay.h, linux/firmware.h, linux/io.h, linux/of_irq.h,
// linux/of_platform.h, linux/of_reserved_mem.h, linux/module.h,
// sound/sof.h, sound/sof/xtensa.h, ../../ops.h, ../../sof-of-dev.h,
// ../adsp_helper.h, ../mtk-adsp-common.h, mt8186.h, mt8186-clk.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u32 = c_uint;
type phys_addr_t = usize;
type size_t = usize;

const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_uint = 0;
const PLATFORM_DEVID_NONE: c_int = -1;

const SOF_FW_BLK_TYPE_IRAM: usize = 0;
const SOF_FW_BLK_TYPE_SRAM: usize = 1;
const DSP_REG_BAR: usize = 2;
const DSP_SECREG_BAR: usize = 3;
const DSP_BUSREG_BAR: usize = 4;
const SOF_IPC_TYPE_3: usize = 3;
const SOF_IPC_TYPE_3_U32: u32 = 3;

const MBOX_OFFSET: c_int = 0;
const DRAM_REMAP_MASK: phys_addr_t = 0;
const TOTAL_SIZE_SHARED_DRAM_FROM_TAIL: usize = 0;
const DRAM_PHYS_BASE_FROM_DSP_VIEW: phys_addr_t = 0;
const DRAM_REMAP_SHIFT: u32 = 0;
const SRAM_PHYS_BASE_FROM_DSP_VIEW: u32 = 0;
const ADSP_SRAM_POOL_CON: u32 = 0;
const DSP_SRAM_POOL_PD_MASK: u32 = 0;
const DSP_C0_EMI_MAP_ADDR: u32 = 0;
const DSP_C0_DMAEMI_MAP_ADDR: u32 = 0;
const DSP_PDEBUGPC: u32 = 0;
const DSP_PDEBUGDATA: u32 = 0;
const DSP_PDEBUGINST: u32 = 0;
const DSP_PDEBUGLS0STAT: u32 = 0;
const DSP_PDEBUGSTATUS: u32 = 0;
const DSP_PFAULTINFO: u32 = 0;

const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_PAUSE: u32 = 0;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32 = 0;

const fn BIT(nr: usize) -> u32 {
    1u32 << nr
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
    pub pm: *const c_void,
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
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct resource {
    pub start: phys_addr_t,
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
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dsp_box {
    pub offset: c_int,
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
pub struct mtk_adsp_chip_info {
    pub pa_dram: phys_addr_t,
    pub dramsize: usize,
    pub va_cfgreg: *mut c_void,
    pub pa_cfgreg: phys_addr_t,
    pub cfgregsize: usize,
    pub pa_sram: phys_addr_t,
    pub sramsize: usize,
    pub va_secreg: *mut c_void,
    pub pa_secreg: phys_addr_t,
    pub secregsize: usize,
    pub va_busreg: *mut c_void,
    pub pa_busreg: phys_addr_t,
    pub busregsize: usize,
    pub dram_offset: u32,
    pub va_sram: *mut c_void,
    pub va_dram: *mut c_void,
}

#[repr(C)]
pub struct mtk_adsp_ipc {
    pub ops: *const mtk_adsp_ipc_ops,
}

#[repr(C)]
pub struct mtk_adsp_ipc_ops {
    pub handle_reply: Option<unsafe extern "C" fn()>,
    pub handle_request: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dsp_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub block_read: Option<unsafe extern "C" fn()>,
    pub block_write: Option<unsafe extern "C" fn()>,
    pub mailbox_read: Option<unsafe extern "C" fn()>,
    pub mailbox_write: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn()>,
    pub read: Option<unsafe extern "C" fn()>,
    pub write64: Option<unsafe extern "C" fn()>,
    pub read64: Option<unsafe extern "C" fn()>,
    pub send_msg: Option<unsafe extern "C" fn()>,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub ipc_msg_data: Option<unsafe extern "C" fn()>,
    pub set_stream_data_offset: Option<unsafe extern "C" fn()>,
    pub get_bar_index: Option<unsafe extern "C" fn()>,
    pub pcm_open: Option<unsafe extern "C" fn()>,
    pub pcm_hw_params: Option<unsafe extern "C" fn()>,
    pub pcm_pointer: Option<unsafe extern "C" fn()>,
    pub pcm_close: Option<unsafe extern "C" fn()>,
    pub load_firmware: Option<unsafe extern "C" fn()>,
    pub dsp_arch_ops: *const c_void,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub debugfs_add_region_item: Option<unsafe extern "C" fn()>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub hw_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_of_mach {
    pub compatible: *const c_char,
    pub sof_tplg_filename: *const c_char,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub of_machines: *mut snd_sof_of_mach,
    pub ipc_supported_mask: u32,
    pub ipc_default: u32,
    pub default_fw_path: [*const c_char; 4],
    pub default_tplg_path: [*const c_char; 4],
    pub default_fw_filename: [*const c_char; 4],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const snd_sof_dsp_ops,
    pub ops_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

unsafe extern "C" {
    static sof_xtensa_arch_ops: c_void;
    static sof_of_pm: c_void;

    fn mtk_adsp_handle_reply();
    fn mtk_adsp_handle_request();
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn of_reserved_mem_region_to_resource(
        node: *mut device_node,
        index: c_int,
        res: *mut resource,
    ) -> c_int;
    fn resource_size(res: *const resource) -> usize;
    fn platform_get_resource_byname(
        pdev: *mut platform_device,
        ty: c_uint,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: phys_addr_t, size: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: size_t,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn mtk_adsp_ipc_set_data(ipc: *mut mtk_adsp_ipc, data: *mut adsp_priv);
    fn mt8186_sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
    fn mt8186_sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
    fn mt8186_adsp_init_clock(sdev: *mut snd_sof_dev) -> c_int;
    fn mt8186_adsp_clock_on(sdev: *mut snd_sof_dev) -> c_int;
    fn mt8186_adsp_clock_off(sdev: *mut snd_sof_dev);
    fn snd_sof_suspend(dev: *mut device) -> c_int;
    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: usize, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: usize, offset: u32, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: usize, offset: u32) -> u32;
    fn mtk_adsp_dump(sdev: *mut snd_sof_dev, flags: u32);
    fn sof_of_probe(pdev: *mut platform_device) -> c_int;
    fn sof_of_remove(pdev: *mut platform_device) -> c_int;
    fn sof_of_shutdown(pdev: *mut platform_device);

    fn sof_block_read();
    fn sof_block_write();
    fn sof_mailbox_read();
    fn sof_mailbox_write();
    fn sof_io_write();
    fn sof_io_read();
    fn sof_io_write64();
    fn sof_io_read64();
    fn mtk_adsp_send_msg();
    fn sof_ipc_msg_data();
    fn sof_set_stream_data_offset();
    fn mtk_adsp_get_bar_index();
    fn sof_stream_pcm_open();
    fn mtk_adsp_stream_pcm_hw_params();
    fn mtk_adsp_stream_pcm_pointer();
    fn sof_stream_pcm_close();
    fn snd_sof_load_firmware_memcpy();
    fn snd_sof_debugfs_add_region_item_iomem();
}

unsafe fn to_platform_device(dev: *mut device) -> *mut platform_device {
    dev as *mut platform_device
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    let value = ptr as isize;
    value < 0 && value >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn dev_err(_dev: *mut device, _fmt: *const c_char) {}
unsafe fn dev_dbg(_dev: *mut device, _fmt: *const c_char) {}
unsafe fn dev_info(_dev: *mut device, _fmt: *const c_char) {}

unsafe extern "C" fn mt8186_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    MBOX_OFFSET
}

unsafe extern "C" fn mt8186_get_window_offset(_sdev: *mut snd_sof_dev, _id: u32) -> c_int {
    MBOX_OFFSET
}

static dsp_ops: mtk_adsp_ipc_ops = mtk_adsp_ipc_ops {
    handle_reply: Some(mtk_adsp_handle_reply),
    handle_request: Some(mtk_adsp_handle_request),
};

unsafe extern "C" fn platform_parse_resource(pdev: *mut platform_device, data: *mut c_void) -> c_int {
    let mut mmio: *mut resource;
    let mut res: resource = mem::zeroed();
    let dev: *mut device = ptr::addr_of_mut!((*pdev).dev);
    let adsp: *mut mtk_adsp_chip_info = data as *mut mtk_adsp_chip_info;
    let mut ret: c_int;

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_err(dev, c"of_reserved_mem_device_init failed\n".as_ptr());
        return ret;
    }

    ret = of_reserved_mem_region_to_resource((*dev).of_node, 1, &mut res);
    if ret != 0 {
        dev_err(dev, c"of_address_to_resource sysmem failed\n".as_ptr());
        return ret;
    }

    (*adsp).pa_dram = res.start as phys_addr_t;
    if ((*adsp).pa_dram & DRAM_REMAP_MASK) != 0 {
        dev_err(dev, c"adsp memory(%#x) is not 4K-aligned\n".as_ptr());
        return -EINVAL;
    }

    (*adsp).dramsize = resource_size(&res);
    if (*adsp).dramsize < TOTAL_SIZE_SHARED_DRAM_FROM_TAIL {
        dev_err(dev, c"adsp memory(%#x) is not enough for share\n".as_ptr());
        return -EINVAL;
    }

    dev_dbg(dev, c"dram pbase=%pa size=%#x\n".as_ptr());

    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"cfg".as_ptr());
    if mmio.is_null() {
        dev_err(dev, c"no ADSP-CFG register resource\n".as_ptr());
        return -ENXIO;
    }

    (*adsp).va_cfgreg = devm_ioremap_resource(dev, mmio);
    if IS_ERR((*adsp).va_cfgreg) {
        return PTR_ERR((*adsp).va_cfgreg);
    }

    (*adsp).pa_cfgreg = (*mmio).start as phys_addr_t;
    (*adsp).cfgregsize = resource_size(mmio);

    dev_dbg(dev, c"cfgreg pbase=%pa size=%#x\n".as_ptr());

    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"sram".as_ptr());
    if mmio.is_null() {
        dev_err(dev, c"no SRAM resource\n".as_ptr());
        return -ENXIO;
    }

    (*adsp).pa_sram = (*mmio).start as phys_addr_t;
    (*adsp).sramsize = resource_size(mmio);

    dev_dbg(dev, c"sram pbase=%pa size=%#x\n".as_ptr());

    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"sec".as_ptr());
    if mmio.is_null() {
        dev_err(dev, c"no SEC register resource\n".as_ptr());
        return -ENXIO;
    }

    (*adsp).va_secreg = devm_ioremap_resource(dev, mmio);
    if IS_ERR((*adsp).va_secreg) {
        return PTR_ERR((*adsp).va_secreg);
    }

    (*adsp).pa_secreg = (*mmio).start as phys_addr_t;
    (*adsp).secregsize = resource_size(mmio);

    dev_dbg(dev, c"secreg pbase=%pa size=%#x\n".as_ptr());

    mmio = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"bus".as_ptr());
    if mmio.is_null() {
        dev_err(dev, c"no BUS register resource\n".as_ptr());
        return -ENXIO;
    }

    (*adsp).va_busreg = devm_ioremap_resource(dev, mmio);
    if IS_ERR((*adsp).va_busreg) {
        return PTR_ERR((*adsp).va_busreg);
    }

    (*adsp).pa_busreg = (*mmio).start as phys_addr_t;
    (*adsp).busregsize = resource_size(mmio);

    dev_dbg(dev, c"busreg pbase=%pa size=%#x\n".as_ptr());

    0
}

unsafe extern "C" fn adsp_sram_power_on(sdev: *mut snd_sof_dev) {
    snd_sof_dsp_update_bits(
        sdev,
        DSP_BUSREG_BAR,
        ADSP_SRAM_POOL_CON,
        DSP_SRAM_POOL_PD_MASK,
        0,
    );
}

unsafe extern "C" fn adsp_sram_power_off(sdev: *mut snd_sof_dev) {
    snd_sof_dsp_update_bits(
        sdev,
        DSP_BUSREG_BAR,
        ADSP_SRAM_POOL_CON,
        DSP_SRAM_POOL_PD_MASK,
        DSP_SRAM_POOL_PD_MASK,
    );
}

/*  Init the basic DSP DRAM address */
unsafe extern "C" fn adsp_memory_remap_init(
    sdev: *mut snd_sof_dev,
    adsp: *mut mtk_adsp_chip_info,
) -> c_int {
    let mut offset: u32;

    offset = ((*adsp).pa_dram.wrapping_sub(DRAM_PHYS_BASE_FROM_DSP_VIEW)) as u32;
    (*adsp).dram_offset = offset;
    offset >>= DRAM_REMAP_SHIFT;

    dev_dbg((*sdev).dev, c"adsp->pa_dram %pa, offset %#x\n".as_ptr());

    snd_sof_dsp_write(sdev, DSP_BUSREG_BAR, DSP_C0_EMI_MAP_ADDR, offset);
    snd_sof_dsp_write(sdev, DSP_BUSREG_BAR, DSP_C0_DMAEMI_MAP_ADDR, offset);

    if offset != snd_sof_dsp_read(sdev, DSP_BUSREG_BAR, DSP_C0_EMI_MAP_ADDR)
        || offset != snd_sof_dsp_read(sdev, DSP_BUSREG_BAR, DSP_C0_DMAEMI_MAP_ADDR)
    {
        dev_err((*sdev).dev, c"emi remap fail\n".as_ptr());
        return -EIO;
    }

    0
}

unsafe extern "C" fn mt8186_run(sdev: *mut snd_sof_dev) -> c_int {
    let adsp_bootup_addr: u32;

    adsp_bootup_addr = SRAM_PHYS_BASE_FROM_DSP_VIEW;
    dev_dbg((*sdev).dev, c"HIFIxDSP boot from base : 0x%08X\n".as_ptr());
    mt8186_sof_hifixdsp_boot_sequence(sdev, adsp_bootup_addr);

    0
}

unsafe extern "C" fn mt8186_dsp_probe(sdev: *mut snd_sof_dev) -> c_int {
    let pdev: *mut platform_device = to_platform_device((*sdev).dev);
    let priv_: *mut adsp_priv;
    let mut ret: c_int;

    priv_ = devm_kzalloc(&mut (*pdev).dev, mem::size_of::<adsp_priv>(), GFP_KERNEL) as *mut adsp_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*(*sdev).pdata).hw_pdata = priv_ as *mut c_void;
    (*priv_).dev = (*sdev).dev;
    (*priv_).sdev = sdev;

    (*priv_).adsp = devm_kzalloc(
        &mut (*pdev).dev,
        mem::size_of::<mtk_adsp_chip_info>(),
        GFP_KERNEL,
    ) as *mut mtk_adsp_chip_info;
    if (*priv_).adsp.is_null() {
        return -ENOMEM;
    }

    ret = platform_parse_resource(pdev, (*priv_).adsp as *mut c_void);
    if ret != 0 {
        return ret;
    }

    (*sdev).bar[SOF_FW_BLK_TYPE_IRAM] =
        devm_ioremap((*sdev).dev, (*(*priv_).adsp).pa_sram, (*(*priv_).adsp).sramsize);
    if (*sdev).bar[SOF_FW_BLK_TYPE_IRAM].is_null() {
        dev_err((*sdev).dev, c"failed to ioremap base %pa size %#x\n".as_ptr());
        return -ENOMEM;
    }

    (*(*priv_).adsp).va_sram = (*sdev).bar[SOF_FW_BLK_TYPE_IRAM];

    (*sdev).bar[SOF_FW_BLK_TYPE_SRAM] =
        devm_ioremap((*sdev).dev, (*(*priv_).adsp).pa_dram, (*(*priv_).adsp).dramsize);

    if (*sdev).bar[SOF_FW_BLK_TYPE_SRAM].is_null() {
        dev_err((*sdev).dev, c"failed to ioremap base %pa size %#x\n".as_ptr());
        return -ENOMEM;
    }

    (*(*priv_).adsp).va_dram = (*sdev).bar[SOF_FW_BLK_TYPE_SRAM];

    (*sdev).bar[DSP_REG_BAR] = (*(*priv_).adsp).va_cfgreg;
    (*sdev).bar[DSP_SECREG_BAR] = (*(*priv_).adsp).va_secreg;
    (*sdev).bar[DSP_BUSREG_BAR] = (*(*priv_).adsp).va_busreg;

    (*sdev).mmio_bar = SOF_FW_BLK_TYPE_SRAM as c_int;
    (*sdev).mailbox_bar = SOF_FW_BLK_TYPE_SRAM as c_int;

    /* set default mailbox offset for FW ready message */
    (*sdev).dsp_box.offset = mt8186_get_mailbox_offset(sdev);

    ret = adsp_memory_remap_init(sdev, (*priv_).adsp);
    if ret != 0 {
        dev_err((*sdev).dev, c"adsp_memory_remap_init fail!\n".as_ptr());
        return ret;
    }

    /* enable adsp clock before touching registers */
    ret = mt8186_adsp_init_clock(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, c"mt8186_adsp_init_clock failed\n".as_ptr());
        return ret;
    }

    ret = mt8186_adsp_clock_on(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, c"mt8186_adsp_clock_on fail!\n".as_ptr());
        return ret;
    }

    adsp_sram_power_on(sdev);

    (*priv_).ipc_dev = platform_device_register_data(
        &mut (*pdev).dev,
        c"mtk-adsp-ipc".as_ptr(),
        PLATFORM_DEVID_NONE,
        pdev as *const c_void,
        mem::size_of::<platform_device>(),
    );
    if IS_ERR((*priv_).ipc_dev as *const c_void) {
        ret = PTR_ERR((*priv_).ipc_dev);
        dev_err((*sdev).dev, c"failed to create mtk-adsp-ipc device\n".as_ptr());
        adsp_sram_power_off(sdev);
        mt8186_adsp_clock_off(sdev);
        return ret;
    }

    (*priv_).dsp_ipc = dev_get_drvdata(&mut (*(*priv_).ipc_dev).dev) as *mut mtk_adsp_ipc;
    if (*priv_).dsp_ipc.is_null() {
        ret = -EPROBE_DEFER;
        dev_err((*sdev).dev, c"failed to get drvdata\n".as_ptr());
        platform_device_unregister((*priv_).ipc_dev);
        adsp_sram_power_off(sdev);
        mt8186_adsp_clock_off(sdev);
        return ret;
    }

    mtk_adsp_ipc_set_data((*priv_).dsp_ipc, priv_);
    (*(*priv_).dsp_ipc).ops = &dsp_ops;

    0
}

unsafe extern "C" fn mt8186_dsp_remove(sdev: *mut snd_sof_dev) {
    let priv_: *mut adsp_priv = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;

    platform_device_unregister((*priv_).ipc_dev);
    mt8186_sof_hifixdsp_shutdown(sdev);
    adsp_sram_power_off(sdev);
    mt8186_adsp_clock_off(sdev);
}

unsafe extern "C" fn mt8186_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    snd_sof_suspend((*sdev).dev)
}

unsafe extern "C" fn mt8186_dsp_suspend(sdev: *mut snd_sof_dev, _target_state: u32) -> c_int {
    mt8186_sof_hifixdsp_shutdown(sdev);
    adsp_sram_power_off(sdev);
    mt8186_adsp_clock_off(sdev);

    0
}

unsafe extern "C" fn mt8186_dsp_resume(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret: c_int;

    ret = mt8186_adsp_clock_on(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, c"mt8186_adsp_clock_on fail!\n".as_ptr());
        return ret;
    }

    adsp_sram_power_on(sdev);

    ret
}

unsafe extern "C" fn mt8186_adsp_dump(sdev: *mut snd_sof_dev, flags: u32) {
    let dbg_pc: u32;
    let dbg_data: u32;
    let dbg_inst: u32;
    let dbg_ls0stat: u32;
    let dbg_status: u32;
    let faultinfo: u32;

    /* dump debug registers */
    dbg_pc = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGPC);
    dbg_data = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGDATA);
    dbg_inst = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGINST);
    dbg_ls0stat = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGLS0STAT);
    dbg_status = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PDEBUGSTATUS);
    faultinfo = snd_sof_dsp_read(sdev, DSP_REG_BAR, DSP_PFAULTINFO);

    dev_info((*sdev).dev, c"adsp dump : pc %#x, data %#x, dbg_inst %#x,".as_ptr());
    dev_info((*sdev).dev, c"ls0stat %#x, status %#x, faultinfo %#x".as_ptr());

    mtk_adsp_dump(sdev, flags);
}

static mut mt8186_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: c"SOF_DL1".as_ptr(),
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
        name: c"SOF_DL2".as_ptr(),
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
        name: c"SOF_UL1".as_ptr(),
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
        name: c"SOF_UL2".as_ptr(),
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

/* mt8186 ops */
static sof_mt8186_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* probe and remove */
    probe: Some(mt8186_dsp_probe),
    remove: Some(mt8186_dsp_remove),
    shutdown: Some(mt8186_dsp_shutdown),

    /* DSP core boot */
    run: Some(mt8186_run),

    /* Block IO */
    block_read: Some(sof_block_read),
    block_write: Some(sof_block_write),

    /* Mailbox IO */
    mailbox_read: Some(sof_mailbox_read),
    mailbox_write: Some(sof_mailbox_write),

    /* Register IO */
    write: Some(sof_io_write),
    read: Some(sof_io_read),
    write64: Some(sof_io_write64),
    read64: Some(sof_io_read64),

    /* ipc */
    send_msg: Some(mtk_adsp_send_msg),
    get_mailbox_offset: Some(mt8186_get_mailbox_offset),
    get_window_offset: Some(mt8186_get_window_offset),
    ipc_msg_data: Some(sof_ipc_msg_data),
    set_stream_data_offset: Some(sof_set_stream_data_offset),

    /* misc */
    get_bar_index: Some(mtk_adsp_get_bar_index),

    /* stream callbacks */
    pcm_open: Some(sof_stream_pcm_open),
    pcm_hw_params: Some(mtk_adsp_stream_pcm_hw_params),
    pcm_pointer: Some(mtk_adsp_stream_pcm_pointer),
    pcm_close: Some(sof_stream_pcm_close),

    /* firmware loading */
    load_firmware: Some(snd_sof_load_firmware_memcpy),

    /* Firmware ops */
    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops as *const c_void },

    /* DAI drivers */
    drv: unsafe { mt8186_dai.as_mut_ptr() },
    num_drv: 4,

    /* Debug information */
    dbg_dump: Some(mt8186_adsp_dump),
    debugfs_add_region_item: Some(snd_sof_debugfs_add_region_item_iomem),

    /* PM */
    suspend: Some(mt8186_dsp_suspend),
    resume: Some(mt8186_dsp_resume),

    /* ALSA HW info flags */
    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
};

static mut sof_mt8186_machs: [snd_sof_of_mach; 2] = [
    snd_sof_of_mach {
        compatible: c"mediatek,mt8186".as_ptr(),
        sof_tplg_filename: c"sof-mt8186.tplg".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: ptr::null(),
        sof_tplg_filename: ptr::null(),
    },
];

static sof_of_mt8186_desc: sof_dev_desc = sof_dev_desc {
    of_machines: unsafe { sof_mt8186_machs.as_mut_ptr() },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3_U32,
    default_fw_path: [ptr::null(), ptr::null(), ptr::null(), c"mediatek/sof".as_ptr()],
    default_tplg_path: [ptr::null(), ptr::null(), ptr::null(), c"mediatek/sof-tplg".as_ptr()],
    default_fw_filename: [ptr::null(), ptr::null(), ptr::null(), c"sof-mt8186.ri".as_ptr()],
    nocodec_tplg_filename: c"sof-mt8186-nocodec.tplg".as_ptr(),
    ops: &sof_mt8186_ops,
    ops_init: None,
};

/*
 * DL2, DL3, UL4, UL5 are registered as SOF FE, so creating the corresponding
 * SOF BE to complete the pipeline.
 */
static mut mt8188_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: c"SOF_DL2".as_ptr(),
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
        name: c"SOF_DL3".as_ptr(),
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
        name: c"SOF_UL4".as_ptr(),
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
        name: c"SOF_UL5".as_ptr(),
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

/* mt8188 ops */
static mut sof_mt8188_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    probe: None,
    remove: None,
    shutdown: None,
    run: None,
    block_read: None,
    block_write: None,
    mailbox_read: None,
    mailbox_write: None,
    write: None,
    read: None,
    write64: None,
    read64: None,
    send_msg: None,
    get_mailbox_offset: None,
    get_window_offset: None,
    ipc_msg_data: None,
    set_stream_data_offset: None,
    get_bar_index: None,
    pcm_open: None,
    pcm_hw_params: None,
    pcm_pointer: None,
    pcm_close: None,
    load_firmware: None,
    dsp_arch_ops: ptr::null(),
    drv: ptr::null_mut(),
    num_drv: 0,
    dbg_dump: None,
    debugfs_add_region_item: None,
    suspend: None,
    resume: None,
    hw_info: 0,
};

unsafe extern "C" fn sof_mt8188_ops_init(_sdev: *mut snd_sof_dev) -> c_int {
    /* common defaults */
    ptr::copy_nonoverlapping(&sof_mt8186_ops, &mut sof_mt8188_ops, 1);

    sof_mt8188_ops.drv = mt8188_dai.as_mut_ptr();
    sof_mt8188_ops.num_drv = 4;

    0
}

static mut sof_mt8188_machs: [snd_sof_of_mach; 2] = [
    snd_sof_of_mach {
        compatible: c"mediatek,mt8188".as_ptr(),
        sof_tplg_filename: c"sof-mt8188.tplg".as_ptr(),
    },
    snd_sof_of_mach {
        compatible: ptr::null(),
        sof_tplg_filename: ptr::null(),
    },
];

static sof_of_mt8188_desc: sof_dev_desc = sof_dev_desc {
    of_machines: unsafe { sof_mt8188_machs.as_mut_ptr() },
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3_U32,
    default_fw_path: [ptr::null(), ptr::null(), ptr::null(), c"mediatek/sof".as_ptr()],
    default_tplg_path: [ptr::null(), ptr::null(), ptr::null(), c"mediatek/sof-tplg".as_ptr()],
    default_fw_filename: [ptr::null(), ptr::null(), ptr::null(), c"sof-mt8188.ri".as_ptr()],
    nocodec_tplg_filename: c"sof-mt8188-nocodec.tplg".as_ptr(),
    ops: unsafe { &sof_mt8188_ops },
    ops_init: Some(sof_mt8188_ops_init),
};

static sof_of_mt8186_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: c"mediatek,mt8186-dsp".as_ptr(),
        data: &sof_of_mt8186_desc as *const sof_dev_desc as *const c_void,
    },
    of_device_id {
        compatible: c"mediatek,mt8188-dsp".as_ptr(),
        data: &sof_of_mt8188_desc as *const sof_dev_desc as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, sof_of_mt8186_ids);

/* DT driver definition */
static snd_sof_of_mt8186_driver: platform_driver = platform_driver {
    probe: Some(sof_of_probe),
    remove: Some(sof_of_remove),
    shutdown: Some(sof_of_shutdown),
    driver: device_driver {
        name: c"sof-audio-of-mt8186".as_ptr(),
        pm: unsafe { &sof_of_pm as *const c_void },
        of_match_table: sof_of_mt8186_ids.as_ptr(),
    },
};

// module_platform_driver(snd_sof_of_mt8186_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for MT8186/MT8188 platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");
// MODULE_IMPORT_NS("SND_SOC_SOF_MTK_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
