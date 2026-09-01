// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//	    Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for HDA DSP code loader
 */

/* C dependencies:
 * <linux/firmware.h>
 * <sound/hdaudio_ext.h>
 * <sound/hda_register.h>
 * <sound/sof.h>
 * <sound/sof/ipc4/header.h>
 * "ext_manifest.h"
 * "../ipc4-priv.h"
 * "../ops.h"
 * "../sof-priv.h"
 * "hda.h"
 */
use crate::*;

static mut persistent_cl_buffer: bool = true;
/* module_param(persistent_cl_buffer, bool, 0444);
 * MODULE_PARM_DESC(persistent_cl_buffer, "Persistent Code Loader DMA buffer "
 *		   "(default = Y, use N to force buffer re-allocation)");
 */

unsafe fn hda_ssp_set_cbp_cfp(sdev: *mut snd_sof_dev) {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let mut i: i32;

    /* DSP is powered up, set all SSPs to clock consumer/codec provider mode */
    i = 0;
    while i < (*chip).ssp_count {
        snd_sof_dsp_update_bits_unlocked(
            sdev,
            HDA_DSP_BAR,
            (*chip).ssp_base_offset
                + (i as u32) * SSP_DEV_MEM_SIZE
                + SSP_SSC1_OFFSET,
            SSP_SET_CBP_CFP,
            SSP_SET_CBP_CFP,
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_cl_prepare(
    dev: *mut device,
    format: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
    dmab: *mut snd_dma_buffer,
    persistent_buffer: bool,
    direction: ::core::ffi::c_int,
    is_iccmax: bool,
) -> *mut hdac_ext_stream {
    hda_data_stream_prepare(
        dev,
        format,
        size,
        dmab,
        persistent_buffer,
        direction,
        is_iccmax,
        false,
    )
}
/* EXPORT_SYMBOL_NS(hda_cl_prepare, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

/*
 * first boot sequence has some extra steps.
 * power on all host managed cores and only unstall/run the boot core to boot the
 * DSP then turn off all non boot cores (if any) is powered on.
 */
#[no_mangle]
pub unsafe extern "C" fn cl_dsp_init(
    sdev: *mut snd_sof_dev,
    stream_tag: ::core::ffi::c_int,
    imr_boot: bool,
) -> ::core::ffi::c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let mut status: ::core::ffi::c_uint = 0;
    let target_status: ::core::ffi::c_uint;
    let mut flags: u32;
    let mut ipc_hdr: u32;
    let mut j: u32;
    let mut mask: ::core::ffi::c_ulong;
    let dump_msg: *mut ::core::ffi::c_char;
    let mut ret: ::core::ffi::c_int;
    let mut goto_err: bool;

    /* step 1: power up corex */
    ret = hda_dsp_core_power_up(sdev, (*chip).host_managed_cores_mask);
    if ret < 0 {
        if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
            dev_err((*sdev).dev, c_str!("error: dsp core 0/1 power up failed\n"));
        }
        goto_err = true;
    } else {
        goto_err = false;
    }
    if !goto_err {
        hda_ssp_set_cbp_cfp(sdev);

        /* step 2: Send ROM_CONTROL command (stream_tag is ignored for IMR boot) */
        ipc_hdr = (*chip).ipc_req_mask | HDA_DSP_ROM_IPC_CONTROL;
        if !imr_boot {
            ipc_hdr |= HDA_DSP_ROM_IPC_PURGE_FW | (((stream_tag - 1) as u32) << 9);
        }

        snd_sof_dsp_write(sdev, HDA_DSP_BAR, (*chip).ipc_req, ipc_hdr);

        /* step 3: unset core 0 reset state & unstall/run core 0 */
        ret = hda_dsp_core_run(sdev, (*chip).init_core_mask);
        if ret < 0 {
            if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
                dev_err((*sdev).dev, c_str!("error: dsp core start failed %d\n"), ret);
            }
            ret = -EIO;
            goto_err = true;
        }
    }
    if !goto_err {
        /* step 4: wait for IPC DONE bit from ROM */
        ret = snd_sof_dsp_read_poll_timeout!(
            sdev,
            HDA_DSP_BAR,
            (*chip).ipc_ack,
            status,
            (status & (*chip).ipc_ack_mask) == (*chip).ipc_ack_mask,
            HDA_DSP_REG_POLL_INTERVAL_US,
            HDA_DSP_INIT_TIMEOUT_US
        );

        if ret < 0 {
            if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
                dev_err(
                    (*sdev).dev,
                    c_str!("error: %s: timeout for HIPCIE done\n"),
                    c_str!("cl_dsp_init"),
                );
            }
            goto_err = true;
        }
    }
    if !goto_err {
        /* set DONE bit to clear the reply IPC message */
        snd_sof_dsp_update_bits_forced(
            sdev,
            HDA_DSP_BAR,
            (*chip).ipc_ack,
            (*chip).ipc_ack_mask,
            (*chip).ipc_ack_mask,
        );

        /* step 5: power down cores that are no longer needed */
        ret = hda_dsp_core_reset_power_down(
            sdev,
            (*chip).host_managed_cores_mask & !((*chip).init_core_mask),
        );
        if ret < 0 {
            if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
                dev_err((*sdev).dev, c_str!("error: dsp core x power down failed\n"));
            }
            goto_err = true;
        }
    }
    if !goto_err {
        /* step 6: enable IPC interrupts */
        hda_dsp_ipc_int_enable(sdev);

        /*
         * step 7:
         * - Cold/Full boot: wait for ROM init to proceed to download the firmware
         * - IMR boot: wait for ROM firmware entered (firmware booted up from IMR)
         */
        if imr_boot {
            target_status = FSR_STATE_FW_ENTERED;
        } else {
            target_status = FSR_STATE_INIT_DONE;
        }

        ret = snd_sof_dsp_read_poll_timeout!(
            sdev,
            HDA_DSP_BAR,
            (*chip).rom_status_reg,
            status,
            FSR_TO_STATE_CODE(status) == target_status,
            HDA_DSP_REG_POLL_INTERVAL_US,
            (*chip).rom_init_timeout * USEC_PER_MSEC
        );
        if ret == 0 {
            /* set enabled cores mask and increment ref count for cores in init_core_mask */
            (*sdev).enabled_cores_mask |= (*chip).init_core_mask;
            mask = (*sdev).enabled_cores_mask as ::core::ffi::c_ulong;
            j = 0;
            while j < SOF_MAX_DSP_NUM_CORES {
                if (mask & (1 as ::core::ffi::c_ulong).wrapping_shl(j)) != 0 {
                    (*sdev).dsp_core_ref_count[j as usize] += 1;
                }
                j += 1;
            }
            return 0;
        }

        if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
            dev_err(
                (*sdev).dev,
                c_str!("%s: timeout with rom_status_reg (%#x) read\n"),
                c_str!("cl_dsp_init"),
                (*chip).rom_status_reg,
            );
        }
    }

    flags = SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX | SOF_DBG_DUMP_OPTIONAL;

    /* after max boot attempts make sure that the dump is printed */
    if (*hda).boot_iteration == HDA_FW_BOOT_ATTEMPTS {
        flags &= !SOF_DBG_DUMP_OPTIONAL;
    }

    dump_msg = kasprintf(
        GFP_KERNEL,
        c_str!("Boot iteration failed: %d/%d"),
        (*hda).boot_iteration,
        HDA_FW_BOOT_ATTEMPTS,
    );
    snd_sof_dsp_dbg_dump(sdev, dump_msg, flags);
    hda_dsp_core_reset_power_down(sdev, (*chip).host_managed_cores_mask);

    kfree(dump_msg as *mut ::core::ffi::c_void);
    ret
}
/* EXPORT_SYMBOL_NS(cl_dsp_init, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_cl_trigger(
    dev: *mut device,
    hext_stream: *mut hdac_ext_stream,
    cmd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let sdev = dev_get_drvdata(dev) as *mut snd_sof_dev;
    let hstream = &mut (*hext_stream).hstream as *mut hdac_stream;
    let sd_offset = SOF_STREAM_SD_OFFSET(hstream);
    let hda_stream: *mut sof_intel_hda_stream;

    /* code loader is special case that reuses stream ops */
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            hda_stream = container_of!(hext_stream, sof_intel_hda_stream, hext_stream);
            reinit_completion(&mut (*hda_stream).ioc);

            snd_sof_dsp_update_bits(
                sdev,
                HDA_DSP_HDA_BAR,
                SOF_HDA_INTCTL,
                1 << (*hstream).index,
                1 << (*hstream).index,
            );

            snd_sof_dsp_update_bits(
                sdev,
                HDA_DSP_HDA_BAR,
                sd_offset,
                SOF_HDA_SD_CTL_DMA_START | SOF_HDA_CL_DMA_SD_INT_MASK,
                SOF_HDA_SD_CTL_DMA_START | SOF_HDA_CL_DMA_SD_INT_MASK,
            );

            (*hstream).running = true;
            0
        }
        _ => hda_dsp_stream_trigger(sdev, hext_stream, cmd),
    }
}
/* EXPORT_SYMBOL_NS(hda_cl_trigger, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_cl_cleanup(
    dev: *mut device,
    dmab: *mut snd_dma_buffer,
    persistent_buffer: bool,
    hext_stream: *mut hdac_ext_stream,
    is_iccmax: bool,
) -> ::core::ffi::c_int {
    hda_data_stream_cleanup(dev, dmab, persistent_buffer, hext_stream, is_iccmax, false)
}
/* EXPORT_SYMBOL_NS(hda_cl_cleanup, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

const HDA_CL_DMA_IOC_TIMEOUT_MS: ::core::ffi::c_int = 500;

#[no_mangle]
pub unsafe extern "C" fn hda_cl_copy_fw(
    sdev: *mut snd_sof_dev,
    hext_stream: *mut hdac_ext_stream,
) -> ::core::ffi::c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let mut reg: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int;

    dev_dbg((*sdev).dev, c_str!("Code loader DMA starting\n"));

    ret = hda_cl_trigger((*sdev).dev, hext_stream, SNDRV_PCM_TRIGGER_START);
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("error: DMA trigger start failed\n"));
        return ret;
    }

    dev_dbg((*sdev).dev, c_str!("waiting for FW_ENTERED status\n"));

    status = snd_sof_dsp_read_poll_timeout!(
        sdev,
        HDA_DSP_BAR,
        (*chip).rom_status_reg,
        reg,
        FSR_TO_STATE_CODE(reg) == FSR_STATE_FW_ENTERED,
        HDA_DSP_REG_POLL_INTERVAL_US,
        HDA_DSP_BASEFW_TIMEOUT_US
    );

    /*
     * even in case of errors we still need to stop the DMAs,
     * but we return the initial error should the DMA stop also fail
     */

    if status < 0 {
        dev_err(
            (*sdev).dev,
            c_str!("%s: timeout with rom_status_reg (%#x) read\n"),
            c_str!("hda_cl_copy_fw"),
            (*chip).rom_status_reg,
        );
    } else {
        dev_dbg((*sdev).dev, c_str!("Code loader FW_ENTERED status\n"));
    }

    ret = hda_cl_trigger((*sdev).dev, hext_stream, SNDRV_PCM_TRIGGER_STOP);
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("error: DMA trigger stop failed\n"));
        if status == 0 {
            status = ret;
        }
    } else {
        dev_dbg((*sdev).dev, c_str!("Code loader DMA stopped\n"));
    }

    status
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_cl_boot_firmware_iccmax(
    sdev: *mut snd_sof_dev,
) -> ::core::ffi::c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let iccmax_stream: *mut hdac_ext_stream;
    let mut ret: ::core::ffi::c_int;
    let ret1: ::core::ffi::c_int;
    let original_gb: u8;

    /* save the original LTRP guardband value */
    original_gb =
        snd_sof_dsp_read8(sdev, HDA_DSP_HDA_BAR, HDA_VS_INTEL_LTRP) & HDA_VS_INTEL_LTRP_GB_MASK;

    /*
     * Prepare capture stream for ICCMAX. We do not need to store
     * the data, so use a buffer of PAGE_SIZE for receiving.
     */
    iccmax_stream = hda_cl_prepare(
        (*sdev).dev,
        HDA_CL_STREAM_FORMAT,
        PAGE_SIZE,
        &mut (*hda).iccmax_dmab,
        persistent_cl_buffer,
        SNDRV_PCM_STREAM_CAPTURE,
        true,
    );
    if IS_ERR(iccmax_stream) {
        dev_err((*sdev).dev, c_str!("error: dma prepare for ICCMAX stream failed\n"));
        return PTR_ERR(iccmax_stream);
    }

    ret = hda_dsp_cl_boot_firmware(sdev);

    /*
     * Perform iccmax stream cleanup. This should be done even if firmware loading fails.
     * If the cleanup also fails, we return the initial error
     */
    ret1 = hda_cl_cleanup(
        (*sdev).dev,
        &mut (*hda).iccmax_dmab,
        persistent_cl_buffer,
        iccmax_stream,
        true,
    );
    if ret1 < 0 {
        dev_err((*sdev).dev, c_str!("error: ICCMAX stream cleanup failed\n"));

        /* set return value to indicate cleanup failure */
        if ret == 0 {
            ret = ret1;
        }
    }

    /* restore the original guardband value after FW boot */
    snd_sof_dsp_update8(
        sdev,
        HDA_DSP_HDA_BAR,
        HDA_VS_INTEL_LTRP,
        HDA_VS_INTEL_LTRP_GB_MASK,
        original_gb,
    );

    ret
}
/* EXPORT_SYMBOL_NS(hda_dsp_cl_boot_firmware_iccmax, "SND_SOC_SOF_INTEL_CNL"); */

unsafe fn hda_dsp_boot_imr(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let chip_info: *const sof_intel_dsp_desc;
    let mut ret: ::core::ffi::c_int;

    chip_info = get_chip_info((*sdev).pdata);
    if (*chip_info).cl_init.is_some() {
        ret = ((*chip_info).cl_init.unwrap())(sdev, 0, true);
    } else {
        ret = -EINVAL;
    }

    if ret == 0 {
        hda_sdw_process_wakeen(sdev);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_cl_boot_firmware(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let plat_data = (*sdev).pdata;
    let desc = (*plat_data).desc;
    let chip_info: *const sof_intel_dsp_desc;
    let hext_stream: *mut hdac_ext_stream;
    let mut stripped_firmware: firmware = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int = 0;
    let ret1: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut goto_cleanup: bool;

    if (*hda).imrboot_supported && !(*sdev).first_boot && !(*hda).skip_imr_boot {
        dev_dbg((*sdev).dev, c_str!("IMR restore supported, booting from IMR directly\n"));
        (*hda).boot_iteration = 0;
        ret = hda_dsp_boot_imr(sdev);
        if ret == 0 {
            (*hda).booted_from_imr = true;
            return 0;
        }

        dev_warn((*sdev).dev, c_str!("IMR restore failed, trying to cold boot\n"));
    }

    (*hda).booted_from_imr = false;

    chip_info = (*desc).chip_info;

    if (*(*sdev).basefw.fw).size <= (*sdev).basefw.payload_offset {
        dev_err((*sdev).dev, c_str!("error: firmware size must be greater than firmware offset\n"));
        return -EINVAL;
    }

    /* init for booting wait */
    init_waitqueue_head(&mut (*sdev).boot_wait);

    /* prepare DMA for code loader stream */
    stripped_firmware.size = (*(*sdev).basefw.fw).size - (*sdev).basefw.payload_offset;
    hext_stream = hda_cl_prepare(
        (*sdev).dev,
        HDA_CL_STREAM_FORMAT,
        stripped_firmware.size as ::core::ffi::c_uint,
        &mut (*hda).cl_dmab,
        persistent_cl_buffer,
        SNDRV_PCM_STREAM_PLAYBACK,
        false,
    );
    if IS_ERR(hext_stream) {
        dev_err((*sdev).dev, c_str!("error: dma prepare for fw loading failed\n"));
        return PTR_ERR(hext_stream);
    }

    /*
     * Copy the payload to the DMA buffer if it is temporary or if the
     * buffer is  persistent but it does not have the basefw payload either
     * because this is the first boot and the buffer needs to be initialized,
     * or a library got loaded and it replaced the basefw.
     */
    if !persistent_cl_buffer || !(*hda).cl_dmab_contains_basefw {
        stripped_firmware.data = (*(*sdev).basefw.fw).data.add((*sdev).basefw.payload_offset);
        memcpy(
            (*hda).cl_dmab.area,
            stripped_firmware.data as *const ::core::ffi::c_void,
            stripped_firmware.size,
        );
        (*hda).cl_dmab_contains_basefw = true;
    }

    /* try ROM init a few times before giving up */
    i = 0;
    while i < HDA_FW_BOOT_ATTEMPTS {
        dev_dbg(
            (*sdev).dev,
            c_str!("Attempting iteration %d of Core En/ROM load...\n"),
            i,
        );

        (*hda).boot_iteration = i + 1;
        if (*chip_info).cl_init.is_some() {
            ret = ((*chip_info).cl_init.unwrap())(sdev, (*hext_stream).hstream.stream_tag, false);
        } else {
            ret = -EINVAL;
        }

        /* don't retry anymore if successful */
        if ret == 0 {
            break;
        }
        i += 1;
    }

    if i == HDA_FW_BOOT_ATTEMPTS {
        dev_err(
            (*sdev).dev,
            c_str!("error: dsp init failed after %d attempts with err: %d\n"),
            i,
            ret,
        );
        goto_cleanup = true;
    } else {
        goto_cleanup = false;
    }

    if !goto_cleanup {
        /*
         * When a SoundWire link is in clock stop state, a Slave
         * device may trigger in-band wakes for events such as jack
         * insertion or acoustic event detection. This event will lead
         * to a WAKEEN interrupt, handled by the PCI device and routed
         * to PME if the PCI device is in D3. The resume function in
         * audio PCI driver will be invoked by ACPI for PME event and
         * initialize the device and process WAKEEN interrupt.
         *
         * The WAKEEN interrupt should be processed ASAP to prevent an
         * interrupt flood, otherwise other interrupts, such IPC,
         * cannot work normally.  The WAKEEN is handled after the ROM
         * is initialized successfully, which ensures power rails are
         * enabled before accessing the SoundWire SHIM registers
         */
        if !(*sdev).first_boot {
            hda_sdw_process_wakeen(sdev);
        }

        /*
         * Set the boot_iteration to the last attempt, indicating that the
         * DSP ROM has been initialized and from this point there will be no
         * retry done to boot.
         *
         * Continue with code loading and firmware boot
         */
        (*hda).boot_iteration = HDA_FW_BOOT_ATTEMPTS;
        ret = hda_cl_copy_fw(sdev, hext_stream);
        if ret == 0 {
            dev_dbg((*sdev).dev, c_str!("Firmware download successful, booting...\n"));
            (*hda).skip_imr_boot = false;
        } else {
            snd_sof_dsp_dbg_dump(
                sdev,
                c_str!("Firmware download failed"),
                SOF_DBG_DUMP_PCI | SOF_DBG_DUMP_MBOX,
            );
            (*hda).skip_imr_boot = true;
        }
    }

    /*
     * Perform codeloader stream cleanup.
     * This should be done even if firmware loading fails.
     * If the cleanup also fails, we return the initial error
     */
    ret1 = hda_cl_cleanup(
        (*sdev).dev,
        &mut (*hda).cl_dmab,
        persistent_cl_buffer,
        hext_stream,
        false,
    );
    if ret1 < 0 {
        dev_err((*sdev).dev, c_str!("error: Code loader DSP cleanup failed\n"));

        /* set return value to indicate cleanup failure */
        if ret == 0 {
            ret = ret1;
        }
    }

    /*
     * return primary core id if both fw copy
     * and stream clean up are successful
     */
    if ret == 0 {
        return (*chip_info).init_core_mask as ::core::ffi::c_int;
    }

    /* disable DSP */
    hda_dsp_ctrl_ppcap_enable(sdev, false);

    ret
}
/* EXPORT_SYMBOL_NS(hda_dsp_cl_boot_firmware, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ipc4_load_library(
    sdev: *mut snd_sof_dev,
    fw_lib: *mut sof_ipc4_fw_library,
    reload: bool,
) -> ::core::ffi::c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let hext_stream: *mut hdac_ext_stream;
    let mut stripped_firmware: firmware = ::core::mem::zeroed();
    let mut msg: sof_ipc4_msg = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;
    let mut ret1: ::core::ffi::c_int;
    let mut goto_cleanup: bool;

    /*
     * if IMR booting is enabled and libraries have been restored during fw
     * boot, skip the loading
     */
    if reload && (*hda).booted_from_imr && (*ipc4_data).libraries_restored {
        return 0;
    }

    /* the fw_lib has been verified during loading, we can trust the validity here */
    stripped_firmware.data =
        (*(*fw_lib).sof_fw.fw).data.add((*fw_lib).sof_fw.payload_offset);
    stripped_firmware.size = (*(*fw_lib).sof_fw.fw).size - (*fw_lib).sof_fw.payload_offset;

    /*
     * force re-allocation of the cl_dmab if the preserved DMA buffer is
     * smaller than what is needed for the library
     */
    if persistent_cl_buffer && stripped_firmware.size > (*hda).cl_dmab.bytes {
        snd_dma_free_pages(&mut (*hda).cl_dmab);
        (*hda).cl_dmab.area = ::core::ptr::null_mut();
        (*hda).cl_dmab.bytes = 0;
    }

    /* prepare DMA for code loader stream */
    hext_stream = hda_cl_prepare(
        (*sdev).dev,
        HDA_CL_STREAM_FORMAT,
        stripped_firmware.size as ::core::ffi::c_uint,
        &mut (*hda).cl_dmab,
        persistent_cl_buffer,
        SNDRV_PCM_STREAM_PLAYBACK,
        false,
    );
    if IS_ERR(hext_stream) {
        dev_err((*sdev).dev, c_str!("%s: DMA prepare failed\n"), c_str!("hda_dsp_ipc4_load_library"));
        return PTR_ERR(hext_stream);
    }

    memcpy(
        (*hda).cl_dmab.area,
        stripped_firmware.data as *const ::core::ffi::c_void,
        stripped_firmware.size,
    );
    (*hda).cl_dmab_contains_basefw = false;

    /*
     * 1st stage: SOF_IPC4_GLB_LOAD_LIBRARY_PREPARE
     * Message includes the dma_id to be prepared for the library loading.
     * If the firmware does not have support for the message, we will
     * receive -EOPNOTSUPP. In this case we will use single step library
     * loading and proceed to send the LOAD_LIBRARY message.
     */
    msg.primary = ((*hext_stream).hstream.stream_tag - 1) as u32;
    msg.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_LOAD_LIBRARY_PREPARE);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_FW_GEN_MSG);
    ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, 0);
    if ret == 0 {
        let sd_offset = SOF_STREAM_SD_OFFSET(&mut (*hext_stream).hstream);
        let mut status: ::core::ffi::c_uint = 0;

        /*
         * Make sure that the FIFOS value is not 0 in SDxFIFOS register
         * which indicates that the firmware set the GEN bit and we can
         * continue to start the DMA
         */
        ret = snd_sof_dsp_read_poll_timeout!(
            sdev,
            HDA_DSP_HDA_BAR,
            sd_offset + SOF_HDA_ADSP_REG_SD_FIFOSIZE,
            status,
            (status & SOF_HDA_SD_FIFOSIZE_FIFOS_MASK) != 0,
            HDA_DSP_REG_POLL_INTERVAL_US,
            HDA_DSP_BASEFW_TIMEOUT_US
        );

        if ret < 0 {
            dev_warn(
                (*sdev).dev,
                c_str!("%s: timeout waiting for FIFOS\n"),
                c_str!("hda_dsp_ipc4_load_library"),
            );
        }
    } else if ret != -EOPNOTSUPP {
        goto_cleanup = true;
    } else {
        goto_cleanup = false;
    }

    if !goto_cleanup {
        ret = hda_cl_trigger((*sdev).dev, hext_stream, SNDRV_PCM_TRIGGER_START);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c_str!("%s: DMA trigger start failed\n"),
                c_str!("hda_dsp_ipc4_load_library"),
            );
            goto_cleanup = true;
        }
    }

    if !goto_cleanup {
        /*
         * 2nd stage: LOAD_LIBRARY
         * Message includes the dma_id and the lib_id, the dma_id must be
         * identical to the one sent via LOAD_LIBRARY_PREPARE
         */
        msg.primary &= !SOF_IPC4_MSG_TYPE_MASK;
        msg.primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_LOAD_LIBRARY);
        msg.primary |= SOF_IPC4_GLB_LOAD_LIBRARY_LIB_ID((*fw_lib).id);
        ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, 0);

        /* Stop the DMA channel */
        ret1 = hda_cl_trigger((*sdev).dev, hext_stream, SNDRV_PCM_TRIGGER_STOP);
        if ret1 < 0 {
            dev_err(
                (*sdev).dev,
                c_str!("%s: DMA trigger stop failed\n"),
                c_str!("hda_dsp_ipc4_load_library"),
            );
            if ret == 0 {
                ret = ret1;
            }
        }
    }

    /* clean up even in case of error and return the first error */
    ret1 = hda_cl_cleanup(
        (*sdev).dev,
        &mut (*hda).cl_dmab,
        persistent_cl_buffer,
        hext_stream,
        false,
    );
    if ret1 < 0 {
        dev_err(
            (*sdev).dev,
            c_str!("%s: Code loader DSP cleanup failed\n"),
            c_str!("hda_dsp_ipc4_load_library"),
        );

        /* set return value to indicate cleanup failure */
        if ret == 0 {
            ret = ret1;
        }
    }

    ret
}
/* EXPORT_SYMBOL_NS(hda_dsp_ipc4_load_library, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn hda_dsp_ext_man_get_cavs_config_data(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> ::core::ffi::c_int {
    let config_data =
        container_of!(hdr, sof_ext_man_cavs_config_data, hdr) as *const sof_ext_man_cavs_config_data;
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let mut i: ::core::ffi::c_int;
    let elem_num: ::core::ffi::c_int;

    /* calculate total number of config data elements */
    elem_num = (((*hdr).size as usize - ::core::mem::size_of::<sof_ext_man_elem_header>())
        / ::core::mem::size_of::<sof_config_elem>()) as ::core::ffi::c_int;
    if elem_num <= 0 {
        dev_err(
            (*sdev).dev,
            c_str!("cavs config data is inconsistent: %d\n"),
            elem_num,
        );
        return -EINVAL;
    }

    i = 0;
    while i < elem_num {
        match (*(*config_data).elems.as_ptr().add(i as usize)).token {
            SOF_EXT_MAN_CAVS_CONFIG_EMPTY => {
                /* skip empty token */
            }
            SOF_EXT_MAN_CAVS_CONFIG_CAVS_LPRO => {
                (*hda).clk_config_lpro = (*(*config_data).elems.as_ptr().add(i as usize)).value;
                dev_dbg(
                    (*sdev).dev,
                    c_str!("FW clock config: %s\n"),
                    if (*hda).clk_config_lpro != 0 {
                        c_str!("LPRO")
                    } else {
                        c_str!("HPRO")
                    },
                );
            }
            SOF_EXT_MAN_CAVS_CONFIG_OUTBOX_SIZE | SOF_EXT_MAN_CAVS_CONFIG_INBOX_SIZE => {
                /* These elements are defined but not being used yet. No warn is required */
            }
            _ => {
                dev_info(
                    (*sdev).dev,
                    c_str!("unsupported token type: %d\n"),
                    (*(*config_data).elems.as_ptr().add(i as usize)).token,
                );
            }
        }
        i += 1;
    }

    0
}
/* EXPORT_SYMBOL_NS(hda_dsp_ext_man_get_cavs_config_data, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
