// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) STMicroelectronics SA 2015
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          for STMicroelectronics.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

/*
 * Dependencies from Linux, ALSA SoC, and "uniperif.h" are intentionally
 * declared here as external C-compatible items. Their definitions are supplied
 * by the surrounding repository.
 */

type u64_t = u64;
type u_int32_t = u32;

const EIO: c_int = 5;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const SNDRV_MASK_MAX: usize = 256;

extern "C" {
    static SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0: c_uint;
    static SND_ST_UNIPERIF_VERSION_UNI_RDR_1_0: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_uint;
    static SND_ST_UNIPERIF_TYPE_HDMI: uniperif_type;
    static SND_ST_UNIPERIF_TYPE_PCM: uniperif_type;
    static SND_ST_UNIPERIF_TYPE_TDM: uniperif_type;
    static SND_ST_UNIPERIF_TYPE_SPDIF: uniperif_type;
    static SNDRV_PCM_FMTBIT_S16_LE: u64_t;
    static SNDRV_PCM_FMTBIT_S32_LE: u64_t;
    static UNIPERIF_FIFO_FRAMES: c_int;
    static WORD_MAX: c_int;
    static UNIPERIF_STATE_STOPPED: c_uint;
}

/*
 * User frame size shall be 2, 4, 6 or 8 32-bits words length
 * (i.e. 8, 16, 24 or 32 bytes)
 * This constraint comes from allowed values for
 * UNIPERIF_I2S_FMT_NUM_CH register
 */
const UNIPERIF_MAX_FRAME_SZ: c_int = 0x20;
const UNIPERIF_ALLOWED_FRAME_SZ: c_int = 0x08 | 0x10 | 0x18 | UNIPERIF_MAX_FRAME_SZ;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_uniperiph_dev_data {
    pub id: c_uint,      /* Nb available player instances */
    pub version: c_uint, /* player IP version */
    pub stream: c_uint,
    pub dai_names: *const c_char,
    pub type_: uniperif_type,
}

pub type uniperif_type = c_uint;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct tdm_slot {
    pub slots: c_int,
    pub slot_width: c_int,
    pub mask: c_uint,
    pub avail_slots: c_int,
}

#[repr(C)]
pub struct uniperif_hw {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64_t,
}

#[repr(C)]
pub struct uniperif {
    pub id: c_uint,
    pub ver: c_uint,
    pub dev: *mut device,
    pub tdm_slot: tdm_slot,
    pub type_: uniperif_type,
    pub state: c_uint,
    pub daifmt: c_uint,
    pub num_ctrls: c_uint,
    pub snd_ctrls: *mut snd_kcontrol_new,
    pub fifo_phys_address: usize,
    pub base: *mut c_void,
    pub mem_region: *mut resource,
    pub irq: c_int,
    pub dai_ops: *const snd_soc_dai_ops,
    pub hw: *const uniperif_hw,
}

#[repr(C)]
pub struct sti_uniperiph_dai {
    pub uni: *mut uniperif,
    pub stream: c_uint,
    pub dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct sti_uniperiph_data {
    pub pdev: *mut platform_device,
    pub dai: *mut snd_soc_dai_driver,
    pub dai_data: sti_uniperiph_dai,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub index: c_uint,
    pub device: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: usize,
    pub addr_width: c_uint,
    pub maxburst: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64_t,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub ops: *const snd_soc_dai_ops,
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_uint,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    pub bits: [u_int32_t; SNDRV_MASK_MAX / 32],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    fn SET_UNIPERIF_SOFT_RST_SOFT_RST(uni: *mut uniperif);
    fn GET_UNIPERIF_SOFT_RST_SOFT_RST(uni: *mut uniperif) -> c_int;
    fn UNIPERIF_TYPE_IS_TDM(uni: *mut uniperif) -> bool;
    fn UNIPERIF_FIFO_DATA_OFFSET(uni: *mut uniperif) -> usize;
    fn udelay(usecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut sti_uniperiph_data;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut sti_uniperiph_data;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_mask;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn pinctrl_pm_select_sleep_state(dev: *mut device) -> c_int;
    fn pinctrl_pm_select_default_state(dev: *mut device) -> c_int;
    fn uni_player_resume(uni: *mut uniperif) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn of_match_node(matches: *const of_device_id, node: *mut device_node) -> *const of_device_id;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn of_property_read_string(
        node: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn uni_player_init(pdev: *mut platform_device, uni: *mut uniperif) -> c_int;
    fn uni_reader_init(pdev: *mut platform_device, uni: *mut uniperif) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_uint,
    ) -> c_int;
}

static sti_uniplayer_hdmi: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 0,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_PLAYBACK },
    dai_names: b"Uni Player #0 (HDMI)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_HDMI },
};

static sti_uniplayer_pcm_out: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 1,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_PLAYBACK },
    dai_names: b"Uni Player #1 (PCM OUT)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_PCM | SND_ST_UNIPERIF_TYPE_TDM },
};

static sti_uniplayer_dac: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 2,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_PLAYBACK },
    dai_names: b"Uni Player #2 (DAC)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_PCM },
};

static sti_uniplayer_spdif: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 3,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_PLAYBACK },
    dai_names: b"Uni Player #3 (SPDIF)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_SPDIF },
};

static sti_unireader_pcm_in: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 0,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_RDR_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_CAPTURE },
    dai_names: b"Uni Reader #0 (PCM IN)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_PCM | SND_ST_UNIPERIF_TYPE_TDM },
};

static sti_unireader_hdmi_in: sti_uniperiph_dev_data = sti_uniperiph_dev_data {
    id: 1,
    version: unsafe { SND_ST_UNIPERIF_VERSION_UNI_RDR_1_0 },
    stream: unsafe { SNDRV_PCM_STREAM_CAPTURE },
    dai_names: b"Uni Reader #1 (HDMI IN)\0".as_ptr() as *const c_char,
    type_: unsafe { SND_ST_UNIPERIF_TYPE_PCM },
};

static snd_soc_sti_match: [of_device_id; 7] = [
    of_device_id {
        compatible: b"st,stih407-uni-player-hdmi\0".as_ptr() as *const c_char,
        data: &sti_uniplayer_hdmi as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"st,stih407-uni-player-pcm-out\0".as_ptr() as *const c_char,
        data: &sti_uniplayer_pcm_out as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"st,stih407-uni-player-dac\0".as_ptr() as *const c_char,
        data: &sti_uniplayer_dac as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"st,stih407-uni-player-spdif\0".as_ptr() as *const c_char,
        data: &sti_uniplayer_spdif as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"st,stih407-uni-reader-pcm_in\0".as_ptr() as *const c_char,
        data: &sti_unireader_pcm_in as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"st,stih407-uni-reader-hdmi\0".as_ptr() as *const c_char,
        data: &sti_unireader_hdmi_in as *const _ as *const c_void,
    },
    of_device_id {
        compatible: null(),
        data: null(),
    },
];
/* MODULE_DEVICE_TABLE(of, snd_soc_sti_match); */

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_reset(uni: *mut uniperif) -> c_int {
    let mut count: c_int = 10;

    /* Reset uniperipheral uni */
    SET_UNIPERIF_SOFT_RST_SOFT_RST(uni);

    if (*uni).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        while GET_UNIPERIF_SOFT_RST_SOFT_RST(uni) != 0 && count != 0 {
            udelay(5);
            count -= 1;
        }
    }

    if count == 0 {
        dev_err((*uni).dev, b"Failed to reset uniperif\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let uni = (*priv_).dai_data.uni;
    let mut i: c_int;
    let frame_size: c_int;
    let mut avail_slots: c_int;

    if !UNIPERIF_TYPE_IS_TDM(uni) {
        dev_err((*uni).dev, b"cpu dai not in tdm mode\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* store info in unip context */
    (*uni).tdm_slot.slots = slots;
    (*uni).tdm_slot.slot_width = slot_width;
    /* unip is unidirectionnal */
    (*uni).tdm_slot.mask = if tx_mask != 0 { tx_mask } else { rx_mask };

    /* number of available timeslots */
    i = 0;
    avail_slots = 0;
    while i < (*uni).tdm_slot.slots {
        if (((*uni).tdm_slot.mask >> i) & 0x01) != 0 {
            avail_slots += 1;
        }
        i += 1;
    }
    (*uni).tdm_slot.avail_slots = avail_slots;

    /* frame size in bytes */
    frame_size = (*uni).tdm_slot.avail_slots * (*uni).tdm_slot.slot_width / 8;

    /* check frame size is allowed */
    if frame_size > UNIPERIF_MAX_FRAME_SZ
        || (frame_size & !(UNIPERIF_ALLOWED_FRAME_SZ as c_int)) != 0
    {
        dev_err(
            (*uni).dev,
            b"frame size not allowed: %d bytes\n\0".as_ptr() as *const c_char,
            frame_size,
        );
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_fix_tdm_chan(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let uni = (*rule).private as *mut uniperif;
    let mut t = snd_interval {
        min: (*uni).tdm_slot.avail_slots as c_uint,
        max: (*uni).tdm_slot.avail_slots as c_uint,
        openmin: 0,
        openmax: 0,
        integer: 0,
    };

    snd_interval_refine(hw_param_interval(params, (*rule).var), &mut t)
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_fix_tdm_format(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let uni = (*rule).private as *mut uniperif;
    let maskp = hw_param_mask(params, (*rule).var);
    let format: u64_t;

    match (*uni).tdm_slot.slot_width {
        16 => {
            format = SNDRV_PCM_FMTBIT_S16_LE;
        }
        32 => {
            format = SNDRV_PCM_FMTBIT_S32_LE;
        }
        _ => {
            dev_err(
                (*uni).dev,
                b"format not supported: %d bits\n\0".as_ptr() as *const c_char,
                (*uni).tdm_slot.slot_width,
            );
            return -EINVAL;
        }
    }

    (*maskp).bits[0] &= format as u_int32_t;
    (*maskp).bits[1] &= (format >> 32) as u_int32_t;
    /* clear remaining indexes */
    memset(
        (*maskp).bits.as_mut_ptr().add(2) as *mut c_void,
        0,
        (SNDRV_MASK_MAX - 64) / 8,
    );

    if (*maskp).bits[0] == 0 && (*maskp).bits[1] == 0 {
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_get_tdm_word_pos(
    uni: *mut uniperif,
    word_pos: *mut c_uint,
) -> c_int {
    let slot_width: c_int = (*uni).tdm_slot.slot_width / 8;
    let slots_num: c_int = (*uni).tdm_slot.slots;
    let slots_mask: c_uint = (*uni).tdm_slot.mask;
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;
    let mut word16_pos: [c_uint; 4] = [0; 4];

    /* word16_pos:
     * word16_pos[0] = WORDX_LSB
     * word16_pos[1] = WORDX_MSB,
     * word16_pos[2] = WORDX+1_LSB
     * word16_pos[3] = WORDX+1_MSB
     */

    /* set unip word position */
    i = 0;
    j = 0;
    k = 0;
    while i < slots_num && k < WORD_MAX {
        if ((slots_mask >> i) & 0x01) != 0 {
            word16_pos[j as usize] = (i * slot_width) as c_uint;

            if slot_width == 4 {
                word16_pos[(j + 1) as usize] = word16_pos[j as usize] + 2;
                j += 1;
            }
            j += 1;

            if j > 3 {
                *word_pos.add(k as usize) = word16_pos[1]
                    | (word16_pos[0] << 8)
                    | (word16_pos[3] << 16)
                    | (word16_pos[2] << 24);
                j = 0;
                k += 1;
            }
        }
        i += 1;
    }

    0
}

/*
 * sti_uniperiph_dai_create_ctrl
 * This function is used to create Ctrl associated to DAI but also pcm device.
 * Request is done by front end to associate ctrl with pcm device id
 */
unsafe extern "C" fn sti_uniperiph_dai_create_ctrl(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let uni = (*priv_).dai_data.uni;
    let mut ctrl: *mut snd_kcontrol_new;
    let mut i: c_int;

    if (*uni).num_ctrls == 0 {
        return 0;
    }

    i = 0;
    while i < (*uni).num_ctrls as c_int {
        /*
         * Several Control can have same name. Controls are indexed on
         * Uniperipheral instance ID
         */
        ctrl = (*uni).snd_ctrls.add(i as usize);
        (*ctrl).index = (*uni).id;
        (*ctrl).device = (*uni).id;
        i += 1;
    }

    snd_soc_add_dai_controls(dai, (*uni).snd_ctrls, (*uni).num_ctrls)
}

/*
 * DAI
 */
#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let uni = (*priv_).dai_data.uni;
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let transfer_size: c_int;

    if (*uni).type_ == SND_ST_UNIPERIF_TYPE_TDM {
        /* transfer size = user frame size (in 32-bits FIFO cell) */
        transfer_size = snd_soc_params_to_frame_size(params) / 32;
    } else {
        transfer_size = params_channels(params) * UNIPERIF_FIFO_FRAMES;
    }

    dma_data = snd_soc_dai_get_dma_data(dai, substream);
    (*dma_data).maxburst = transfer_size;

    0
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_dai_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);

    (*(*priv_).dai_data.uni).daifmt = fmt;

    0
}

unsafe extern "C" fn sti_uniperiph_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);
    let uni = (*priv_).dai_data.uni;
    let ret: c_int;

    /* The uniperipheral should be in stopped state */
    if (*uni).state != UNIPERIF_STATE_STOPPED {
        dev_err(
            (*uni).dev,
            b"%s: invalid uni state( %d)\n\0".as_ptr() as *const c_char,
            b"sti_uniperiph_suspend\0".as_ptr() as *const c_char,
            (*uni).state as c_int,
        );
        return -EBUSY;
    }

    /* Pinctrl: switch pinstate to sleep */
    ret = pinctrl_pm_select_sleep_state((*uni).dev);
    if ret != 0 {
        dev_err(
            (*uni).dev,
            b"%s: failed to select pinctrl state\n\0".as_ptr() as *const c_char,
            b"sti_uniperiph_suspend\0".as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn sti_uniperiph_resume(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);
    let uni = (*priv_).dai_data.uni;
    let mut ret: c_int;

    if (*priv_).dai_data.stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = uni_player_resume(uni);
        if ret != 0 {
            return ret;
        }
    }

    /* pinctrl: switch pinstate to default */
    ret = pinctrl_pm_select_default_state((*uni).dev);
    if ret != 0 {
        dev_err(
            (*uni).dev,
            b"%s: failed to select pinctrl state\n\0".as_ptr() as *const c_char,
            b"sti_uniperiph_resume\0".as_ptr() as *const c_char,
        );
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sti_uniperiph_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let dai_data = &mut (*priv_).dai_data as *mut sti_uniperiph_dai;

    /* DMA settings*/
    if (*priv_).dai_data.stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_dai_init_dma_data(dai, &mut (*dai_data).dma_data, null_mut());
    } else {
        snd_soc_dai_init_dma_data(dai, null_mut(), &mut (*dai_data).dma_data);
    }

    (*dai_data).dma_data.addr = (*(*dai_data).uni).fifo_phys_address;
    (*dai_data).dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;

    sti_uniperiph_dai_create_ctrl(dai)
}

static sti_uniperiph_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(sti_uniperiph_dai_probe),
};

static sti_uniperiph_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: &sti_uniperiph_dai_ops,
    name: null(),
    playback: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    },
};

static sti_uniperiph_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"sti_cpu_dai\0".as_ptr() as *const c_char,
    suspend: Some(sti_uniperiph_suspend),
    resume: Some(sti_uniperiph_resume),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn sti_uniperiph_cpu_dai_of(
    node: *mut device_node,
    priv_: *mut sti_uniperiph_data,
) -> c_int {
    let dev = &mut (*(*priv_).pdev).dev as *mut device;
    let dai_data = &mut (*priv_).dai_data as *mut sti_uniperiph_dai;
    let dai = (*priv_).dai;
    let stream: *mut snd_soc_pcm_stream;
    let uni: *mut uniperif;
    let of_id: *const of_device_id;
    let dev_data: *const sti_uniperiph_dev_data;
    let mut mode: *const c_char = null();
    let ret: c_int;

    /* Populate data structure depending on compatibility */
    of_id = of_match_node(snd_soc_sti_match.as_ptr(), node);
    if (*of_id).data.is_null() {
        dev_err(dev, b"data associated to device is missing\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    dev_data = (*of_id).data as *const sti_uniperiph_dev_data;

    uni = devm_kzalloc(dev, size_of::<uniperif>(), GFP_KERNEL) as *mut uniperif;
    if uni.is_null() {
        return -ENOMEM;
    }

    (*uni).id = (*dev_data).id;
    (*uni).ver = (*dev_data).version;

    *dai = sti_uniperiph_dai_template;
    (*dai).name = (*dev_data).dai_names;

    /* Get resources and base address */
    (*uni).base = devm_platform_get_and_ioremap_resource(
        (*priv_).pdev,
        0,
        &mut (*uni).mem_region,
    );
    if IS_ERR((*uni).base) {
        return PTR_ERR((*uni).base);
    }

    (*uni).fifo_phys_address =
        (*(*uni).mem_region).start + UNIPERIF_FIFO_DATA_OFFSET(uni);

    (*uni).irq = platform_get_irq((*priv_).pdev, 0);
    if (*uni).irq < 0 {
        return -ENXIO;
    }

    (*uni).type_ = (*dev_data).type_;

    /* check if player should be configured for tdm */
    if ((*dev_data).type_ & SND_ST_UNIPERIF_TYPE_TDM) != 0 {
        if of_property_read_string(
            node,
            b"st,tdm-mode\0".as_ptr() as *const c_char,
            &mut mode,
        ) == 0
        {
            (*uni).type_ = SND_ST_UNIPERIF_TYPE_TDM;
        } else {
            (*uni).type_ = SND_ST_UNIPERIF_TYPE_PCM;
        }
    }

    (*dai_data).uni = uni;
    (*dai_data).stream = (*dev_data).stream;

    if (*priv_).dai_data.stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = uni_player_init((*priv_).pdev, uni);
        stream = &mut (*dai).playback;
    } else {
        ret = uni_reader_init((*priv_).pdev, uni);
        stream = &mut (*dai).capture;
    }
    if ret < 0 {
        return ret;
    }

    (*dai).ops = (*uni).dai_ops;

    (*stream).stream_name = (*dai).name;
    (*stream).channels_min = (*(*uni).hw).channels_min;
    (*stream).channels_max = (*(*uni).hw).channels_max;
    (*stream).rates = (*(*uni).hw).rates;
    (*stream).formats = (*(*uni).hw).formats;

    0
}

unsafe extern "C" fn sti_uniperiph_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut sti_uniperiph_data;
    let node = (*pdev).dev.of_node;
    let mut ret: c_int;

    /* Allocate the private data and the CPU_DAI array */
    priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<sti_uniperiph_data>(), GFP_KERNEL)
        as *mut sti_uniperiph_data;
    if priv_.is_null() {
        return -ENOMEM;
    }
    (*priv_).dai = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if (*priv_).dai.is_null() {
        return -ENOMEM;
    }

    (*priv_).pdev = pdev;

    ret = sti_uniperiph_cpu_dai_of(node, priv_);
    if ret < 0 {
        return ret;
    }

    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sti_uniperiph_dai_component,
        (*priv_).dai,
        1,
    );
    if ret < 0 {
        return ret;
    }

    devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, null(), 0)
}

static mut sti_uniperiph_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"sti-uniperiph-dai\0".as_ptr() as *const c_char,
        of_match_table: snd_soc_sti_match.as_ptr(),
    },
    probe: Some(sti_uniperiph_probe),
};
/* module_platform_driver(sti_uniperiph_driver); */

/* MODULE_DESCRIPTION("uniperipheral DAI driver"); */
/* MODULE_AUTHOR("Arnaud Pouliquen <arnaud.pouliquen@st.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
