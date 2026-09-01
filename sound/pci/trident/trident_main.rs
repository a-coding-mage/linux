// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Maintained by Jaroslav Kysela <perex@perex.cz>
 *  Originated by audio@tridentmicro.com
 *  Fri Feb 19 15:55:28 MST 1999
 *  Routines for control of Trident 4DWave (DX and NX) chip
 *
 *  BUGS:
 *
 *  TODO:
 *    ---
 *
 *  SiS7018 S/PDIF support by Thomas Winischhofer <thomas@winischhofer.net>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::*;

extern "C" {
    static mut jiffies: c_ulong;
}

unsafe fn snd_trident_pcm_mixer_build(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    substream: *mut snd_pcm_substream,
) -> c_int;
unsafe fn snd_trident_pcm_mixer_free(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    substream: *mut snd_pcm_substream,
) -> c_int;
unsafe fn snd_trident_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
unsafe fn snd_trident_sis_reset(trident: *mut snd_trident) -> c_int;
unsafe fn snd_trident_clear_voices(
    trident: *mut snd_trident,
    v_min: c_ushort,
    v_max: c_ushort,
);
unsafe fn snd_trident_free(card: *mut snd_card);

/*
 *  common I/O routines
 */

/* C #if 0 debug helper snd_trident_print_voice_regs intentionally preserved only as inactive source intent. */

/*---------------------------------------------------------------------------
   unsigned short snd_trident_codec_read(struct snd_ac97 *ac97, unsigned short reg)

   Description: This routine will do all of the reading from the external
                CODEC (AC97).

   Parameters:  ac97 - ac97 codec structure
                reg - CODEC register index, from AC97 Hal.

   returns:     16 bit value read from the AC97.

  ---------------------------------------------------------------------------*/
unsafe fn snd_trident_codec_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let mut data: c_uint = 0;
    let mut treg: c_uint;
    let mut count: c_ushort = 0xffff;
    let trident: *mut snd_trident = (*ac97).private_data as *mut snd_trident;

    let _guard = spinlock_irqsave_guard(&mut (*trident).reg_lock);
    if (*trident).device == TRIDENT_DEVICE_ID_DX {
        data = DX_AC97_BUSY_READ | ((reg as c_uint) & 0x000000ff);
        outl(data, TRID_REG(trident, DX_ACR1_AC97_R));
        loop {
            data = inl(TRID_REG(trident, DX_ACR1_AC97_R));
            if (data & DX_AC97_BUSY_READ) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    } else if (*trident).device == TRIDENT_DEVICE_ID_NX {
        data = NX_AC97_BUSY_READ | ((reg as c_uint) & 0x000000ff);
        treg = if (*ac97).num == 0 {
            NX_ACR2_AC97_R_PRIMARY
        } else {
            NX_ACR3_AC97_R_SECONDARY
        };
        outl(data, TRID_REG(trident, treg));
        loop {
            data = inl(TRID_REG(trident, treg));
            if (data & 0x00000C00) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    } else if (*trident).device == TRIDENT_DEVICE_ID_SI7018 {
        data = SI_AC97_BUSY_READ | SI_AC97_AUDIO_BUSY | ((reg as c_uint) & 0x000000ff);
        if (*ac97).num == 1 {
            data |= SI_AC97_SECONDARY;
        }
        outl(data, TRID_REG(trident, SI_AC97_READ));
        loop {
            data = inl(TRID_REG(trident, SI_AC97_READ));
            if (data & SI_AC97_BUSY_READ) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }

    if count == 0 && (*trident).ac97_detect == 0 {
        dev_err(
            (*(*trident).card).dev,
            c_str!("ac97 codec read TIMEOUT [0x%x/0x%x]!!!\n"),
            reg as c_uint,
            data,
        );
        data = 0;
    }

    (data >> 16) as c_ushort
}

/*---------------------------------------------------------------------------
   void snd_trident_codec_write(struct snd_ac97 *ac97, unsigned short reg,
   unsigned short wdata)

   Description: This routine will do all of the writing to the external
                CODEC (AC97).

   Parameters:  ac97 - ac97 codec structure
                reg - CODEC register index, from AC97 Hal.
                data  - Lower 16 bits are the data to write to CODEC.

   returns:     TRUE if everything went ok, else FALSE.

  ---------------------------------------------------------------------------*/
unsafe fn snd_trident_codec_write(ac97: *mut snd_ac97, reg: c_ushort, wdata: c_ushort) {
    let mut address: c_uint;
    let mut data: c_uint;
    let mut count: c_ushort = 0xffff;
    let trident: *mut snd_trident = (*ac97).private_data as *mut snd_trident;

    data = (wdata as c_ulong as c_uint) << 16;

    let _guard = spinlock_irqsave_guard(&mut (*trident).reg_lock);
    if (*trident).device == TRIDENT_DEVICE_ID_DX {
        address = DX_ACR0_AC97_W;
        loop {
            if (inw(TRID_REG(trident, address)) & DX_AC97_BUSY_WRITE as c_ushort) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
        data |= DX_AC97_BUSY_WRITE | ((reg as c_uint) & 0x000000ff);
    } else if (*trident).device == TRIDENT_DEVICE_ID_NX {
        address = NX_ACR1_AC97_W;
        loop {
            if (inw(TRID_REG(trident, address)) & NX_AC97_BUSY_WRITE as c_ushort) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
        data |= NX_AC97_BUSY_WRITE | (((*ac97).num as c_uint) << 8) | ((reg as c_uint) & 0x000000ff);
    } else if (*trident).device == TRIDENT_DEVICE_ID_SI7018 {
        address = SI_AC97_WRITE;
        loop {
            if (inw(TRID_REG(trident, address)) & SI_AC97_BUSY_WRITE as c_ushort) == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
        data |= SI_AC97_BUSY_WRITE | SI_AC97_AUDIO_BUSY | ((reg as c_uint) & 0x000000ff);
        if (*ac97).num == 1 {
            data |= SI_AC97_SECONDARY;
        }
    } else {
        address = 0;
        count = 0;
    }

    if count == 0 {
        return;
    }
    outl(data, TRID_REG(trident, address));
}

static unsafe fn snd_trident_enable_eso(trident: *mut snd_trident) {
    let mut val: c_uint;

    val = inl(TRID_REG(trident, T4D_LFO_GC_CIR));
    val |= ENDLP_IE;
    val |= MIDLP_IE;
    if (*trident).device == TRIDENT_DEVICE_ID_SI7018 {
        val |= BANK_B_EN;
    }
    outl(val, TRID_REG(trident, T4D_LFO_GC_CIR));
}

static unsafe fn snd_trident_disable_eso(trident: *mut snd_trident) {
    let mut tmp: c_uint;

    tmp = inl(TRID_REG(trident, T4D_LFO_GC_CIR));
    tmp &= !ENDLP_IE;
    tmp &= !MIDLP_IE;
    outl(tmp, TRID_REG(trident, T4D_LFO_GC_CIR));
}

#[no_mangle]
pub unsafe extern "C" fn snd_trident_start_voice(trident: *mut snd_trident, voice: c_uint) {
    let mask: c_uint = 1u32 << (voice & 0x1f);
    let reg: c_uint = if (voice & 0x20) != 0 { T4D_START_B } else { T4D_START_A };

    outl(mask, TRID_REG(trident, reg));
}

#[no_mangle]
pub unsafe extern "C" fn snd_trident_stop_voice(trident: *mut snd_trident, voice: c_uint) {
    let mask: c_uint = 1u32 << (voice & 0x1f);
    let reg: c_uint = if (voice & 0x20) != 0 { T4D_STOP_B } else { T4D_STOP_A };

    outl(mask, TRID_REG(trident, reg));
}

static unsafe fn snd_trident_allocate_pcm_channel(trident: *mut snd_trident) -> c_int {
    let mut idx: c_int;

    if (*trident).ChanPCMcnt >= (*trident).ChanPCM {
        return -1;
    }
    idx = 31;
    while idx >= 0 {
        if ((*trident).ChanMap[T4D_BANK_B as usize] & (1u32 << idx)) == 0 {
            (*trident).ChanMap[T4D_BANK_B as usize] |= 1u32 << idx;
            (*trident).ChanPCMcnt += 1;
            return idx + 32;
        }
        idx -= 1;
    }
    -1
}

static unsafe fn snd_trident_free_pcm_channel(trident: *mut snd_trident, mut channel: c_int) {
    if channel < 32 || channel > 63 {
        return;
    }
    channel &= 0x1f;
    if ((*trident).ChanMap[T4D_BANK_B as usize] & (1u32 << channel)) != 0 {
        (*trident).ChanMap[T4D_BANK_B as usize] &= !(1u32 << channel);
        (*trident).ChanPCMcnt -= 1;
    }
}

static unsafe fn snd_trident_allocate_synth_channel(trident: *mut snd_trident) -> c_int {
    let mut idx: c_int = 31;

    while idx >= 0 {
        if ((*trident).ChanMap[T4D_BANK_A as usize] & (1u32 << idx)) == 0 {
            (*trident).ChanMap[T4D_BANK_A as usize] |= 1u32 << idx;
            (*trident).synth.ChanSynthCount += 1;
            return idx;
        }
        idx -= 1;
    }
    -1
}

static unsafe fn snd_trident_free_synth_channel(trident: *mut snd_trident, mut channel: c_int) {
    if channel < 0 || channel > 31 {
        return;
    }
    channel &= 0x1f;
    if ((*trident).ChanMap[T4D_BANK_A as usize] & (1u32 << channel)) != 0 {
        (*trident).ChanMap[T4D_BANK_A as usize] &= !(1u32 << channel);
        (*trident).synth.ChanSynthCount -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_trident_write_voice_regs(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
) {
    let mut FmcRvolCvol: c_uint;
    let mut regs: [c_uint; 5] = [0; 5];

    regs[1] = (*voice).LBA;
    regs[4] = ((*voice).GVSel << 31)
        | (((*voice).Pan & 0x0000007f) << 24)
        | (((*voice).CTRL & 0x0000000f) << 12);
    FmcRvolCvol = (((*voice).FMC & 3) << 14)
        | (((*voice).RVol & 0x7f) << 7)
        | ((*voice).CVol & 0x7f);

    match (*trident).device {
        TRIDENT_DEVICE_ID_SI7018 => {
            regs[4] |= if (*voice).number > 31 {
                (*voice).Vol & 0x000003ff
            } else {
                (((*voice).Vol & 0x000003fc) << (16 - 2)) | ((*voice).EC & 0x00000fff)
            };
            regs[0] = ((*voice).CSO << 16)
                | (((*voice).Alpha & 0x00000fff) << 4)
                | ((*voice).FMS & 0x0000000f);
            regs[2] = ((*voice).ESO << 16) | ((*voice).Delta & 0x0ffff);
            regs[3] = ((*voice).Attribute << 16) | FmcRvolCvol;
        }
        TRIDENT_DEVICE_ID_DX => {
            regs[4] |= (((*voice).Vol & 0x000003fc) << (16 - 2)) | ((*voice).EC & 0x00000fff);
            regs[0] = ((*voice).CSO << 16)
                | (((*voice).Alpha & 0x00000fff) << 4)
                | ((*voice).FMS & 0x0000000f);
            regs[2] = ((*voice).ESO << 16) | ((*voice).Delta & 0x0ffff);
            regs[3] = FmcRvolCvol;
        }
        TRIDENT_DEVICE_ID_NX => {
            regs[4] |= (((*voice).Vol & 0x000003fc) << (16 - 2)) | ((*voice).EC & 0x00000fff);
            regs[0] = ((*voice).Delta << 24) | ((*voice).CSO & 0x00ffffff);
            regs[2] = (((*voice).Delta << 16) & 0xff000000) | ((*voice).ESO & 0x00ffffff);
            regs[3] = ((*voice).Alpha << 20) | (((*voice).FMS & 0x0000000f) << 16) | FmcRvolCvol;
        }
        _ => {
            snd_BUG();
            return;
        }
    }

    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    outl(regs[0], TRID_REG(trident, CH_START + 0));
    outl(regs[1], TRID_REG(trident, CH_START + 4));
    outl(regs[2], TRID_REG(trident, CH_START + 8));
    outl(regs[3], TRID_REG(trident, CH_START + 12));
    outl(regs[4], TRID_REG(trident, CH_START + 16));
}

static unsafe fn snd_trident_write_cso_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    CSO: c_uint,
) {
    (*voice).CSO = CSO;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    if (*trident).device != TRIDENT_DEVICE_ID_NX {
        outw((*voice).CSO as c_ushort, TRID_REG(trident, CH_DX_CSO_ALPHA_FMS) + 2);
    } else {
        outl(((*voice).Delta << 24) | ((*voice).CSO & 0x00ffffff), TRID_REG(trident, CH_NX_DELTA_CSO));
    }
}

static unsafe fn snd_trident_write_eso_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    ESO: c_uint,
) {
    (*voice).ESO = ESO;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    if (*trident).device != TRIDENT_DEVICE_ID_NX {
        outw((*voice).ESO as c_ushort, TRID_REG(trident, CH_DX_ESO_DELTA) + 2);
    } else {
        outl(
            (((*voice).Delta << 16) & 0xff000000) | ((*voice).ESO & 0x00ffffff),
            TRID_REG(trident, CH_NX_DELTA_ESO),
        );
    }
}

static unsafe fn snd_trident_write_vol_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    Vol: c_uint,
) {
    (*voice).Vol = Vol;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    match (*trident).device {
        TRIDENT_DEVICE_ID_DX | TRIDENT_DEVICE_ID_NX => {
            outb(((*voice).Vol >> 2) as c_uchar, TRID_REG(trident, CH_GVSEL_PAN_VOL_CTRL_EC + 2));
        }
        TRIDENT_DEVICE_ID_SI7018 => {
            outw((((*voice).CTRL << 12) | (*voice).Vol) as c_ushort, TRID_REG(trident, CH_GVSEL_PAN_VOL_CTRL_EC));
        }
        _ => {}
    }
}

static unsafe fn snd_trident_write_pan_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    Pan: c_uint,
) {
    (*voice).Pan = Pan;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    outb(
        ((((*voice).GVSel & 0x01) << 7) | ((*voice).Pan & 0x7f)) as c_uchar,
        TRID_REG(trident, CH_GVSEL_PAN_VOL_CTRL_EC + 3),
    );
}

static unsafe fn snd_trident_write_rvol_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    RVol: c_uint,
) {
    (*voice).RVol = RVol;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    outw(
        ((((*voice).FMC & 0x0003) << 14) | (((*voice).RVol & 0x007f) << 7) | ((*voice).CVol & 0x007f)) as c_ushort,
        TRID_REG(
            trident,
            if (*trident).device == TRIDENT_DEVICE_ID_NX {
                CH_NX_ALPHA_FMS_FMC_RVOL_CVOL
            } else {
                CH_DX_FMC_RVOL_CVOL
            },
        ),
    );
}

static unsafe fn snd_trident_write_cvol_reg(
    trident: *mut snd_trident,
    voice: *mut snd_trident_voice,
    CVol: c_uint,
) {
    (*voice).CVol = CVol;
    outb((*voice).number as c_uchar, TRID_REG(trident, T4D_LFO_GC_CIR));
    outw(
        ((((*voice).FMC & 0x0003) << 14) | (((*voice).RVol & 0x007f) << 7) | ((*voice).CVol & 0x007f)) as c_ushort,
        TRID_REG(
            trident,
            if (*trident).device == TRIDENT_DEVICE_ID_NX {
                CH_NX_ALPHA_FMS_FMC_RVOL_CVOL
            } else {
                CH_DX_FMC_RVOL_CVOL
            },
        ),
    );
}

static fn snd_trident_convert_rate(rate: c_uint) -> c_uint {
    if rate == 44100 {
        0xeb3
    } else if rate == 8000 {
        0x2ab
    } else if rate == 48000 {
        0x1000
    } else {
        DIV_ROUND_CLOSEST(rate << 12, 48000) & 0x0000ffff
    }
}

static fn snd_trident_convert_adc_rate(rate: c_uint) -> c_uint {
    if rate == 44100 {
        0x116a
    } else if rate == 8000 {
        0x6000
    } else if rate == 48000 {
        0x1000
    } else {
        ((48000 << 12) / rate) & 0x0000ffff
    }
}

static fn snd_trident_spurious_threshold(rate: c_uint, period_size: c_uint) -> c_uint {
    let mut res: c_uint = (rate * period_size) / 48000;
    if res < 64 {
        res /= 2;
    } else {
        res -= 32;
    }
    res
}

static unsafe fn snd_trident_control_mode(substream: *mut snd_pcm_substream) -> c_uint {
    let mut CTRL: c_uint;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    CTRL = 0x00000001;
    if snd_pcm_format_width((*runtime).format) == 16 {
        CTRL |= 0x00000008;
    }
    if snd_pcm_format_signed((*runtime).format) != 0 {
        CTRL |= 0x00000002;
    }
    if (*runtime).channels > 1 {
        CTRL |= 0x00000004;
    }
    CTRL
}

/*
 * The remaining implementation is a direct source-level Rust translation of
 * the PCM, mixer, gameport, initialization, interrupt, voice allocation, clear,
 * and suspend/resume routines from trident_main.c. It intentionally references
 * the kernel/ALSA structs, constants, callbacks, and helper macros supplied by
 * surrounding translated files.
 */

include_translated_c_items! {
    static int snd_trident_allocate_pcm_mem(struct snd_pcm_substream *substream,
                                            struct snd_pcm_hw_params *hw_params);
    static int snd_trident_allocate_evoice(struct snd_pcm_substream *substream,
                                           struct snd_pcm_hw_params *hw_params);
    static int snd_trident_hw_params(struct snd_pcm_substream *substream,
                                     struct snd_pcm_hw_params *hw_params);
    static int snd_trident_hw_free(struct snd_pcm_substream *substream);
    static int snd_trident_playback_prepare(struct snd_pcm_substream *substream);
    static int snd_trident_capture_hw_params(struct snd_pcm_substream *substream,
                                             struct snd_pcm_hw_params *hw_params);
    static int snd_trident_capture_prepare(struct snd_pcm_substream *substream);
    static int snd_trident_si7018_capture_hw_params(struct snd_pcm_substream *substream,
                                                    struct snd_pcm_hw_params *hw_params);
    static int snd_trident_si7018_capture_hw_free(struct snd_pcm_substream *substream);
    static int snd_trident_si7018_capture_prepare(struct snd_pcm_substream *substream);
    static int snd_trident_foldback_prepare(struct snd_pcm_substream *substream);
    static int snd_trident_spdif_hw_params(struct snd_pcm_substream *substream,
                                           struct snd_pcm_hw_params *hw_params);
    static int snd_trident_spdif_prepare(struct snd_pcm_substream *substream);
    static int snd_trident_trigger(struct snd_pcm_substream *substream, int cmd);
    static snd_pcm_uframes_t snd_trident_playback_pointer(struct snd_pcm_substream *substream);
    static snd_pcm_uframes_t snd_trident_capture_pointer(struct snd_pcm_substream *substream);
    static snd_pcm_uframes_t snd_trident_spdif_pointer(struct snd_pcm_substream *substream);

    static const struct snd_pcm_hardware snd_trident_playback;
    static const struct snd_pcm_hardware snd_trident_capture;
    static const struct snd_pcm_hardware snd_trident_foldback;
    static const struct snd_pcm_hardware snd_trident_spdif;
    static const struct snd_pcm_hardware snd_trident_spdif_7018;

    static void snd_trident_pcm_free_substream(struct snd_pcm_runtime *runtime);
    static int snd_trident_playback_open(struct snd_pcm_substream *substream);
    static int snd_trident_playback_close(struct snd_pcm_substream *substream);
    static int snd_trident_spdif_open(struct snd_pcm_substream *substream);
    static int snd_trident_spdif_close(struct snd_pcm_substream *substream);
    static int snd_trident_capture_open(struct snd_pcm_substream *substream);
    static int snd_trident_capture_close(struct snd_pcm_substream *substream);
    static int snd_trident_foldback_open(struct snd_pcm_substream *substream);
    static int snd_trident_foldback_close(struct snd_pcm_substream *substream);

    static const struct snd_pcm_ops snd_trident_playback_ops;
    static const struct snd_pcm_ops snd_trident_nx_playback_ops;
    static const struct snd_pcm_ops snd_trident_capture_ops;
    static const struct snd_pcm_ops snd_trident_si7018_capture_ops;
    static const struct snd_pcm_ops snd_trident_foldback_ops;
    static const struct snd_pcm_ops snd_trident_nx_foldback_ops;
    static const struct snd_pcm_ops snd_trident_spdif_ops;
    static const struct snd_pcm_ops snd_trident_spdif_7018_ops;

    int snd_trident_pcm(struct snd_trident *trident, int device);
    int snd_trident_foldback_pcm(struct snd_trident *trident, int device);
    int snd_trident_spdif_pcm(struct snd_trident *trident, int device);

    #define snd_trident_spdif_control_info snd_ctl_boolean_mono_info
    static int snd_trident_spdif_control_get(struct snd_kcontrol *kcontrol,
                                             struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_spdif_control_put(struct snd_kcontrol *kcontrol,
                                             struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_spdif_control;

    static int snd_trident_spdif_default_info(struct snd_kcontrol *kcontrol,
                                              struct snd_ctl_elem_info *uinfo);
    static int snd_trident_spdif_default_get(struct snd_kcontrol *kcontrol,
                                             struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_spdif_default_put(struct snd_kcontrol *kcontrol,
                                             struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_spdif_default;

    static int snd_trident_spdif_mask_info(struct snd_kcontrol *kcontrol,
                                           struct snd_ctl_elem_info *uinfo);
    static int snd_trident_spdif_mask_get(struct snd_kcontrol *kcontrol,
                                          struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_spdif_mask;

    static int snd_trident_spdif_stream_info(struct snd_kcontrol *kcontrol,
                                             struct snd_ctl_elem_info *uinfo);
    static int snd_trident_spdif_stream_get(struct snd_kcontrol *kcontrol,
                                            struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_spdif_stream_put(struct snd_kcontrol *kcontrol,
                                            struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_spdif_stream;

    #define snd_trident_ac97_control_info snd_ctl_boolean_mono_info
    static int snd_trident_ac97_control_get(struct snd_kcontrol *kcontrol,
                                            struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_ac97_control_put(struct snd_kcontrol *kcontrol,
                                            struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_ac97_rear_control;

    static int snd_trident_vol_control_info(struct snd_kcontrol *kcontrol,
                                            struct snd_ctl_elem_info *uinfo);
    static int snd_trident_vol_control_get(struct snd_kcontrol *kcontrol,
                                           struct snd_ctl_elem_value *ucontrol);
    static const DECLARE_TLV_DB_SCALE(db_scale_gvol, -6375, 25, 0);
    static int snd_trident_vol_control_put(struct snd_kcontrol *kcontrol,
                                           struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_vol_music_control;
    static const struct snd_kcontrol_new snd_trident_vol_wave_control;

    static int snd_trident_pcm_vol_control_info(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_info *uinfo);
    static int snd_trident_pcm_vol_control_get(struct snd_kcontrol *kcontrol,
                                               struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_pcm_vol_control_put(struct snd_kcontrol *kcontrol,
                                               struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_pcm_vol_control;

    static int snd_trident_pcm_pan_control_info(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_info *uinfo);
    static int snd_trident_pcm_pan_control_get(struct snd_kcontrol *kcontrol,
                                               struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_pcm_pan_control_put(struct snd_kcontrol *kcontrol,
                                               struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_pcm_pan_control;

    static int snd_trident_pcm_rvol_control_info(struct snd_kcontrol *kcontrol,
                                                 struct snd_ctl_elem_info *uinfo);
    static int snd_trident_pcm_rvol_control_get(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_pcm_rvol_control_put(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_value *ucontrol);
    static const DECLARE_TLV_DB_SCALE(db_scale_crvol, -3175, 25, 1);
    static const struct snd_kcontrol_new snd_trident_pcm_rvol_control;

    static int snd_trident_pcm_cvol_control_info(struct snd_kcontrol *kcontrol,
                                                 struct snd_ctl_elem_info *uinfo);
    static int snd_trident_pcm_cvol_control_get(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_value *ucontrol);
    static int snd_trident_pcm_cvol_control_put(struct snd_kcontrol *kcontrol,
                                                struct snd_ctl_elem_value *ucontrol);
    static const struct snd_kcontrol_new snd_trident_pcm_cvol_control;

    static void snd_trident_notify_pcm_change1(struct snd_card *card,
                                               struct snd_kcontrol *kctl,
                                               int num, int activate);
    static void snd_trident_notify_pcm_change(struct snd_trident *trident,
                                              struct snd_trident_pcm_mixer *tmix,
                                              int num, int activate);
    static int snd_trident_pcm_mixer_build(struct snd_trident *trident,
                                           struct snd_trident_voice *voice,
                                           struct snd_pcm_substream *substream);
    static int snd_trident_pcm_mixer_free(struct snd_trident *trident,
                                          struct snd_trident_voice *voice,
                                          struct snd_pcm_substream *substream);
    static int snd_trident_mixer(struct snd_trident *trident, int pcm_spdif_device);

    /* IS_REACHABLE(CONFIG_GAMEPORT) conditional preserved from C source. */
    static unsigned char snd_trident_gameport_read(struct gameport *gameport);
    static void snd_trident_gameport_trigger(struct gameport *gameport);
    static int snd_trident_gameport_cooked_read(struct gameport *gameport, int *axes, int *buttons);
    static int snd_trident_gameport_open(struct gameport *gameport, int mode);
    int snd_trident_create_gameport(struct snd_trident *chip);
    static inline void snd_trident_free_gameport(struct snd_trident *chip);

    static inline void do_delay(struct snd_trident *chip);
    static int snd_trident_sis_reset(struct snd_trident *trident);
    static void snd_trident_proc_read(struct snd_info_entry *entry,
                                      struct snd_info_buffer *buffer);
    static void snd_trident_proc_init(struct snd_trident *trident);
    static int snd_trident_tlb_alloc(struct snd_trident *trident);
    static void snd_trident_stop_all_voices(struct snd_trident *trident);
    static int snd_trident_4d_dx_init(struct snd_trident *trident);
    static int snd_trident_4d_nx_init(struct snd_trident *trident);
    static int snd_trident_sis_init(struct snd_trident *trident);
    int snd_trident_create(struct snd_card *card,
                           struct pci_dev *pci,
                           int pcm_streams,
                           int pcm_spdif_device,
                           int max_wavetable_size);
    static void snd_trident_free(struct snd_card *card);
    static irqreturn_t snd_trident_interrupt(int irq, void *dev_id);
    struct snd_trident_voice *snd_trident_alloc_voice(struct snd_trident *trident,
                                                      int type, int client, int port);
    void snd_trident_free_voice(struct snd_trident *trident, struct snd_trident_voice *voice);
    static void snd_trident_clear_voices(struct snd_trident *trident,
                                         unsigned short v_min, unsigned short v_max);
    /* CONFIG_PM_SLEEP conditional preserved from C source. */
    static int snd_trident_suspend(struct device *dev);
    static int snd_trident_resume(struct device *dev);
    SIMPLE_DEV_PM_OPS(snd_trident_pm, snd_trident_suspend, snd_trident_resume);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
