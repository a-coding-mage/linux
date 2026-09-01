// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

/*
 * Hardware interface for audio DSP on Broadwell
 */

/* C include dependencies:
 * linux/module.h, sound/sof.h, sound/sof/xtensa.h, sound/soc-acpi.h,
 * sound/soc-acpi-intel-match.h, sound/intel-dsp-config.h, ../ops.h,
 * shim.h, ../sof-acpi-dev.h, ../sof-audio.h
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub num_cores: c_int,
    pub bar: [*mut c_void; 2],
    pub mmio_bar: c_int,
    pub mailbox_bar: c_int,
    pub dsp_oops_offset: u32,
    pub ipc_lock: spinlock_t,
    pub host_box: sof_mailbox,
    pub dsp_box: sof_mailbox,
    pub ipc_irq: c_int,
}

#[repr(C)]
pub struct sof_mailbox {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
    pub tplg_filename: *const c_char,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *mut snd_soc_acpi_mach,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const sof_intel_dsp_desc,
    pub ipc_supported_mask: u32,
    pub ipc_default: c_int,
    pub default_fw_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_tplg_path: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub default_fw_filename: [*const c_char; SOF_IPC_TYPE_COUNT],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const snd_sof_dsp_ops,
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: c_int,
    pub offset: u32,
    pub size: u32,
    pub access: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_hdr,
}

#[repr(C)]
pub struct sof_ipc_hdr {
    pub totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    pub msg_data: *const c_void,
    pub msg_size: usize,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: c_int,
    pub host_managed_cores_mask: u32,
    pub hw_ip_version: u32,
}

#[repr(C)]
pub struct resource {
    pub start: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub sof_tplg_filename: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub acpi_ipc_irq_index: c_int,
    pub platform: *const c_char,
    pub num_dai_drivers: c_int,
    pub dai_drivers: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32,
    pub channels_max: u32,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub block_read: *const c_void,
    pub block_write: *const c_void,
    pub mailbox_read: *const c_void,
    pub mailbox_write: *const c_void,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int>,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub ipc_msg_data: *const c_void,
    pub set_stream_data_offset: *const c_void,
    pub machine_select: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> *mut snd_soc_acpi_mach>,
    pub machine_register: *const c_void,
    pub machine_unregister: *const c_void,
    pub set_mach_params: Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach, *mut snd_sof_dev)>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub debugfs_add_region_item: *const c_void,
    pub pcm_open: *const c_void,
    pub pcm_close: *const c_void,
    pub load_firmware: *const c_void,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: usize,
    pub hw_info: u32,
    pub dsp_arch_ops: *const c_void,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 8],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: *const c_void,
    pub driver: platform_driver_inner,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

pub type irqreturn_t = c_int;

const BDW_DSP_BAR: c_int = 0;
const BDW_PCI_BAR: c_int = 1;

const IRAM_OFFSET: u32 = 0xA0000;
const BDW_IRAM_SIZE: u32 = 10 * 32 * 1024;
const DRAM_OFFSET: u32 = 0x00000;
const BDW_DRAM_SIZE: u32 = 20 * 32 * 1024;
const SHIM_OFFSET: u32 = 0xFB000;
const SHIM_SIZE: u32 = 0x100;
const MBOX_OFFSET: u32 = 0x9E000;
const MBOX_SIZE: u32 = 0x1000;
const MBOX_DUMP_SIZE: u32 = 0x30;
const EXCEPT_OFFSET: u32 = 0x800;
const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;

const DMAC0_OFFSET: u32 = 0xFE000;
const DMAC1_OFFSET: u32 = 0xFF000;
const DMAC_SIZE: u32 = 0x420;
const SSP0_OFFSET: u32 = 0xFC000;
const SSP1_OFFSET: u32 = 0xFD000;
const SSP_SIZE: u32 = 0x100;

const BDW_STACK_DUMP_SIZE: usize = 32;

const fn BDW_PANIC_OFFSET(x: u32) -> u32 {
    x & 0xFFFF
}

const SOF_IPC_TYPE_3: usize = 3;
const SOF_IPC_TYPE_COUNT: usize = 4;

extern "C" {
    static sof_block_read: c_void;
    static sof_block_write: c_void;
    static sof_mailbox_read: c_void;
    static sof_mailbox_write: c_void;
    static sof_ipc_msg_data: c_void;
    static sof_set_stream_data_offset: c_void;
    static sof_machine_register: c_void;
    static sof_machine_unregister: c_void;
    static snd_sof_debugfs_add_region_item_iomem: c_void;
    static sof_stream_pcm_open: c_void;
    static sof_stream_pcm_close: c_void;
    static snd_sof_load_firmware_memcpy: c_void;
    static sof_xtensa_arch_ops: c_void;
    static snd_soc_acpi_intel_broadwell_machines: *mut snd_soc_acpi_mach;
    static sof_acpi_pm: c_void;
    static sof_acpi_remove: c_void;

    fn snd_sof_dsp_update_bits(sdev: *mut snd_sof_dev, bar: c_int, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_update_bits_unlocked(sdev: *mut snd_sof_dev, bar: c_int, offset: u32, mask: u32, value: u32);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: c_int, offset: u32) -> u32;
    fn snd_sof_dsp_read64(sdev: *mut snd_sof_dev, bar: c_int, offset: u32) -> u32;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_int, offset: u32, value: u32);
    fn readl(addr: *mut c_void) -> u32;
    fn mdelay(ms: c_ulong);
    fn msleep(ms: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn sof_mailbox_read_fn(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: usize);
    fn sof_mailbox_write_fn(sdev: *mut snd_sof_dev, offset: u32, src: *const c_void, bytes: usize);
    fn sof_print_oops_and_stack(
        sdev: *mut snd_sof_dev,
        level: *const c_char,
        status: u32,
        panic: u32,
        xoops: *mut sof_ipc_dsp_oops_xtensa,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut u32,
        stack_words: usize,
    );
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn str_yes_no(value: u32) -> *const c_char;
    fn snd_sof_ipc_process_reply(sdev: *mut snd_sof_dev, ipcx: u32);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool);
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn platform_get_resource(pdev: *mut platform_device, ty: u32, index: c_int) -> *mut resource;
    fn resource_size(res: *mut resource) -> u32;
    fn devm_ioremap(dev: *mut device, base: u32, size: u32) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: c_int) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: u32,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_soc_acpi_find_machine(machines: *mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn acpi_match_device(table: *const acpi_device_id, dev: *mut device) -> *const acpi_device_id;
    fn snd_intel_acpi_dsp_driver_probe(dev: *mut device, id: *const c_char) -> c_int;
    fn sof_acpi_probe(pdev: *mut platform_device, desc: *const sof_dev_desc) -> c_int;
    fn pm_ptr(ptr: *const c_void) -> *const c_void;
}

extern "C" {
    static KERN_ERR: *const c_char;
}

extern "C" {
    static SHIM_HMDC: u32;
    static SHIM_HMDC_HDDA_E0_ALLCH: u32;
    static SHIM_HMDC_HDDA_E1_ALLCH: u32;
    static SHIM_CSR: u32;
    static SHIM_CSR_STALL: u32;
    static SHIM_CSR_RST: u32;
    static PCI_VDRTCTL2: u32;
    static PCI_VDRTCL2_DCLCGE: u32;
    static PCI_VDRTCL2_DTCGE: u32;
    static PCI_VDRTCTL0: u32;
    static PCI_VDRTCL0_D3PGD: u32;
    static PCI_PMCS: u32;
    static PCI_PMCS_PS_MASK: u32;
    static SHIM_CSR_S1IOCS: u32;
    static SHIM_CSR_SBCS1: u32;
    static SHIM_CSR_LPCS: u32;
    static SHIM_CSR_DCS_MASK: u32;
    static SHIM_CLKCTL: u32;
    static SHIM_CLKCTL_MASK: u32;
    static SHIM_CLKCTL_DCPLCG: u32;
    static SHIM_CLKCTL_SCOE0: u32;
    static PCI_VDRTCL2_APLLSE_MASK: u32;
    static SHIM_CSR2: u32;
    static SHIM_CSR2_SDFD_SSP1: u32;
    static SHIM_IMRX: u32;
    static SHIM_IMRX_BUSY: u32;
    static SHIM_IMRX_DONE: u32;
    static SHIM_IMRD: u32;
    static SHIM_IMRD_DONE: u32;
    static SHIM_IMRD_BUSY: u32;
    static SHIM_IMRD_SSP0: u32;
    static SHIM_IMRD_DMAC: u32;
    static SHIM_IPCX: u32;
    static SHIM_IPCX_BUSY: u32;
    static SHIM_IPCX_DONE: u32;
    static SHIM_IPCD: u32;
    static SHIM_IPCD_BUSY: u32;
    static SHIM_IPCD_DONE: u32;
    static SHIM_ISRX: u32;
    static SHIM_ISRX_DONE: u32;
    static SHIM_ISRX_BUSY: u32;
    static SOF_IPC_PANIC_MAGIC_MASK: u32;
    static SOF_IPC_PANIC_MAGIC: u32;
}

extern "C" {
    fn SHIM_CSR_DCS(x: u32) -> u32;
    fn BIT(x: usize) -> u32;
    fn DMA_BIT_MASK(x: u32) -> u64;
}

const SOF_DEBUGFS_ACCESS_ALWAYS: u32 = 0;
const SOF_DEBUGFS_ACCESS_D0_ONLY: u32 = 1;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_WAKE_THREAD: irqreturn_t = 2;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: u32 = 0x00000080;
const IORESOURCE_MEM: u32 = 0x00000200;
const SND_INTEL_DSP_DRIVER_ANY: c_int = 0;
const SND_INTEL_DSP_DRIVER_SOF: c_int = 3;
const SOF_INTEL_BROADWELL: u32 = 0;
const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_INFO_PAUSE: u32 = 0x00080000;
const SNDRV_PCM_INFO_BATCH: u32 = 0x01000000;

static bdw_debugfs: [snd_sof_debugfs_map; 7] = [
    snd_sof_debugfs_map { name: b"dmac0\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: DMAC0_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: b"dmac1\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: DMAC1_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: b"ssp0\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: SSP0_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: b"ssp1\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: SSP1_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: b"iram\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: IRAM_OFFSET, size: BDW_IRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: b"dram\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: DRAM_OFFSET, size: BDW_DRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: b"shim\0".as_ptr() as *const c_char, bar: BDW_DSP_BAR, offset: SHIM_OFFSET, size: SHIM_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
];

unsafe extern "C" fn bdw_run(sdev: *mut snd_sof_dev) -> c_int {
    snd_sof_dsp_update_bits(
        sdev,
        BDW_DSP_BAR,
        SHIM_HMDC,
        SHIM_HMDC_HDDA_E0_ALLCH | SHIM_HMDC_HDDA_E1_ALLCH,
        0,
    );
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_CSR, SHIM_CSR_STALL, 0x0);
    1
}

unsafe extern "C" fn bdw_reset(sdev: *mut snd_sof_dev) -> c_int {
    snd_sof_dsp_update_bits_unlocked(
        sdev,
        BDW_DSP_BAR,
        SHIM_CSR,
        SHIM_CSR_RST | SHIM_CSR_STALL,
        SHIM_CSR_RST | SHIM_CSR_STALL,
    );
    mdelay(10);
    snd_sof_dsp_update_bits_unlocked(
        sdev,
        BDW_DSP_BAR,
        SHIM_CSR,
        SHIM_CSR_RST | SHIM_CSR_STALL,
        SHIM_CSR_STALL,
    );
    0
}

unsafe extern "C" fn bdw_set_dsp_D0(sdev: *mut snd_sof_dev) -> c_int {
    let mut tries: c_int = 10;
    let mut reg: u32;

    snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_VDRTCTL2, PCI_VDRTCL2_DCLCGE | PCI_VDRTCL2_DTCGE, 0);
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_VDRTCTL0, PCI_VDRTCL0_D3PGD, PCI_VDRTCL0_D3PGD);
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_PMCS, PCI_PMCS_PS_MASK, 0);

    while tries != 0 {
        tries -= 1;
        reg = readl(((*sdev).bar[BDW_PCI_BAR as usize] as *mut u8).add(PCI_PMCS as usize) as *mut c_void) & PCI_PMCS_PS_MASK;
        if reg == 0 {
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_CSR, SHIM_CSR_S1IOCS | SHIM_CSR_SBCS1 | SHIM_CSR_LPCS, 0x0);
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_CSR, SHIM_CSR_STALL | SHIM_CSR_DCS_MASK, SHIM_CSR_STALL | SHIM_CSR_DCS(4));
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_CLKCTL, SHIM_CLKCTL_MASK | SHIM_CLKCTL_DCPLCG | SHIM_CLKCTL_SCOE0, SHIM_CLKCTL_MASK | SHIM_CLKCTL_DCPLCG | SHIM_CLKCTL_SCOE0);
            bdw_reset(sdev);
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_VDRTCTL2, PCI_VDRTCL2_DCLCGE | PCI_VDRTCL2_DTCGE, PCI_VDRTCL2_DCLCGE | PCI_VDRTCL2_DTCGE);
            usleep_range(50, 55);
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_VDRTCTL2, PCI_VDRTCL2_APLLSE_MASK, 0);
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_PCI_BAR, PCI_VDRTCTL0, 0xfffffffC, 0x0);
            snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_CSR2, SHIM_CSR2_SDFD_SSP1, SHIM_CSR2_SDFD_SSP1);
            snd_sof_dsp_update_bits(sdev, BDW_DSP_BAR, SHIM_HMDC, SHIM_HMDC_HDDA_E0_ALLCH | SHIM_HMDC_HDDA_E1_ALLCH, SHIM_HMDC_HDDA_E0_ALLCH | SHIM_HMDC_HDDA_E1_ALLCH);
            snd_sof_dsp_update_bits(sdev, BDW_DSP_BAR, SHIM_IMRX, SHIM_IMRX_BUSY | SHIM_IMRX_DONE, 0x0);
            snd_sof_dsp_update_bits(sdev, BDW_DSP_BAR, SHIM_IMRD, SHIM_IMRD_DONE | SHIM_IMRD_BUSY | SHIM_IMRD_SSP0 | SHIM_IMRD_DMAC, 0x0);
            snd_sof_dsp_write(sdev, BDW_DSP_BAR, SHIM_IPCX, 0x0);
            snd_sof_dsp_write(sdev, BDW_DSP_BAR, SHIM_IPCD, 0x0);
            snd_sof_dsp_write(sdev, BDW_DSP_BAR, 0x80, 0x6);
            snd_sof_dsp_write(sdev, BDW_DSP_BAR, 0xe0, 0x300a);
            return 0;
        }
        msleep(20);
    }

    -ENODEV
}

unsafe extern "C" fn bdw_get_registers(
    sdev: *mut snd_sof_dev,
    xoops: *mut sof_ipc_dsp_oops_xtensa,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut u32,
    stack_words: usize,
) {
    let mut offset = (*sdev).dsp_oops_offset;

    sof_mailbox_read_fn(sdev, offset, xoops as *mut c_void, size_of::<sof_ipc_dsp_oops_xtensa>());

    /* note: variable AR register array is not read */

    if (*xoops).arch_hdr.totalsize > EXCEPT_MAX_HDR_SIZE {
        dev_err((*sdev).dev, b"invalid header size 0x%x. FW oops is bogus\n\0".as_ptr() as *const c_char, (*xoops).arch_hdr.totalsize);
        return;
    }

    offset += (*xoops).arch_hdr.totalsize;
    sof_mailbox_read_fn(sdev, offset, panic_info as *mut c_void, size_of::<sof_ipc_panic_info>());

    offset += size_of::<sof_ipc_panic_info>() as u32;
    sof_mailbox_read_fn(sdev, offset, stack as *mut c_void, stack_words * size_of::<u32>());
}

unsafe extern "C" fn bdw_dump(sdev: *mut snd_sof_dev, _flags: u32) {
    let mut xoops: sof_ipc_dsp_oops_xtensa = core::mem::zeroed();
    let mut panic_info: sof_ipc_panic_info = core::mem::zeroed();
    let mut stack: [u32; BDW_STACK_DUMP_SIZE] = [0; BDW_STACK_DUMP_SIZE];
    let status = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IPCD);
    let panic = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IPCX);
    bdw_get_registers(sdev, &mut xoops, &mut panic_info, stack.as_mut_ptr(), BDW_STACK_DUMP_SIZE);
    sof_print_oops_and_stack(sdev, KERN_ERR, status, panic, &mut xoops, &mut panic_info, stack.as_mut_ptr(), BDW_STACK_DUMP_SIZE);

    let imrx = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IMRX);
    let imrd = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IMRD);
    dev_err((*sdev).dev, b"error: ipc host -> DSP: pending %s complete %s raw 0x%8.8x\n\0".as_ptr() as *const c_char, str_yes_no(panic & SHIM_IPCX_BUSY), str_yes_no(panic & SHIM_IPCX_DONE), panic);
    dev_err((*sdev).dev, b"error: mask host: pending %s complete %s raw 0x%8.8x\n\0".as_ptr() as *const c_char, str_yes_no(imrx & SHIM_IMRX_BUSY), str_yes_no(imrx & SHIM_IMRX_DONE), imrx);
    dev_err((*sdev).dev, b"error: ipc DSP -> host: pending %s complete %s raw 0x%8.8x\n\0".as_ptr() as *const c_char, str_yes_no(status & SHIM_IPCD_BUSY), str_yes_no(status & SHIM_IPCD_DONE), status);
    dev_err((*sdev).dev, b"error: mask DSP: pending %s complete %s raw 0x%8.8x\n\0".as_ptr() as *const c_char, str_yes_no(imrd & SHIM_IMRD_BUSY), str_yes_no(imrd & SHIM_IMRD_DONE), imrd);
}

unsafe extern "C" fn bdw_irq_handler(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let isr = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_ISRX);
    let mut ret = IRQ_NONE;
    if (isr & (SHIM_ISRX_DONE | SHIM_ISRX_BUSY)) != 0 {
        ret = IRQ_WAKE_THREAD;
    }
    ret
}

unsafe extern "C" fn bdw_irq_thread(_irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let imrx = snd_sof_dsp_read64(sdev, BDW_DSP_BAR, SHIM_IMRX);
    let ipcx = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IPCX);

    if (ipcx & SHIM_IPCX_DONE) != 0 && (imrx & SHIM_IMRX_DONE) == 0 {
        snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IMRX, SHIM_IMRX_DONE, SHIM_IMRX_DONE);
        /* C used guard(spinlock_irq)(&sdev->ipc_lock) for this critical section. */
        snd_sof_ipc_process_reply(sdev, ipcx);
        bdw_dsp_done(sdev);
    }

    let ipcd = snd_sof_dsp_read(sdev, BDW_DSP_BAR, SHIM_IPCD);
    if (ipcd & SHIM_IPCD_BUSY) != 0 && (imrx & SHIM_IMRX_BUSY) == 0 {
        snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IMRX, SHIM_IMRX_BUSY, SHIM_IMRX_BUSY);
        if (ipcd & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
            snd_sof_dsp_panic(sdev, BDW_PANIC_OFFSET(ipcx) + MBOX_OFFSET, true);
        } else {
            snd_sof_ipc_msgs_rx(sdev);
        }
        bdw_host_done(sdev);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn bdw_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int {
    sof_mailbox_write_fn(sdev, (*sdev).host_box.offset, (*msg).msg_data, (*msg).msg_size);
    snd_sof_dsp_write(sdev, BDW_DSP_BAR, SHIM_IPCX, SHIM_IPCX_BUSY);
    0
}

unsafe extern "C" fn bdw_get_mailbox_offset(_sdev: *mut snd_sof_dev) -> c_int {
    MBOX_OFFSET as c_int
}

unsafe extern "C" fn bdw_get_window_offset(_sdev: *mut snd_sof_dev, _id: u32) -> c_int {
    MBOX_OFFSET as c_int
}

unsafe extern "C" fn bdw_host_done(sdev: *mut snd_sof_dev) {
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IPCD, SHIM_IPCD_BUSY | SHIM_IPCD_DONE, SHIM_IPCD_DONE);
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IMRX, SHIM_IMRX_BUSY, 0);
}

unsafe extern "C" fn bdw_dsp_done(sdev: *mut snd_sof_dev) {
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IPCX, SHIM_IPCX_DONE, 0);
    snd_sof_dsp_update_bits_unlocked(sdev, BDW_DSP_BAR, SHIM_IMRX, SHIM_IMRX_DONE, 0);
}

unsafe extern "C" fn bdw_probe(sdev: *mut snd_sof_dev) -> c_int {
    let pdata = (*sdev).pdata;
    let desc = (*pdata).desc;
    let pdev = to_platform_device((*sdev).dev);
    let chip = get_chip_info((*sdev).pdata);
    if chip.is_null() {
        dev_err((*sdev).dev, b"error: no such device supported\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    (*sdev).num_cores = (*chip).cores_num;

    let mut mmio = platform_get_resource(pdev, IORESOURCE_MEM, (*desc).resindex_lpe_base);
    let (mut base, mut size): (u32, u32);
    if !mmio.is_null() {
        base = (*mmio).start;
        size = resource_size(mmio);
    } else {
        dev_err((*sdev).dev, b"error: failed to get LPE base at idx %d\n\0".as_ptr() as *const c_char, (*desc).resindex_lpe_base);
        return -EINVAL;
    }

    dev_dbg((*sdev).dev, b"LPE PHY base at 0x%x size 0x%x\0".as_ptr() as *const c_char, base, size);
    (*sdev).bar[BDW_DSP_BAR as usize] = devm_ioremap((*sdev).dev, base, size);
    if (*sdev).bar[BDW_DSP_BAR as usize].is_null() {
        dev_err((*sdev).dev, b"error: failed to ioremap LPE base 0x%x size 0x%x\n\0".as_ptr() as *const c_char, base, size);
        return -ENODEV;
    }
    dev_dbg((*sdev).dev, b"LPE VADDR %p\n\0".as_ptr() as *const c_char, (*sdev).bar[BDW_DSP_BAR as usize]);

    (*sdev).mmio_bar = BDW_DSP_BAR;
    (*sdev).mailbox_bar = BDW_DSP_BAR;
    (*sdev).dsp_oops_offset = MBOX_OFFSET;

    mmio = platform_get_resource(pdev, IORESOURCE_MEM, (*desc).resindex_pcicfg_base);
    if !mmio.is_null() {
        base = (*mmio).start;
        size = resource_size(mmio);
    } else {
        dev_err((*sdev).dev, b"error: failed to get PCI base at idx %d\n\0".as_ptr() as *const c_char, (*desc).resindex_pcicfg_base);
        return -ENODEV;
    }

    dev_dbg((*sdev).dev, b"PCI base at 0x%x size 0x%x\0".as_ptr() as *const c_char, base, size);
    (*sdev).bar[BDW_PCI_BAR as usize] = devm_ioremap((*sdev).dev, base, size);
    if (*sdev).bar[BDW_PCI_BAR as usize].is_null() {
        dev_err((*sdev).dev, b"error: failed to ioremap PCI base 0x%x size 0x%x\n\0".as_ptr() as *const c_char, base, size);
        return -ENODEV;
    }
    dev_dbg((*sdev).dev, b"PCI VADDR %p\n\0".as_ptr() as *const c_char, (*sdev).bar[BDW_PCI_BAR as usize]);

    (*sdev).ipc_irq = platform_get_irq(pdev, (*desc).irqindex_host_ipc);
    if (*sdev).ipc_irq < 0 {
        return (*sdev).ipc_irq;
    }

    dev_dbg((*sdev).dev, b"using IRQ %d\n\0".as_ptr() as *const c_char, (*sdev).ipc_irq);
    let mut ret = devm_request_threaded_irq(
        (*sdev).dev,
        (*sdev).ipc_irq,
        Some(bdw_irq_handler),
        Some(bdw_irq_thread),
        IRQF_SHARED,
        b"AudioDSP\0".as_ptr() as *const c_char,
        sdev as *mut c_void,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"error: failed to register IRQ %d\n\0".as_ptr() as *const c_char, (*sdev).ipc_irq);
        return ret;
    }

    ret = bdw_set_dsp_D0(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, b"error: failed to set DSP D0\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = dma_coerce_mask_and_coherent((*sdev).dev, DMA_BIT_MASK(31));
    if ret < 0 {
        dev_err((*sdev).dev, b"error: failed to set DMA mask %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*sdev).dsp_box.offset = MBOX_OFFSET;
    ret
}

unsafe extern "C" fn bdw_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach {
    let sof_pdata = (*sdev).pdata;
    let desc = (*sof_pdata).desc;
    let mach = snd_soc_acpi_find_machine((*desc).machines);
    if mach.is_null() {
        dev_warn((*sdev).dev, b"warning: No matching ASoC machine driver found\n\0".as_ptr() as *const c_char);
        return core::ptr::null_mut();
    }

    (*sof_pdata).tplg_filename = (*mach).sof_tplg_filename;
    (*mach).mach_params.acpi_ipc_irq_index = (*desc).irqindex_host_ipc;
    mach
}

unsafe extern "C" fn bdw_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev) {
    let pdata = (*sdev).pdata;
    let desc = (*pdata).desc;
    let mach_params = &mut (*mach).mach_params;
    mach_params.platform = dev_name((*sdev).dev);
    mach_params.num_dai_drivers = (*(*desc).ops).num_drv as c_int;
    mach_params.dai_drivers = (*(*desc).ops).drv;
}

static mut bdw_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"ssp0-port\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
    },
    snd_soc_dai_driver {
        name: b"ssp1-port\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
        capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 8 },
    },
];

static sof_bdw_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    probe: Some(bdw_probe),
    run: Some(bdw_run),
    reset: Some(bdw_reset),
    block_read: unsafe { &sof_block_read as *const _ as *const c_void },
    block_write: unsafe { &sof_block_write as *const _ as *const c_void },
    mailbox_read: unsafe { &sof_mailbox_read as *const _ as *const c_void },
    mailbox_write: unsafe { &sof_mailbox_write as *const _ as *const c_void },
    send_msg: Some(bdw_send_msg),
    get_mailbox_offset: Some(bdw_get_mailbox_offset),
    get_window_offset: Some(bdw_get_window_offset),
    ipc_msg_data: unsafe { &sof_ipc_msg_data as *const _ as *const c_void },
    set_stream_data_offset: unsafe { &sof_set_stream_data_offset as *const _ as *const c_void },
    machine_select: Some(bdw_machine_select),
    machine_register: unsafe { &sof_machine_register as *const _ as *const c_void },
    machine_unregister: unsafe { &sof_machine_unregister as *const _ as *const c_void },
    set_mach_params: Some(bdw_set_mach_params),
    debug_map: bdw_debugfs.as_ptr(),
    debug_map_count: bdw_debugfs.len(),
    dbg_dump: Some(bdw_dump),
    debugfs_add_region_item: unsafe { &snd_sof_debugfs_add_region_item_iomem as *const _ as *const c_void },
    pcm_open: unsafe { &sof_stream_pcm_open as *const _ as *const c_void },
    pcm_close: unsafe { &sof_stream_pcm_close as *const _ as *const c_void },
    load_firmware: unsafe { &snd_sof_load_firmware_memcpy as *const _ as *const c_void },
    drv: unsafe { bdw_dai.as_ptr() as *mut snd_soc_dai_driver },
    num_drv: 2,
    hw_info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_BATCH,
    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops as *const _ as *const c_void },
};

static bdw_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 1,
    host_managed_cores_mask: 1,
    hw_ip_version: SOF_INTEL_BROADWELL,
};

static sof_acpi_broadwell_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_broadwell_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: 1,
    resindex_imr_base: -1,
    irqindex_host_ipc: 0,
    chip_info: &bdw_chip_info,
    ipc_supported_mask: unsafe { BIT(SOF_IPC_TYPE_3) },
    ipc_default: SOF_IPC_TYPE_3 as c_int,
    default_fw_path: [core::ptr::null(), core::ptr::null(), core::ptr::null(), b"intel/sof\0".as_ptr() as *const c_char],
    default_tplg_path: [core::ptr::null(), core::ptr::null(), core::ptr::null(), b"intel/sof-tplg\0".as_ptr() as *const c_char],
    default_fw_filename: [core::ptr::null(), core::ptr::null(), core::ptr::null(), b"sof-bdw.ri\0".as_ptr() as *const c_char],
    nocodec_tplg_filename: b"sof-bdw-nocodec.tplg\0".as_ptr() as *const c_char,
    ops: &sof_bdw_ops,
};

static sof_broadwell_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [b'I' as c_char, b'N' as c_char, b'T' as c_char, b'3' as c_char, b'4' as c_char, b'3' as c_char, b'8' as c_char, 0],
        driver_data: &sof_acpi_broadwell_desc as *const sof_dev_desc as c_ulong,
    },
    acpi_device_id {
        id: [0; 8],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, sof_broadwell_match); */

unsafe extern "C" fn sof_broadwell_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let id = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
    if id.is_null() {
        return -ENODEV;
    }

    let ret = snd_intel_acpi_dsp_driver_probe(dev, (*id).id.as_ptr());
    if ret != SND_INTEL_DSP_DRIVER_ANY && ret != SND_INTEL_DSP_DRIVER_SOF {
        dev_dbg(dev, b"SOF ACPI driver not selected, aborting probe\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    let desc = (*id).driver_data as *const sof_dev_desc;
    sof_acpi_probe(pdev, desc)
}

static mut snd_sof_acpi_intel_bdw_driver: platform_driver = platform_driver {
    probe: Some(sof_broadwell_probe),
    remove: unsafe { &sof_acpi_remove as *const _ as *const c_void },
    driver: platform_driver_inner {
        name: b"sof-audio-acpi-intel-bdw\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&sof_acpi_pm as *const _ as *const c_void) },
        acpi_match_table: sof_broadwell_match.as_ptr(),
    },
};
/* module_platform_driver(snd_sof_acpi_intel_bdw_driver); */

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF support for Broadwell platforms"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_ACPI_DEV"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
