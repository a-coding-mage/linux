// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Intel Corporation
//
//

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};

// Dependencies from:
// <sound/pcm_params.h>
// "ipc3-priv.h"
// "ops.h"
// "sof-priv.h"
// "sof-audio.h"

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_sof_find_spcm_dai(
        component: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_sof_pcm;
    fn sof_ipc_tx_message_no_reply(ipc: *mut c_void, msg: *mut c_void, msg_bytes: usize) -> c_int;
    fn sof_ipc_tx_message(
        ipc: *mut c_void,
        msg: *mut c_void,
        msg_bytes: usize,
        reply: *mut c_void,
        reply_bytes: usize,
    ) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn snd_sof_set_stream_data_offset(
        sdev: *mut snd_sof_dev,
        stream: *mut snd_sof_pcm_stream,
        posn_offset: u32,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_sof_find_dai(component: *mut snd_soc_component, name: *mut c_char) -> *mut snd_sof_dai;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn spcm_dbg(spcm: *mut snd_sof_pcm, stream: c_int, fmt: *const c_char, ...) -> c_int;
    fn spcm_err(spcm: *mut snd_sof_pcm, stream: c_int, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_bytes: usize,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub trigger: [c_int; 2],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    pub stream_tag: c_int,
    pub use_phy_address: bool,
    pub phy_addr: u64,
    pub no_ipc_position: bool,
    pub cont_update_posn: bool,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub ipc: *mut c_void,
    pub fw_ready: sof_ipc_fw_ready,
    pub dai_list: list_head,
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc_fw_ready {
    pub version: sof_ipc_fw_version,
}

#[repr(C)]
pub struct sof_ipc_fw_version {
    pub abi_version: u32,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub prepared: [bool; 2],
    pub stream: [snd_sof_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub comp_id: u32,
    pub page_table: sof_page_table,
}

#[repr(C)]
pub struct sof_page_table {
    pub addr: u64,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_stream {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
}

#[repr(C)]
pub struct sof_ipc_pcm_params_reply {
    pub posn_offset: u32,
}

#[repr(C)]
pub struct sof_ipc_pcm_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub direction: c_int,
    pub sample_valid_bytes: c_int,
    pub buffer_fmt: u32,
    pub rate: c_int,
    pub channels: c_int,
    pub host_period_bytes: c_int,
    pub sample_container_bytes: c_int,
    pub frame_fmt: u32,
    pub stream_tag: c_int,
    pub no_stream_position: c_int,
    pub cont_update_posn: c_int,
}

#[repr(C)]
pub struct sof_ipc_host_buffer {
    pub pages: usize,
    pub phy_addr: u64,
    pub size: usize,
}

#[repr(C)]
pub struct sof_ipc_pcm_params_msg {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub params: sof_ipc_pcm_params,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_sof_dai {
    pub name: *const c_char,
    pub list: list_head,
    pub number_configs: c_int,
    pub private: *mut sof_dai_private_data,
    pub current_config: c_int,
}

#[repr(C)]
pub struct sof_dai_private_data {
    pub dai_config: *mut sof_ipc_dai_config,
    pub comp_dai: *mut sof_ipc_comp_dai,
}

#[repr(C)]
pub struct sof_ipc_comp_dai {
    pub config: sof_ipc_dai_config_data,
}

#[repr(C)]
pub struct sof_ipc_dai_config_data {
    pub frame_fmt: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub type_: u32,
    pub reserved: u32,
    pub ssp: sof_ipc_dai_ssp_params,
    pub alh: sof_ipc_dai_alh_params,
    pub esai: sof_ipc_dai_esai_params,
    pub afe: sof_ipc_dai_afe_params,
    pub sai: sof_ipc_dai_sai_params,
    pub acpbt: sof_ipc_dai_acp_params,
    pub acpsp: sof_ipc_dai_acp_params,
    pub acphs: sof_ipc_dai_acp_params,
    pub acpdmic: sof_ipc_dai_acpdmic_params,
    pub micfil: sof_ipc_dai_acpdmic_params,
    pub acp_sdw: sof_ipc_dai_afe_params,
    pub acp_i2s: sof_ipc_dai_acp_params,
}

#[repr(C)]
pub struct sof_ipc_dai_ssp_params {
    pub fsync_rate: u32,
    pub tdm_slots: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_alh_params {
    pub channels: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_esai_params {
    pub fsync_rate: u32,
    pub tdm_slots: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_afe_params {
    pub rate: u32,
    pub channels: u32,
    pub format: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_sai_params {
    pub fsync_rate: u32,
    pub tdm_slots: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_acp_params {
    pub fsync_rate: u32,
    pub tdm_slots: u32,
}

#[repr(C)]
pub struct sof_ipc_dai_acpdmic_params {
    pub pdm_rate: u32,
    pub pdm_ch: u32,
}

#[repr(C)]
pub struct snd_interval {
    pub min: u32,
    pub max: u32,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dpcm {
    pub fe: *mut snd_soc_pcm_runtime,
}

#[repr(C)]
pub struct sof_ipc_pcm_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_sof_platform_stream_params,
        ) -> c_int,
    >,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub dai_link_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub reset_hw_params_during_stop: bool,
    pub d0i3_supported_in_s0ix: bool,
}

const EINVAL: c_int = 22;
const SOF_IPC_GLB_STREAM_MSG: u32 = 0;
const SOF_IPC_STREAM_PCM_FREE: u32 = 0;
const SOF_IPC_STREAM_PCM_PARAMS: u32 = 0;
const SOF_IPC_STREAM_TRIG_PAUSE: u32 = 0;
const SOF_IPC_STREAM_TRIG_RELEASE: u32 = 0;
const SOF_IPC_STREAM_TRIG_START: u32 = 0;
const SOF_IPC_STREAM_TRIG_STOP: u32 = 0;
const SOF_IPC_BUFFER_INTERLEAVED: u32 = 0;
const SOF_IPC_FRAME_S16_LE: u32 = 0;
const SOF_IPC_FRAME_S24_4LE: u32 = 0;
const SOF_IPC_FRAME_S32_LE: u32 = 0;
const SOF_IPC_FRAME_FLOAT: u32 = 0;
const SNDRV_PCM_FORMAT_S16: c_int = 0;
const SNDRV_PCM_FORMAT_S24: c_int = 0;
const SNDRV_PCM_FORMAT_S32: c_int = 0;
const SNDRV_PCM_FORMAT_FLOAT: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 0;
const SOF_DAI_INTEL_SSP: u32 = 0;
const SOF_DAI_INTEL_DMIC: u32 = 0;
const SOF_DAI_INTEL_HDA: u32 = 0;
const SOF_DAI_INTEL_ALH: u32 = 0;
const SOF_DAI_IMX_ESAI: u32 = 0;
const SOF_DAI_MEDIATEK_AFE: u32 = 0;
const SOF_DAI_IMX_SAI: u32 = 0;
const SOF_DAI_AMD_BT: u32 = 0;
const SOF_DAI_AMD_SP: u32 = 0;
const SOF_DAI_AMD_SP_VIRTUAL: u32 = 0;
const SOF_DAI_AMD_HS: u32 = 0;
const SOF_DAI_AMD_HS_VIRTUAL: u32 = 0;
const SOF_DAI_AMD_DMIC: u32 = 0;
const SOF_DAI_IMX_MICFIL: u32 = 0;
const SOF_DAI_AMD_SDW: u32 = 0;
const SOF_DAI_AMD_I2S: u32 = 0;
const SOF_AUDIO_PCM_DRV_NAME: *const c_char = b"sof-audio-component\0".as_ptr() as *const c_char;

const fn SOF_ABI_VER(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 24) | (minor << 12) | patch
}

const fn PFN_UP(x: usize) -> usize {
    (x + 4095) >> 12
}

unsafe extern "C" fn sof_ipc3_pcm_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut stream: sof_ipc_stream = zeroed();
    let spcm: *mut snd_sof_pcm;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    if !(*spcm).prepared[(*substream).stream as usize] {
        return 0;
    }

    stream.hdr.size = size_of::<sof_ipc_stream>() as u32;
    stream.hdr.cmd = SOF_IPC_GLB_STREAM_MSG | SOF_IPC_STREAM_PCM_FREE;
    stream.comp_id = (*spcm).stream[(*substream).stream as usize].comp_id;

    /* send IPC to the DSP */
    sof_ipc_tx_message_no_reply(
        (*sdev).ipc,
        &mut stream as *mut sof_ipc_stream as *mut c_void,
        size_of::<sof_ipc_stream>(),
    )
}

unsafe extern "C" fn sof_ipc3_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
) -> c_int {
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let v: *mut sof_ipc_fw_version = &mut (*sdev).fw_ready.version;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut ipc_params_reply: sof_ipc_pcm_params_reply = zeroed();
    let mut pcm: sof_ipc_pcm_params_msg = zeroed();
    let spcm: *mut snd_sof_pcm;
    let mut ret: c_int;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    /* number of pages should be rounded up */
    pcm.params.buffer.pages = PFN_UP((*runtime).dma_bytes);

    /* set IPC PCM parameters */
    pcm.hdr.size = size_of::<sof_ipc_pcm_params_msg>() as u32;
    pcm.hdr.cmd = SOF_IPC_GLB_STREAM_MSG | SOF_IPC_STREAM_PCM_PARAMS;
    pcm.comp_id = (*spcm).stream[(*substream).stream as usize].comp_id;
    pcm.params.hdr.size = size_of::<sof_ipc_pcm_params>() as u32;
    pcm.params.buffer.phy_addr = (*spcm).stream[(*substream).stream as usize].page_table.addr;
    pcm.params.buffer.size = (*runtime).dma_bytes;
    pcm.params.direction = (*substream).stream;
    pcm.params.sample_valid_bytes = params_width(params) >> 3;
    pcm.params.buffer_fmt = SOF_IPC_BUFFER_INTERLEAVED;
    pcm.params.rate = params_rate(params);
    pcm.params.channels = params_channels(params);
    pcm.params.host_period_bytes = params_period_bytes(params);

    /* container size */
    ret = snd_pcm_format_physical_width(params_format(params));
    if ret < 0 {
        return ret;
    }
    pcm.params.sample_container_bytes = ret >> 3;

    /* format */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16 => {
            pcm.params.frame_fmt = SOF_IPC_FRAME_S16_LE;
        }
        SNDRV_PCM_FORMAT_S24 => {
            pcm.params.frame_fmt = SOF_IPC_FRAME_S24_4LE;
        }
        SNDRV_PCM_FORMAT_S32 => {
            pcm.params.frame_fmt = SOF_IPC_FRAME_S32_LE;
        }
        SNDRV_PCM_FORMAT_FLOAT => {
            pcm.params.frame_fmt = SOF_IPC_FRAME_FLOAT;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* Update the IPC message with information from the platform */
    pcm.params.stream_tag = (*platform_params).stream_tag;

    if (*platform_params).use_phy_address {
        pcm.params.buffer.phy_addr = (*platform_params).phy_addr;
    }

    if (*platform_params).no_ipc_position {
        /* For older ABIs set host_period_bytes to zero to inform
         * FW we don't want position updates. Newer versions use
         * no_stream_position for this purpose.
         */
        if (*v).abi_version < SOF_ABI_VER(3, 10, 0) {
            pcm.params.host_period_bytes = 0;
        } else {
            pcm.params.no_stream_position = 1;
        }
    }

    if (*platform_params).cont_update_posn {
        pcm.params.cont_update_posn = 1;
    }

    spcm_dbg(
        spcm,
        (*substream).stream,
        b"stream_tag %d\n\0".as_ptr() as *const c_char,
        pcm.params.stream_tag,
    );

    /* send hw_params IPC to the DSP */
    ret = sof_ipc_tx_message(
        (*sdev).ipc,
        &mut pcm as *mut sof_ipc_pcm_params_msg as *mut c_void,
        size_of::<sof_ipc_pcm_params_msg>(),
        &mut ipc_params_reply as *mut sof_ipc_pcm_params_reply as *mut c_void,
        size_of::<sof_ipc_pcm_params_reply>(),
    );
    if ret < 0 {
        spcm_err(
            spcm,
            (*substream).stream,
            b"STREAM_PCM_PARAMS ipc failed for stream_tag %d\n\0".as_ptr() as *const c_char,
            pcm.params.stream_tag,
        );
        return ret;
    }

    ret = snd_sof_set_stream_data_offset(
        sdev,
        &mut (*spcm).stream[(*substream).stream as usize],
        ipc_params_reply.posn_offset,
    );
    if ret < 0 {
        spcm_err(
            spcm,
            (*substream).stream,
            b"invalid stream data offset\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn sof_ipc3_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let mut stream: sof_ipc_stream = zeroed();
    let spcm: *mut snd_sof_pcm;

    spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    stream.hdr.size = size_of::<sof_ipc_stream>() as u32;
    stream.hdr.cmd = SOF_IPC_GLB_STREAM_MSG;
    stream.comp_id = (*spcm).stream[(*substream).stream as usize].comp_id;

    match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_PAUSE;
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_RELEASE;
        }
        SNDRV_PCM_TRIGGER_START => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_START;
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            stream.hdr.cmd |= SOF_IPC_STREAM_TRIG_STOP;
        }
        _ => {
            spcm_err(
                spcm,
                (*substream).stream,
                b"Unhandled trigger cmd %d\n\0".as_ptr() as *const c_char,
                cmd,
            );
            return -EINVAL;
        }
    }

    /* send IPC to the DSP */
    sof_ipc_tx_message_no_reply(
        (*sdev).ipc,
        &mut stream as *mut sof_ipc_stream as *mut c_void,
        size_of::<sof_ipc_stream>(),
    )
}

unsafe fn ssp_dai_config_pcm_params_match(
    sdev: *mut snd_sof_dev,
    link_name: *const c_char,
    params: *mut snd_pcm_hw_params,
) {
    let mut config: *mut sof_ipc_dai_config;
    let mut dai: *mut snd_sof_dai;
    let mut i: c_int;

    /*
     * Search for all matching DAIs as we can have both playback and capture DAI
     * associated with the same link.
     */
    /* list_for_each_entry(dai, &sdev->dai_list, list) */
    dai = core::ptr::null_mut();
    while !dai.is_null() {
        if (*dai).name.is_null() || strcmp(link_name, (*dai).name) != 0 {
            continue;
        }
        i = 0;
        while i < (*dai).number_configs {
            let private: *mut sof_dai_private_data = (*dai).private;

            config = (*private).dai_config.add(i as usize);
            if (*config).ssp.fsync_rate == params_rate(params) as u32 {
                dev_dbg(
                    (*sdev).dev,
                    b"DAI config %d matches pcm hw params\n\0".as_ptr() as *const c_char,
                    i,
                );
                (*dai).current_config = i;
                break;
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn sof_ipc3_pcm_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME);
    let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let dai: *mut snd_sof_dai = snd_sof_find_dai(component, (*(*rtd).dai_link).name as *mut c_char);
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let fmt: *mut snd_mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let sdev: *mut snd_sof_dev = snd_soc_component_get_drvdata(component);
    let private: *mut sof_dai_private_data;
    let mut dpcm: *mut snd_soc_dpcm;

    if dai.is_null() {
        dev_err(
            (*component).dev,
            b"%s: No DAI found with name %s\n\0".as_ptr() as *const c_char,
            b"sof_ipc3_pcm_dai_link_fixup\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).name,
        );
        return -EINVAL;
    }

    private = (*dai).private;
    if private.is_null() {
        dev_err(
            (*component).dev,
            b"%s: No private data found for DAI %s\n\0".as_ptr() as *const c_char,
            b"sof_ipc3_pcm_dai_link_fixup\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).name,
        );
        return -EINVAL;
    }

    /* read format from topology */
    snd_mask_none(fmt);

    match (*(*private).comp_dai).config.frame_fmt {
        SOF_IPC_FRAME_S16_LE => {
            snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);
        }
        SOF_IPC_FRAME_S24_4LE => {
            snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
        }
        SOF_IPC_FRAME_S32_LE => {
            snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S32_LE);
        }
        _ => {
            dev_err((*component).dev, b"No available DAI format!\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* read rate and channels from topology */
    match (*(*private).dai_config).type_ {
        SOF_DAI_INTEL_SSP => {
            /* search for config to pcm params match, if not found use default */
            ssp_dai_config_pcm_params_match(sdev, (*(*rtd).dai_link).name, params);

            (*rate).min = (*(*private).dai_config.add((*dai).current_config as usize)).ssp.fsync_rate;
            (*rate).max = (*(*private).dai_config.add((*dai).current_config as usize)).ssp.fsync_rate;
            (*channels).min = (*(*private).dai_config.add((*dai).current_config as usize)).ssp.tdm_slots;
            (*channels).max = (*(*private).dai_config.add((*dai).current_config as usize)).ssp.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_INTEL_DMIC => {
            /* DMIC only supports 16 or 32 bit formats */
            if (*(*private).comp_dai).config.frame_fmt == SOF_IPC_FRAME_S24_4LE {
                dev_err(
                    (*component).dev,
                    b"Invalid fmt %d for DAI type %d\n\0".as_ptr() as *const c_char,
                    (*(*private).comp_dai).config.frame_fmt,
                    (*(*private).dai_config).type_,
                );
            }
        }
        SOF_DAI_INTEL_HDA => {
            /*
             * HDAudio does not follow the default trigger
             * sequence due to firmware implementation
             */
            /* for_each_dpcm_fe(rtd, SNDRV_PCM_STREAM_PLAYBACK, dpcm) */
            dpcm = core::ptr::null_mut();
            while !dpcm.is_null() {
                let fe: *mut snd_soc_pcm_runtime = (*dpcm).fe;

                (*(*fe).dai_link).trigger[SNDRV_PCM_STREAM_PLAYBACK] = SND_SOC_DPCM_TRIGGER_POST;
            }
        }
        SOF_DAI_INTEL_ALH => {
            /*
             * Dai could run with different channel count compared with
             * front end, so get dai channel count from topology
             */
            (*channels).min = (*(*private).dai_config).alh.channels;
            (*channels).max = (*(*private).dai_config).alh.channels;
        }
        SOF_DAI_IMX_ESAI => {
            (*rate).min = (*(*private).dai_config).esai.fsync_rate;
            (*rate).max = (*(*private).dai_config).esai.fsync_rate;
            (*channels).min = (*(*private).dai_config).esai.tdm_slots;
            (*channels).max = (*(*private).dai_config).esai.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_MEDIATEK_AFE => {
            (*rate).min = (*(*private).dai_config).afe.rate;
            (*rate).max = (*(*private).dai_config).afe.rate;
            (*channels).min = (*(*private).dai_config).afe.channels;
            (*channels).max = (*(*private).dai_config).afe.channels;

            snd_mask_none(fmt);

            match (*(*private).dai_config).afe.format {
                SOF_IPC_FRAME_S16_LE => {
                    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE);
                }
                SOF_IPC_FRAME_S24_4LE => {
                    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
                }
                SOF_IPC_FRAME_S32_LE => {
                    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S32_LE);
                }
                _ => {
                    dev_err((*component).dev, b"Not available format!\n\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
            }

            dev_dbg(
                (*component).dev,
                b"rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_IMX_SAI => {
            (*rate).min = (*(*private).dai_config).sai.fsync_rate;
            (*rate).max = (*(*private).dai_config).sai.fsync_rate;
            (*channels).min = (*(*private).dai_config).sai.tdm_slots;
            (*channels).max = (*(*private).dai_config).sai.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_AMD_BT => {
            (*rate).min = (*(*private).dai_config).acpbt.fsync_rate;
            (*rate).max = (*(*private).dai_config).acpbt.fsync_rate;
            (*channels).min = (*(*private).dai_config).acpbt.tdm_slots;
            (*channels).max = (*(*private).dai_config).acpbt.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"AMD_BT rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"AMD_BT channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_AMD_SP | SOF_DAI_AMD_SP_VIRTUAL => {
            (*rate).min = (*(*private).dai_config).acpsp.fsync_rate;
            (*rate).max = (*(*private).dai_config).acpsp.fsync_rate;
            (*channels).min = (*(*private).dai_config).acpsp.tdm_slots;
            (*channels).max = (*(*private).dai_config).acpsp.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"AMD_SP rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"AMD_SP channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_AMD_HS | SOF_DAI_AMD_HS_VIRTUAL => {
            (*rate).min = (*(*private).dai_config).acphs.fsync_rate;
            (*rate).max = (*(*private).dai_config).acphs.fsync_rate;
            (*channels).min = (*(*private).dai_config).acphs.tdm_slots;
            (*channels).max = (*(*private).dai_config).acphs.tdm_slots;

            dev_dbg(
                (*component).dev,
                b"AMD_HS channel_max: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).max,
                (*rate).max,
            );
        }
        SOF_DAI_AMD_DMIC => {
            (*rate).min = (*(*private).dai_config).acpdmic.pdm_rate;
            (*rate).max = (*(*private).dai_config).acpdmic.pdm_rate;
            (*channels).min = (*(*private).dai_config).acpdmic.pdm_ch;
            (*channels).max = (*(*private).dai_config).acpdmic.pdm_ch;

            dev_dbg(
                (*component).dev,
                b"AMD_DMIC rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"AMD_DMIC channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_IMX_MICFIL => {
            (*rate).min = (*(*private).dai_config).micfil.pdm_rate;
            (*rate).max = (*(*private).dai_config).micfil.pdm_rate;
            (*channels).min = (*(*private).dai_config).micfil.pdm_ch;
            (*channels).max = (*(*private).dai_config).micfil.pdm_ch;

            dev_dbg(
                (*component).dev,
                b"MICFIL PDM rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"MICFIL PDM channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_AMD_SDW => {
            /* change the default trigger sequence as per HW implementation */
            /* for_each_dpcm_fe(rtd, SNDRV_PCM_STREAM_PLAYBACK, dpcm) */
            dpcm = core::ptr::null_mut();
            while !dpcm.is_null() {
                let fe: *mut snd_soc_pcm_runtime = (*dpcm).fe;

                (*(*fe).dai_link).trigger[SNDRV_PCM_STREAM_PLAYBACK] = SND_SOC_DPCM_TRIGGER_POST;
            }

            /* for_each_dpcm_fe(rtd, SNDRV_PCM_STREAM_CAPTURE, dpcm) */
            dpcm = core::ptr::null_mut();
            while !dpcm.is_null() {
                let fe: *mut snd_soc_pcm_runtime = (*dpcm).fe;

                (*(*fe).dai_link).trigger[SNDRV_PCM_STREAM_CAPTURE] = SND_SOC_DPCM_TRIGGER_POST;
            }
            (*rate).min = (*(*private).dai_config).acp_sdw.rate;
            (*rate).max = (*(*private).dai_config).acp_sdw.rate;
            (*channels).min = (*(*private).dai_config).acp_sdw.channels;
            (*channels).max = (*(*private).dai_config).acp_sdw.channels;

            dev_dbg(
                (*component).dev,
                b"AMD_SDW rate_min: %d rate_max: %d\n\0".as_ptr() as *const c_char,
                (*rate).min,
                (*rate).max,
            );
            dev_dbg(
                (*component).dev,
                b"AMD_SDW channels_min: %d channels_max: %d\n\0".as_ptr() as *const c_char,
                (*channels).min,
                (*channels).max,
            );
        }
        SOF_DAI_AMD_I2S => {
            (*rate).min = (*(*private).dai_config).acp_i2s.fsync_rate;
            (*rate).max = (*(*private).dai_config).acp_i2s.fsync_rate;
            (*channels).min = (*(*private).dai_config).acp_i2s.tdm_slots;
            (*channels).max = (*(*private).dai_config).acp_i2s.tdm_slots;
        }
        _ => {
            dev_err(
                (*component).dev,
                b"Invalid DAI type %d\n\0".as_ptr() as *const c_char,
                (*(*private).dai_config).type_,
            );
        }
    }

    0
}

#[no_mangle]
pub static ipc3_pcm_ops: sof_ipc_pcm_ops = sof_ipc_pcm_ops {
    hw_params: Some(sof_ipc3_pcm_hw_params),
    hw_free: Some(sof_ipc3_pcm_hw_free),
    trigger: Some(sof_ipc3_pcm_trigger),
    dai_link_fixup: Some(sof_ipc3_pcm_dai_link_fixup),
    reset_hw_params_during_stop: true,
    d0i3_supported_in_s0ix: true,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
