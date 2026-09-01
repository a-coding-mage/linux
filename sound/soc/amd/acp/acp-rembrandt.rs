// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//          V sujith kumar Reddy <Vsujithkumar.Reddy@amd.com>
/*
 * Hardware interface for Renoir ACP block
 */

// C dependencies:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h, linux/dma-mapping.h,
// linux/pci.h, linux/pm_runtime.h, asm/amd/node.h, amd.h,
// ../mach-config.h, acp-mach.h.

const DRV_NAME: &[u8] = b"acp_asoc_rembrandt\0";

const MP1_C2PMSG_69: u32 = 0x3B10A14;
const MP1_C2PMSG_85: u32 = 0x3B10A54;
const MP1_C2PMSG_93: u32 = 0x3B10A74;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub rates: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const i8,
    pub id: i32,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct resource_info {
    pub soc_mclk: bool,
}

#[repr(C)]
pub struct acp_chip_info {
    pub base: *mut core::ffi::c_void,
    pub acp_rev: i32,
    pub dev: *mut device,
    pub dai_driver: *mut snd_soc_dai_driver,
    pub num_dai: usize,
    pub is_i2s_config: bool,
    pub rsrc: *mut resource_info,
    pub acp_lock: spinlock_t,
    pub stream_list: list_head,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct acp_stream {
    pub substream: *mut snd_pcm_substream,
    pub dai_id: i32,
    pub list: list_head,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const i8,
    pub pm: *const dev_pm_ops,
}

#[allow(non_camel_case_types)]
pub type snd_pcm_uframes_t = u64;

#[allow(non_camel_case_types)]
pub type u64_t = u64;

extern "C" {
    static asoc_acp_cpu_dai_ops: snd_soc_dai_ops;
    static acp_dmic_dai_ops: snd_soc_dai_ops;

    static I2S_SP_INSTANCE: i32;
    static I2S_BT_INSTANCE: i32;
    static I2S_HS_INSTANCE: i32;
    static DMIC_INSTANCE: i32;
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_RATE_8000_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static DELAY_US: u64;
    static ACP_TIMEOUT: u64;
    static ACP_RMB_PCI_ID: i32;
    static ACP_SUSPEND_DELAY_MS: i32;
    static ENODEV: i32;

    fn amd_smn_write(node: u32, address: u32, value: u32) -> i32;
    fn smn_read_register(address: u32) -> i32;
    fn dev_get_platdata(dev: *mut device) -> *mut acp_chip_info;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn acp_hw_en_interrupts(chip: *mut acp_chip_info) -> i32;
    fn acp_hw_dis_interrupts(chip: *mut acp_chip_info) -> i32;
    fn acp_platform_register(dev: *mut device);
    fn acp_platform_unregister(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: i32);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u64;
    fn config_pte_for_stream(chip: *mut acp_chip_info, stream: *mut acp_stream);
    fn config_acp_dma(chip: *mut acp_chip_info, stream: *mut acp_stream, buf_size: u64);
    fn restore_acp_i2s_params(
        substream: *mut snd_pcm_substream,
        chip: *mut acp_chip_info,
        stream: *mut acp_stream,
    );
    fn restore_acp_pdm_params(substream: *mut snd_pcm_substream, chip: *mut acp_chip_info);
}

unsafe fn read_poll_timeout_smn_read_register(
    mut data: i32,
    address: u32,
) -> i32 {
    // Direct Rust equivalent of:
    // read_poll_timeout(smn_read_register, data, data > 0, DELAY_US,
    //                   ACP_TIMEOUT, false, MP1_C2PMSG_93)
    let _delay_us = DELAY_US;
    let timeout = ACP_TIMEOUT;
    let mut elapsed: u64 = 0;

    loop {
        data = smn_read_register(address);
        if data > 0 {
            return 0;
        }
        if elapsed >= timeout {
            return data;
        }
        elapsed = elapsed.wrapping_add(_delay_us);
    }
}

static mut ACP_RMB_DAI: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: b"acp-i2s-sp\0".as_ptr() as *const i8,
        id: unsafe { I2S_SP_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S SP Playback\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S SP Capture\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops as *const snd_soc_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-bt\0".as_ptr() as *const i8,
        id: unsafe { I2S_BT_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S BT Playback\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S BT Capture\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops as *const snd_soc_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-hs\0".as_ptr() as *const i8,
        id: unsafe { I2S_HS_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S HS Playback\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S HS Capture\0".as_ptr() as *const i8,
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe {
                SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_U8
                    | SNDRV_PCM_FMTBIT_S32_LE
            },
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 48000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops as *const snd_soc_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-pdm-dmic\0".as_ptr() as *const i8,
        id: unsafe { DMIC_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: core::ptr::null(),
            rates: 0,
            formats: 0,
            channels_min: 0,
            channels_max: 0,
            rate_min: 0,
            rate_max: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: core::ptr::null(),
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
        },
        ops: unsafe { &acp_dmic_dai_ops as *const snd_soc_dai_ops },
    },
];

unsafe extern "C" fn acp6x_master_clock_generate(dev: *mut device) -> i32 {
    let mut data: i32 = 0;
    let mut rc: i32;

    let _ = dev;

    rc = amd_smn_write(0, MP1_C2PMSG_93, 0);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, MP1_C2PMSG_85, 0xC4);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, MP1_C2PMSG_69, 0x4);
    if rc != 0 {
        return rc;
    }

    read_poll_timeout_smn_read_register(data, MP1_C2PMSG_93)
}

unsafe extern "C" fn rembrandt_audio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let chip: *mut acp_chip_info;
    let mut ret: i32;

    chip = dev_get_platdata(&mut (*pdev).dev as *mut device);
    if chip.is_null() || (*chip).base.is_null() {
        dev_err(
            &mut (*pdev).dev as *mut device,
            b"ACP chip data is NULL\n\0".as_ptr() as *const i8,
        );
        return -ENODEV;
    }

    if (*chip).acp_rev != ACP_RMB_PCI_ID {
        dev_err(
            &mut (*pdev).dev as *mut device,
            b"Un-supported ACP Revision %d\n\0".as_ptr() as *const i8,
            (*chip).acp_rev,
        );
        return -ENODEV;
    }

    (*chip).dev = dev;
    (*chip).dai_driver = ACP_RMB_DAI.as_mut_ptr();
    (*chip).num_dai = ACP_RMB_DAI.len();

    if (*chip).is_i2s_config && (*(*chip).rsrc).soc_mclk {
        ret = acp6x_master_clock_generate(dev);
        if ret != 0 {
            return ret;
        }
    }
    ret = acp_hw_en_interrupts(chip);
    if ret != 0 {
        dev_err(dev, b"ACP en-interrupts failed\n\0".as_ptr() as *const i8);
        return ret;
    }
    acp_platform_register(dev);
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev as *mut device, ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&mut (*pdev).dev as *mut device);
    pm_runtime_mark_last_busy(&mut (*pdev).dev as *mut device);
    pm_runtime_set_active(&mut (*pdev).dev as *mut device);
    pm_runtime_enable(&mut (*pdev).dev as *mut device);
    0
}

unsafe extern "C" fn rembrandt_audio_remove(pdev: *mut platform_device) {
    let dev = &mut (*pdev).dev as *mut device;
    let chip = dev_get_platdata(dev);
    let ret: i32;

    ret = acp_hw_dis_interrupts(chip);
    if ret != 0 {
        dev_err(dev, b"ACP dis-interrupts failed\n\0".as_ptr() as *const i8);
    }

    acp_platform_unregister(dev);
    pm_runtime_disable(&mut (*pdev).dev as *mut device);
}

unsafe extern "C" fn rmb_pcm_resume(dev: *mut device) -> i32 {
    let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    let mut stream: *mut acp_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut buf_in_frames: snd_pcm_uframes_t;
    let mut buf_size: u64;

    if (*chip).is_i2s_config && (*(*chip).rsrc).soc_mclk {
        acp6x_master_clock_generate(dev);
    }

    spin_lock(&mut (*chip).acp_lock as *mut spinlock_t);
    // list_for_each_entry(stream, &chip->stream_list, list) {
    stream = list_first_entry(
        &mut (*chip).stream_list as *mut list_head,
    );
    while !list_entry_is_head(
        stream,
        &mut (*chip).stream_list as *mut list_head,
    ) {
        substream = (*stream).substream;
        if !substream.is_null() && !(*substream).runtime.is_null() {
            buf_in_frames = (*(*substream).runtime).buffer_size;
            buf_size = frames_to_bytes((*substream).runtime, buf_in_frames);
            config_pte_for_stream(chip, stream);
            config_acp_dma(chip, stream, buf_size);
            if (*stream).dai_id != 0 {
                restore_acp_i2s_params(substream, chip, stream);
            } else {
                restore_acp_pdm_params(substream, chip);
            }
        }
        stream = list_next_entry(stream);
    }
    spin_unlock(&mut (*chip).acp_lock as *mut spinlock_t);
    0
}

extern "C" {
    fn list_first_entry(head: *mut list_head) -> *mut acp_stream;
    fn list_entry_is_head(entry: *mut acp_stream, head: *mut list_head) -> bool;
    fn list_next_entry(entry: *mut acp_stream) -> *mut acp_stream;
}

// static const struct dev_pm_ops rmb_dma_pm_ops = {
//      SYSTEM_SLEEP_PM_OPS(NULL, rmb_pcm_resume)
// };
static RMB_DMA_PM_OPS: dev_pm_ops = dev_pm_ops {
    resume: Some(rmb_pcm_resume),
};

static mut REMBRANDT_DRIVER: platform_driver = platform_driver {
    probe: Some(rembrandt_audio_probe),
    remove: Some(rembrandt_audio_remove),
    driver: device_driver {
        name: b"acp_asoc_rembrandt\0".as_ptr() as *const i8,
        pm: &RMB_DMA_PM_OPS as *const dev_pm_ops,
    },
};

// module_platform_driver(rembrandt_driver);
// MODULE_DESCRIPTION("AMD ACP Rembrandt Driver");
// MODULE_IMPORT_NS("SND_SOC_ACP_COMMON");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
