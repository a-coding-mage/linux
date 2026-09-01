// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Syed Saba kareem <syed.sabakareem@amd.com>
/*
 * Hardware interface for ACP7.0 block
 */

// Rust translation of dependencies originally included from Linux, ALSA SoC,
// AMD ACP, and asm/amd/node headers.

const DRV_NAME: *const core::ffi::c_char = b"acp_asoc_acp70\0".as_ptr() as *const core::ffi::c_char;

const CLK7_CLK0_DFS_CNTL_N1: u32 = 0x0006C1A4;
const CLK0_DIVIDER: u32 = 0x19;

extern "C" {
    static asoc_acp_cpu_dai_ops: snd_soc_dai_ops;
    static acp_dmic_dai_ops: snd_soc_dai_ops;

    fn dev_get_platdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn amd_smn_write(node: u16, address: u32, value: u32) -> core::ffi::c_int;
    fn acp_hw_en_interrupts(chip: *mut acp_chip_info) -> core::ffi::c_int;
    fn acp_hw_dis_interrupts(chip: *mut acp_chip_info) -> core::ffi::c_int;
    fn acp_platform_register(dev: *mut device);
    fn acp_platform_unregister(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: core::ffi::c_int);
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
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
    pub stream_name: *const core::ffi::c_char,
    pub rates: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const core::ffi::c_char,
    pub id: core::ffi::c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct acp_chip_info {
    pub base: *mut core::ffi::c_void,
    pub acp_rev: core::ffi::c_int,
    pub dev: *mut device,
    pub dai_driver: *mut snd_soc_dai_driver,
    pub num_dai: usize,
    pub acp_lock: spinlock_t,
    pub stream_list: list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct acp_stream {
    pub substream: *mut snd_pcm_substream,
    pub dai_id: core::ffi::c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub buffer_size: snd_pcm_uframes_t,
}

#[allow(non_camel_case_types)]
pub type snd_pcm_uframes_t = u64;

#[repr(C)]
pub struct dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(NULL, acp70_pcm_resume)
    pub resume: Option<unsafe extern "C" fn(*mut device) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

const I2S_SP_INSTANCE: core::ffi::c_int = 0;
const I2S_BT_INSTANCE: core::ffi::c_int = 1;
const I2S_HS_INSTANCE: core::ffi::c_int = 2;
const DMIC_INSTANCE: core::ffi::c_int = 3;
const ACP70_PCI_ID: core::ffi::c_int = 0;
const ACP71_PCI_ID: core::ffi::c_int = 0;
const ACP72_PCI_ID: core::ffi::c_int = 0;
const ACP_SUSPEND_DELAY_MS: core::ffi::c_int = 0;
const ENODEV: core::ffi::c_int = 19;
const SNDRV_PCM_RATE_8000_192000: u32 = 0;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_FMTBIT_U8: u64 = 0;
const SNDRV_PCM_FMTBIT_S8: u64 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

const ACP70_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut acp70_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: b"acp-i2s-sp\0".as_ptr() as *const core::ffi::c_char,
        id: I2S_SP_INSTANCE,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S SP Playback\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S SP Capture\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-bt\0".as_ptr() as *const core::ffi::c_char,
        id: I2S_BT_INSTANCE,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S BT Playback\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S BT Capture\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-hs\0".as_ptr() as *const core::ffi::c_char,
        id: I2S_HS_INSTANCE,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S HS Playback\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"I2S HS Capture\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: ACP70_FORMATS,
            channels_min: 2,
            channels_max: 32,
            rate_min: 8000,
            rate_max: 192000,
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-pdm-dmic\0".as_ptr() as *const core::ffi::c_char,
        id: DMIC_INSTANCE,
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
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
        },
        ops: unsafe { &acp_dmic_dai_ops },
    },
];

unsafe extern "C" fn acp_acp70_audio_probe(
    pdev: *mut platform_device,
) -> core::ffi::c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut chip: *mut acp_chip_info;
    let mut ret: core::ffi::c_int;

    chip = dev_get_platdata(&mut (*pdev).dev) as *mut acp_chip_info;
    if chip.is_null() || (*chip).base.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"ACP chip data is NULL\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return -ENODEV;
    }

    match (*chip).acp_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {}
        _ => {
            dev_err(
                &mut (*pdev).dev,
                b"Un-supported ACP Revision %d\n\0".as_ptr() as *const core::ffi::c_char,
                (*chip).acp_rev,
            );
            return -ENODEV;
        }
    }

    (*chip).dev = dev;
    (*chip).dai_driver = acp70_dai.as_mut_ptr();
    (*chip).num_dai = acp70_dai.len();

    /* Set clk7 DFS clock divider register value to get mclk as 196.608MHz*/
    ret = amd_smn_write(0, CLK7_CLK0_DFS_CNTL_N1, CLK0_DIVIDER);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Failed to set I2S master clock as 196.608MHz\n\0".as_ptr()
                as *const core::ffi::c_char,
        );
        return ret;
    }
    ret = acp_hw_en_interrupts(chip);
    if ret != 0 {
        dev_err(
            dev,
            b"ACP en-interrupts failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return ret;
    }
    acp_platform_register(dev);
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn acp_acp70_audio_remove(pdev: *mut platform_device) {
    let dev: *mut device = &mut (*pdev).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev) as *mut acp_chip_info;
    let ret: core::ffi::c_int;

    ret = acp_hw_dis_interrupts(chip);
    if ret != 0 {
        dev_err(
            dev,
            b"ACP dis-interrupts failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    acp_platform_unregister(dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp70_pcm_resume(dev: *mut device) -> core::ffi::c_int {
    let chip: *mut acp_chip_info = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
    let mut stream: *mut acp_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut buf_in_frames: snd_pcm_uframes_t;
    let mut buf_size: u64;

    spin_lock(&mut (*chip).acp_lock);
    // list_for_each_entry(stream, &chip->stream_list, list) {
    stream = core::ptr::null_mut();
    while !stream.is_null() {
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
        // Iteration is supplied by Linux list_for_each_entry in the original C source.
        break;
    }
    spin_unlock(&mut (*chip).acp_lock);
    0
}

static acp70_dma_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(NULL, acp70_pcm_resume)
    resume: Some(acp70_pcm_resume),
};

static mut acp70_driver: platform_driver = platform_driver {
    probe: Some(acp_acp70_audio_probe),
    remove: Some(acp_acp70_audio_remove),
    driver: device_driver {
        name: b"acp_asoc_acp70\0".as_ptr() as *const core::ffi::c_char,
        pm: unsafe { pm_ptr(&acp70_dma_pm_ops) },
    },
};

// module_platform_driver(acp70_driver);
// MODULE_DESCRIPTION("AMD ACP ACP70 Driver");
// MODULE_IMPORT_NS("SND_SOC_ACP_COMMON");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
