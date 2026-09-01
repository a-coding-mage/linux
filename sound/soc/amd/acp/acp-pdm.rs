// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//	    Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//

/*
 * Generic Hardware interface for ACP Audio PDM controller
 */

// C dependencies:
// linux/err.h, linux/io.h, linux/module.h, linux/platform_device.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h, and "amd.h".

const DRV_NAME: &str = "acp-pdm";

unsafe extern "C" fn acp_dmic_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let stream: *mut acp_stream = (*(*substream).runtime).private_data as *mut acp_stream;
    let dev: *mut device = (*(*dai).component).dev;
    let chip: *mut acp_chip_info;
    let physical_addr: u32;
    let size_dmic: u32;
    let period_bytes: u32;
    let mut dmic_ctrl: core::ffi::c_uint;

    chip = dev_get_platdata(dev) as *mut acp_chip_info;
    /* Enable default DMIC clk */
    writel(PDM_CLK_FREQ_MASK, (*chip).base.add(ACP_WOV_CLK_CTRL as usize));
    dmic_ctrl = readl((*chip).base.add(ACP_WOV_MISC_CTRL as usize));
    dmic_ctrl |= PDM_MISC_CTRL_MASK;
    writel(dmic_ctrl, (*chip).base.add(ACP_WOV_MISC_CTRL as usize));

    period_bytes = frames_to_bytes(
        (*substream).runtime,
        (*(*substream).runtime).period_size,
    );
    size_dmic = frames_to_bytes(
        (*substream).runtime,
        (*(*substream).runtime).buffer_size,
    );

    if (*chip).acp_rev >= ACP70_PCI_ID {
        physical_addr = ACP7x_DMIC_MEM_WINDOW_START;
    } else {
        physical_addr = (*stream).reg_offset + MEM_WINDOW_START;
    }

    /* Init DMIC Ring buffer */
    writel(physical_addr, (*chip).base.add(ACP_WOV_RX_RINGBUFADDR as usize));
    writel(size_dmic, (*chip).base.add(ACP_WOV_RX_RINGBUFSIZE as usize));
    writel(period_bytes, (*chip).base.add(ACP_WOV_RX_INTR_WATERMARK_SIZE as usize));
    writel(0x01, (*chip).base.add(ACPAXI2AXI_ATU_CTRL as usize));

    0
}

unsafe extern "C" fn acp_dmic_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dev: *mut device = (*(*dai).component).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev) as *mut acp_chip_info;
    let mut dma_enable: core::ffi::c_uint;
    let mut ret: core::ffi::c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            dma_enable = readl((*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
            if (dma_enable & DMA_EN_MASK) == 0 {
                writel(PDM_ENABLE, (*chip).base.add(ACP_WOV_PDM_ENABLE as usize));
                writel(PDM_ENABLE, (*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
            }

            ret = readl_poll_timeout_atomic!(
                (*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize),
                dma_enable,
                (dma_enable & DMA_EN_MASK) != 0,
                DELAY_US,
                PDM_TIMEOUT
            );
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            dma_enable = readl((*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
            if (dma_enable & DMA_EN_MASK) != 0 {
                writel(PDM_DISABLE, (*chip).base.add(ACP_WOV_PDM_ENABLE as usize));
                writel(PDM_DISABLE, (*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
            }

            ret = readl_poll_timeout_atomic!(
                (*chip).base.add(ACP_WOV_PDM_DMA_ENABLE as usize),
                dma_enable,
                (dma_enable & DMA_EN_MASK) == 0,
                DELAY_US,
                PDM_TIMEOUT
            );
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn acp_dmic_hwparams(
    substream: *mut snd_pcm_substream,
    hwparams: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dev: *mut device = (*(*dai).component).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev) as *mut acp_chip_info;
    let channels: core::ffi::c_uint;
    let ch_mask: core::ffi::c_uint;

    channels = params_channels(hwparams);
    match channels {
        2 => {
            ch_mask = 0;
        }
        4 => {
            ch_mask = 1;
        }
        6 => {
            ch_mask = 2;
        }
        _ => {
            dev_err(dev, c"Invalid channels %d\n".as_ptr(), channels);
            return -EINVAL;
        }
    }

    (*chip).ch_mask = ch_mask;
    if params_format(hwparams) != SNDRV_PCM_FORMAT_S32_LE {
        dev_err(
            (*dai).dev,
            c"Invalid format:%d\n".as_ptr(),
            params_format(hwparams),
        );
        return -EINVAL;
    }

    writel(ch_mask, (*chip).base.add(ACP_WOV_PDM_NO_OF_CHANNELS as usize));
    writel(PDM_DEC_64, (*chip).base.add(ACP_WOV_PDM_DECIMATION_FACTOR as usize));

    0
}

unsafe extern "C" fn acp_dmic_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let stream: *mut acp_stream = (*(*substream).runtime).private_data as *mut acp_stream;
    let dev: *mut device = (*(*dai).component).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev) as *mut acp_chip_info;
    let mut ext_int_ctrl: u32;

    (*stream).dai_id = DMIC_INSTANCE;
    (*stream).irq_bit = BIT(PDM_DMA_STAT);
    (*stream).pte_offset = ACP_SRAM_PDM_PTE_OFFSET;
    (*stream).reg_offset = ACP_REGION2_OFFSET;

    /* Enable DMIC Interrupts */
    ext_int_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, 0));
    ext_int_ctrl |= PDM_DMA_INTR_MASK;
    writel(ext_int_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, 0));

    0
}

unsafe extern "C" fn acp_dmic_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dev: *mut device = (*(*dai).component).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev) as *mut acp_chip_info;
    let mut ext_int_ctrl: u32;

    /* Disable DMIC interrupts */
    ext_int_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, 0));
    ext_int_ctrl &= !PDM_DMA_INTR_MASK;
    writel(ext_int_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, 0));
}

#[no_mangle]
pub static acp_dmic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(acp_dmic_prepare),
    hw_params: Some(acp_dmic_hwparams),
    trigger: Some(acp_dmic_dai_trigger),
    startup: Some(acp_dmic_dai_startup),
    shutdown: Some(acp_dmic_dai_shutdown),
};
// EXPORT_SYMBOL_NS_GPL(acp_dmic_dai_ops, "SND_SOC_ACP_COMMON");

// MODULE_DESCRIPTION("AMD ACP Audio PDM controller");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS(DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
