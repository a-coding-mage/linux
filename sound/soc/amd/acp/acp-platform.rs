// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Generic interface for ACP audio blck PCM component
 */

// C dependencies:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h, linux/dma-mapping.h,
// amd.h, acp-mach.h

const DRV_NAME: *const i8 = b"acp_i2s_dma\0".as_ptr() as *const i8;

static acp_pcm_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 8,
    rates: SNDRV_PCM_RATE_8000_96000,
    rate_min: 8000,
    rate_max: 96000,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
};

static acp_pcm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

static acp6x_pcm_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 32,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
};

static acp6x_pcm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 32,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

#[no_mangle]
pub unsafe extern "C" fn config_pte_for_stream(
    chip: *mut acp_chip_info,
    stream: *mut acp_stream,
) {
    let rsrc: *mut acp_resource = (*chip).rsrc;
    let reg_val: u32;

    reg_val = (*rsrc).sram_pte_offset;
    (*stream).reg_offset = 0x02000000;

    writel(
        (reg_val + GRP1_OFFSET) | BIT(31),
        (*chip).base + ACPAXI2AXI_ATU_BASE_ADDR_GRP_1,
    );
    writel(
        PAGE_SIZE_4K_ENABLE,
        (*chip).base + ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1,
    );

    writel(
        (reg_val + GRP2_OFFSET) | BIT(31),
        (*chip).base + ACPAXI2AXI_ATU_BASE_ADDR_GRP_2,
    );
    writel(
        PAGE_SIZE_4K_ENABLE,
        (*chip).base + ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2,
    );

    writel(
        reg_val | BIT(31),
        (*chip).base + ACPAXI2AXI_ATU_BASE_ADDR_GRP_5,
    );
    writel(
        PAGE_SIZE_4K_ENABLE,
        (*chip).base + ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5,
    );

    writel(0x01, (*chip).base + ACPAXI2AXI_ATU_CTRL);
}
// EXPORT_SYMBOL_NS_GPL(config_pte_for_stream, "SND_SOC_ACP_COMMON");

#[no_mangle]
pub unsafe extern "C" fn config_acp_dma(
    chip: *mut acp_chip_info,
    stream: *mut acp_stream,
    size: libc::c_int,
) {
    let substream: *mut snd_pcm_substream = (*stream).substream;
    let rsrc: *mut acp_resource = (*chip).rsrc;
    let mut addr: dma_addr_t = (*substream).dma_buffer.addr;
    let num_pages: libc::c_int = (PAGE_ALIGN(size) >> PAGE_SHIFT) as libc::c_int;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let mut page_idx: u16;

    match (*chip).acp_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            match (*stream).dai_id {
                I2S_SP_INSTANCE => {
                    if (*stream).dir == SNDRV_PCM_STREAM_PLAYBACK {
                        val = 0x0;
                    } else {
                        val = 0x1000;
                    }
                }
                I2S_BT_INSTANCE => {
                    if (*stream).dir == SNDRV_PCM_STREAM_PLAYBACK {
                        val = 0x2000;
                    } else {
                        val = 0x3000;
                    }
                }
                I2S_HS_INSTANCE => {
                    if (*stream).dir == SNDRV_PCM_STREAM_PLAYBACK {
                        val = 0x4000;
                    } else {
                        val = 0x5000;
                    }
                }
                DMIC_INSTANCE => {
                    val = 0x6000;
                }
                _ => {
                    dev_err(
                        (*chip).dev,
                        b"Invalid dai id %x\n\0".as_ptr() as *const i8,
                        (*stream).dai_id,
                    );
                    return;
                }
            }
        }
        _ => {
            val = (*stream).pte_offset;
        }
    }

    page_idx = 0;
    while (page_idx as libc::c_int) < num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);
        writel(low, (*chip).base + (*rsrc).scratch_reg_offset + val);
        high |= BIT(31);
        writel(high, (*chip).base + (*rsrc).scratch_reg_offset + val + 4);

        /* Move to next physically contiguous page */
        val += 8;
        addr += PAGE_SIZE;
        page_idx = page_idx.wrapping_add(1);
    }
}
// EXPORT_SYMBOL_NS_GPL(config_acp_dma, "SND_SOC_ACP_COMMON");

unsafe extern "C" fn acp_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let dev: *mut device = (*component).dev;
    let chip: *mut acp_chip_info;
    let stream: *mut acp_stream;
    let mut ret: libc::c_int;

    stream = kzalloc_obj_acp_stream();
    if stream.is_null() {
        return -ENOMEM;
    }

    (*stream).substream = substream;
    chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    match (*chip).acp_rev {
        ACP63_PCI_ID | ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                (*runtime).hw = acp6x_pcm_hardware_playback;
            } else {
                (*runtime).hw = acp6x_pcm_hardware_capture;
            }
        }
        _ => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                (*runtime).hw = acp_pcm_hardware_playback;
            } else {
                (*runtime).hw = acp_pcm_hardware_capture;
            }
        }
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, DMA_SIZE);
    if ret != 0 {
        dev_err(
            (*component).dev,
            b"set hw constraint HW_PARAM_PERIOD_BYTES failed\n\0".as_ptr() as *const i8,
        );
        kfree(stream as *mut libc::c_void);
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, DMA_SIZE);
    if ret != 0 {
        dev_err(
            (*component).dev,
            b"set hw constraint HW_PARAM_BUFFER_BYTES failed\n\0".as_ptr() as *const i8,
        );
        kfree(stream as *mut libc::c_void);
        return ret;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err(
            (*component).dev,
            b"set integer constraint failed\n\0".as_ptr() as *const i8,
        );
        kfree(stream as *mut libc::c_void);
        return ret;
    }
    (*runtime).private_data = stream as *mut libc::c_void;

    writel(1, ACP_EXTERNAL_INTR_ENB(chip));

    spin_lock_irq(&mut (*chip).acp_lock);
    list_add_tail(&mut (*stream).list, &mut (*chip).stream_list);
    spin_unlock_irq(&mut (*chip).acp_lock);

    ret
}

unsafe extern "C" fn acp_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> libc::c_int {
    let dev: *mut device = (*component).dev;
    let chip: *mut acp_chip_info = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    let stream: *mut acp_stream = (*(*substream).runtime).private_data as *mut acp_stream;
    let size: u64 = params_buffer_bytes(params);

    /* Configure ACP DMA block with params */
    config_pte_for_stream(chip, stream);
    config_acp_dma(chip, stream, size as libc::c_int);

    0
}

unsafe extern "C" fn acp_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let dev: *mut device = (*component).dev;
    let chip: *mut acp_chip_info = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    let stream: *mut acp_stream = (*(*substream).runtime).private_data as *mut acp_stream;
    let pos: u32;
    let buffersize: u32;
    let mut bytescount: u64;

    buffersize = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);

    bytescount = acp_get_byte_count(chip, (*stream).dai_id, (*substream).stream);

    if bytescount > (*stream).bytescount {
        bytescount -= (*stream).bytescount;
    }

    pos = do_div(&mut bytescount, buffersize);

    bytes_to_frames((*substream).runtime, pos)
}

unsafe extern "C" fn acp_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> libc::c_int {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    0
}

unsafe extern "C" fn acp_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    let dev: *mut device = (*component).dev;
    let chip: *mut acp_chip_info = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    let stream: *mut acp_stream = (*(*substream).runtime).private_data as *mut acp_stream;

    /* Remove entry from list */
    spin_lock_irq(&mut (*chip).acp_lock);
    list_del(&mut (*stream).list);
    spin_unlock_irq(&mut (*chip).acp_lock);
    kfree(stream as *mut libc::c_void);

    0
}

static acp_pcm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(acp_dma_open),
    close: Some(acp_dma_close),
    hw_params: Some(acp_dma_hw_params),
    pointer: Some(acp_dma_pointer),
    pcm_new: Some(acp_dma_new),
    legacy_dai_naming: 1,
};

#[no_mangle]
pub unsafe extern "C" fn acp_platform_register(dev: *mut device) -> libc::c_int {
    let chip: *mut acp_chip_info;
    // C declaration-only statement preserved from source: struct snd_soc_dai_driver;
    let status: libc::c_uint;

    chip = dev_get_platdata(dev) as *mut acp_chip_info;
    if chip.is_null() || (*chip).base.is_null() {
        dev_err(dev, b"ACP chip data is NULL\n\0".as_ptr() as *const i8);
        return -ENODEV;
    }

    status = devm_snd_soc_register_component(
        dev,
        &acp_pcm_component,
        (*chip).dai_driver,
        (*chip).num_dai,
    );
    if status != 0 {
        dev_err(
            dev,
            b"Fail to register acp i2s component\n\0".as_ptr() as *const i8,
        );
        return status as libc::c_int;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(acp_platform_register, "SND_SOC_ACP_COMMON");

#[no_mangle]
pub unsafe extern "C" fn acp_platform_unregister(_dev: *mut device) -> libc::c_int {
    0
}
// EXPORT_SYMBOL_NS_GPL(acp_platform_unregister, "SND_SOC_ACP_COMMON");

// MODULE_DESCRIPTION("AMD ACP PCM Driver");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS(DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
