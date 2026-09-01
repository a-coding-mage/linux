// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// C includes translated as external dependencies:
// linux/module.h, linux/pci.h, sound/soc-acpi.h,
// sound/soc-acpi-intel-match.h, sound/sof.h, ../ops.h, atom.h,
// ../sof-pci-dev.h, ../sof-audio.h, shim.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static atom_dai: *const c_void;
    static sof_xtensa_arch_ops: c_void;
    static sof_pci_pm: c_void;

    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pci_resource_start(pdev: *mut pci_dev, bar: c_int) -> u32;
    fn pci_resource_len(pdev: *mut pci_dev, bar: c_int) -> u32;
    fn devm_ioremap(dev: *mut device, offset: u32, size: u32) -> *mut c_void;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn snd_sof_dsp_update_bits64(
        sdev: *mut snd_sof_dev,
        bar: c_uint,
        offset: c_uint,
        mask: u64,
        value: u64,
    );

    fn atom_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn atom_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn atom_run(sdev: *mut snd_sof_dev) -> c_int;
    fn atom_reset(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_block_read(
        sdev: *mut snd_sof_dev,
        bar: c_int,
        offset: u32,
        dest: *mut c_void,
        bytes: usize,
    );
    fn sof_block_write(
        sdev: *mut snd_sof_dev,
        bar: c_int,
        offset: u32,
        src: *mut c_void,
        bytes: usize,
    );
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: usize);
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: usize);
    fn atom_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    fn atom_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
    fn atom_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
    fn sof_ipc_msg_data(
        sdev: *mut snd_sof_dev,
        msg: *mut snd_sof_ipc_msg,
        data: *mut c_void,
        sz: usize,
    );
    fn sof_set_stream_data_offset(sdev: *mut snd_sof_dev, substream: *mut c_void, posn_offset: usize);
    fn atom_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    fn sof_machine_register(sdev: *mut snd_sof_dev, pdata: *mut c_void) -> c_int;
    fn sof_machine_unregister(sdev: *mut snd_sof_dev, pdata: *mut c_void);
    fn atom_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev);
    fn atom_dump(sdev: *mut snd_sof_dev, flags: u32);
    fn snd_sof_debugfs_add_region_item_iomem(
        sdev: *mut snd_sof_dev,
        map: *const snd_sof_debugfs_map,
    ) -> c_int;
    fn sof_stream_pcm_open(sdev: *mut snd_sof_dev, substream: *mut c_void) -> c_int;
    fn sof_stream_pcm_close(sdev: *mut snd_sof_dev, substream: *mut c_void) -> c_int;
    fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_pci_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
    fn sof_pci_remove(pci: *mut pci_dev);
    fn sof_pci_shutdown(pci: *mut pci_dev);
    fn pm_ptr(pm: *const c_void) -> *const c_void;
}

type c_ulong = u64;
type irqreturn_t = c_int;

const EIO: c_int = 5;
const ENODEV: c_int = 19;
const DSP_BAR: usize = 0;
const IMR_BAR: usize = 1;
const PCI_BAR_SIZE: u32 = 0;
const DMAC0_OFFSET: u32 = 0;
const DMAC1_OFFSET: u32 = 0;
const DMAC_SIZE: u32 = 0;
const SSP0_OFFSET: u32 = 0;
const SSP1_OFFSET: u32 = 0;
const SSP2_OFFSET: u32 = 0;
const SSP_SIZE: u32 = 0;
const IRAM_OFFSET: u32 = 0;
const IRAM_SIZE: u32 = 0;
const DRAM_OFFSET: u32 = 0;
const DRAM_SIZE: u32 = 0;
const SHIM_OFFSET: u32 = 0;
const SHIM_SIZE_BYT: u32 = 0;
const SOF_DEBUGFS_ACCESS_ALWAYS: u32 = 0;
const SOF_DEBUGFS_ACCESS_D0_ONLY: u32 = 0;
const SHIM_IMRX: c_uint = 0;
const SHIM_IMRX_BUSY: u64 = 0;
const SHIM_IMRX_DONE: u64 = 0;
const MBOX_OFFSET: u32 = 0;
const SOF_INTEL_TANGIER: u32 = 0;
const SOF_IPC_TYPE_3: usize = 3;
const SNDRV_PCM_INFO_MMAP: u64 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u64 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u64 = 0;
const SNDRV_PCM_INFO_PAUSE: u64 = 0;
const SNDRV_PCM_INFO_BATCH: u64 = 0;

const fn bit(nr: usize) -> u64 {
    1u64 << nr
}

const fn dma_bit_mask(nr: u32) -> u64 {
    (1u64 << nr) - 1
}

const fn pci_device_data(vendor: u32, device: u32, data: *const sof_dev_desc) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0xffff,
        subdevice: 0xffff,
        class: 0,
        class_mask: 0,
        driver_data: data as usize,
    }
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_uint,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_mailbox {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut device,
    pub num_cores: c_uint,
    pub bar: [*mut c_void; 2],
    pub ipc_irq: c_uint,
    pub dsp_box: snd_sof_mailbox,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub desc: *const sof_dev_desc,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: *const c_char,
    pub drv_name: *const c_char,
    pub sof_tplg_filename: *const c_char,
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: usize,
    pub offset: u32,
    pub size: u32,
    pub access_type: u32,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub block_read: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, u32, *mut c_void, usize)>,
    pub block_write: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_int, u32, *mut c_void, usize)>,
    pub mailbox_read: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32, *mut c_void, usize)>,
    pub mailbox_write: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32, *mut c_void, usize)>,
    pub irq_handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub irq_thread: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int>,
    pub get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    pub ipc_msg_data:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg, *mut c_void, usize)>,
    pub set_stream_data_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void, usize)>,
    pub machine_select: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> *mut snd_soc_acpi_mach>,
    pub machine_register: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub machine_unregister: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void)>,
    pub set_mach_params: Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach, *mut snd_sof_dev)>,
    pub debug_map: *const snd_sof_debugfs_map,
    pub debug_map_count: usize,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub debugfs_add_region_item:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *const snd_sof_debugfs_map) -> c_int>,
    pub pcm_open: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub pcm_close: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void) -> c_int>,
    pub load_firmware: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub drv: *const c_void,
    pub num_drv: c_int,
    pub hw_info: u64,
    pub dsp_arch_ops: *const c_void,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
    pub cores_num: c_uint,
    pub host_managed_cores_mask: c_uint,
    pub hw_ip_version: u32,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *mut snd_soc_acpi_mach,
    pub resindex_lpe_base: c_int,
    pub resindex_pcicfg_base: c_int,
    pub resindex_imr_base: c_int,
    pub irqindex_host_ipc: c_int,
    pub chip_info: *const sof_intel_dsp_desc,
    pub ipc_supported_mask: u64,
    pub ipc_default: usize,
    pub default_fw_path: [*const c_char; 4],
    pub default_tplg_path: [*const c_char; 4],
    pub default_fw_filename: [*const c_char; 4],
    pub nocodec_tplg_filename: *const c_char,
    pub ops: *const snd_sof_dsp_ops,
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
pub struct driver_inner {
    pub pm: *const c_void,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub driver: driver_inner,
}

/* platform specific devices */
static mut sof_tng_machines: [snd_soc_acpi_mach; 2] = [
    snd_soc_acpi_mach {
        id: b"INT343A\0".as_ptr() as *const c_char,
        drv_name: b"edison\0".as_ptr() as *const c_char,
        sof_tplg_filename: b"sof-byt.tplg\0".as_ptr() as *const c_char,
    },
    snd_soc_acpi_mach {
        id: core::ptr::null(),
        drv_name: core::ptr::null(),
        sof_tplg_filename: core::ptr::null(),
    },
];

static tng_debugfs: [snd_sof_debugfs_map; 8] = [
    snd_sof_debugfs_map {
        name: b"dmac0\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: DMAC0_OFFSET,
        size: DMAC_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"dmac1\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: DMAC1_OFFSET,
        size: DMAC_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"ssp0\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: SSP0_OFFSET,
        size: SSP_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"ssp1\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: SSP1_OFFSET,
        size: SSP_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"ssp2\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: SSP2_OFFSET,
        size: SSP_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
    snd_sof_debugfs_map {
        name: b"iram\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: IRAM_OFFSET,
        size: IRAM_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_D0_ONLY,
    },
    snd_sof_debugfs_map {
        name: b"dram\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: DRAM_OFFSET,
        size: DRAM_SIZE,
        access_type: SOF_DEBUGFS_ACCESS_D0_ONLY,
    },
    snd_sof_debugfs_map {
        name: b"shim\0".as_ptr() as *const c_char,
        bar: DSP_BAR,
        offset: SHIM_OFFSET,
        size: SHIM_SIZE_BYT,
        access_type: SOF_DEBUGFS_ACCESS_ALWAYS,
    },
];

unsafe extern "C" fn tangier_pci_probe(sdev: *mut snd_sof_dev) -> c_int {
    let pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let desc: *const sof_dev_desc = (*pdata).desc;
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let mut chip: *const sof_intel_dsp_desc;
    let mut base: u32;
    let mut size: u32;
    let mut ret: c_int;

    chip = get_chip_info((*sdev).pdata);
    if chip.is_null() {
        dev_err(
            (*sdev).dev,
            b"error: no such device supported\n\0".as_ptr() as *const c_char,
        );
        return -EIO;
    }

    (*sdev).num_cores = (*chip).cores_num;

    /* DSP DMA can only access low 31 bits of host memory */
    ret = dma_coerce_mask_and_coherent(&mut (*pci).dev, dma_bit_mask(31));
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed to set DMA mask %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* LPE base */
    base = pci_resource_start(pci, (*desc).resindex_lpe_base) - IRAM_OFFSET;
    size = PCI_BAR_SIZE;

    dev_dbg(
        (*sdev).dev,
        b"LPE PHY base at 0x%x size 0x%x\0".as_ptr() as *const c_char,
        base,
        size,
    );
    (*sdev).bar[DSP_BAR] = devm_ioremap((*sdev).dev, base, size);
    if (*sdev).bar[DSP_BAR].is_null() {
        dev_err(
            (*sdev).dev,
            b"error: failed to ioremap LPE base 0x%x size 0x%x\n\0".as_ptr() as *const c_char,
            base,
            size,
        );
        return -ENODEV;
    }
    dev_dbg(
        (*sdev).dev,
        b"LPE VADDR %p\n\0".as_ptr() as *const c_char,
        (*sdev).bar[DSP_BAR],
    );

    /* IMR base - optional */
    if (*desc).resindex_imr_base == -1 {
        /* goto irq */
    } else {
        base = pci_resource_start(pci, (*desc).resindex_imr_base);
        size = pci_resource_len(pci, (*desc).resindex_imr_base);

        /* some BIOSes don't map IMR */
        if base == 0x55aa55aa || base == 0x0 {
            dev_info(
                (*sdev).dev,
                b"IMR not set by BIOS. Ignoring\n\0".as_ptr() as *const c_char,
            );
        } else {
            dev_dbg(
                (*sdev).dev,
                b"IMR base at 0x%x size 0x%x\0".as_ptr() as *const c_char,
                base,
                size,
            );
            (*sdev).bar[IMR_BAR] = devm_ioremap((*sdev).dev, base, size);
            if (*sdev).bar[IMR_BAR].is_null() {
                dev_err(
                    (*sdev).dev,
                    b"error: failed to ioremap IMR base 0x%x size 0x%x\n\0".as_ptr()
                        as *const c_char,
                    base,
                    size,
                );
                return -ENODEV;
            }
            dev_dbg(
                (*sdev).dev,
                b"IMR VADDR %p\n\0".as_ptr() as *const c_char,
                (*sdev).bar[IMR_BAR],
            );
        }
    }

    /* register our IRQ */
    (*sdev).ipc_irq = (*pci).irq;
    dev_dbg(
        (*sdev).dev,
        b"using IRQ %d\n\0".as_ptr() as *const c_char,
        (*sdev).ipc_irq,
    );
    ret = devm_request_threaded_irq(
        (*sdev).dev,
        (*sdev).ipc_irq,
        atom_irq_handler,
        atom_irq_thread,
        0,
        b"AudioDSP\0".as_ptr() as *const c_char,
        sdev as *mut c_void,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"error: failed to register IRQ %d\n\0".as_ptr() as *const c_char,
            (*sdev).ipc_irq,
        );
        return ret;
    }

    /* enable BUSY and disable DONE Interrupt by default */
    snd_sof_dsp_update_bits64(
        sdev,
        DSP_BAR as c_uint,
        SHIM_IMRX,
        SHIM_IMRX_BUSY | SHIM_IMRX_DONE,
        SHIM_IMRX_DONE,
    );

    /* set default mailbox offset for FW ready message */
    (*sdev).dsp_box.offset = MBOX_OFFSET;

    ret
}

#[no_mangle]
pub static sof_tng_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* device init */
    probe: Some(tangier_pci_probe),

    /* DSP core boot / reset */
    run: Some(atom_run),
    reset: Some(atom_reset),

    /* Register IO uses direct mmio */

    /* Block IO */
    block_read: Some(sof_block_read),
    block_write: Some(sof_block_write),

    /* Mailbox IO */
    mailbox_read: Some(sof_mailbox_read),
    mailbox_write: Some(sof_mailbox_write),

    /* doorbell */
    irq_handler: Some(atom_irq_handler),
    irq_thread: Some(atom_irq_thread),

    /* ipc */
    send_msg: Some(atom_send_msg),
    get_mailbox_offset: Some(atom_get_mailbox_offset),
    get_window_offset: Some(atom_get_window_offset),

    ipc_msg_data: Some(sof_ipc_msg_data),
    set_stream_data_offset: Some(sof_set_stream_data_offset),

    /* machine driver */
    machine_select: Some(atom_machine_select),
    machine_register: Some(sof_machine_register),
    machine_unregister: Some(sof_machine_unregister),
    set_mach_params: Some(atom_set_mach_params),

    /* debug */
    debug_map: tng_debugfs.as_ptr(),
    debug_map_count: tng_debugfs.len(),
    dbg_dump: Some(atom_dump),
    debugfs_add_region_item: Some(snd_sof_debugfs_add_region_item_iomem),

    /* stream callbacks */
    pcm_open: Some(sof_stream_pcm_open),
    pcm_close: Some(sof_stream_pcm_close),

    /*Firmware loading */
    load_firmware: Some(snd_sof_load_firmware_memcpy),

    /* DAI drivers */
    drv: unsafe { atom_dai },
    num_drv: 3, /* we have only 3 SSPs on byt*/

    /* ALSA HW info flags */
    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_BATCH,

    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops as *const c_void },
};

#[no_mangle]
pub static tng_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 1,
    host_managed_cores_mask: 1,
    hw_ip_version: SOF_INTEL_TANGIER,
};

static tng_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { sof_tng_machines.as_mut_ptr() },
    resindex_lpe_base: 3, /* IRAM, but subtract IRAM offset */
    resindex_pcicfg_base: -1,
    resindex_imr_base: 0,
    irqindex_host_ipc: -1,
    chip_info: &tng_chip_info,
    ipc_supported_mask: bit(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof\0".as_ptr() as *const c_char,
    ],
    default_tplg_path: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"intel/sof-tplg\0".as_ptr() as *const c_char,
    ],
    default_fw_filename: [
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null(),
        b"sof-byt.ri\0".as_ptr() as *const c_char,
    ],
    nocodec_tplg_filename: b"sof-byt.tplg\0".as_ptr() as *const c_char,
    ops: &sof_tng_ops,
};

/* PCI IDs */
static sof_pci_ids: [pci_device_id; 2] = [
    pci_device_data(0x8086, 0, &tng_desc),
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
// MODULE_DEVICE_TABLE(pci, sof_pci_ids);

/* pci_driver definition */
static mut snd_sof_pci_intel_tng_driver: pci_driver = pci_driver {
    name: b"sof-audio-pci-intel-tng\0".as_ptr() as *const c_char,
    id_table: sof_pci_ids.as_ptr(),
    probe: Some(sof_pci_probe),
    remove: Some(sof_pci_remove),
    shutdown: Some(sof_pci_shutdown),
    driver: driver_inner {
        pm: unsafe { pm_ptr(&sof_pci_pm as *const c_void) },
    },
};
// module_pci_driver(snd_sof_pci_intel_tng_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for Tangier platforms");
// MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");
// MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_ATOM_HIFI_EP");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
