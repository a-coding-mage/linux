// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) STMicroelectronics SA 2015
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          for STMicroelectronics.
 */

// Dependencies from <sound/soc.h> and "uniperif.h" are provided externally.

use core::ffi::{c_int, c_uint, c_void};

const UNIPERIF_READER_I2S_IN: c_int = 0; /* reader id connected to I2S/TDM TX bus */

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: c_int,
    pub channels: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub buffer_bytes_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn() -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct dai_data {
    pub uni: *mut uniperif,
}

#[repr(C)]
pub struct sti_uniperiph_data {
    pub dai_data: dai_data,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct uniperif {
    pub dev: *mut device,
    pub state: c_int,
    pub dai_ops: *const snd_soc_dai_ops,
    pub hw: *const snd_pcm_hardware,
    pub irq: c_int,
    pub irq_lock: spinlock_t,
    pub substream: *mut snd_pcm_substream,
    pub daifmt: c_uint,
    pub type_: c_int,
    pub ver: c_int,
    pub underflow_enabled: bool,
}

pub type irqreturn_t = c_uint;

unsafe extern "C" {
    static uni_tdm_hw: snd_pcm_hardware;

    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static PAGE_SIZE: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static SNDRV_PCM_STATE_XRUN: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static UNIPERIF_STATE_STOPPED: c_int;
    static UNIPERIF_STATE_STARTED: c_int;
    static SND_ST_UNIPERIF_TYPE_TDM: c_int;
    static SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0: c_int;
    static UNIPERIF_FIFO_FRAMES: c_int;
    static UNIPERIF_FIFO_SIZE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static IRQF_SHARED: c_uint;
    static WORD_1_2: usize;
    static WORD_3_4: usize;
    static WORD_5_6: usize;
    static WORD_7_8: usize;

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_stream_lock(substream: *mut snd_pcm_substream);
    fn snd_pcm_stream_unlock(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int);
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(),
        private: *mut c_void,
        dep: c_int,
        last: c_int,
    ) -> c_int;
    fn sti_uniperiph_get_user_frame_size(runtime: *mut snd_pcm_runtime) -> c_int;
    fn sti_uniperiph_get_tdm_word_pos(reader: *mut uniperif, word_pos: *mut c_uint);
    fn sti_uniperiph_reset(reader: *mut uniperif) -> c_int;
    fn sti_uniperiph_fix_tdm_chan();
    fn sti_uniperiph_fix_tdm_format();
    fn sti_uniperiph_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn sti_uniperiph_dai_hw_params() -> c_int;
    fn sti_uniperiph_dai_set_fmt() -> c_int;
    fn sti_uniperiph_set_tdm_slot() -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const i8,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const i8;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn dev_warn(dev: *mut device, fmt: *const i8, ...);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);

    fn GET_UNIPERIF_ITS(reader: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITS_BCLR(reader: *mut uniperif, value: c_uint);
    fn UNIPERIF_ITS_FIFO_ERROR_MASK(reader: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_I2S_FMT_NBIT_32(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NBIT_16(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(reader: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_MEM_FMT_16_16(reader: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_MEM_FMT_16_0(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NUM_CH(reader: *mut uniperif, value: c_uint);
    fn SET_UNIPERIF_I2S_FMT_ORDER_MSB(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(reader: *mut uniperif);
    fn SET_UNIPERIF_TDM_ENABLE_TDM_ENABLE(reader: *mut uniperif);
    fn SET_UNIPERIF_TDM_WORD_POS_1_2(reader: *mut uniperif, value: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_3_4(reader: *mut uniperif, value: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_5_6(reader: *mut uniperif, value: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_7_8(reader: *mut uniperif, value: c_uint);
    fn UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK(reader: *mut uniperif) -> c_int;
    fn SET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT(reader: *mut uniperif, value: c_int);
    fn UNIPERIF_TYPE_IS_TDM(reader: *mut uniperif) -> bool;
    fn SET_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_ALIGN_RIGHT(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_LR_POL_LOW(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_LR_POL_HIG(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(reader: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ(reader: *mut uniperif, value: c_uint);
    fn SET_UNIPERIF_ITM_BSET_DMA_ERROR(reader: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_FIFO_ERROR(reader: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_MEM_BLK_READ(reader: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE(reader: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED(reader: *mut uniperif);
    fn SET_UNIPERIF_ITS_BCLR_FIFO_ERROR(reader: *mut uniperif);
    fn SET_UNIPERIF_CTRL_OPERATION_PCM_DATA(reader: *mut uniperif);
    fn SET_UNIPERIF_CTRL_OPERATION_OFF(reader: *mut uniperif);
    fn GET_UNIPERIF_ITM(reader: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITM_BCLR(reader: *mut uniperif, value: c_uint);
}

type c_ulong = core::ffi::c_ulong;

/*
 * Note: snd_pcm_hardware is linked to DMA controller but is declared here to
 * integrate unireader capability in term of rate and supported channels
 */
static uni_reader_pcm_hw: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID,
        formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S16_LE,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 96000,
        channels_min: 2,
        channels_max: 8,
        periods_min: 2,
        periods_max: 48,
        period_bytes_min: 128,
        period_bytes_max: 64 * PAGE_SIZE,
        buffer_bytes_max: 256 * PAGE_SIZE,
    }
};

/*
 * uni_reader_irq_handler
 * In case of error audio stream is stopped; stop action is protected via PCM
 * stream lock  to avoid race condition with trigger callback.
 */
unsafe extern "C" fn uni_reader_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mut ret = IRQ_NONE;
    let reader = dev_id as *mut uniperif;
    let status: c_uint;

    spin_lock(core::ptr::addr_of_mut!((*reader).irq_lock));
    if (*reader).substream.is_null() {
        spin_unlock(core::ptr::addr_of_mut!((*reader).irq_lock));
        return ret;
    }

    snd_pcm_stream_lock((*reader).substream);
    if (*reader).state == UNIPERIF_STATE_STOPPED {
        /* Unexpected IRQ: do nothing */
        dev_warn((*reader).dev, c"unexpected IRQ\n".as_ptr());
        snd_pcm_stream_unlock((*reader).substream);
        spin_unlock(core::ptr::addr_of_mut!((*reader).irq_lock));
        return ret;
    }

    /* Get interrupt status & clear them immediately */
    status = GET_UNIPERIF_ITS(reader);
    SET_UNIPERIF_ITS_BCLR(reader, status);

    /* Check for fifo overflow error */
    if status & UNIPERIF_ITS_FIFO_ERROR_MASK(reader) != 0 {
        dev_err((*reader).dev, c"FIFO error detected\n".as_ptr());

        snd_pcm_stop((*reader).substream, SNDRV_PCM_STATE_XRUN);

        ret = IRQ_HANDLED;
    }

    snd_pcm_stream_unlock((*reader).substream);
    spin_unlock(core::ptr::addr_of_mut!((*reader).irq_lock));

    ret
}

unsafe extern "C" fn uni_reader_prepare_pcm(
    runtime: *mut snd_pcm_runtime,
    reader: *mut uniperif,
) -> c_int {
    let slot_width: c_int;

    /* Force slot width to 32 in I2S mode */
    if ((*reader).daifmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
        slot_width = 32;
    } else {
        match (*runtime).format {
            x if x == SNDRV_PCM_FORMAT_S16_LE => {
                slot_width = 16;
            }
            _ => {
                slot_width = 32;
            }
        }
    }

    /* Number of bits per subframe (i.e one channel sample) on input. */
    match slot_width {
        32 => {
            SET_UNIPERIF_I2S_FMT_NBIT_32(reader);
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(reader);
        }
        16 => {
            SET_UNIPERIF_I2S_FMT_NBIT_16(reader);
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(reader);
        }
        _ => {
            dev_err((*reader).dev, c"subframe format not supported\n".as_ptr());
            return -EINVAL;
        }
    }

    /* Configure data memory format */
    match (*runtime).format {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            /* One data word contains two samples */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_16(reader);
        }
        x if x == SNDRV_PCM_FORMAT_S32_LE => {
            /*
             * Actually "16 bits/0 bits" means "32/28/24/20/18/16 bits
             * on the MSB then zeros (if less than 32 bytes)"...
             */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_0(reader);
        }
        _ => {
            dev_err((*reader).dev, c"format not supported\n".as_ptr());
            return -EINVAL;
        }
    }

    /* Number of channels must be even */
    if ((*runtime).channels % 2) != 0 || (*runtime).channels < 2 || (*runtime).channels > 10 {
        dev_err((*reader).dev, c"%s: invalid nb of channels\n".as_ptr(), c"uni_reader_prepare_pcm".as_ptr());
        return -EINVAL;
    }

    SET_UNIPERIF_I2S_FMT_NUM_CH(reader, (*runtime).channels / 2);
    SET_UNIPERIF_I2S_FMT_ORDER_MSB(reader);

    0
}

unsafe extern "C" fn uni_reader_prepare_tdm(
    runtime: *mut snd_pcm_runtime,
    reader: *mut uniperif,
) -> c_int {
    let frame_size: c_int; /* user tdm frame size in bytes */
    /* default unip TDM_WORD_POS_X_Y */
    let mut word_pos: [c_uint; 4] = [0x04060002, 0x0C0E080A, 0x14161012, 0x1C1E181A];

    frame_size = sti_uniperiph_get_user_frame_size(runtime);

    /* fix 16/0 format */
    SET_UNIPERIF_CONFIG_MEM_FMT_16_0(reader);
    SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(reader);

    /* number of words inserted on the TDM line */
    SET_UNIPERIF_I2S_FMT_NUM_CH(reader, (frame_size / 4 / 2) as c_uint);

    SET_UNIPERIF_I2S_FMT_ORDER_MSB(reader);
    SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(reader);
    SET_UNIPERIF_TDM_ENABLE_TDM_ENABLE(reader);

    /*
     * set the timeslots allocation for words in FIFO
     *
     * HW bug: (LSB word < MSB word) => this config is not possible
     *         So if we want (LSB word < MSB) word, then it shall be
     *         handled by user
     */
    sti_uniperiph_get_tdm_word_pos(reader, word_pos.as_mut_ptr());
    SET_UNIPERIF_TDM_WORD_POS_1_2(reader, word_pos[WORD_1_2]);
    SET_UNIPERIF_TDM_WORD_POS_3_4(reader, word_pos[WORD_3_4]);
    SET_UNIPERIF_TDM_WORD_POS_5_6(reader, word_pos[WORD_5_6]);
    SET_UNIPERIF_TDM_WORD_POS_7_8(reader, word_pos[WORD_7_8]);

    0
}

unsafe extern "C" fn uni_reader_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut sti_uniperiph_data;
    let reader = (*priv_).dai_data.uni;
    let runtime = (*substream).runtime;
    let transfer_size: c_int;
    let trigger_limit: c_int;
    let ret: c_int;

    /* The reader should be stopped */
    if (*reader).state != UNIPERIF_STATE_STOPPED {
        dev_err(
            (*reader).dev,
            c"%s: invalid reader state %d\n".as_ptr(),
            c"uni_reader_prepare".as_ptr(),
            (*reader).state,
        );
        return -EINVAL;
    }

    /* Calculate transfer size (in fifo cells and bytes) for frame count */
    if (*reader).type_ == SND_ST_UNIPERIF_TYPE_TDM {
        /* transfer size = unip frame size (in 32 bits FIFO cell) */
        transfer_size = sti_uniperiph_get_user_frame_size(runtime) / 4;
    } else {
        transfer_size = (*runtime).channels as c_int * UNIPERIF_FIFO_FRAMES;
    }

    /* Calculate number of empty cells available before asserting DREQ */
    if (*reader).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        trigger_limit = UNIPERIF_FIFO_SIZE - transfer_size;
    } else {
        /*
         * Since SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0
         * FDMA_TRIGGER_LIMIT also controls when the state switches
         * from OFF or STANDBY to AUDIO DATA.
         */
        trigger_limit = transfer_size;
    }

    /* Trigger limit must be an even number */
    if ((!trigger_limit) % 2) != 0
        || (trigger_limit != 1 && transfer_size % 2 != 0)
        || (trigger_limit > UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK(reader))
    {
        dev_err((*reader).dev, c"invalid trigger limit %d\n".as_ptr(), trigger_limit);
        return -EINVAL;
    }

    SET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT(reader, trigger_limit);

    if UNIPERIF_TYPE_IS_TDM(reader) {
        ret = uni_reader_prepare_tdm(runtime, reader);
    } else {
        ret = uni_reader_prepare_pcm(runtime, reader);
    }
    if ret != 0 {
        return ret;
    }

    match (*reader).daifmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(reader);
            SET_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(reader);
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(reader);
            SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(reader);
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            SET_UNIPERIF_I2S_FMT_ALIGN_RIGHT(reader);
            SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(reader);
        }
        _ => {
            dev_err((*reader).dev, c"format not supported\n".as_ptr());
            return -EINVAL;
        }
    }

    /* Data clocking (changing) on the rising/falling edge */
    match (*reader).daifmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_LOW(reader);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(reader);
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_HIG(reader);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(reader);
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_LOW(reader);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(reader);
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_HIG(reader);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(reader);
        }
        _ => {}
    }

    /* Clear any pending interrupts */
    SET_UNIPERIF_ITS_BCLR(reader, GET_UNIPERIF_ITS(reader));

    SET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ(reader, 0);

    /* Set the interrupt mask */
    SET_UNIPERIF_ITM_BSET_DMA_ERROR(reader);
    SET_UNIPERIF_ITM_BSET_FIFO_ERROR(reader);
    SET_UNIPERIF_ITM_BSET_MEM_BLK_READ(reader);

    /* Enable underflow recovery interrupts */
    if (*reader).underflow_enabled {
        SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE(reader);
        SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED(reader);
    }

    /* Reset uniperipheral reader */
    sti_uniperiph_reset(reader)
}

unsafe extern "C" fn uni_reader_start(reader: *mut uniperif) -> c_int {
    /* The reader should be stopped */
    if (*reader).state != UNIPERIF_STATE_STOPPED {
        dev_err((*reader).dev, c"%s: invalid reader state\n".as_ptr(), c"uni_reader_start".as_ptr());
        return -EINVAL;
    }

    /* Enable reader interrupts (and clear possible stalled ones) */
    SET_UNIPERIF_ITS_BCLR_FIFO_ERROR(reader);
    SET_UNIPERIF_ITM_BSET_FIFO_ERROR(reader);

    /* Launch the reader */
    SET_UNIPERIF_CTRL_OPERATION_PCM_DATA(reader);

    /* Update state to started */
    (*reader).state = UNIPERIF_STATE_STARTED;
    0
}

unsafe extern "C" fn uni_reader_stop(reader: *mut uniperif) -> c_int {
    /* The reader should not be in stopped state */
    if (*reader).state == UNIPERIF_STATE_STOPPED {
        dev_err((*reader).dev, c"%s: invalid reader state\n".as_ptr(), c"uni_reader_stop".as_ptr());
        return -EINVAL;
    }

    /* Turn the reader off */
    SET_UNIPERIF_CTRL_OPERATION_OFF(reader);

    /* Disable interrupts */
    SET_UNIPERIF_ITM_BCLR(reader, GET_UNIPERIF_ITM(reader));

    /* Update state to stopped and return */
    (*reader).state = UNIPERIF_STATE_STOPPED;

    0
}

unsafe extern "C" fn uni_reader_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut sti_uniperiph_data;
    let reader = (*priv_).dai_data.uni;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => uni_reader_start(reader),
        x if x == SNDRV_PCM_TRIGGER_STOP => uni_reader_stop(reader),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn uni_reader_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut sti_uniperiph_data;
    let reader = (*priv_).dai_data.uni;
    let ret: c_int;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(core::ptr::addr_of_mut!((*reader).irq_lock), &mut flags);
    (*reader).substream = substream;
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*reader).irq_lock), flags);

    if !UNIPERIF_TYPE_IS_TDM(reader) {
        return 0;
    }

    /* refine hw constraint in tdm mode */
    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        core::mem::transmute(sti_uniperiph_fix_tdm_chan as unsafe extern "C" fn()),
        reader as *mut c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret < 0 {
        return ret;
    }

    snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_FORMAT,
        core::mem::transmute(sti_uniperiph_fix_tdm_format as unsafe extern "C" fn()),
        reader as *mut c_void,
        SNDRV_PCM_HW_PARAM_FORMAT,
        -1,
    )
}

unsafe extern "C" fn uni_reader_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut sti_uniperiph_data;
    let reader = (*priv_).dai_data.uni;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(core::ptr::addr_of_mut!((*reader).irq_lock), &mut flags);
    if (*reader).state != UNIPERIF_STATE_STOPPED {
        /* Stop the reader */
        uni_reader_stop(reader);
    }
    (*reader).substream = core::ptr::null_mut();
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*reader).irq_lock), flags);
}

static uni_reader_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(uni_reader_startup),
    shutdown: Some(uni_reader_shutdown),
    prepare: Some(uni_reader_prepare),
    probe: Some(sti_uniperiph_dai_probe),
    trigger: Some(uni_reader_trigger),
    hw_params: Some(sti_uniperiph_dai_hw_params),
    set_fmt: Some(sti_uniperiph_dai_set_fmt),
    set_tdm_slot: Some(sti_uniperiph_set_tdm_slot),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uni_reader_init(
    pdev: *mut platform_device,
    reader: *mut uniperif,
) -> c_int {
    let mut ret: c_int = 0;

    (*reader).dev = core::ptr::addr_of_mut!((*pdev).dev);
    (*reader).state = UNIPERIF_STATE_STOPPED;
    (*reader).dai_ops = &uni_reader_dai_ops;

    if UNIPERIF_TYPE_IS_TDM(reader) {
        (*reader).hw = &uni_tdm_hw;
    } else {
        (*reader).hw = &uni_reader_pcm_hw;
    }

    ret = devm_request_irq(
        core::ptr::addr_of_mut!((*pdev).dev),
        (*reader).irq,
        uni_reader_irq_handler,
        IRQF_SHARED,
        dev_name(core::ptr::addr_of_mut!((*pdev).dev)),
        reader as *mut c_void,
    );
    if ret < 0 {
        dev_err(core::ptr::addr_of_mut!((*pdev).dev), c"Failed to request IRQ\n".as_ptr());
        return -EBUSY;
    }

    spin_lock_init(core::ptr::addr_of_mut!((*reader).irq_lock));

    0
}

// EXPORT_SYMBOL_GPL(uni_reader_init);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
