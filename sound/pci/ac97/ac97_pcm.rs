// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com) and to datasheets
 *  for specific codecs.
 */

/*
 * C dependencies:
 * <linux/delay.h>, <linux/init.h>, <linux/slab.h>, <linux/mutex.h>,
 * <linux/export.h>, <sound/core.h>, <sound/pcm.h>, <sound/control.h>,
 * <sound/ac97_codec.h>, <sound/asoundef.h>, "ac97_id.h", "ac97_local.h"
 */

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/*
 *  PCM support
 */

const rate_reg_tables: [[[u8; 9]; 4]; 2] = [
    [
        /* standard rates */
        [
            /* 3&4 front, 7&8 rear, 6&9 center/lfe */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 3 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 4 */
            0xff,                          /* slot 5 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 6 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 7 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 8 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 9 */
            0xff,                          /* slot 10 */
            0xff,                          /* slot 11 */
        ],
        [
            /* 7&8 front, 6&9 rear, 10&11 center/lfe */
            0xff,                          /* slot 3 */
            0xff,                          /* slot 4 */
            0xff,                          /* slot 5 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 6 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 7 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 8 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 9 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 10 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 11 */
        ],
        [
            /* 6&9 front, 10&11 rear, 3&4 center/lfe */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 3 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 4 */
            0xff,                          /* slot 5 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 6 */
            0xff,                          /* slot 7 */
            0xff,                          /* slot 8 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 9 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 10 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 11 */
        ],
        [
            /* 10&11 front, 3&4 rear, 7&8 center/lfe */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 3 */
            AC97_PCM_SURR_DAC_RATE as u8,  /* slot 4 */
            0xff,                          /* slot 5 */
            0xff,                          /* slot 6 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 7 */
            AC97_PCM_LFE_DAC_RATE as u8,   /* slot 8 */
            0xff,                          /* slot 9 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 10 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 11 */
        ],
    ],
    [
        /* double rates */
        [
            /* 3&4 front, 7&8 front (t+1) */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 3 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 4 */
            0xff,                          /* slot 5 */
            0xff,                          /* slot 6 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 7 */
            AC97_PCM_FRONT_DAC_RATE as u8, /* slot 8 */
            0xff,                          /* slot 9 */
            0xff,                          /* slot 10 */
            0xff,                          /* slot 11 */
        ],
        [
            /* not specified in the specification */
            0xff, /* slot 3 */
            0xff, /* slot 4 */
            0xff, /* slot 5 */
            0xff, /* slot 6 */
            0xff, /* slot 7 */
            0xff, /* slot 8 */
            0xff, /* slot 9 */
            0xff, /* slot 10 */
            0xff, /* slot 11 */
        ],
        [
            0xff, /* slot 3 */
            0xff, /* slot 4 */
            0xff, /* slot 5 */
            0xff, /* slot 6 */
            0xff, /* slot 7 */
            0xff, /* slot 8 */
            0xff, /* slot 9 */
            0xff, /* slot 10 */
            0xff, /* slot 11 */
        ],
        [
            0xff, /* slot 3 */
            0xff, /* slot 4 */
            0xff, /* slot 5 */
            0xff, /* slot 6 */
            0xff, /* slot 7 */
            0xff, /* slot 8 */
            0xff, /* slot 9 */
            0xff, /* slot 10 */
            0xff, /* slot 11 */
        ],
    ],
];

/* FIXME: more various mappings for ADC? */
const rate_cregs: [u8; 9] = [
    AC97_PCM_LR_ADC_RATE as u8,  /* 3 */
    AC97_PCM_LR_ADC_RATE as u8,  /* 4 */
    0xff,                        /* 5 */
    AC97_PCM_MIC_ADC_RATE as u8, /* 6 */
    0xff,                        /* 7 */
    0xff,                        /* 8 */
    0xff,                        /* 9 */
    0xff,                        /* 10 */
    0xff,                        /* 11 */
];

unsafe fn get_slot_reg(pcm: *mut ac97_pcm, cidx: u16, slot: u16, dbl: c_int) -> u8 {
    if slot < 3 {
        return 0xff;
    }
    if slot > 11 {
        return 0xff;
    }
    if (*pcm).spdif != 0 {
        return AC97_SPDIF as u8; /* pseudo register */
    }
    if (*pcm).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return rate_reg_tables[dbl as usize][(*pcm).r[dbl as usize].rate_table[cidx as usize] as usize]
            [(slot - 3) as usize];
    } else {
        return rate_cregs[(slot - 3) as usize];
    }
}

unsafe fn set_spdif_rate(ac97: *mut snd_ac97, rate: u16) -> c_int {
    let old: u16;
    let mut bits: u16;
    let reg: u16;
    let mask: u16;
    let mut sbits: c_uint;

    if ((*ac97).ext_id & AC97_EI_SPDIF as u32) == 0 {
        return -ENODEV;
    }

    /* TODO: double rate support */
    if ((*ac97).flags & AC97_CS_SPDIF as u32) != 0 {
        match rate as c_int {
            48000 => bits = 0,
            44100 => bits = (1 << AC97_SC_SPSR_SHIFT) as u16,
            _ => {
                /* invalid - disable output */
                snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
                return -EINVAL;
            }
        }
        reg = AC97_CSR_SPDIF;
        mask = (1 << AC97_SC_SPSR_SHIFT) as u16;
    } else {
        if (*ac97).id == AC97_ID_CM9739 && rate as c_int != 48000 {
            snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
            return -EINVAL;
        }
        match rate as c_int {
            44100 => bits = AC97_SC_SPSR_44K,
            48000 => bits = AC97_SC_SPSR_48K,
            32000 => bits = AC97_SC_SPSR_32K,
            _ => {
                /* invalid - disable output */
                snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
                return -EINVAL;
            }
        }
        reg = AC97_SPDIF;
        mask = AC97_SC_SPSR_MASK;
    }

    mutex_lock(&mut (*ac97).reg_mutex);
    old = snd_ac97_read(ac97, reg) & mask;
    if old != bits {
        snd_ac97_update_bits_nolock(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
        snd_ac97_update_bits_nolock(ac97, reg, mask, bits);
        /* update the internal spdif bits */
        sbits = (*ac97).spdif_status;
        if (sbits & IEC958_AES0_PROFESSIONAL) != 0 {
            sbits &= !IEC958_AES0_PRO_FS;
            match rate as c_int {
                44100 => sbits |= IEC958_AES0_PRO_FS_44100,
                48000 => sbits |= IEC958_AES0_PRO_FS_48000,
                32000 => sbits |= IEC958_AES0_PRO_FS_32000,
                _ => {}
            }
        } else {
            sbits &= !(IEC958_AES3_CON_FS << 24);
            match rate as c_int {
                44100 => sbits |= IEC958_AES3_CON_FS_44100 << 24,
                48000 => sbits |= IEC958_AES3_CON_FS_48000 << 24,
                32000 => sbits |= IEC958_AES3_CON_FS_32000 << 24,
                _ => {}
            }
        }
        (*ac97).spdif_status = sbits;
    }
    snd_ac97_update_bits_nolock(
        ac97,
        AC97_EXTENDED_STATUS,
        AC97_EA_SPDIF,
        AC97_EA_SPDIF,
    );
    mutex_unlock(&mut (*ac97).reg_mutex);
    return 0;
}

/**
 * snd_ac97_set_rate - change the rate of the given input/output.
 * @ac97: the ac97 instance
 * @reg: the register to change
 * @rate: the sample rate to set
 *
 * Changes the rate of the given input/output on the codec.
 * If the codec doesn't support VAR, the rate must be 48000 (except
 * for SPDIF).
 *
 * The valid registers are AC97_PCM_MIC_ADC_RATE,
 * AC97_PCM_FRONT_DAC_RATE, AC97_PCM_LR_ADC_RATE.
 * AC97_PCM_SURR_DAC_RATE and AC97_PCM_LFE_DAC_RATE are accepted
 * if the codec supports them.
 * AC97_SPDIF is accepted as a pseudo register to modify the SPDIF
 * status bits.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, mut rate: c_uint) -> c_int {
    let dbl: c_int;
    let tmp: c_uint;

    dbl = (rate > 48000) as c_int;
    if dbl != 0 {
        if ((*ac97).flags & AC97_DOUBLE_RATE as u32) == 0 {
            return -EINVAL;
        }
        if reg != AC97_PCM_FRONT_DAC_RATE as c_int {
            return -EINVAL;
        }
    }

    snd_ac97_update_power(ac97, reg, 1);
    match reg as u16 {
        AC97_PCM_MIC_ADC_RATE => {
            if ((*ac97).regs[AC97_EXTENDED_STATUS as usize] & AC97_EA_VRM) == 0 {
                /* MIC VRA */
                if rate != 48000 {
                    return -EINVAL;
                }
            }
        }
        AC97_PCM_FRONT_DAC_RATE | AC97_PCM_LR_ADC_RATE => {
            if ((*ac97).regs[AC97_EXTENDED_STATUS as usize] & AC97_EA_VRA) == 0 {
                /* VRA */
                if rate != 48000 && rate != 96000 {
                    return -EINVAL;
                }
            }
        }
        AC97_PCM_SURR_DAC_RATE => {
            if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) == 0 {
                return -EINVAL;
            }
        }
        AC97_PCM_LFE_DAC_RATE => {
            if ((*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC as u32) == 0 {
                return -EINVAL;
            }
        }
        AC97_SPDIF => {
            /* special case */
            return set_spdif_rate(ac97, rate as u16);
        }
        _ => return -EINVAL,
    }
    if dbl != 0 {
        rate /= 2;
    }
    tmp = (rate * (*(*ac97).bus).clock) / 48000;
    if tmp > 65535 {
        return -EINVAL;
    }
    if ((*ac97).ext_id & AC97_EI_DRA as u32) != 0 && reg == AC97_PCM_FRONT_DAC_RATE as c_int {
        snd_ac97_update_bits(
            ac97,
            AC97_EXTENDED_STATUS,
            AC97_EA_DRA,
            if dbl != 0 { AC97_EA_DRA } else { 0 },
        );
    }
    snd_ac97_update(ac97, reg, (tmp & 0xffff) as u16);
    snd_ac97_read(ac97, reg as u16);
    if ((*ac97).ext_id & AC97_EI_DRA as u32) != 0 && reg == AC97_PCM_FRONT_DAC_RATE as c_int {
        /* Intel controllers require double rate data to be put in
         * slots 7+8
         */
        snd_ac97_update_bits(
            ac97,
            AC97_GENERAL_PURPOSE,
            AC97_GP_DRSS_MASK,
            if dbl != 0 { AC97_GP_DRSS_78 } else { 0 },
        );
        snd_ac97_read(ac97, AC97_GENERAL_PURPOSE);
    }
    return 0;
}

/* EXPORT_SYMBOL(snd_ac97_set_rate); */

unsafe fn get_pslots(ac97: *mut snd_ac97, rate_table: *mut u8, spdif_slots: *mut u16) -> u16 {
    if !ac97_is_audio(ac97) {
        return 0;
    }
    if ac97_is_rev22(ac97) || ac97_can_amap(ac97) {
        let mut slots: u16 = 0;
        if ac97_is_rev22(ac97) {
            /* Note: it's simply emulation of AMAP behaviour */
            let mut es: u16;
            (*ac97).regs[AC97_EXTENDED_ID as usize] &= !AC97_EI_DACS_SLOT_MASK;
            es = (*ac97).regs[AC97_EXTENDED_ID as usize];
            match (*ac97).addr as c_int {
                1 | 2 => es |= (1 << AC97_EI_DACS_SLOT_SHIFT) as u16,
                3 => es |= (2 << AC97_EI_DACS_SLOT_SHIFT) as u16,
                _ => {}
            }
            snd_ac97_write_cache(ac97, AC97_EXTENDED_ID, es);
        }
        match (*ac97).addr as c_int {
            0 => {
                slots |= (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT);
                if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) != 0 {
                    slots |= (1 << AC97_SLOT_PCM_SLEFT) | (1 << AC97_SLOT_PCM_SRIGHT);
                }
                if ((*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC as u32) != 0 {
                    slots |= (1 << AC97_SLOT_PCM_CENTER) | (1 << AC97_SLOT_LFE);
                }
                if ((*ac97).ext_id & AC97_EI_SPDIF as u32) != 0 {
                    if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) == 0 {
                        *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT) | (1 << AC97_SLOT_SPDIF_RIGHT);
                    } else if ((*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC as u32) == 0 {
                        *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT1) | (1 << AC97_SLOT_SPDIF_RIGHT1);
                    } else {
                        *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT2) | (1 << AC97_SLOT_SPDIF_RIGHT2);
                    }
                }
                *rate_table = 0;
            }
            1 | 2 => {
                slots |= (1 << AC97_SLOT_PCM_SLEFT) | (1 << AC97_SLOT_PCM_SRIGHT);
                if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) != 0 {
                    slots |= (1 << AC97_SLOT_PCM_CENTER) | (1 << AC97_SLOT_LFE);
                }
                if ((*ac97).ext_id & AC97_EI_SPDIF as u32) != 0 {
                    if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) == 0 {
                        *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT1) | (1 << AC97_SLOT_SPDIF_RIGHT1);
                    } else {
                        *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT2) | (1 << AC97_SLOT_SPDIF_RIGHT2);
                    }
                }
                *rate_table = 1;
            }
            3 => {
                slots |= (1 << AC97_SLOT_PCM_CENTER) | (1 << AC97_SLOT_LFE);
                if ((*ac97).ext_id & AC97_EI_SPDIF as u32) != 0 {
                    *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT2) | (1 << AC97_SLOT_SPDIF_RIGHT2);
                }
                *rate_table = 2;
            }
            _ => {}
        }
        return slots;
    } else {
        let mut slots: u16;
        slots = (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT);
        if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) != 0 {
            slots |= (1 << AC97_SLOT_PCM_SLEFT) | (1 << AC97_SLOT_PCM_SRIGHT);
        }
        if ((*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC as u32) != 0 {
            slots |= (1 << AC97_SLOT_PCM_CENTER) | (1 << AC97_SLOT_LFE);
        }
        if ((*ac97).ext_id & AC97_EI_SPDIF as u32) != 0 {
            if ((*ac97).scaps & AC97_SCAP_SURROUND_DAC as u32) == 0 {
                *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT) | (1 << AC97_SLOT_SPDIF_RIGHT);
            } else if ((*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC as u32) == 0 {
                *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT1) | (1 << AC97_SLOT_SPDIF_RIGHT1);
            } else {
                *spdif_slots = (1 << AC97_SLOT_SPDIF_LEFT2) | (1 << AC97_SLOT_SPDIF_RIGHT2);
            }
        }
        *rate_table = 0;
        return slots;
    }
}

unsafe fn get_cslots(ac97: *mut snd_ac97) -> u16 {
    let mut slots: u16;

    if !ac97_is_audio(ac97) {
        return 0;
    }
    slots = (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT);
    slots |= 1 << AC97_SLOT_MIC;
    return slots;
}

unsafe fn get_rates(pcm: *mut ac97_pcm, cidx: c_uint, slots: u16, dbl: c_int) -> c_uint {
    let mut i: c_int;
    let idx: c_int;
    let mut rates: c_uint = !0;
    let reg: u8;

    i = 3;
    while i < 12 {
        if (slots & (1 << i)) == 0 {
            i += 1;
            continue;
        }
        reg = get_slot_reg(pcm, cidx as u16, i as u16, dbl);
        idx = match reg as u16 {
            AC97_PCM_FRONT_DAC_RATE => AC97_RATES_FRONT_DAC,
            AC97_PCM_SURR_DAC_RATE => AC97_RATES_SURR_DAC,
            AC97_PCM_LFE_DAC_RATE => AC97_RATES_LFE_DAC,
            AC97_PCM_LR_ADC_RATE => AC97_RATES_ADC,
            AC97_PCM_MIC_ADC_RATE => AC97_RATES_MIC_ADC,
            _ => AC97_RATES_SPDIF,
        };
        rates &= (*(*pcm).r[dbl as usize].codec[cidx as usize]).rates[idx as usize];
        i += 1;
    }
    if dbl == 0 {
        rates &= !(SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000);
    }
    return rates;
}

/**
 * snd_ac97_pcm_assign - assign AC97 slots to given PCM streams
 * @bus: the ac97 bus instance
 * @pcms_count: count of PCMs to be assigned
 * @pcms: PCMs to be assigned
 *
 * It assigns available AC97 slots for given PCMs. If none or only
 * some slots are available, pcm->xxx.slots and pcm->xxx.rslots[] members
 * are reduced and might be zero.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_pcm_assign(
    bus: *mut snd_ac97_bus,
    pcms_count: u16,
    pcms: *const ac97_pcm,
) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;
    let mut pcm: *const ac97_pcm;
    let rpcms: *mut ac97_pcm;
    let mut rpcm: *mut ac97_pcm;
    let mut avail_slots: [[u16; 4]; 2] = [[0; 4]; 2];
    let mut rate_table: [[u8; 4]; 2] = [[0; 4]; 2];
    let mut tmp: u16;
    let mut slots: u16;
    let mut spdif_slots: [u16; 4] = [0; 4];
    let mut rates: c_uint;
    let mut codec: *mut snd_ac97;

    rpcms = kzalloc((size_of::<ac97_pcm>() * pcms_count as usize) as c_uint, GFP_KERNEL) as *mut ac97_pcm;
    if rpcms.is_null() {
        return -ENOMEM;
    }
    i = 0;
    while i < 4 {
        codec = (*bus).codec[i as usize];
        if codec.is_null() {
            i += 1;
            continue;
        }
        avail_slots[0][i as usize] =
            get_pslots(codec, &mut rate_table[0][i as usize], &mut spdif_slots[i as usize]);
        avail_slots[1][i as usize] = get_cslots(codec);
        if ((*codec).scaps & AC97_SCAP_INDEP_SDIN as u32) == 0 {
            j = 0;
            while j < i {
                if !(*bus).codec[j as usize].is_null() {
                    avail_slots[1][i as usize] &= !avail_slots[1][j as usize];
                }
                j += 1;
            }
        }
        i += 1;
    }
    /* first step - exclusive devices */
    i = 0;
    while i < pcms_count as c_int {
        pcm = pcms.add(i as usize);
        rpcm = rpcms.add(i as usize);
        /* low-level driver thinks that it's more clever */
        if (*pcm).copy_flag != 0 {
            *rpcm = *pcm;
            i += 1;
            continue;
        }
        (*rpcm).stream = (*pcm).stream;
        (*rpcm).exclusive = (*pcm).exclusive;
        (*rpcm).spdif = (*pcm).spdif;
        (*rpcm).private_value = (*pcm).private_value;
        (*rpcm).bus = bus;
        (*rpcm).rates = !0;
        slots = (*pcm).r[0].slots;
        j = 0;
        while j < 4 && slots != 0 {
            if (*bus).codec[j as usize].is_null() {
                j += 1;
                continue;
            }
            rates = !0;
            if (*pcm).spdif != 0 && (*pcm).stream == 0 {
                tmp = spdif_slots[j as usize];
            } else {
                tmp = avail_slots[(*pcm).stream as usize][j as usize];
            }
            if (*pcm).exclusive != 0 {
                /* exclusive access */
                tmp &= slots;
                k = 0;
                while k < i {
                    if (*rpcm).stream == (*rpcms.add(k as usize)).stream {
                        tmp &= !(*rpcms.add(k as usize)).r[0].rslots[j as usize];
                    }
                    k += 1;
                }
            } else {
                /* non-exclusive access */
                tmp &= (*pcm).r[0].slots;
            }
            if tmp != 0 {
                (*rpcm).r[0].rslots[j as usize] = tmp;
                (*rpcm).r[0].codec[j as usize] = (*bus).codec[j as usize];
                (*rpcm).r[0].rate_table[j as usize] = rate_table[(*pcm).stream as usize][j as usize];
                if (*bus).no_vra != 0 {
                    rates = SNDRV_PCM_RATE_48000;
                } else {
                    rates = get_rates(rpcm, j as c_uint, tmp, 0);
                }
                if (*pcm).exclusive != 0 {
                    avail_slots[(*pcm).stream as usize][j as usize] &= !tmp;
                }
            }
            slots &= !tmp;
            (*rpcm).r[0].slots |= tmp;
            (*rpcm).rates &= rates;
            j += 1;
        }
        /* for double rate, we check the first codec only */
        if (*pcm).stream == SNDRV_PCM_STREAM_PLAYBACK
            && !(*bus).codec[0].is_null()
            && ((*(*bus).codec[0]).flags & AC97_DOUBLE_RATE as u32) != 0
            && rate_table[(*pcm).stream as usize][0] == 0
        {
            tmp = (1 << AC97_SLOT_PCM_LEFT)
                | (1 << AC97_SLOT_PCM_RIGHT)
                | (1 << AC97_SLOT_PCM_LEFT_0)
                | (1 << AC97_SLOT_PCM_RIGHT_0);
            if (tmp & (*pcm).r[1].slots) == tmp {
                (*rpcm).r[1].slots = tmp;
                (*rpcm).r[1].rslots[0] = tmp;
                (*rpcm).r[1].rate_table[0] = 0;
                (*rpcm).r[1].codec[0] = (*bus).codec[0];
                if (*pcm).exclusive != 0 {
                    avail_slots[(*pcm).stream as usize][0] &= !tmp;
                }
                if (*bus).no_vra != 0 {
                    rates = SNDRV_PCM_RATE_96000;
                } else {
                    rates = get_rates(rpcm, 0, tmp, 1);
                }
                (*rpcm).rates |= rates;
            }
        }
        if (*rpcm).rates == !0 {
            (*rpcm).rates = 0; /* not used */
        }
        i += 1;
    }
    (*bus).pcms_count = pcms_count;
    (*bus).pcms = rpcms;
    return 0;
}

/* EXPORT_SYMBOL(snd_ac97_pcm_assign); */

/**
 * snd_ac97_pcm_open - opens the given AC97 pcm
 * @pcm: the ac97 pcm instance
 * @rate: rate in Hz, if codec does not support VRA, this value must be 48000Hz
 * @cfg: output stream characteristics
 * @slots: a subset of allocated slots (snd_ac97_pcm_assign) for this pcm
 *
 * It locks the specified slots and sets the given rate to AC97 registers.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_pcm_open(
    pcm: *mut ac97_pcm,
    rate: c_uint,
    cfg: ac97_pcm_cfg,
    slots: u16,
) -> c_int {
    let bus: *mut snd_ac97_bus;
    let mut i: c_int;
    let mut cidx: c_int;
    let r: c_int;
    let mut ok_flag: c_int;
    let mut reg_ok: [c_uint; 4] = [0, 0, 0, 0];
    let mut reg: u8;
    let mut err: c_int = 0;

    r = (rate > 48000) as c_int;
    bus = (*pcm).bus;
    if cfg == AC97_PCM_CFG_SPDIF {
        cidx = 0;
        while cidx < 4 {
            if !(*bus).codec[cidx as usize].is_null()
                && ((*(*bus).codec[cidx as usize]).ext_id & AC97_EI_SPDIF as u32) != 0
            {
                err = set_spdif_rate((*bus).codec[cidx as usize], rate as u16);
                if err < 0 {
                    return err;
                }
            }
            cidx += 1;
        }
    }
    spin_lock_irq(&mut (*bus).bus_lock);
    i = 3;
    while i < 12 {
        if (slots & (1 << i)) == 0 {
            i += 1;
            continue;
        }
        ok_flag = 0;
        cidx = 0;
        while cidx < 4 {
            if ((*bus).used_slots[(*pcm).stream as usize][cidx as usize] & (1 << i)) != 0 {
                err = -EBUSY;
                goto_error(pcm, bus, slots, &mut err);
                return err;
            }
            if ((*pcm).r[r as usize].rslots[cidx as usize] & (1 << i)) != 0 {
                (*bus).used_slots[(*pcm).stream as usize][cidx as usize] |= 1 << i;
                ok_flag += 1;
            }
            cidx += 1;
        }
        if ok_flag == 0 {
            dev_err(
                (*(*bus).card).dev,
                b"cannot find configuration for AC97 slot %i\n\0".as_ptr(),
                i,
            );
            err = -EAGAIN;
            goto_error(pcm, bus, slots, &mut err);
            return err;
        }
        i += 1;
    }
    (*pcm).cur_dbl = r;
    spin_unlock_irq(&mut (*bus).bus_lock);

    i = 3;
    while i < 12 {
        if (slots & (1 << i)) == 0 {
            i += 1;
            continue;
        }
        cidx = 0;
        while cidx < 4 {
            if ((*pcm).r[r as usize].rslots[cidx as usize] & (1 << i)) != 0 {
                reg = get_slot_reg(pcm, cidx as u16, i as u16, r);
                if reg == 0xff {
                    dev_err(
                        (*(*bus).card).dev,
                        b"invalid AC97 slot %i?\n\0".as_ptr(),
                        i,
                    );
                    cidx += 1;
                    continue;
                }
                if (reg_ok[cidx as usize] & (1 << (reg as u16 - AC97_PCM_FRONT_DAC_RATE))) != 0 {
                    cidx += 1;
                    continue;
                }
                dev_dbg(
                    (*(*bus).card).dev,
                    b"setting ac97 reg 0x%x to rate %d\n\0".as_ptr(),
                    reg as c_int,
                    rate as c_int,
                );
                err = snd_ac97_set_rate((*pcm).r[r as usize].codec[cidx as usize], reg as c_int, rate);
                if err < 0 {
                    dev_err(
                        (*(*bus).card).dev,
                        b"error in snd_ac97_set_rate: cidx=%d, reg=0x%x, rate=%d, err=%d\n\0".as_ptr(),
                        cidx,
                        reg as c_int,
                        rate as c_int,
                        err,
                    );
                } else {
                    reg_ok[cidx as usize] |= 1 << (reg as u16 - AC97_PCM_FRONT_DAC_RATE);
                }
            }
            cidx += 1;
        }
        i += 1;
    }
    (*pcm).aslots = slots;
    return 0;
}

unsafe fn goto_error(pcm: *mut ac97_pcm, bus: *mut snd_ac97_bus, slots: u16, err: &mut c_int) {
    spin_unlock_irq(&mut (*bus).bus_lock);
    (*pcm).aslots = slots;
    snd_ac97_pcm_close(pcm);
    let _ = err;
}

/* EXPORT_SYMBOL(snd_ac97_pcm_open); */

/**
 * snd_ac97_pcm_close - closes the given AC97 pcm
 * @pcm: the ac97 pcm instance
 *
 * It frees the locked AC97 slots.
 *
 * Return: Zero.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_pcm_close(pcm: *mut ac97_pcm) -> c_int {
    let bus: *mut snd_ac97_bus;
    let slots: u16 = (*pcm).aslots;
    let mut i: c_int;
    let mut cidx: c_int;

    /*
     * CONFIG_SND_AC97_POWER_SAVE:
     * int r = pcm->cur_dbl;
     * for each active slot, update codec power with get_slot_reg(...), reg, 0.
     */
    #[cfg(CONFIG_SND_AC97_POWER_SAVE)]
    {
        let r: c_int = (*pcm).cur_dbl;
        i = 3;
        while i < 12 {
            if (slots & (1 << i)) == 0 {
                i += 1;
                continue;
            }
            cidx = 0;
            while cidx < 4 {
                if ((*pcm).r[r as usize].rslots[cidx as usize] & (1 << i)) != 0 {
                    let reg: c_int = get_slot_reg(pcm, cidx as u16, i as u16, r) as c_int;
                    snd_ac97_update_power((*pcm).r[r as usize].codec[cidx as usize], reg, 0);
                }
                cidx += 1;
            }
            i += 1;
        }
    }

    bus = (*pcm).bus;
    spin_lock_irq(&mut (*bus).bus_lock);
    i = 3;
    while i < 12 {
        if (slots & (1 << i)) == 0 {
            i += 1;
            continue;
        }
        cidx = 0;
        while cidx < 4 {
            (*bus).used_slots[(*pcm).stream as usize][cidx as usize] &= !(1 << i);
            cidx += 1;
        }
        i += 1;
    }
    (*pcm).aslots = 0;
    (*pcm).cur_dbl = 0;
    spin_unlock_irq(&mut (*bus).bus_lock);
    return 0;
}

/* EXPORT_SYMBOL(snd_ac97_pcm_close); */

unsafe fn double_rate_hw_constraint_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let _ = rule;
    if (*channels).min > 2 {
        static single_rates: snd_interval = snd_interval {
            min: 1,
            max: 48000,
        };
        let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        return snd_interval_refine(rate, &single_rates);
    }
    return 0;
}

unsafe fn double_rate_hw_constraint_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let _ = rule;
    if (*rate).min > 48000 {
        static double_rate_channels: snd_interval = snd_interval {
            min: 2,
            max: 2,
        };
        let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
        return snd_interval_refine(channels, &double_rate_channels);
    }
    return 0;
}

/**
 * snd_ac97_pcm_double_rate_rules - set double rate constraints
 * @runtime: the runtime of the ac97 front playback pcm
 *
 * Installs the hardware constraint rules to prevent using double rates and
 * more than two channels at the same time.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_pcm_double_rate_rules(runtime: *mut snd_pcm_runtime) -> c_int {
    let mut err: c_int;

    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(double_rate_hw_constraint_rate),
        ptr::null_mut(),
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(double_rate_hw_constraint_channels),
        ptr::null_mut(),
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );
    return err;
}

/* EXPORT_SYMBOL(snd_ac97_pcm_double_rate_rules); */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_pcm {
    pub copy_flag: c_int,
    pub stream: c_int,
    pub exclusive: c_int,
    pub spdif: c_int,
    pub private_value: c_uint,
    pub bus: *mut snd_ac97_bus,
    pub rates: c_uint,
    pub r: [ac97_pcm_rec; 2],
    pub aslots: u16,
    pub cur_dbl: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_pcm_rec {
    pub slots: u16,
    pub rslots: [u16; 4],
    pub rate_table: [u8; 4],
    pub codec: [*mut snd_ac97; 4],
}

#[repr(C)]
pub struct snd_ac97 {
    pub ext_id: u32,
    pub flags: u32,
    pub id: u32,
    pub spdif_status: c_uint,
    pub reg_mutex: mutex,
    pub regs: [u16; 256],
    pub bus: *mut snd_ac97_bus,
    pub scaps: u32,
    pub addr: c_uint,
    pub rates: [c_uint; 6],
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub clock: c_uint,
    pub codec: [*mut snd_ac97; 4],
    pub no_vra: c_int,
    pub used_slots: [[u16; 4]; 2],
    pub pcms_count: u16,
    pub pcms: *mut ac97_pcm,
    pub bus_lock: spinlock_t,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

pub type ac97_pcm_cfg = c_uint;

extern "C" {
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: u16, mask: u16, value: u16) -> c_int;
    fn snd_ac97_update_bits_nolock(ac97: *mut snd_ac97, reg: u16, mask: u16, value: u16) -> c_int;
    fn snd_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16;
    fn snd_ac97_update_power(ac97: *mut snd_ac97, reg: c_int, powerup: c_int);
    fn snd_ac97_update(ac97: *mut snd_ac97, reg: c_int, value: u16) -> c_int;
    fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: u16, value: u16);
    fn ac97_is_audio(ac97: *mut snd_ac97) -> bool;
    fn ac97_is_rev22(ac97: *mut snd_ac97) -> bool;
    fn ac97_can_amap(ac97: *mut snd_ac97) -> bool;
    fn kzalloc(size: c_uint, flags: c_uint) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn dev_err(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const u8, ...);
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: Option<unsafe fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int>,
        private: *mut c_void,
        dep: c_int,
        terminator: c_int,
    ) -> c_int;
}

extern "C" {
    static AC97_PCM_FRONT_DAC_RATE: u16;
    static AC97_PCM_LFE_DAC_RATE: u16;
    static AC97_PCM_SURR_DAC_RATE: u16;
    static AC97_PCM_LR_ADC_RATE: u16;
    static AC97_PCM_MIC_ADC_RATE: u16;
    static AC97_SPDIF: u16;
    static AC97_EXTENDED_STATUS: u16;
    static AC97_EA_SPDIF: u16;
    static AC97_CS_SPDIF: c_uint;
    static AC97_SC_SPSR_SHIFT: c_int;
    static AC97_CSR_SPDIF: u16;
    static AC97_ID_CM9739: u32;
    static AC97_SC_SPSR_44K: u16;
    static AC97_SC_SPSR_48K: u16;
    static AC97_SC_SPSR_32K: u16;
    static AC97_SC_SPSR_MASK: u16;
    static IEC958_AES0_PROFESSIONAL: c_uint;
    static IEC958_AES0_PRO_FS: c_uint;
    static IEC958_AES0_PRO_FS_44100: c_uint;
    static IEC958_AES0_PRO_FS_48000: c_uint;
    static IEC958_AES0_PRO_FS_32000: c_uint;
    static IEC958_AES3_CON_FS: c_uint;
    static IEC958_AES3_CON_FS_44100: c_uint;
    static IEC958_AES3_CON_FS_48000: c_uint;
    static IEC958_AES3_CON_FS_32000: c_uint;
    static AC97_DOUBLE_RATE: c_uint;
    static AC97_EA_VRM: u16;
    static AC97_EA_VRA: u16;
    static AC97_SCAP_SURROUND_DAC: c_uint;
    static AC97_SCAP_CENTER_LFE_DAC: c_uint;
    static AC97_EI_DRA: c_uint;
    static AC97_EA_DRA: u16;
    static AC97_GENERAL_PURPOSE: u16;
    static AC97_GP_DRSS_MASK: u16;
    static AC97_GP_DRSS_78: u16;
    static AC97_EXTENDED_ID: u16;
    static AC97_EI_DACS_SLOT_MASK: u16;
    static AC97_EI_DACS_SLOT_SHIFT: c_int;
    static AC97_SLOT_PCM_LEFT: c_int;
    static AC97_SLOT_PCM_RIGHT: c_int;
    static AC97_SLOT_PCM_SLEFT: c_int;
    static AC97_SLOT_PCM_SRIGHT: c_int;
    static AC97_SLOT_PCM_CENTER: c_int;
    static AC97_SLOT_LFE: c_int;
    static AC97_SLOT_SPDIF_LEFT: c_int;
    static AC97_SLOT_SPDIF_RIGHT: c_int;
    static AC97_SLOT_SPDIF_LEFT1: c_int;
    static AC97_SLOT_SPDIF_RIGHT1: c_int;
    static AC97_SLOT_SPDIF_LEFT2: c_int;
    static AC97_SLOT_SPDIF_RIGHT2: c_int;
    static AC97_SLOT_MIC: c_int;
    static AC97_SCAP_INDEP_SDIN: c_uint;
    static AC97_SLOT_PCM_LEFT_0: c_int;
    static AC97_SLOT_PCM_RIGHT_0: c_int;
    static AC97_RATES_FRONT_DAC: c_int;
    static AC97_RATES_SURR_DAC: c_int;
    static AC97_RATES_LFE_DAC: c_int;
    static AC97_RATES_ADC: c_int;
    static AC97_RATES_MIC_ADC: c_int;
    static AC97_RATES_SPDIF: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_RATE_64000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static AC97_PCM_CFG_SPDIF: ac97_pcm_cfg;
    static AC97_EI_SPDIF: c_uint;
    static GFP_KERNEL: c_uint;
    static ENODEV: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EBUSY: c_int;
    static EAGAIN: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
