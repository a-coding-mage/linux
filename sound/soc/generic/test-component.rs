// SPDX-License-Identifier: GPL-2.0
//
// test-component.c  --  Test Audio Component driver
//
// Copyright (C) 2020 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

use core::ffi::{c_char, c_int, c_uint, c_void};

use crate::bindings::*;

const TEST_NAME_LEN: usize = 32;

#[repr(C)]
struct test_dai_name {
    name: [c_char; TEST_NAME_LEN],
    name_playback: [c_char; TEST_NAME_LEN],
    name_capture: [c_char; TEST_NAME_LEN],
}

#[repr(C)]
struct test_priv {
    dev: *mut device,
    substream: *mut snd_pcm_substream,
    dwork: delayed_work,
    component_driver: *mut snd_soc_component_driver,
    dai_driver: *mut snd_soc_dai_driver,
    name: *mut test_dai_name,
}

#[repr(C)]
struct test_adata {
    is_cpu: u32,
    cmp_v: u32,
    dai_v: u32,
}

macro_rules! mile_stone {
    ($d:expr) => {
        dev_info!((*($d)).dev, "%s() : %s", __func__, (*(*($d)).driver).name)
    };
}

macro_rules! mile_stone_x {
    ($dev:expr) => {
        dev_info!($dev, "%s()", __func__)
    };
}

unsafe extern "C" fn test_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_set_pll(
    dai: *mut snd_soc_dai,
    pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_set_clkdiv(
    dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    let clock = fmt & SND_SOC_DAIFMT_CLOCK_MASK;
    let inv = fmt & SND_SOC_DAIFMT_INV_MASK;
    let master = fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    let mut str_: *const c_char;

    dev_info!((*dai).dev, "name   : %s", (*dai).name);

    str_ = c"unknown".as_ptr();
    match format {
        SND_SOC_DAIFMT_I2S => str_ = c"i2s".as_ptr(),
        SND_SOC_DAIFMT_RIGHT_J => str_ = c"right_j".as_ptr(),
        SND_SOC_DAIFMT_LEFT_J => str_ = c"left_j".as_ptr(),
        SND_SOC_DAIFMT_DSP_A => str_ = c"dsp_a".as_ptr(),
        SND_SOC_DAIFMT_DSP_B => str_ = c"dsp_b".as_ptr(),
        SND_SOC_DAIFMT_AC97 => str_ = c"ac97".as_ptr(),
        SND_SOC_DAIFMT_PDM => str_ = c"pdm".as_ptr(),
        _ => {}
    }
    dev_info!((*dai).dev, "format : %s", str_);

    if clock == SND_SOC_DAIFMT_CONT {
        str_ = c"continuous".as_ptr();
    } else {
        str_ = c"gated".as_ptr();
    }
    dev_info!((*dai).dev, "clock  : %s", str_);

    str_ = c"unknown".as_ptr();
    match master {
        SND_SOC_DAIFMT_BP_FP => str_ = c"clk provider, frame provider".as_ptr(),
        SND_SOC_DAIFMT_BC_FP => str_ = c"clk consumer, frame provider".as_ptr(),
        SND_SOC_DAIFMT_BP_FC => str_ = c"clk provider, frame consumer".as_ptr(),
        SND_SOC_DAIFMT_BC_FC => str_ = c"clk consumer, frame consumer".as_ptr(),
        _ => {}
    }
    dev_info!((*dai).dev, "clock  : codec is %s", str_);

    str_ = c"unknown".as_ptr();
    match inv {
        SND_SOC_DAIFMT_NB_NF => str_ = c"normal bit, normal frame".as_ptr(),
        SND_SOC_DAIFMT_NB_IF => str_ = c"normal bit, invert frame".as_ptr(),
        SND_SOC_DAIFMT_IB_NF => str_ = c"invert bit, normal frame".as_ptr(),
        SND_SOC_DAIFMT_IB_IF => str_ = c"invert bit, invert frame".as_ptr(),
        _ => {}
    }
    dev_info!((*dai).dev, "signal : %s", str_);

    0
}

unsafe extern "C" fn test_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    dev_info!(
        (*dai).dev,
        "set tdm slot: tx_mask=0x%08X, rx_mask=0x%08X, slots=%d, slot_width=%d\n",
        tx_mask,
        rx_mask,
        slots,
        slot_width
    );
    0
}

unsafe extern "C" fn test_dai_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    stream: c_int,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    mile_stone!(dai);
}

unsafe extern "C" fn test_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    mile_stone!(dai);

    0
}

unsafe extern "C" fn test_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    mile_stone!(dai);

    0
}

static test_dai_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_DSP_A
    | SND_SOC_POSSIBLE_DAIFMT_DSP_B
    | SND_SOC_POSSIBLE_DAIFMT_AC97
    | SND_SOC_POSSIBLE_DAIFMT_PDM
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF
    | SND_SOC_POSSIBLE_DAIFMT_NB_IF
    | SND_SOC_POSSIBLE_DAIFMT_IB_NF
    | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static test_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(test_dai_set_fmt),
    set_tdm_slot: Some(test_dai_set_tdm_slot),
    startup: Some(test_dai_startup),
    shutdown: Some(test_dai_shutdown),
    auto_selectable_formats: &test_dai_formats,
    num_auto_selectable_formats: 1,
    ..unsafe { core::mem::zeroed() }
};

static test_verbose_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(test_dai_set_sysclk),
    set_pll: Some(test_dai_set_pll),
    set_clkdiv: Some(test_dai_set_clkdiv),
    set_fmt: Some(test_dai_set_fmt),
    set_tdm_slot: Some(test_dai_set_tdm_slot),
    mute_stream: Some(test_dai_mute_stream),
    startup: Some(test_dai_startup),
    shutdown: Some(test_dai_shutdown),
    hw_params: Some(test_dai_hw_params),
    hw_free: Some(test_dai_hw_free),
    trigger: Some(test_dai_trigger),
    auto_selectable_formats: &test_dai_formats,
    num_auto_selectable_formats: 1,
    ..unsafe { core::mem::zeroed() }
};

const STUB_RATES: u64 = SNDRV_PCM_RATE_CONTINUOUS;
const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_U32_LE;

unsafe extern "C" fn test_component_probe(component: *mut snd_soc_component) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_remove(component: *mut snd_soc_component) {
    mile_stone!(component);
}

unsafe extern "C" fn test_component_suspend(component: *mut snd_soc_component) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_resume(component: *mut snd_soc_component) -> c_int {
    mile_stone!(component);

    0
}

const PREALLOC_BUFFER: usize = 32 * 1024;

unsafe extern "C" fn test_component_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    mile_stone!(component);

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*(*(*rtd).card).snd_card).dev,
        PREALLOC_BUFFER,
        PREALLOC_BUFFER,
    );

    0
}

unsafe extern "C" fn test_component_pcm_free(
    component: *mut snd_soc_component,
    pcm: *mut snd_pcm,
) {
    mile_stone!(component);
}

unsafe extern "C" fn test_component_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    source: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_set_pll(
    component: *mut snd_soc_component,
    pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    data: *mut c_void,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_seq_notifier(
    component: *mut snd_soc_component,
    type_: snd_soc_dapm_type,
    subseq: c_int,
) {
    mile_stone!(component);
}

unsafe extern "C" fn test_component_stream_event(
    component: *mut snd_soc_component,
    event: c_int,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    mile_stone!(component);

    0
}

static test_component_hardware: snd_pcm_hardware = snd_pcm_hardware {
    /* Random values to keep userspace happy when checking constraints */
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 8192,
    periods_min: 1,
    periods_max: 128,
    fifo_size: 256,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn test_component_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);

    mile_stone!(component);

    /* BE's dont need dummy params */
    if (*(*rtd).dai_link).no_pcm == 0 {
        snd_soc_set_runtime_hwparams(substream, &test_component_hardware);
    }

    0
}

unsafe extern "C" fn test_component_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_ioctl(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_uint,
    arg: *mut c_void,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe fn test_component_timer_stop(priv_: *mut test_priv) {
    cancel_delayed_work(&mut (*priv_).dwork);
}

unsafe fn test_component_timer_start(priv_: *mut test_priv) {
    schedule_delayed_work(&mut (*priv_).dwork, msecs_to_jiffies(10));
}

unsafe extern "C" fn test_component_dwork(work: *mut work_struct) {
    let priv_: *mut test_priv = container_of!(work, test_priv, dwork.work);

    if !(*priv_).substream.is_null() {
        snd_pcm_period_elapsed((*priv_).substream);
    }

    test_component_timer_start(priv_);
}

unsafe extern "C" fn test_component_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let priv_: *mut test_priv = dev_get_drvdata((*component).dev) as *mut test_priv;

    mile_stone!(component);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            test_component_timer_start(priv_);
            (*priv_).substream = substream; /* set substream later */
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*priv_).substream = core::ptr::null_mut();
            test_component_timer_stop(priv_);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn test_component_sync_stop(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    static mut pointer: c_int = 0;

    if runtime.is_null() {
        return 0;
    }

    pointer += 10;
    if pointer > PREALLOC_BUFFER as c_int {
        pointer = 0;
    }

    /* mile_stone(component); */

    bytes_to_frames(runtime, pointer)
}

unsafe extern "C" fn test_component_get_time_info(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    system_ts: *mut timespec64,
    audio_ts: *mut timespec64,
    audio_tstamp_config: *mut snd_pcm_audio_tstamp_config,
    audio_tstamp_report: *mut snd_pcm_audio_tstamp_report,
) -> c_int {
    mile_stone!(component);

    0
}

unsafe extern "C" fn test_component_be_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    mile_stone_x!((*rtd).dev);

    0
}

/* CPU */
static test_cpu: test_adata = test_adata { is_cpu: 1, cmp_v: 0, dai_v: 0 };
static test_cpu_vv: test_adata = test_adata { is_cpu: 1, cmp_v: 1, dai_v: 1 };
static test_cpu_nv: test_adata = test_adata { is_cpu: 1, cmp_v: 0, dai_v: 1 };
static test_cpu_vn: test_adata = test_adata { is_cpu: 1, cmp_v: 1, dai_v: 0 };
/* Codec */
static test_codec: test_adata = test_adata { is_cpu: 0, cmp_v: 0, dai_v: 0 };
static test_codec_vv: test_adata = test_adata { is_cpu: 0, cmp_v: 1, dai_v: 1 };
static test_codec_nv: test_adata = test_adata { is_cpu: 0, cmp_v: 0, dai_v: 1 };
static test_codec_vn: test_adata = test_adata { is_cpu: 0, cmp_v: 1, dai_v: 0 };

static test_of_match: [of_device_id; 9] = [
    of_device_id {
        compatible: c"test-cpu".as_ptr(),
        data: &test_cpu as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-cpu-verbose".as_ptr(),
        data: &test_cpu_vv as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-cpu-verbose-dai".as_ptr(),
        data: &test_cpu_nv as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-cpu-verbose-component".as_ptr(),
        data: &test_cpu_vn as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-codec".as_ptr(),
        data: &test_codec as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-codec-verbose".as_ptr(),
        data: &test_codec_vv as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-codec-verbose-dai".as_ptr(),
        data: &test_codec_nv as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"test-codec-verbose-component".as_ptr(),
        data: &test_codec_vn as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
module_device_table!(of, test_of_match);

static widgets: [snd_soc_dapm_widget; 2] = [
    /*
     * FIXME
     *
     * Just IN/OUT is OK for now,
     * but need to be updated ?
     */
    SND_SOC_DAPM_INPUT!(c"IN".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT".as_ptr()),
];

unsafe extern "C" fn test_driver_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let node: *mut device_node = (*dev).of_node;
    let adata: *const test_adata = of_device_get_match_data(&mut (*pdev).dev) as *const test_adata;
    let cdriv: *mut snd_soc_component_driver;
    let ddriv: *mut snd_soc_dai_driver;
    let dname: *mut test_dai_name;
    let priv_: *mut test_priv;
    let mut num: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    num = of_graph_get_endpoint_count(node);
    if num == 0 {
        dev_err!(dev, "no port exits\n");
        return -EINVAL;
    }

    priv_ = devm_kzalloc(dev, core::mem::size_of::<test_priv>(), GFP_KERNEL) as *mut test_priv;
    cdriv = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_component_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_component_driver;
    ddriv = devm_kcalloc(
        dev,
        num as usize,
        core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    dname = devm_kcalloc(
        dev,
        num as usize,
        core::mem::size_of::<test_dai_name>(),
        GFP_KERNEL,
    ) as *mut test_dai_name;
    if priv_.is_null() || cdriv.is_null() || ddriv.is_null() || dname.is_null() || adata.is_null() {
        return -EINVAL;
    }

    (*priv_).dev = dev;
    (*priv_).component_driver = cdriv;
    (*priv_).dai_driver = ddriv;
    (*priv_).name = dname;

    INIT_DELAYED_WORK!(&mut (*priv_).dwork, test_component_dwork);
    dev_set_drvdata(dev, priv_ as *mut c_void);

    if (*adata).is_cpu != 0 {
        (*cdriv).name = c"test_cpu".as_ptr();
        (*cdriv).pcm_new = Some(test_component_pcm_new);
        (*cdriv).pointer = Some(test_component_pointer);
        (*cdriv).trigger = Some(test_component_trigger);
        (*cdriv).legacy_dai_naming = 1;
    } else {
        (*cdriv).name = c"test_codec".as_ptr();
        (*cdriv).idle_bias_on = 1;
        (*cdriv).endianness = 1;
    }

    (*cdriv).open = Some(test_component_open);
    (*cdriv).dapm_widgets = widgets.as_ptr();
    (*cdriv).num_dapm_widgets = ARRAY_SIZE!(&widgets);

    if (*adata).cmp_v != 0 {
        (*cdriv).probe = Some(test_component_probe);
        (*cdriv).remove = Some(test_component_remove);
        (*cdriv).suspend = Some(test_component_suspend);
        (*cdriv).resume = Some(test_component_resume);
        (*cdriv).set_sysclk = Some(test_component_set_sysclk);
        (*cdriv).set_pll = Some(test_component_set_pll);
        (*cdriv).set_jack = Some(test_component_set_jack);
        (*cdriv).seq_notifier = Some(test_component_seq_notifier);
        (*cdriv).stream_event = Some(test_component_stream_event);
        (*cdriv).set_bias_level = Some(test_component_set_bias_level);
        (*cdriv).close = Some(test_component_close);
        (*cdriv).ioctl = Some(test_component_ioctl);
        (*cdriv).hw_params = Some(test_component_hw_params);
        (*cdriv).hw_free = Some(test_component_hw_free);
        (*cdriv).prepare = Some(test_component_prepare);
        (*cdriv).sync_stop = Some(test_component_sync_stop);
        (*cdriv).get_time_info = Some(test_component_get_time_info);
        (*cdriv).be_hw_params_fixup = Some(test_component_be_hw_params_fixup);

        if (*adata).is_cpu != 0 {
            (*cdriv).pcm_free = Some(test_component_pcm_free);
        }
    }

    i = 0;
    for_each_of_graph_port!(node, port, {
        snprintf(
            (*dname.offset(i as isize)).name.as_mut_ptr(),
            TEST_NAME_LEN,
            c"%s.%d".as_ptr(),
            (*node).name,
            i,
        );
        (*ddriv.offset(i as isize)).name = (*dname.offset(i as isize)).name.as_mut_ptr();

        snprintf(
            (*dname.offset(i as isize)).name_playback.as_mut_ptr(),
            TEST_NAME_LEN,
            c"DAI%d Playback".as_ptr(),
            i,
        );
        (*ddriv.offset(i as isize)).playback.stream_name =
            (*dname.offset(i as isize)).name_playback.as_mut_ptr();
        (*ddriv.offset(i as isize)).playback.channels_min = 1;
        (*ddriv.offset(i as isize)).playback.channels_max = 384;
        (*ddriv.offset(i as isize)).playback.rates = STUB_RATES;
        (*ddriv.offset(i as isize)).playback.formats = STUB_FORMATS;

        snprintf(
            (*dname.offset(i as isize)).name_capture.as_mut_ptr(),
            TEST_NAME_LEN,
            c"DAI%d Capture".as_ptr(),
            i,
        );
        (*ddriv.offset(i as isize)).capture.stream_name =
            (*dname.offset(i as isize)).name_capture.as_mut_ptr();
        (*ddriv.offset(i as isize)).capture.channels_min = 1;
        (*ddriv.offset(i as isize)).capture.channels_max = 384;
        (*ddriv.offset(i as isize)).capture.rates = STUB_RATES;
        (*ddriv.offset(i as isize)).capture.formats = STUB_FORMATS;

        if (*adata).dai_v != 0 {
            (*ddriv.offset(i as isize)).ops = &test_verbose_ops;
        } else {
            (*ddriv.offset(i as isize)).ops = &test_ops;
        }

        i += 1;
    });

    ret = devm_snd_soc_register_component(dev, cdriv, ddriv, num);
    if ret < 0 {
        return ret;
    }

    mile_stone_x!(dev);

    0
}

unsafe extern "C" fn test_driver_remove(pdev: *mut platform_device) {
    mile_stone_x!(&mut (*pdev).dev);
}

static mut test_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"test-component".as_ptr(),
        of_match_table: test_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(test_driver_probe),
    remove: Some(test_driver_remove),
    ..unsafe { core::mem::zeroed() }
};
module_platform_driver!(test_driver);

module_alias!("platform:asoc-test-component");
module_author!("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
module_description!("ASoC Test Component");
module_license!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
