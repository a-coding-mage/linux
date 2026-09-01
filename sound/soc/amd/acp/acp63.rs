// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Syed Saba kareem <syed.sabakareem@amd.com>
/*
 * Hardware interface for ACP6.3 block
 */

// Dependencies in the original C source:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h, linux/dma-mapping.h,
// linux/pm_runtime.h, linux/pci.h, asm/amd/node.h, amd.h, acp-mach.h,
// ../mach-config.h

const DRV_NAME: &[u8] = b"acp_asoc_acp63\0";

const CLK_PLL_PWR_REQ_N0: u32 = 0x0006C2C0;
const CLK_SPLL_FIELD_2_N0: u32 = 0x0006C114;
const CLK_PLL_REQ_N0: u32 = 0x0006C0DC;
const CLK_DFSBYPASS_CONTR: u32 = 0x0006C2C8;
const CLK_DFS_CNTL_N0: u32 = 0x0006C1A4;

const PLL_AUTO_STOP_REQ: u32 = 1 << 4;
const PLL_AUTO_START_REQ: u32 = 1 << 0;
const PLL_FRANCE_EN: u32 = 1 << 4;
const EXIT_DPF_BYPASS_0: u32 = 1 << 16;
const EXIT_DPF_BYPASS_1: u32 = 1 << 17;
const CLK0_DIVIDER: u32 = 0x30;

const NULL: *mut core::ffi::c_void = core::ptr::null_mut();

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
    pub id: i32,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct resource {
    pub soc_mclk: bool,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp_chip_info {
    pub base: *mut core::ffi::c_void,
    pub acp_rev: i32,
    pub dev: *mut device,
    pub dai_driver: *mut snd_soc_dai_driver,
    pub num_dai: usize,
    pub is_i2s_config: bool,
    pub rsrc: *mut resource,
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
    pub list: list_head,
    pub substream: *mut snd_pcm_substream,
    pub dai_id: i32,
}

#[allow(non_camel_case_types)]
pub type snd_pcm_uframes_t = u64;

#[repr(C)]
pub struct dev_pm_ops {
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
union clk_pll_req_no {
    bitfields: clk_pll_req_no_bits,
    bits: clk_pll_req_no_bits,
    clk_pll_req_no_reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct clk_pll_req_no_bits {
    value: u32,
}

impl clk_pll_req_no_bits {
    unsafe fn set_fb_mult_int(&mut self, val: u32) {
        self.value = (self.value & !(0x1ff << 0)) | ((val & 0x1ff) << 0);
    }

    unsafe fn set_pll_spine_div(&mut self, val: u32) {
        self.value = (self.value & !(0x0f << 12)) | ((val & 0x0f) << 12);
    }

    unsafe fn set_gb_mult_frac(&mut self, val: u32) {
        self.value = (self.value & !(0xffff << 16)) | ((val & 0xffff) << 16);
    }
}

unsafe extern "C" {
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
    static ACP63_PCI_ID: i32;
    static ACP_SUSPEND_DELAY_MS: i32;
    static ENODEV: i32;

    fn amd_smn_read(node: u16, address: u32, value: *mut u32) -> i32;
    fn amd_smn_write(node: u16, address: u32, value: u32) -> i32;
    fn dev_get_platdata(dev: *mut device) -> *mut acp_chip_info;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
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

unsafe fn list_entry(ptr: *mut list_head) -> *mut acp_stream {
    (ptr as *mut u8).sub(core::mem::offset_of!(acp_stream, list)) as *mut acp_stream
}

static mut acp63_dai: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: b"acp-i2s-sp\0".as_ptr() as *const core::ffi::c_char,
        id: unsafe { I2S_SP_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S SP Playback\0".as_ptr() as *const core::ffi::c_char,
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
            stream_name: b"I2S SP Capture\0".as_ptr() as *const core::ffi::c_char,
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
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-bt\0".as_ptr() as *const core::ffi::c_char,
        id: unsafe { I2S_BT_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S BT Playback\0".as_ptr() as *const core::ffi::c_char,
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
            stream_name: b"I2S BT Capture\0".as_ptr() as *const core::ffi::c_char,
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
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-hs\0".as_ptr() as *const core::ffi::c_char,
        id: unsafe { I2S_HS_INSTANCE },
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S HS Playback\0".as_ptr() as *const core::ffi::c_char,
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
            stream_name: b"I2S HS Capture\0".as_ptr() as *const core::ffi::c_char,
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
        ops: unsafe { &asoc_acp_cpu_dai_ops },
    },
    snd_soc_dai_driver {
        name: b"acp-pdm-dmic\0".as_ptr() as *const core::ffi::c_char,
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
        ops: unsafe { &acp_dmic_dai_ops },
    },
];

unsafe extern "C" fn acp63_i2s_master_clock_generate(chip: *mut acp_chip_info) -> i32 {
    let mut rc: i32;
    let mut data: u32 = 0;
    let mut clk_pll = clk_pll_req_no {
        clk_pll_req_no_reg: 0,
    };

    /* Clk5 pll register values to get mclk as 196.6MHz*/
    clk_pll.bits.set_fb_mult_int(0x31);
    clk_pll.bits.set_pll_spine_div(0);
    clk_pll.bits.set_gb_mult_frac(0x26E9);

    rc = amd_smn_read(0, CLK_PLL_PWR_REQ_N0, &mut data);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, CLK_PLL_PWR_REQ_N0, data | PLL_AUTO_STOP_REQ);
    if rc != 0 {
        return rc;
    }

    rc = amd_smn_read(0, CLK_SPLL_FIELD_2_N0, &mut data);
    if rc != 0 {
        return rc;
    }
    if (data & PLL_FRANCE_EN) != 0 {
        rc = amd_smn_write(0, CLK_SPLL_FIELD_2_N0, data | PLL_FRANCE_EN);
        if rc != 0 {
            return rc;
        }
    }

    rc = amd_smn_write(0, CLK_PLL_REQ_N0, clk_pll.clk_pll_req_no_reg);
    if rc != 0 {
        return rc;
    }

    rc = amd_smn_read(0, CLK_PLL_PWR_REQ_N0, &mut data);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, CLK_PLL_PWR_REQ_N0, data | PLL_AUTO_START_REQ);
    if rc != 0 {
        return rc;
    }

    rc = amd_smn_read(0, CLK_DFSBYPASS_CONTR, &mut data);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, CLK_DFSBYPASS_CONTR, data | EXIT_DPF_BYPASS_0);
    if rc != 0 {
        return rc;
    }
    rc = amd_smn_write(0, CLK_DFSBYPASS_CONTR, data | EXIT_DPF_BYPASS_1);
    if rc != 0 {
        return rc;
    }

    amd_smn_write(0, CLK_DFS_CNTL_N0, CLK0_DIVIDER)
}

unsafe extern "C" fn acp63_audio_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let chip: *mut acp_chip_info;
    let mut ret: i32;

    chip = dev_get_platdata(&mut (*pdev).dev);
    if chip.is_null() || (*chip).base.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"ACP chip data is NULL\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return -ENODEV;
    }

    if (*chip).acp_rev != ACP63_PCI_ID {
        dev_err(
            &mut (*pdev).dev,
            b"Un-supported ACP Revision %d\n\0".as_ptr() as *const core::ffi::c_char,
            (*chip).acp_rev,
        );
        return -ENODEV;
    }

    (*chip).dev = dev;
    (*chip).dai_driver = core::ptr::addr_of_mut!(acp63_dai) as *mut snd_soc_dai_driver;
    (*chip).num_dai = acp63_dai.len();

    if (*chip).is_i2s_config && (*(*chip).rsrc).soc_mclk {
        ret = acp63_i2s_master_clock_generate(chip);
        if ret != 0 {
            return ret;
        }
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

unsafe extern "C" fn acp63_audio_remove(pdev: *mut platform_device) {
    let dev: *mut device = &mut (*pdev).dev;
    let chip: *mut acp_chip_info = dev_get_platdata(dev);
    let ret: i32;

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

unsafe extern "C" fn acp63_pcm_resume(dev: *mut device) -> i32 {
    let chip: *mut acp_chip_info = dev_get_drvdata(*(dev as *mut *mut device)) as *mut acp_chip_info;
    let mut stream: *mut acp_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut buf_in_frames: snd_pcm_uframes_t;
    let mut buf_size: u64;

    if (*chip).is_i2s_config && (*(*chip).rsrc).soc_mclk {
        acp63_i2s_master_clock_generate(chip);
    }

    spin_lock(&mut (*chip).acp_lock);
    let mut pos = (*chip).stream_list.next;
    while pos != &mut (*chip).stream_list {
        stream = list_entry(pos);
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
        pos = (*pos).next;
    }
    spin_unlock(&mut (*chip).acp_lock);
    0
}

static acp63_dma_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(NULL, acp63_pcm_resume)
    resume: Some(acp63_pcm_resume),
};

static mut acp63_driver: platform_driver = platform_driver {
    probe: Some(acp63_audio_probe),
    remove: Some(acp63_audio_remove),
    driver: device_driver {
        name: b"acp_asoc_acp63\0".as_ptr() as *const core::ffi::c_char,
        // pm_ptr(&acp63_dma_pm_ops)
        pm: &acp63_dma_pm_ops,
    },
};

// module_platform_driver(acp63_driver);
// MODULE_DESCRIPTION("AMD ACP acp63 Driver");
// MODULE_IMPORT_NS("SND_SOC_ACP_COMMON");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
