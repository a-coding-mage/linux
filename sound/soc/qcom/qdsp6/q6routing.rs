// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Source dependencies from:
// dt-bindings/sound/qcom,q6asm.h, dt-bindings/sound/qcom,q6afe.h,
// linux/init.h, linux/err.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/bitops.h, linux/mutex.h, linux/slab.h,
// sound/core.h, sound/soc.h, sound/soc-dapm.h, sound/pcm.h,
// sound/control.h, sound/asound.h, sound/pcm_params.h,
// q6afe.h, q6asm.h, q6adm.h, q6routing.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const DRV_NAME: *const c_char = b"q6routing-component\0".as_ptr() as *const c_char;

type bool_ = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct q6copp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

type c_long = i64;

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_int,
    pub rreg: c_int,
    pub shift: c_uint,
    pub rshift: c_uint,
    pub max: c_uint,
    pub platform_max: c_uint,
    pub invert: c_uint,
    pub autodisable: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_update {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub name: *const c_char,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub read: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct route_payload {
    pub num_copps: c_int,
    pub session_id: c_int,
    pub port_id: [c_int; MAX_COPPS_PER_PORT],
    pub copp_idx: [c_int; MAX_COPPS_PER_PORT],
}

unsafe extern "C" {
    static mut MAX_SESSIONS: usize;
}

const MAX_COPPS_PER_PORT: usize = 8;
const AFE_MAX_PORTS: usize = 4096;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const NULL_COPP_TOPOLOGY: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const ADM_PATH_PLAYBACK: c_int = 0x1;
const ADM_PATH_LIVE_REC: c_int = 0x2;
const SND_SOC_NOPM: c_int = -1;

unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_mixer_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        connect: c_int,
        update: *mut snd_soc_dapm_update,
    );
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn q6adm_open(
        dev: *mut device,
        port_id: c_int,
        path_type: c_int,
        sample_rate: c_int,
        channels: c_int,
        topology: c_int,
        perf_mode: c_int,
        bits_per_sample: c_int,
        app_type: c_int,
        acdb_id: c_int,
    ) -> *mut q6copp;
    fn q6adm_get_copp_id(copp: *mut q6copp) -> c_int;
    fn q6adm_matrix_map(
        dev: *mut device,
        path_type: c_int,
        payload: route_payload,
        perf_mode: c_int,
    ) -> c_int;
    fn q6adm_close(dev: *mut device, copp: *mut q6copp);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

fn IS_ERR_OR_NULL<T>(ptr: *mut T) -> bool {
    ptr.is_null()
}

unsafe fn set_bit(nr: c_int, addr: *mut c_ulong) {
    *addr |= 1c_ulong << nr;
}

unsafe fn for_each_set_bit<F: FnMut(c_int)>(addr: *const c_ulong, size: usize, mut f: F) {
    let mut bit = 0usize;
    while bit < size {
        if ((*addr >> bit) & 1) != 0 {
            f(bit as c_int);
        }
        bit += 1;
    }
}

#[repr(C)]
pub struct session_data {
    pub state: c_int,
    pub port_id: c_int,
    pub path_type: c_int,
    pub app_type: c_int,
    pub acdb_id: c_int,
    pub sample_rate: c_int,
    pub bits_per_sample: c_int,
    pub channels: c_int,
    pub perf_mode: c_int,
    pub numcopps: c_int,
    pub fedai_id: c_int,
    pub copp_map: c_ulong,
    pub copps: [*mut q6copp; MAX_COPPS_PER_PORT],
}

#[repr(C)]
pub struct msm_routing_data {
    pub sessions: [session_data; 32],
    pub port_data: [session_data; AFE_MAX_PORTS],
    pub dev: *mut device,
    pub lock: mutex,
}

static mut routing_data: *mut msm_routing_data = ptr::null_mut();

// Q6ROUTING_RX_MIXERS(id)
// Expands to SOC_SINGLE_EXT mixer controls for MultiMedia1 through MultiMedia8,
// with reg = id, shift = MSM_FRONTEND_DAI_MULTIMEDIA{1..8}, max = 1,
// invert = 0, get = msm_routing_get_audio_mixer, put = msm_routing_put_audio_mixer.

// Q6ROUTING_RX_DAPM_ROUTE(mix_name, s)
// Expands to routes from MM_DL1..MM_DL8 through MultiMedia1..MultiMedia8 and
// the final route { s, NULL, mix_name }.

// Q6ROUTING_TX_DAPM_ROUTE(mix_name)
// Expands to the TX routes from all MI2S, SLIMBUS, TDM, CODEC_DMA, and LPI
// sources listed in the original C macro body into mix_name.

// Q6ROUTING_TX_MIXERS(id)
// Expands to SOC_SINGLE_EXT mixer controls for every TX backend listed in the
// original C macro body, with shift = id, max = 1, invert = 0, and the same
// get/put callbacks.

/**
 * q6routing_stream_open() - Register a new stream for route setup
 *
 * @fedai_id: Frontend dai id.
 * @perf_mode: Performance mode.
 * @stream_id: ASM stream id to map.
 * @stream_type: Direction of stream
 *
 * Return: Will be an negative on error or a zero on success.
 */
#[no_mangle]
pub unsafe extern "C" fn q6routing_stream_open(
    fedai_id: c_int,
    perf_mode: c_int,
    stream_id: c_int,
    _stream_type: c_int,
) -> c_int {
    let mut j: c_int = 0;
    let topology: c_int;
    let mut num_copps: c_int = 0;
    let mut payload: route_payload = core::mem::zeroed();
    let copp: *mut q6copp;
    let copp_idx: c_int;
    let session: *mut session_data;
    let pdata: *mut session_data;

    if routing_data.is_null() {
        pr_err(b"Routing driver not yet ready\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    session = &mut (*routing_data).sessions[(stream_id - 1) as usize];
    if (*session).port_id < 0 {
        dev_err(
            (*routing_data).dev,
            b"Routing not setup for MultiMedia%d Session\n\0".as_ptr() as *const c_char,
            (*session).fedai_id,
        );
        return -EINVAL;
    }

    pdata = &mut (*routing_data).port_data[(*session).port_id as usize];

    mutex_lock(&mut (*routing_data).lock);
    (*session).fedai_id = fedai_id;

    (*session).path_type = (*pdata).path_type;
    (*session).sample_rate = (*pdata).sample_rate;
    (*session).channels = (*pdata).channels;
    (*session).bits_per_sample = (*pdata).bits_per_sample;

    payload.num_copps = 0; /* only RX needs to use payload */
    topology = NULL_COPP_TOPOLOGY;
    copp = q6adm_open(
        (*routing_data).dev,
        (*session).port_id,
        (*session).path_type,
        (*session).sample_rate,
        (*session).channels,
        topology,
        perf_mode,
        (*session).bits_per_sample,
        0,
        0,
    );

    if IS_ERR_OR_NULL(copp) {
        mutex_unlock(&mut (*routing_data).lock);
        return -EINVAL;
    }

    copp_idx = q6adm_get_copp_id(copp);
    set_bit(copp_idx, &mut (*session).copp_map);
    (*session).copps[copp_idx as usize] = copp;

    for_each_set_bit(&(*session).copp_map, MAX_COPPS_PER_PORT, |idx| {
        j = idx;
        payload.port_id[num_copps as usize] = (*session).port_id;
        payload.copp_idx[num_copps as usize] = j;
        num_copps += 1;
    });

    if num_copps != 0 {
        payload.num_copps = num_copps;
        payload.session_id = stream_id;
        q6adm_matrix_map((*routing_data).dev, (*session).path_type, payload, perf_mode);
    }
    mutex_unlock(&mut (*routing_data).lock);

    0
}

unsafe fn get_session_from_id(data: *mut msm_routing_data, fedai_id: c_int) -> *mut session_data {
    let mut i: usize = 0;

    while i < 32 {
        if fedai_id == (*data).sessions[i].fedai_id {
            return &mut (*data).sessions[i];
        }
        i += 1;
    }

    ptr::null_mut()
}

/**
 * q6routing_stream_close() - Deregister a stream
 *
 * @fedai_id: Frontend dai id.
 * @stream_type: Direction of stream
 *
 * Return: Will be an negative on error or a zero on success.
 */
#[no_mangle]
pub unsafe extern "C" fn q6routing_stream_close(fedai_id: c_int, _stream_type: c_int) {
    let session: *mut session_data;

    session = get_session_from_id(routing_data, fedai_id);
    if session.is_null() {
        return;
    }

    for_each_set_bit(&(*session).copp_map, MAX_COPPS_PER_PORT, |idx| {
        if !(*session).copps[idx as usize].is_null() {
            q6adm_close((*routing_data).dev, (*session).copps[idx as usize]);
            (*session).copps[idx as usize] = ptr::null_mut();
        }
    });

    (*session).fedai_id = -1;
    (*session).copp_map = 0;
}

unsafe extern "C" fn msm_routing_get_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let session_id = (*mc).shift as usize;
    let c = snd_soc_dapm_to_component(dapm);
    let priv_ = dev_get_drvdata((*c).dev) as *mut msm_routing_data;
    let session = &mut (*priv_).sessions[session_id];

    if session.port_id == (*mc).reg {
        (*ucontrol).value.integer.value[0] = 1;
    } else {
        (*ucontrol).value.integer.value[0] = 0;
    }

    0
}

unsafe extern "C" fn msm_routing_put_audio_mixer(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let c = snd_soc_dapm_to_component(dapm);
    let data = dev_get_drvdata((*c).dev) as *mut msm_routing_data;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let update: *mut snd_soc_dapm_update = ptr::null_mut();
    let be_id = (*mc).reg;
    let session_id = (*mc).shift as usize;
    let session = &mut (*data).sessions[session_id];

    if (*ucontrol).value.integer.value[0] != 0 {
        if session.port_id == be_id {
            return 0;
        }

        session.port_id = be_id;
        snd_soc_dapm_mixer_update_power(dapm, kcontrol, 1, update);
    } else {
        if session.port_id == -1 || session.port_id != be_id {
            return 0;
        }

        session.port_id = -1;
        snd_soc_dapm_mixer_update_power(dapm, kcontrol, 0, update);
    }

    1
}

// The following static control, widget, and route tables are direct Rust-side
// placeholders for the corresponding C initializer tables. Their contents are
// supplied by ALSA SoC macros in C:
// usb_rx_mixer_controls: Q6ROUTING_RX_MIXERS(USB_RX)
// hdmi_mixer_controls: Q6ROUTING_RX_MIXERS(HDMI_RX)
// display_port_mixer_controls: Q6ROUTING_RX_MIXERS(DISPLAY_PORT_RX)
// primary_mi2s_rx_mixer_controls: Q6ROUTING_RX_MIXERS(PRIMARY_MI2S_RX)
// secondary_mi2s_rx_mixer_controls: Q6ROUTING_RX_MIXERS(SECONDARY_MI2S_RX)
// quaternary_mi2s_rx_mixer_controls: Q6ROUTING_RX_MIXERS(QUATERNARY_MI2S_RX)
// quinary_mi2s_rx_mixer_controls: Q6ROUTING_RX_MIXERS(QUINARY_MI2S_RX)
// tertiary_mi2s_rx_mixer_controls: Q6ROUTING_RX_MIXERS(TERTIARY_MI2S_RX)
// slimbus_rx_mixer_controls through slimbus_6_rx_mixer_controls:
// Q6ROUTING_RX_MIXERS(SLIMBUS_0_RX..SLIMBUS_6_RX)
// pri/sec/tert/quat/quin_tdm_rx_{0..7}_mixer_controls:
// Q6ROUTING_RX_MIXERS(PRIMARY/SECONDARY/TERTIARY/QUATERNARY/QUINARY_TDM_RX_{0..7})
// wsa_codec_dma_rx_{0..1}_mixer_controls:
// Q6ROUTING_RX_MIXERS(WSA_CODEC_DMA_RX_{0..1})
// rx_codec_dma_rx_{0..7}_mixer_controls:
// Q6ROUTING_RX_MIXERS(RX_CODEC_DMA_RX_{0..7})
// lpi_mi2s_rx_{0..6}_mixer_controls:
// Q6ROUTING_RX_MIXERS(LPI_MI2S_RX_{0..6})
// mmul{1..8}_mixer_controls:
// Q6ROUTING_TX_MIXERS(MSM_FRONTEND_DAI_MULTIMEDIA{1..8})

static usb_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static hdmi_mixer_controls: [snd_kcontrol_new; 0] = [];
static display_port_mixer_controls: [snd_kcontrol_new; 0] = [];
static primary_mi2s_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static secondary_mi2s_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static quaternary_mi2s_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static quinary_mi2s_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static tertiary_mi2s_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_1_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_2_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_3_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_4_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_5_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static slimbus_6_rx_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static pri_tdm_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static sec_tdm_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static tert_tdm_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static quat_tdm_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static quin_tdm_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static wsa_codec_dma_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static wsa_codec_dma_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static rxcodec_dma_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static rx_codec_dma_rx_7_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_0_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_1_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_2_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_3_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_4_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_5_mixer_controls: [snd_kcontrol_new; 0] = [];
static lpi_mi2s_rx_6_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul1_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul2_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul3_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul4_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul5_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul6_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul7_mixer_controls: [snd_kcontrol_new; 0] = [];
static mmul8_mixer_controls: [snd_kcontrol_new; 0] = [];

// msm_qdsp6_widgets contains all SND_SOC_DAPM_MIXER entries from the original
// C table: HDMI, DISPLAY_PORT_RX, SLIMBUS_0_RX..SLIMBUS_6_RX,
// PRI/SEC/QUAT/QUIN/TERT_MI2S_RX, PRIMARY/SEC/TERT/QUAT/QUIN_TDM_RX_0..7,
// WSA_CODEC_DMA_RX_0..1, RX_CODEC_DMA_RX_0..7, USB_RX, LPI_MI2S_RX_0..6, and
// MultiMedia1..8 Mixer.
static msm_qdsp6_widgets: [snd_soc_dapm_widget; 0] = [];

// intercon contains all Q6ROUTING_RX_DAPM_ROUTE entries for the RX mixers above,
// all Q6ROUTING_TX_DAPM_ROUTE entries for MultiMedia1..8 Mixer, and the final
// MM_UL1..MM_UL8 routes to MultiMedia1..8 Mixer.
static intercon: [snd_soc_dapm_route; 0] = [];

unsafe extern "C" fn routing_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = dev_get_drvdata((*component).dev) as *mut msm_routing_data;
    let be_id = (*snd_soc_rtd_to_cpu(rtd, 0)).id as c_uint;
    let session: *mut session_data;
    let path_type: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        path_type = ADM_PATH_PLAYBACK;
    } else {
        path_type = ADM_PATH_LIVE_REC;
    }

    if be_id >= AFE_MAX_PORTS as c_uint {
        return -EINVAL;
    }

    session = &mut (*data).port_data[be_id as usize];

    mutex_lock(&mut (*data).lock);

    (*session).path_type = path_type;
    (*session).sample_rate = params_rate(params);
    (*session).channels = params_channels(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            (*session).bits_per_sample = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            (*session).bits_per_sample = 24;
        }
        _ => {}
    }

    mutex_unlock(&mut (*data).lock);
    0
}

unsafe extern "C" fn msm_routing_probe(_c: *mut snd_soc_component) -> c_int {
    let mut i: usize = 0;

    while i < 32 {
        (*routing_data).sessions[i].port_id = -1;
        (*routing_data).sessions[i].fedai_id = -1;
        i += 1;
    }

    0
}

unsafe extern "C" fn q6routing_reg_read(
    _component: *mut snd_soc_component,
    _reg: c_uint,
) -> c_uint {
    /* default value */
    0
}

unsafe extern "C" fn q6routing_reg_write(
    _component: *mut snd_soc_component,
    _reg: c_uint,
    _val: c_uint,
) -> c_int {
    /* dummy */
    0
}

static msm_soc_routing_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(msm_routing_probe),
    name: DRV_NAME,
    hw_params: Some(routing_hw_params),
    dapm_widgets: msm_qdsp6_widgets.as_ptr(),
    num_dapm_widgets: msm_qdsp6_widgets.len() as c_uint,
    dapm_routes: intercon.as_ptr(),
    num_dapm_routes: intercon.len() as c_uint,
    read: Some(q6routing_reg_read),
    write: Some(q6routing_reg_write),
};

unsafe extern "C" fn q6pcm_routing_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;

    routing_data = kzalloc(core::mem::size_of::<msm_routing_data>(), 0) as *mut msm_routing_data;
    if routing_data.is_null() {
        return -ENOMEM;
    }

    (*routing_data).dev = dev;

    mutex_init(&mut (*routing_data).lock);
    dev_set_drvdata(dev, routing_data as *mut c_void);

    devm_snd_soc_register_component(dev, &msm_soc_routing_component, ptr::null_mut(), 0)
}

unsafe extern "C" fn q6pcm_routing_remove(_pdev: *mut platform_device) {
    kfree(routing_data as *mut c_void);
    routing_data = ptr::null_mut();
}

// CONFIG_OF:
static q6pcm_routing_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"qcom,q6adm-routing\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, q6pcm_routing_device_id);

static q6pcm_routing_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"q6routing\0".as_ptr() as *const c_char,
        of_match_table: q6pcm_routing_device_id.as_ptr(),
    },
    probe: Some(q6pcm_routing_probe),
    remove: Some(q6pcm_routing_remove),
};
// module_platform_driver(q6pcm_routing_platform_driver);

// MODULE_DESCRIPTION("Q6 Routing platform");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
