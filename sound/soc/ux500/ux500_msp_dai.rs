// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>,
//         Roger Nilsson <roger.xr.nilsson@stericsson.com>
//         for ST-Ericsson.
//
// Depends on: linux/module.h, linux/slab.h, linux/bitops.h,
// linux/platform_device.h, linux/clk.h, linux/of.h,
// linux/regulator/consumer.h, linux/mfd/db8500-prcmu.h,
// sound/soc.h, sound/soc-dai.h, sound/dmaengine_pcm.h,
// ux500_msp_i2s.h, ux500_msp_dai.h, ux500_pcm.h

use core::ffi::{c_char, c_int, c_uint};
use core::mem;
use core::ptr;

extern "C" {
    fn dev_get_drvdata(dev: *const c_void) -> *mut c_void;
    fn dev_dbg(dev: *const c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *const c_void, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *const c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_get(dev: *const c_void, id: *const c_char) -> *mut c_void;
    fn regulator_enable(regulator: *mut c_void) -> c_int;
    fn regulator_disable(regulator: *mut c_void) -> c_int;
    fn devm_clk_get(dev: *const c_void, id: *const c_char) -> *mut c_void;
    fn clk_prepare_enable(clk: *mut c_void) -> c_int;
    fn clk_disable_unprepare(clk: *mut c_void);
    fn prcmu_qos_update_requirement(req: c_uint, name: *const c_char, value: c_int);
    fn prcmu_qos_add_requirement(req: c_uint, name: *const c_char, value: c_int);
    fn prcmu_qos_remove_requirement(req: c_uint, name: *const c_char);
    fn hweight32(w: u32) -> c_int;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut c_void,
        var: c_uint,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_single(runtime: *mut c_void, var: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *const c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut c_void,
        playback: *mut c_void,
        capture: *mut c_void,
    );
    fn snd_soc_register_component(
        dev: *const c_void,
        component_driver: *const SndSocComponentDriver,
        dai_drv: *const SndSocDaiDriver,
        num_dai: c_uint,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *const c_void);
    fn ux500_msp_i2s_init_msp(pdev: *mut c_void, msp: *mut *mut c_void) -> c_int;
    fn ux500_msp_i2s_cleanup_msp(pdev: *mut c_void, msp: *mut c_void);
    fn ux500_msp_i2s_close(msp: *mut c_void, dir: c_uint) -> c_int;
    fn ux500_msp_i2s_open(msp: *mut c_void, config: *const UX500MspConfig) -> c_int;
    fn ux500_msp_i2s_trigger(msp: *mut c_void, cmd: c_int, stream: c_uint) -> c_int;
    fn ux500_pcm_register_platform(pdev: *mut c_void) -> c_int;
    fn ux500_pcm_unregister_platform(pdev: *mut c_void);
    fn snd_pcm_stream_str(stream: c_uint) -> *const c_char;
}

const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;

const GFP_KERNEL: c_uint = 0xd0;

const TX_FIFO_ENABLE: c_uint = 1;
const RX_FIFO_ENABLE: c_uint = 1;

const MSP_DIR_TX: c_uint = 0;
const MSP_DIR_RX: c_uint = 1;

const MSP_DATA_BITS_32: c_uint = 32;
const MSP_DATA_BITS_16: c_uint = 16;

const MSP_I2S_PROTOCOL: c_uint = 0;
const MSP_PCM_PROTOCOL: c_uint = 1;

const MSP_SINGLE_PHASE: c_uint = 0;
const MSP_DUAL_PHASE: c_uint = 1;

const MSP_PHASE2_START_MODE_IMEDIATE: c_uint = 0;
const MSP_PHASE2_START_MODE_FSYNC: c_uint = 1;

const MSP_BTF_MS_BIT_FIRST: c_uint = 0;

const MSP_FSYNC_POL_ACT_HI: c_uint = 1;
const MSP_FSYNC_POL_ACT_LO: c_uint = 0;

const MSP_RISING_EDGE: c_uint = 0;
const MSP_FALLING_EDGE: c_uint = 1;

const MSP_DELAY_0: c_uint = 0;
const MSP_DELAY_1: c_uint = 1;

const MSP_SWAP_NONE: c_uint = 0;

const MSP_COMPRESS_MODE_LINEAR: c_uint = 0;
const MSP_EXPAND_MODE_LINEAR: c_uint = 0;

const MSP_FSYNC_IGNORE: c_uint = 0;

const MSP_COMPARISON_DISABLED: c_uint = 0;

const FRAME_PER_SINGLE_SLOT_8_KHZ: u32 = 1;
const FRAME_PER_SINGLE_SLOT_16_KHZ: u32 = 2;
const FRAME_PER_SINGLE_SLOT_44_1_KHZ: u32 = 5;
const FRAME_PER_SINGLE_SLOT_48_KHZ: u32 = 6;
const FRAME_PER_2_SLOTS: u32 = 2;
const FRAME_PER_8_SLOTS: u32 = 8;
const FRAME_PER_16_SLOTS: u32 = 16;

const MSP_FRAME_LEN_1: c_uint = 1;
const MSP_FRAME_LEN_2: c_uint = 2;
const MSP_FRAME_LEN_8: c_uint = 8;
const MSP_FRAME_LEN_16: c_uint = 16;

const MSP_ELEM_LEN_16: c_uint = 16;

const TFSPOL_SHIFT: c_uint = 0;
const RFSPOL_SHIFT: c_uint = 1;
const TFSSEL_SHIFT: c_uint = 2;
const SCKSEL_SHIFT: c_uint = 3;

const TX_CLK_SEL_SRG: c_uint = 1;
const TX_SYNC_SRG_PROG: c_uint = 2;
const RX_CLK_SEL_SRG: c_uint = 1;
const RX_SYNC_SRG: c_uint = 2;

const UX500_MSP_MASTER_CLOCK: c_int = 0;

const MSP_INPUT_FREQ_APB: u32 = 48000000;

const UX500_MSP_MIN_CHANNELS: c_uint = 1;
const UX500_MSP_MAX_CHANNELS: c_uint = 8;

const UX500_I2S_RATES: c_uint = 0;
const UX500_I2S_FORMATS: c_uint = 0;

const PRCMU_QOS_APE_OPP: c_uint = 0;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x0f;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;

const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 3;
const SND_SOC_DAIFMT_DSP_B: c_uint = 4;

const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x10;

const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x100;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x300;

const SNDRV_PCM_STREAM_PLAYBACK: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_uint = 1;

const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 1;

#[repr(C)]
struct UX500MspI2sDrvdata {
    fmt: c_uint,
    slots: c_int,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slot_width: c_int,
    master_clk: u32,
    reg_vape: *mut c_void,
    pclk: *mut c_void,
    clk: *mut c_void,
    msp: *mut c_void,
    vape_opp_constraint: c_int,
}

#[repr(C)]
struct MspMultichannelConfig {
    tx_multichannel_enable: bool,
    rx_multichannel_enable: bool,
    rx_comparison_enable_mode: c_uint,
    tx_channel_0_enable: c_uint,
    tx_channel_1_enable: c_uint,
    tx_channel_2_enable: c_uint,
    tx_channel_3_enable: c_uint,
    rx_channel_0_enable: c_uint,
    rx_channel_1_enable: c_uint,
    rx_channel_2_enable: c_uint,
    rx_channel_3_enable: c_uint,
}

#[repr(C)]
struct MspProtdesc {
    rx_phase_mode: c_uint,
    tx_phase_mode: c_uint,
    rx_phase2_start_mode: c_uint,
    tx_phase2_start_mode: c_uint,
    rx_byte_order: c_uint,
    tx_byte_order: c_uint,
    tx_fsync_pol: c_uint,
    rx_fsync_pol: c_uint,
    rx_frame_len_1: c_uint,
    rx_frame_len_2: c_uint,
    tx_frame_len_1: c_uint,
    tx_frame_len_2: c_uint,
    rx_elem_len_1: c_uint,
    rx_elem_len_2: c_uint,
    tx_elem_len_1: c_uint,
    tx_elem_len_2: c_uint,
    rx_clk_pol: c_uint,
    tx_clk_pol: c_uint,
    rx_data_delay: c_uint,
    tx_data_delay: c_uint,
    tx_half_word_swap: c_uint,
    rx_half_word_swap: c_uint,
    compression_mode: c_uint,
    expansion_mode: c_uint,
    frame_sync_ignore: c_uint,
    frame_period: u32,
    clocks_per_frame: u32,
    frame_width: u32,
}

#[repr(C)]
struct UX500MspConfig {
    f_inputclk: u32,
    tx_fifo_config: c_uint,
    rx_fifo_config: c_uint,
    def_elem_len: c_int,
    direction: c_uint,
    data_size: c_uint,
    frame_freq: u32,
    default_protdesc: c_int,
    protocol: c_uint,
    multichannel_configured: c_int,
    tx_fsync_pol: c_uint,
    rx_fsync_pol: c_uint,
    tx_fsync_sel: c_uint,
    rx_fsync_sel: c_uint,
    tx_clk_sel: c_uint,
    rx_clk_sel: c_uint,
    srg_clk_sel: c_uint,
    iodelay: c_uint,
    protdesc: MspProtdesc,
    multichannel_config: MspMultichannelConfig,
}

#[repr(C)]
struct SndSocDaiOps {
    probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut c_void, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut c_void, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint, c_int, c_int) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    prepare: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>,
}

#[repr(C)]
struct SndSocDaiDriver {
    playback_channels_min: c_uint,
    playback_channels_max: c_uint,
    playback_rates: c_uint,
    playback_formats: c_uint,
    capture_channels_min: c_uint,
    capture_channels_max: c_uint,
    capture_rates: c_uint,
    capture_formats: c_uint,
    ops: *const SndSocDaiOps,
}

#[repr(C)]
struct SndSocComponentDriver {
    name: *const c_char,
    legacy_dai_naming: c_int,
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct PlatformDriverDriver {
    name: *const c_char,
    of_match_table: *const OfDeviceId,
}

#[repr(C)]
struct PlatformDriver {
    driver: PlatformDriverDriver,
    probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut c_void)>,
}

fn setup_pcm_multichan(
    dai: *mut c_void,
    msp_config: *mut UX500MspConfig,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;
        let multi = &mut (*msp_config).multichannel_config;

        if (*drvdata).slots > 1 {
            (*msp_config).multichannel_configured = 1;

            multi.tx_multichannel_enable = true;
            multi.rx_multichannel_enable = true;
            multi.rx_comparison_enable_mode = MSP_COMPARISON_DISABLED;

            multi.tx_channel_0_enable = (*drvdata).tx_mask;
            multi.tx_channel_1_enable = 0;
            multi.tx_channel_2_enable = 0;
            multi.tx_channel_3_enable = 0;

            multi.rx_channel_0_enable = (*drvdata).rx_mask;
            multi.rx_channel_1_enable = 0;
            multi.rx_channel_2_enable = 0;
            multi.rx_channel_3_enable = 0;

            dev_dbg(
                dai,
                b"%s: Multichannel enabled. Slots: %d, TX: %u, RX: %u\n\0".as_ptr() as *const c_char,
                "setup_pcm_multichan\0".as_ptr() as *const c_char,
                (*drvdata).slots,
                multi.tx_channel_0_enable,
                multi.rx_channel_0_enable,
            );
        }

        0
    }
}

fn setup_frameper(
    dai: *mut c_void,
    rate: c_uint,
    prot_desc: *mut MspProtdesc,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        match (*drvdata).slots {
            1 => {
                match rate {
                    8000 => {
                        (*prot_desc).frame_period = FRAME_PER_SINGLE_SLOT_8_KHZ;
                    }
                    16000 => {
                        (*prot_desc).frame_period = FRAME_PER_SINGLE_SLOT_16_KHZ;
                    }
                    44100 => {
                        (*prot_desc).frame_period = FRAME_PER_SINGLE_SLOT_44_1_KHZ;
                    }
                    48000 => {
                        (*prot_desc).frame_period = FRAME_PER_SINGLE_SLOT_48_KHZ;
                    }
                    _ => {
                        dev_err(
                            dai,
                            b"%s: Error: Unsupported sample-rate (freq = %d)!\n\0".as_ptr() as *const c_char,
                            "setup_frameper\0".as_ptr() as *const c_char,
                            rate,
                        );
                        return EINVAL;
                    }
                }
            }
            2 => {
                (*prot_desc).frame_period = FRAME_PER_2_SLOTS;
            }
            8 => {
                (*prot_desc).frame_period = FRAME_PER_8_SLOTS;
            }
            16 => {
                (*prot_desc).frame_period = FRAME_PER_16_SLOTS;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported slot-count (slots = %d)!\n\0".as_ptr() as *const c_char,
                    "setup_frameper\0".as_ptr() as *const c_char,
                    (*drvdata).slots,
                );
                return EINVAL;
            }
        }

        (*prot_desc).clocks_per_frame = (*prot_desc).frame_period.wrapping_add(1);

        dev_dbg(
            dai,
            b"%s: Clocks per frame: %u\n\0".as_ptr() as *const c_char,
            "setup_frameper\0".as_ptr() as *const c_char,
            (*prot_desc).clocks_per_frame,
        );

        0
    }
}

fn setup_pcm_framing(
    dai: *mut c_void,
    rate: c_uint,
    prot_desc: *mut MspProtdesc,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;
        let mut frame_length = MSP_FRAME_LEN_1;

        (*prot_desc).frame_width = 0;

        match (*drvdata).slots {
            1 => {
                frame_length = MSP_FRAME_LEN_1;
            }
            2 => {
                frame_length = MSP_FRAME_LEN_2;
            }
            8 => {
                frame_length = MSP_FRAME_LEN_8;
            }
            16 => {
                frame_length = MSP_FRAME_LEN_16;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported slot-count (slots = %d)!\n\0".as_ptr() as *const c_char,
                    "setup_pcm_framing\0".as_ptr() as *const c_char,
                    (*drvdata).slots,
                );
                return EINVAL;
            }
        }

        (*prot_desc).tx_frame_len_1 = frame_length;
        (*prot_desc).rx_frame_len_1 = frame_length;
        (*prot_desc).tx_frame_len_2 = frame_length;
        (*prot_desc).rx_frame_len_2 = frame_length;

        (*prot_desc).tx_elem_len_1 = MSP_ELEM_LEN_16;
        (*prot_desc).rx_elem_len_1 = MSP_ELEM_LEN_16;
        (*prot_desc).tx_elem_len_2 = MSP_ELEM_LEN_16;
        (*prot_desc).rx_elem_len_2 = MSP_ELEM_LEN_16;

        setup_frameper(dai, rate, prot_desc)
    }
}

fn setup_clocking(
    dai: *mut c_void,
    fmt: c_uint,
    msp_config: *mut UX500MspConfig,
) -> c_int {
    unsafe {
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => {
                (*msp_config).tx_fsync_pol ^= 1 << TFSPOL_SHIFT;
                (*msp_config).rx_fsync_pol ^= 1 << RFSPOL_SHIFT;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported inversion (fmt = 0x%x)!\n\0".as_ptr() as *const c_char,
                    "setup_clocking\0".as_ptr() as *const c_char,
                    fmt,
                );
                return EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BC_FC => {
                dev_dbg(
                    dai,
                    b"%s: Codec is master.\n\0".as_ptr() as *const c_char,
                    "setup_clocking\0".as_ptr() as *const c_char,
                );

                (*msp_config).iodelay = 0x20;
                (*msp_config).rx_fsync_sel = 0;
                (*msp_config).tx_fsync_sel = 1 << TFSSEL_SHIFT;
                (*msp_config).tx_clk_sel = 0;
                (*msp_config).rx_clk_sel = 0;
                (*msp_config).srg_clk_sel = 0x2 << SCKSEL_SHIFT;
            }
            SND_SOC_DAIFMT_BP_FP => {
                dev_dbg(
                    dai,
                    b"%s: Codec is slave.\n\0".as_ptr() as *const c_char,
                    "setup_clocking\0".as_ptr() as *const c_char,
                );

                (*msp_config).tx_clk_sel = TX_CLK_SEL_SRG;
                (*msp_config).tx_fsync_sel = TX_SYNC_SRG_PROG;
                (*msp_config).rx_clk_sel = RX_CLK_SEL_SRG;
                (*msp_config).rx_fsync_sel = RX_SYNC_SRG;
                (*msp_config).srg_clk_sel = 1 << SCKSEL_SHIFT;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported master (fmt = 0x%x)!\n\0".as_ptr() as *const c_char,
                    "setup_clocking\0".as_ptr() as *const c_char,
                    fmt,
                );
                return EINVAL;
            }
        }

        0
    }
}

fn setup_pcm_protdesc(
    dai: *mut c_void,
    fmt: c_uint,
    prot_desc: *mut MspProtdesc,
) -> c_int {
    unsafe {
        (*prot_desc).rx_phase_mode = MSP_SINGLE_PHASE;
        (*prot_desc).tx_phase_mode = MSP_SINGLE_PHASE;
        (*prot_desc).rx_phase2_start_mode = MSP_PHASE2_START_MODE_IMEDIATE;
        (*prot_desc).tx_phase2_start_mode = MSP_PHASE2_START_MODE_IMEDIATE;
        (*prot_desc).rx_byte_order = MSP_BTF_MS_BIT_FIRST;
        (*prot_desc).tx_byte_order = MSP_BTF_MS_BIT_FIRST;
        (*prot_desc).tx_fsync_pol = MSP_FSYNC_POL_ACT_HI;
        (*prot_desc).rx_fsync_pol = MSP_FSYNC_POL_ACT_HI << RFSPOL_SHIFT;

        if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
            dev_dbg(
                dai,
                b"%s: DSP_A.\n\0".as_ptr() as *const c_char,
                "setup_pcm_protdesc\0".as_ptr() as *const c_char,
            );
            (*prot_desc).rx_clk_pol = MSP_RISING_EDGE;
            (*prot_desc).tx_clk_pol = MSP_FALLING_EDGE;

            (*prot_desc).rx_data_delay = MSP_DELAY_1;
            (*prot_desc).tx_data_delay = MSP_DELAY_1;
        } else {
            dev_dbg(
                dai,
                b"%s: DSP_B.\n\0".as_ptr() as *const c_char,
                "setup_pcm_protdesc\0".as_ptr() as *const c_char,
            );
            (*prot_desc).rx_clk_pol = MSP_FALLING_EDGE;
            (*prot_desc).tx_clk_pol = MSP_RISING_EDGE;

            (*prot_desc).rx_data_delay = MSP_DELAY_0;
            (*prot_desc).tx_data_delay = MSP_DELAY_0;
        }

        (*prot_desc).rx_half_word_swap = MSP_SWAP_NONE;
        (*prot_desc).tx_half_word_swap = MSP_SWAP_NONE;
        (*prot_desc).compression_mode = MSP_COMPRESS_MODE_LINEAR;
        (*prot_desc).expansion_mode = MSP_EXPAND_MODE_LINEAR;
        (*prot_desc).frame_sync_ignore = MSP_FSYNC_IGNORE;

        0
    }
}

fn setup_i2s_protdesc(prot_desc: *mut MspProtdesc) -> c_int {
    unsafe {
        (*prot_desc).rx_phase_mode = MSP_DUAL_PHASE;
        (*prot_desc).tx_phase_mode = MSP_DUAL_PHASE;
        (*prot_desc).rx_phase2_start_mode = MSP_PHASE2_START_MODE_FSYNC;
        (*prot_desc).tx_phase2_start_mode = MSP_PHASE2_START_MODE_FSYNC;
        (*prot_desc).rx_byte_order = MSP_BTF_MS_BIT_FIRST;
        (*prot_desc).tx_byte_order = MSP_BTF_MS_BIT_FIRST;
        (*prot_desc).tx_fsync_pol = MSP_FSYNC_POL_ACT_LO;
        (*prot_desc).rx_fsync_pol = MSP_FSYNC_POL_ACT_LO << RFSPOL_SHIFT;

        (*prot_desc).rx_frame_len_1 = MSP_FRAME_LEN_1;
        (*prot_desc).rx_frame_len_2 = MSP_FRAME_LEN_1;
        (*prot_desc).tx_frame_len_1 = MSP_FRAME_LEN_1;
        (*prot_desc).tx_frame_len_2 = MSP_FRAME_LEN_1;
        (*prot_desc).rx_elem_len_1 = MSP_ELEM_LEN_16;
        (*prot_desc).rx_elem_len_2 = MSP_ELEM_LEN_16;
        (*prot_desc).tx_elem_len_1 = MSP_ELEM_LEN_16;
        (*prot_desc).tx_elem_len_2 = MSP_ELEM_LEN_16;

        (*prot_desc).rx_clk_pol = MSP_RISING_EDGE;
        (*prot_desc).tx_clk_pol = MSP_FALLING_EDGE;

        (*prot_desc).rx_data_delay = MSP_DELAY_0;
        (*prot_desc).tx_data_delay = MSP_DELAY_0;

        (*prot_desc).tx_half_word_swap = MSP_SWAP_NONE;
        (*prot_desc).rx_half_word_swap = MSP_SWAP_NONE;
        (*prot_desc).compression_mode = MSP_COMPRESS_MODE_LINEAR;
        (*prot_desc).expansion_mode = MSP_EXPAND_MODE_LINEAR;
        (*prot_desc).frame_sync_ignore = MSP_FSYNC_IGNORE;

        0
    }
}

fn setup_msp_config(
    substream: *mut c_void,
    dai: *mut c_void,
    msp_config: *mut UX500MspConfig,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;
        let prot_desc = &mut (*msp_config).protdesc;
        let fmt = (*drvdata).fmt;

        memset(msp_config as *mut c_void, 0, mem::size_of::<UX500MspConfig>());

        (*msp_config).f_inputclk = (*drvdata).master_clk;

        (*msp_config).tx_fifo_config = TX_FIFO_ENABLE;
        (*msp_config).rx_fifo_config = RX_FIFO_ENABLE;
        (*msp_config).def_elem_len = 1;
        (*msp_config).direction = 0;
        (*msp_config).data_size = MSP_DATA_BITS_32;
        (*msp_config).frame_freq = 0;

        dev_dbg(
            dai,
            b"%s: f_inputclk = %u, frame_freq = %u.\n\0".as_ptr() as *const c_char,
            "setup_msp_config\0".as_ptr() as *const c_char,
            (*msp_config).f_inputclk,
            (*msp_config).frame_freq,
        );
        prot_desc.clocks_per_frame = 1;

        dev_dbg(
            dai,
            b"%s: rate: %u, channels: %d.\n\0".as_ptr() as *const c_char,
            "setup_msp_config\0".as_ptr() as *const c_char,
            0,
            0,
        );

        match fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
            _ if (fmt & SND_SOC_DAIFMT_FORMAT_MASK == SND_SOC_DAIFMT_I2S)
                && ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP) =>
            {
                dev_dbg(
                    dai,
                    b"%s: SND_SOC_DAIFMT_I2S.\n\0".as_ptr() as *const c_char,
                    "setup_msp_config\0".as_ptr() as *const c_char,
                );

                (*msp_config).default_protdesc = 1;
                (*msp_config).protocol = MSP_I2S_PROTOCOL;
            }
            _ if (fmt & SND_SOC_DAIFMT_FORMAT_MASK == SND_SOC_DAIFMT_I2S)
                && ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BC_FC) =>
            {
                dev_dbg(
                    dai,
                    b"%s: SND_SOC_DAIFMT_I2S.\n\0".as_ptr() as *const c_char,
                    "setup_msp_config\0".as_ptr() as *const c_char,
                );

                (*msp_config).data_size = MSP_DATA_BITS_16;
                (*msp_config).protocol = MSP_I2S_PROTOCOL;

                let ret = setup_i2s_protdesc(prot_desc);
                if ret < 0 {
                    return ret;
                }
            }
            _ if ((fmt & SND_SOC_DAIFMT_FORMAT_MASK == SND_SOC_DAIFMT_DSP_A)
                || (fmt & SND_SOC_DAIFMT_FORMAT_MASK == SND_SOC_DAIFMT_DSP_B)) =>
            {
                dev_dbg(
                    dai,
                    b"%s: PCM format.\n\0".as_ptr() as *const c_char,
                    "setup_msp_config\0".as_ptr() as *const c_char,
                );

                (*msp_config).data_size = MSP_DATA_BITS_16;
                (*msp_config).protocol = MSP_PCM_PROTOCOL;

                let ret = setup_pcm_protdesc(dai, fmt, prot_desc);
                if ret < 0 {
                    return ret;
                }

                let ret = setup_pcm_multichan(dai, msp_config);
                if ret < 0 {
                    return ret;
                }

                let ret = setup_pcm_framing(dai, 0, prot_desc);
                if ret < 0 {
                    return ret;
                }
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported format (%d)!\n\0".as_ptr() as *const c_char,
                    "setup_msp_config\0".as_ptr() as *const c_char,
                    fmt,
                );
                return EINVAL;
            }
        }

        setup_clocking(dai, fmt, msp_config)
    }
}

fn ux500_msp_dai_startup(substream: *mut c_void, dai: *mut c_void) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d (%s): Enter.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_startup\0".as_ptr() as *const c_char,
            0,
            snd_pcm_stream_str(0),
        );

        let ret = regulator_enable((*drvdata).reg_vape);
        if ret != 0 {
            dev_err(
                (*drvdata).msp,
                b"%s: Failed to enable regulator!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_startup\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        dev_dbg(
            dai,
            b"%s: Enabling MSP-clocks.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_startup\0".as_ptr() as *const c_char,
        );
        let ret = clk_prepare_enable((*drvdata).pclk);
        if ret != 0 {
            dev_err(
                (*drvdata).msp,
                b"%s: Failed to prepare/enable pclk!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_startup\0".as_ptr() as *const c_char,
            );
            regulator_disable((*drvdata).reg_vape);
            return ret;
        }

        let ret = clk_prepare_enable((*drvdata).clk);
        if ret != 0 {
            dev_err(
                (*drvdata).msp,
                b"%s: Failed to prepare/enable clk!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_startup\0".as_ptr() as *const c_char,
            );
            clk_disable_unprepare((*drvdata).pclk);
            regulator_disable((*drvdata).reg_vape);
            return ret;
        }

        0
    }
}

fn ux500_msp_dai_shutdown(substream: *mut c_void, dai: *mut c_void) {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d (%s): Enter.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_shutdown\0".as_ptr() as *const c_char,
            0,
            snd_pcm_stream_str(0),
        );

        if (*drvdata).vape_opp_constraint == 1 {
            prcmu_qos_update_requirement(PRCMU_QOS_APE_OPP,
                        b"ux500_msp_i2s\0".as_ptr() as *const c_char, 50);
            (*drvdata).vape_opp_constraint = 0;
        }

        if ux500_msp_i2s_close((*drvdata).msp, MSP_DIR_TX) != 0 {
            dev_err(
                dai,
                b"%s: Error: MSP %d (%s): Unable to close i2s.\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_shutdown\0".as_ptr() as *const c_char,
                0,
                snd_pcm_stream_str(0),
            );
        }

        clk_disable_unprepare((*drvdata).clk);
        clk_disable_unprepare((*drvdata).pclk);

        let ret = regulator_disable((*drvdata).reg_vape);
        if ret < 0 {
            dev_err(
                dai,
                b"%s: ERROR: Failed to disable regulator (%d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_shutdown\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }
}

fn ux500_msp_dai_prepare(substream: *mut c_void, dai: *mut c_void) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d (%s): Enter (rate = %d).\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_prepare\0".as_ptr() as *const c_char,
            0,
            snd_pcm_stream_str(0),
            0,
        );

        let mut msp_config: UX500MspConfig = mem::zeroed();
        setup_msp_config(substream, dai, &mut msp_config);

        let ret = ux500_msp_i2s_open((*drvdata).msp, &msp_config);
        if ret < 0 {
            dev_err(
                dai,
                b"%s: Error: msp_setup failed (ret = %d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_prepare\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        if (((*drvdata).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != 0)
            && ((*(*drvdata).msp as *const c_void) as usize != 0)
        {
            prcmu_qos_update_requirement(PRCMU_QOS_APE_OPP,
                        b"ux500-msp-i2s\0".as_ptr() as *const c_char, 100);
            (*drvdata).vape_opp_constraint = 1;
        } else {
            prcmu_qos_update_requirement(PRCMU_QOS_APE_OPP,
                        b"ux500-msp-i2s\0".as_ptr() as *const c_char, 50);
            (*drvdata).vape_opp_constraint = 0;
        }

        0
    }
}

fn ux500_msp_dai_hw_params(
    substream: *mut c_void,
    params: *mut c_void,
    dai: *mut c_void,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d (%s): Enter.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_hw_params\0".as_ptr() as *const c_char,
            0,
            snd_pcm_stream_str(0),
        );

        match (*drvdata).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {
                snd_pcm_hw_constraint_minmax(substream, SNDRV_PCM_HW_PARAM_CHANNELS, 1, 2);
            }
            SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_DSP_A => {
                let mask = (*drvdata).tx_mask;
                let slots_active = hweight32(mask);
                dev_dbg(
                    dai,
                    b"TDM-slots active: %d\0".as_ptr() as *const c_char,
                    slots_active,
                );

                snd_pcm_hw_constraint_single(substream, SNDRV_PCM_HW_PARAM_CHANNELS, slots_active as c_uint);
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported protocol (fmt = 0x%x)!\n\0".as_ptr() as *const c_char,
                    "ux500_msp_dai_hw_params\0".as_ptr() as *const c_char,
                    (*drvdata).fmt,
                );
                return EINVAL;
            }
        }

        0
    }
}

fn ux500_msp_dai_set_dai_fmt(dai: *mut c_void, fmt: c_uint) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d: Enter.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_set_dai_fmt\0".as_ptr() as *const c_char,
            0,
        );

        match fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
            _ if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S => {}
            _ if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B => {}
            _ if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A => {}
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported protocol/master (fmt = 0x%x)!\n\0".as_ptr() as *const c_char,
                    "ux500_msp_dai_set_dai_fmt\0".as_ptr() as *const c_char,
                    (*drvdata).fmt,
                );
                return EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF => {}
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported inversion (fmt = 0x%x)!\n\0".as_ptr() as *const c_char,
                    "ux500_msp_dai_set_dai_fmt\0".as_ptr() as *const c_char,
                    (*drvdata).fmt,
                );
                return EINVAL;
            }
        }

        (*drvdata).fmt = fmt;
        0
    }
}

fn ux500_msp_dai_set_tdm_slot(
    dai: *mut c_void,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;
        let cap: c_uint;

        match slots {
            1 => {
                cap = 0x01;
            }
            2 => {
                cap = 0x03;
            }
            8 => {
                cap = 0xFF;
            }
            16 => {
                cap = 0xFFFF;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: Error: Unsupported slot-count (%d)!\n\0".as_ptr() as *const c_char,
                    "ux500_msp_dai_set_tdm_slot\0".as_ptr() as *const c_char,
                    slots,
                );
                return EINVAL;
            }
        }
        (*drvdata).slots = slots;

        if slot_width != 16 {
            dev_err(
                dai,
                b"%s: Error: Unsupported slot-width (%d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_dai_set_tdm_slot\0".as_ptr() as *const c_char,
                slot_width,
            );
            return EINVAL;
        }
        (*drvdata).slot_width = slot_width;

        (*drvdata).tx_mask = tx_mask & cap;
        (*drvdata).rx_mask = rx_mask & cap;

        0
    }
}

fn ux500_msp_dai_set_dai_sysclk(dai: *mut c_void, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d: Enter. clk-id: %d, freq: %u.\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_set_dai_sysclk\0".as_ptr() as *const c_char,
            0,
            clk_id,
            freq,
        );

        match clk_id {
            UX500_MSP_MASTER_CLOCK => {
                (*drvdata).master_clk = freq;
            }
            _ => {
                dev_err(
                    dai,
                    b"%s: MSP %d: Invalid clk-id (%d)!\n\0".as_ptr() as *const c_char,
                    "ux500_msp_dai_set_dai_sysclk\0".as_ptr() as *const c_char,
                    0,
                    clk_id,
                );
                return EINVAL;
            }
        }

        0
    }
}

fn ux500_msp_dai_trigger(substream: *mut c_void, cmd: c_int, dai: *mut c_void) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        dev_dbg(
            dai,
            b"%s: MSP %d (%s): Enter (msp->id = %d, cmd = %d).\n\0".as_ptr() as *const c_char,
            "ux500_msp_dai_trigger\0".as_ptr() as *const c_char,
            0,
            snd_pcm_stream_str(0),
            0,
            cmd,
        );

        ux500_msp_i2s_trigger((*drvdata).msp, cmd, 0)
    }
}

fn ux500_msp_dai_of_probe(dai: *mut c_void) -> c_int {
    unsafe {
        let drvdata = dev_get_drvdata(dai) as *mut UX500MspI2sDrvdata;

        let playback_dma_data = devm_kzalloc(dai, mem::size_of::<c_void>() * 2, GFP_KERNEL);
        if playback_dma_data.is_null() {
            return ENOMEM;
        }

        let capture_dma_data = devm_kzalloc(dai, mem::size_of::<c_void>() * 2, GFP_KERNEL);
        if capture_dma_data.is_null() {
            return ENOMEM;
        }

        snd_soc_dai_init_dma_data(dai, playback_dma_data, capture_dma_data);

        0
    }
}

static UX500_MSP_DAI_OPS: SndSocDaiOps = SndSocDaiOps {
    probe: Some(ux500_msp_dai_of_probe),
    set_sysclk: Some(ux500_msp_dai_set_dai_sysclk),
    set_fmt: Some(ux500_msp_dai_set_dai_fmt),
    set_tdm_slot: Some(ux500_msp_dai_set_tdm_slot),
    startup: Some(ux500_msp_dai_startup),
    shutdown: Some(ux500_msp_dai_shutdown),
    prepare: Some(ux500_msp_dai_prepare),
    trigger: Some(ux500_msp_dai_trigger),
    hw_params: Some(ux500_msp_dai_hw_params),
};

static UX500_MSP_DAI_DRV: SndSocDaiDriver = SndSocDaiDriver {
    playback_channels_min: UX500_MSP_MIN_CHANNELS,
    playback_channels_max: UX500_MSP_MAX_CHANNELS,
    playback_rates: UX500_I2S_RATES,
    playback_formats: UX500_I2S_FORMATS,
    capture_channels_min: UX500_MSP_MIN_CHANNELS,
    capture_channels_max: UX500_MSP_MAX_CHANNELS,
    capture_rates: UX500_I2S_RATES,
    capture_formats: UX500_I2S_FORMATS,
    ops: &UX500_MSP_DAI_OPS,
};

static UX500_MSP_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: b"ux500-msp\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

fn ux500_msp_drv_probe(pdev: *mut c_void) -> c_int {
    unsafe {
        let drvdata = devm_kzalloc(pdev, mem::size_of::<UX500MspI2sDrvdata>(), GFP_KERNEL);
        if drvdata.is_null() {
            return ENOMEM;
        }

        let drvdata = drvdata as *mut UX500MspI2sDrvdata;

        (*drvdata).fmt = 0;
        (*drvdata).slots = 1;
        (*drvdata).tx_mask = 0x01;
        (*drvdata).rx_mask = 0x01;
        (*drvdata).slot_width = 16;
        (*drvdata).master_clk = MSP_INPUT_FREQ_APB;

        (*drvdata).reg_vape = devm_regulator_get(pdev, b"v-ape\0".as_ptr() as *const c_char);
        if IS_ERR((*drvdata).reg_vape) {
            let ret = PTR_ERR((*drvdata).reg_vape);
            dev_err(
                pdev,
                b"%s: ERROR: Failed to get Vape supply (%d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
        prcmu_qos_add_requirement(PRCMU_QOS_APE_OPP, b"msp_i2s_driver\0".as_ptr() as *const c_char, 50);

        (*drvdata).pclk = devm_clk_get(pdev, b"apb_pclk\0".as_ptr() as *const c_char);
        if IS_ERR((*drvdata).pclk) {
            let ret = PTR_ERR((*drvdata).pclk);
            dev_err(
                pdev,
                b"%s: ERROR: devm_clk_get of pclk failed (%d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        (*drvdata).clk = devm_clk_get(pdev, ptr::null());
        if IS_ERR((*drvdata).clk) {
            let ret = PTR_ERR((*drvdata).clk);
            dev_err(
                pdev,
                b"%s: ERROR: devm_clk_get failed (%d)!\n\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        let mut msp: *mut c_void = ptr::null_mut();
        let ret = ux500_msp_i2s_init_msp(pdev, &mut msp);
        if msp.is_null() {
            dev_err(
                pdev,
                b"%s: ERROR: Failed to init MSP-struct (%d)!\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
        (*drvdata).msp = msp;

        let ret = snd_soc_register_component(pdev, &UX500_MSP_COMPONENT, &UX500_MSP_DAI_DRV, 1);
        if ret < 0 {
            dev_err(
                pdev,
                b"Error: %s: Failed to register MSP%d!\n\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
                0,
            );
            return ret;
        }

        let ret = ux500_pcm_register_platform(pdev);
        if ret < 0 {
            dev_err(
                pdev,
                b"Error: %s: Failed to register PCM platform device!\n\0".as_ptr() as *const c_char,
                "ux500_msp_drv_probe\0".as_ptr() as *const c_char,
            );
            snd_soc_unregister_component(pdev);
            return ret;
        }

        0
    }
}

fn ux500_msp_drv_remove(pdev: *mut c_void) {
    unsafe {
        let drvdata = dev_get_drvdata(pdev) as *mut UX500MspI2sDrvdata;

        ux500_pcm_unregister_platform(pdev);

        snd_soc_unregister_component(pdev);

        prcmu_qos_remove_requirement(PRCMU_QOS_APE_OPP, b"ux500_msp_i2s\0".as_ptr() as *const c_char);

        ux500_msp_i2s_cleanup_msp(pdev, (*drvdata).msp);
    }
}

static UX500_MSP_I2S_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"stericsson,ux500-msp-i2s\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    OfDeviceId {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

static MSP_I2S_DRIVER: PlatformDriver = PlatformDriver {
    driver: PlatformDriverDriver {
        name: b"ux500-msp-i2s\0".as_ptr() as *const c_char,
        of_match_table: unsafe { &UX500_MSP_I2S_MATCH[0] },
    },
    probe: Some(ux500_msp_drv_probe),
    remove: Some(ux500_msp_drv_remove),
};

// MODULE_DEVICE_TABLE(of, ux500_msp_i2s_match);
// module_platform_driver(msp_i2s_driver);
// MODULE_DESCRIPTION("ASoC Ux500 I2S driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
