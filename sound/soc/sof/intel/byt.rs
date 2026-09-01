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
 * Hardware interface for audio DSP on Baytrail, Braswell and Cherrytrail.
 */

// C include dependencies translated as external Rust dependencies:
// <linux/module.h>, <sound/sof.h>, <sound/sof/xtensa.h>,
// <sound/soc-acpi.h>, <sound/soc-acpi-intel-match.h>,
// <sound/intel-dsp-config.h>, ../ops.h, atom.h, shim.h,
// ../sof-acpi-dev.h, ../sof-audio.h, ../../intel/common/soc-intel-quirks.h.

unsafe extern "C" {
    static snd_soc_acpi_intel_baytrail_machines: *const snd_soc_acpi_mach;
    static snd_soc_acpi_intel_cherrytrail_machines: *const snd_soc_acpi_mach;
    static atom_dai: *const snd_soc_dai_driver;
    static sof_xtensa_arch_ops: snd_sof_dsp_arch_ops;
    static sof_acpi_pm: dev_pm_ops;

    fn snd_sof_dsp_update_bits64(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u64,
        value: u64,
    ) -> i32;
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_intel_dsp_desc;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: u32,
        num: i32,
    ) -> *mut resource;
    fn resource_size(res: *mut resource) -> u32;
    fn devm_ioremap(dev: *mut device, offset: u32, size: u32) -> *mut core::ffi::c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: i32) -> i32;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: i32,
        handler: irq_handler_t,
        thread_fn: irq_handler_t,
        irqflags: u64,
        devname: *const i8,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;

    fn atom_irq_handler(irq: i32, context: *mut core::ffi::c_void) -> irqreturn_t;
    fn atom_irq_thread(irq: i32, context: *mut core::ffi::c_void) -> irqreturn_t;
    fn atom_run(sdev: *mut snd_sof_dev) -> i32;
    fn atom_reset(sdev: *mut snd_sof_dev) -> i32;
    fn sof_block_read(
        sdev: *mut snd_sof_dev,
        bar: i32,
        offset: u32,
        dest: *mut core::ffi::c_void,
        size: usize,
    );
    fn sof_block_write(
        sdev: *mut snd_sof_dev,
        bar: i32,
        offset: u32,
        src: *const core::ffi::c_void,
        size: usize,
    );
    fn sof_mailbox_read(
        sdev: *mut snd_sof_dev,
        offset: u32,
        dest: *mut core::ffi::c_void,
        size: usize,
    );
    fn sof_mailbox_write(
        sdev: *mut snd_sof_dev,
        offset: u32,
        src: *const core::ffi::c_void,
        size: usize,
    );
    fn atom_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> i32;
    fn atom_get_mailbox_offset(sdev: *mut snd_sof_dev) -> u32;
    fn atom_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> u32;
    fn sof_ipc_msg_data(
        sdev: *mut snd_sof_dev,
        msg_bytes: *mut snd_sof_ipc_msg,
        p: *mut core::ffi::c_void,
        sz: usize,
    );
    fn sof_set_stream_data_offset(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, posn_offset: usize);
    fn atom_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    fn sof_machine_register(sdev: *mut snd_sof_dev, pdata: *mut core::ffi::c_void) -> i32;
    fn sof_machine_unregister(sdev: *mut snd_sof_dev, pdata: *mut core::ffi::c_void);
    fn atom_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev);
    fn atom_dump(sdev: *mut snd_sof_dev, flags: u32);
    fn snd_sof_debugfs_add_region_item_iomem(
        sdev: *mut snd_sof_dev,
        item: *mut snd_sof_debugfs_map,
    ) -> i32;
    fn sof_stream_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> i32;
    fn sof_stream_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> i32;
    fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> i32;
    fn acpi_match_device(
        ids: *const acpi_device_id,
        dev: *mut device,
    ) -> *const acpi_device_id;
    fn snd_intel_acpi_dsp_driver_probe(dev: *mut device, acpi_id: *const i8) -> i32;
    fn soc_intel_is_byt_cr(pdev: *mut platform_device) -> bool;
    fn sof_acpi_probe(pdev: *mut platform_device, desc: *const sof_dev_desc) -> i32;
    fn sof_acpi_remove(pdev: *mut platform_device) -> i32;

    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

static byt_debugfs: [snd_sof_debugfs_map; 8] = [
    snd_sof_debugfs_map { name: c"dmac0".as_ptr(), bar: DSP_BAR, offset: DMAC0_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"dmac1".as_ptr(), bar: DSP_BAR, offset: DMAC1_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp0".as_ptr(), bar: DSP_BAR, offset: SSP0_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp1".as_ptr(), bar: DSP_BAR, offset: SSP1_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp2".as_ptr(), bar: DSP_BAR, offset: SSP2_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"iram".as_ptr(), bar: DSP_BAR, offset: IRAM_OFFSET, size: IRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: c"dram".as_ptr(), bar: DSP_BAR, offset: DRAM_OFFSET, size: DRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: c"shim".as_ptr(), bar: DSP_BAR, offset: SHIM_OFFSET, size: SHIM_SIZE_BYT, access: SOF_DEBUGFS_ACCESS_ALWAYS },
];

static cht_debugfs: [snd_sof_debugfs_map; 12] = [
    snd_sof_debugfs_map { name: c"dmac0".as_ptr(), bar: DSP_BAR, offset: DMAC0_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"dmac1".as_ptr(), bar: DSP_BAR, offset: DMAC1_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"dmac2".as_ptr(), bar: DSP_BAR, offset: DMAC2_OFFSET, size: DMAC_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp0".as_ptr(), bar: DSP_BAR, offset: SSP0_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp1".as_ptr(), bar: DSP_BAR, offset: SSP1_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp2".as_ptr(), bar: DSP_BAR, offset: SSP2_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp3".as_ptr(), bar: DSP_BAR, offset: SSP3_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp4".as_ptr(), bar: DSP_BAR, offset: SSP4_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"ssp5".as_ptr(), bar: DSP_BAR, offset: SSP5_OFFSET, size: SSP_SIZE, access: SOF_DEBUGFS_ACCESS_ALWAYS },
    snd_sof_debugfs_map { name: c"iram".as_ptr(), bar: DSP_BAR, offset: IRAM_OFFSET, size: IRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: c"dram".as_ptr(), bar: DSP_BAR, offset: DRAM_OFFSET, size: DRAM_SIZE, access: SOF_DEBUGFS_ACCESS_D0_ONLY },
    snd_sof_debugfs_map { name: c"shim".as_ptr(), bar: DSP_BAR, offset: SHIM_OFFSET, size: SHIM_SIZE_CHT, access: SOF_DEBUGFS_ACCESS_ALWAYS },
];

unsafe fn byt_reset_dsp_disable_int(sdev: *mut snd_sof_dev) {
    /* Disable Interrupt from both sides */
    unsafe {
        snd_sof_dsp_update_bits64(sdev, DSP_BAR, SHIM_IMRX, 0x3, 0x3);
        snd_sof_dsp_update_bits64(sdev, DSP_BAR, SHIM_IMRD, 0x3, 0x3);

        /* Put DSP into reset, set reset vector */
        snd_sof_dsp_update_bits64(
            sdev,
            DSP_BAR,
            SHIM_CSR,
            SHIM_BYT_CSR_RST | SHIM_BYT_CSR_VECTOR_SEL,
            SHIM_BYT_CSR_RST | SHIM_BYT_CSR_VECTOR_SEL,
        );
    }
}

unsafe extern "C" fn byt_suspend(sdev: *mut snd_sof_dev, _target_state: u32) -> i32 {
    unsafe {
        byt_reset_dsp_disable_int(sdev);
    }

    0
}

unsafe extern "C" fn byt_resume(sdev: *mut snd_sof_dev) -> i32 {
    /* enable BUSY and disable DONE Interrupt by default */
    unsafe {
        snd_sof_dsp_update_bits64(
            sdev,
            DSP_BAR,
            SHIM_IMRX,
            SHIM_IMRX_BUSY | SHIM_IMRX_DONE,
            SHIM_IMRX_DONE,
        );
    }

    0
}

unsafe extern "C" fn byt_remove(sdev: *mut snd_sof_dev) {
    unsafe {
        byt_reset_dsp_disable_int(sdev);
    }
}

unsafe extern "C" fn byt_acpi_probe(sdev: *mut snd_sof_dev) -> i32 {
    let pdata: *mut snd_sof_pdata = unsafe { (*sdev).pdata };
    let desc: *const sof_dev_desc = unsafe { (*pdata).desc };
    let pdev: *mut platform_device = unsafe { to_platform_device((*sdev).dev) };
    let chip: *const sof_intel_dsp_desc;
    let mut mmio: *mut resource;
    let mut base: u32;
    let mut size: u32;
    let mut ret: i32;

    unsafe {
        chip = get_chip_info((*sdev).pdata);
        if chip.is_null() {
            dev_err((*sdev).dev, c"error: no such device supported\n".as_ptr());
            return -EIO;
        }

        (*sdev).num_cores = (*chip).cores_num;

        /* DSP DMA can only access low 31 bits of host memory */
        ret = dma_coerce_mask_and_coherent((*sdev).dev, DMA_BIT_MASK(31));
        if ret < 0 {
            dev_err((*sdev).dev, c"error: failed to set DMA mask %d\n".as_ptr(), ret);
            return ret;
        }

        /* LPE base */
        mmio = platform_get_resource(pdev, IORESOURCE_MEM, (*desc).resindex_lpe_base);
        if !mmio.is_null() {
            base = (*mmio).start as u32;
            size = resource_size(mmio);
        } else {
            dev_err(
                (*sdev).dev,
                c"error: failed to get LPE base at idx %d\n".as_ptr(),
                (*desc).resindex_lpe_base,
            );
            return -EINVAL;
        }

        dev_dbg((*sdev).dev, c"LPE PHY base at 0x%x size 0x%x".as_ptr(), base, size);
        (*sdev).bar[DSP_BAR as usize] = devm_ioremap((*sdev).dev, base, size);
        if (*sdev).bar[DSP_BAR as usize].is_null() {
            dev_err(
                (*sdev).dev,
                c"error: failed to ioremap LPE base 0x%x size 0x%x\n".as_ptr(),
                base,
                size,
            );
            return -ENODEV;
        }
        dev_dbg((*sdev).dev, c"LPE VADDR %p\n".as_ptr(), (*sdev).bar[DSP_BAR as usize]);

        /* TODO: add offsets */
        (*sdev).mmio_bar = DSP_BAR;
        (*sdev).mailbox_bar = DSP_BAR;

        /* IMR base - optional */
        if (*desc).resindex_imr_base == -1 {
            /* goto irq */
        } else {
            mmio = platform_get_resource(pdev, IORESOURCE_MEM, (*desc).resindex_imr_base);
            if !mmio.is_null() {
                base = (*mmio).start as u32;
                size = resource_size(mmio);
            } else {
                dev_err(
                    (*sdev).dev,
                    c"error: failed to get IMR base at idx %d\n".as_ptr(),
                    (*desc).resindex_imr_base,
                );
                return -ENODEV;
            }

            /* some BIOSes don't map IMR */
            if base == 0x55aa55aa || base == 0x0 {
                dev_info((*sdev).dev, c"IMR not set by BIOS. Ignoring\n".as_ptr());
            } else {
                dev_dbg((*sdev).dev, c"IMR base at 0x%x size 0x%x".as_ptr(), base, size);
                (*sdev).bar[IMR_BAR as usize] = devm_ioremap((*sdev).dev, base, size);
                if (*sdev).bar[IMR_BAR as usize].is_null() {
                    dev_err(
                        (*sdev).dev,
                        c"error: failed to ioremap IMR base 0x%x size 0x%x\n".as_ptr(),
                        base,
                        size,
                    );
                    return -ENODEV;
                }
                dev_dbg((*sdev).dev, c"IMR VADDR %p\n".as_ptr(), (*sdev).bar[IMR_BAR as usize]);
            }
        }

        /* register our IRQ */
        (*sdev).ipc_irq = platform_get_irq(pdev, (*desc).irqindex_host_ipc);
        if (*sdev).ipc_irq < 0 {
            return (*sdev).ipc_irq;
        }

        dev_dbg((*sdev).dev, c"using IRQ %d\n".as_ptr(), (*sdev).ipc_irq);
        ret = devm_request_threaded_irq(
            (*sdev).dev,
            (*sdev).ipc_irq,
            Some(atom_irq_handler),
            Some(atom_irq_thread),
            IRQF_SHARED,
            c"AudioDSP".as_ptr(),
            sdev as *mut core::ffi::c_void,
        );
        if ret < 0 {
            dev_err((*sdev).dev, c"error: failed to register IRQ %d\n".as_ptr(), (*sdev).ipc_irq);
            return ret;
        }

        /* enable BUSY and disable DONE Interrupt by default */
        snd_sof_dsp_update_bits64(
            sdev,
            DSP_BAR,
            SHIM_IMRX,
            SHIM_IMRX_BUSY | SHIM_IMRX_DONE,
            SHIM_IMRX_DONE,
        );

        /* set default mailbox offset for FW ready message */
        (*sdev).dsp_box.offset = MBOX_OFFSET;
    }

    ret
}

/* baytrail ops */
static sof_byt_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* device init */
    probe: Some(byt_acpi_probe),
    remove: Some(byt_remove),

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
    debug_map: byt_debugfs.as_ptr(),
    debug_map_count: byt_debugfs.len(),
    dbg_dump: Some(atom_dump),
    debugfs_add_region_item: Some(snd_sof_debugfs_add_region_item_iomem),

    /* stream callbacks */
    pcm_open: Some(sof_stream_pcm_open),
    pcm_close: Some(sof_stream_pcm_close),

    /*Firmware loading */
    load_firmware: Some(snd_sof_load_firmware_memcpy),

    /* PM */
    suspend: Some(byt_suspend),
    resume: Some(byt_resume),

    /* DAI drivers */
    drv: unsafe { atom_dai },
    num_drv: 3, /* we have only 3 SSPs on byt*/

    /* ALSA HW info flags */
    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_BATCH,

    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops },
};

static byt_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 1,
    host_managed_cores_mask: 1,
    hw_ip_version: SOF_INTEL_BAYTRAIL,
};

/* cherrytrail and braswell ops */
static sof_cht_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* device init */
    probe: Some(byt_acpi_probe),
    remove: Some(byt_remove),

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
    debug_map: cht_debugfs.as_ptr(),
    debug_map_count: cht_debugfs.len(),
    dbg_dump: Some(atom_dump),
    debugfs_add_region_item: Some(snd_sof_debugfs_add_region_item_iomem),

    /* stream callbacks */
    pcm_open: Some(sof_stream_pcm_open),
    pcm_close: Some(sof_stream_pcm_close),

    /*Firmware loading */
    load_firmware: Some(snd_sof_load_firmware_memcpy),

    /* PM */
    suspend: Some(byt_suspend),
    resume: Some(byt_resume),

    /* DAI drivers */
    drv: unsafe { atom_dai },
    /* all 6 SSPs may be available for cherrytrail */
    num_drv: 6,

    /* ALSA HW info flags */
    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_BATCH,

    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops },
};

static cht_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
    cores_num: 1,
    host_managed_cores_mask: 1,
    hw_ip_version: SOF_INTEL_BAYTRAIL,
};

/* BYTCR uses different IRQ index */
static sof_acpi_baytrailcr_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_baytrail_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: 1,
    resindex_imr_base: 2,
    irqindex_host_ipc: 0,
    chip_info: &byt_chip_info,
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof".as_ptr();
        a
    },
    default_tplg_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof-tplg".as_ptr();
        a
    },
    default_fw_filename: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"sof-byt.ri".as_ptr();
        a
    },
    nocodec_tplg_filename: c"sof-byt-nocodec.tplg".as_ptr(),
    ops: &sof_byt_ops,
};

static sof_acpi_baytrail_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_baytrail_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: 1,
    resindex_imr_base: 2,
    irqindex_host_ipc: 5,
    chip_info: &byt_chip_info,
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof".as_ptr();
        a
    },
    default_tplg_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof-tplg".as_ptr();
        a
    },
    default_fw_filename: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"sof-byt.ri".as_ptr();
        a
    },
    nocodec_tplg_filename: c"sof-byt-nocodec.tplg".as_ptr(),
    ops: &sof_byt_ops,
};

static sof_acpi_cherrytrail_desc: sof_dev_desc = sof_dev_desc {
    machines: unsafe { snd_soc_acpi_intel_cherrytrail_machines },
    resindex_lpe_base: 0,
    resindex_pcicfg_base: 1,
    resindex_imr_base: 2,
    irqindex_host_ipc: 5,
    chip_info: &cht_chip_info,
    ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
    ipc_default: SOF_IPC_TYPE_3,
    default_fw_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof".as_ptr();
        a
    },
    default_tplg_path: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"intel/sof-tplg".as_ptr();
        a
    },
    default_fw_filename: {
        let mut a = [core::ptr::null(); SOF_IPC_TYPE_COUNT];
        a[SOF_IPC_TYPE_3 as usize] = c"sof-cht.ri".as_ptr();
        a
    },
    nocodec_tplg_filename: c"sof-cht-nocodec.tplg".as_ptr(),
    ops: &sof_cht_ops,
};

static sof_baytrail_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: *b"80860F28\0\0\0\0\0\0\0\0",
        driver_data: &sof_acpi_baytrail_desc as *const sof_dev_desc as usize as core::ffi::c_ulong,
    },
    acpi_device_id {
        id: *b"808622A8\0\0\0\0\0\0\0\0",
        driver_data: &sof_acpi_cherrytrail_desc as *const sof_dev_desc as usize as core::ffi::c_ulong,
    },
    acpi_device_id {
        id: [0; ACPI_ID_LEN],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, sof_baytrail_match);

unsafe extern "C" fn sof_baytrail_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let mut desc: *const sof_dev_desc;
    let id: *const acpi_device_id;
    let ret: i32;

    unsafe {
        id = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
        if id.is_null() {
            return -ENODEV;
        }

        ret = snd_intel_acpi_dsp_driver_probe(dev, (*id).id.as_ptr());
        if ret != SND_INTEL_DSP_DRIVER_ANY && ret != SND_INTEL_DSP_DRIVER_SOF {
            dev_dbg(dev, c"SOF ACPI driver not selected, aborting probe\n".as_ptr());
            return -ENODEV;
        }

        desc = (*id).driver_data as usize as *const sof_dev_desc;
        if core::ptr::eq(desc, &sof_acpi_baytrail_desc) && soc_intel_is_byt_cr(pdev) {
            desc = &sof_acpi_baytrailcr_desc;
        }

        sof_acpi_probe(pdev, desc)
    }
}

/* acpi_driver definition */
static mut snd_sof_acpi_intel_byt_driver: platform_driver = platform_driver {
    probe: Some(sof_baytrail_probe),
    remove: Some(sof_acpi_remove),
    driver: device_driver {
        name: c"sof-audio-acpi-intel-byt".as_ptr(),
        pm: unsafe { pm_ptr(&sof_acpi_pm) },
        acpi_match_table: sof_baytrail_match.as_ptr(),
    },
};
// module_platform_driver(snd_sof_acpi_intel_byt_driver);

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for Baytrail/Cherrytrail");
// MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");
// MODULE_IMPORT_NS("SND_SOC_SOF_ACPI_DEV");
// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_ATOM_HIFI_EP");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
