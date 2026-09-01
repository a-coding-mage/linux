// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cirrus Logic CS420x HD-audio codec
 *
 * Copyright (c) 2009 Takashi Iwai <tiwai@suse.de>
 */

use crate::*;

#[repr(C)]
pub struct cs_spec {
    pub gen: hda_gen_spec,

    pub gpio_mask: c_uint,
    pub gpio_dir: c_uint,
    pub gpio_data: c_uint,
    pub gpio_eapd_hp: c_uint,      /* EAPD GPIO bit for headphones */
    pub gpio_eapd_speaker: c_uint, /* EAPD GPIO bit for speakers */

    pub vendor_nid: hda_nid_t,

    /* for MBP SPDIF control */
    pub spdif_sw_put: Option<
        unsafe extern "C" fn(
            kcontrol: *mut snd_kcontrol,
            ucontrol: *mut snd_ctl_elem_value,
        ) -> c_int,
    >,
}

/* available models with CS420x */
pub const CS420X_MBP53: c_int = 0;
pub const CS420X_MBP55: c_int = 1;
pub const CS420X_IMAC27: c_int = 2;
pub const CS420X_GPIO_13: c_int = 3;
pub const CS420X_GPIO_23: c_int = 4;
pub const CS420X_MBP101: c_int = 5;
pub const CS420X_MBP81: c_int = 6;
pub const CS420X_MBA42: c_int = 7;
pub const CS420X_AUTO: c_int = 8;
/* aliases */
pub const CS420X_IMAC27_122: c_int = CS420X_GPIO_23;
pub const CS420X_APPLE: c_int = CS420X_GPIO_13;

/* Vendor-specific processing widget */
pub const CS420X_VENDOR_NID: c_uint = 0x11;
pub const CS_DIG_OUT1_PIN_NID: c_uint = 0x10;
pub const CS_DIG_OUT2_PIN_NID: c_uint = 0x15;
pub const CS_DMIC1_PIN_NID: c_uint = 0x0e;
pub const CS_DMIC2_PIN_NID: c_uint = 0x12;

/* coef indices */
pub const IDX_SPDIF_STAT: c_uint = 0x0000;
pub const IDX_SPDIF_CTL: c_uint = 0x0001;
pub const IDX_ADC_CFG: c_uint = 0x0002;
/* SZC bitmask, 4 modes below:
 * 0 = immediate,
 * 1 = digital immediate, analog zero-cross
 * 2 = digtail & analog soft-ramp
 * 3 = digital soft-ramp, analog zero-cross
 */
pub const CS_COEF_ADC_SZC_MASK: c_uint = 3 << 0;
pub const CS_COEF_ADC_MIC_SZC_MODE: c_uint = 3 << 0; /* SZC setup for mic */
pub const CS_COEF_ADC_LI_SZC_MODE: c_uint = 3 << 0; /* SZC setup for line-in */
/* PGA mode: 0 = differential, 1 = signle-ended */
pub const CS_COEF_ADC_MIC_PGA_MODE: c_uint = 1 << 5; /* PGA setup for mic */
pub const CS_COEF_ADC_LI_PGA_MODE: c_uint = 1 << 6; /* PGA setup for line-in */
pub const IDX_DAC_CFG: c_uint = 0x0003;
/* SZC bitmask, 4 modes below:
 * 0 = Immediate
 * 1 = zero-cross
 * 2 = soft-ramp
 * 3 = soft-ramp on zero-cross
 */
pub const CS_COEF_DAC_HP_SZC_MODE: c_uint = 3 << 0; /* nid 0x02 */
pub const CS_COEF_DAC_LO_SZC_MODE: c_uint = 3 << 2; /* nid 0x03 */
pub const CS_COEF_DAC_SPK_SZC_MODE: c_uint = 3 << 4; /* nid 0x04 */

pub const IDX_BEEP_CFG: c_uint = 0x0004;
/* 0x0008 - test reg key */
/* 0x0009 - 0x0014 -> 12 test regs */
/* 0x0015 - visibility reg */

/* Cirrus Logic CS4208 */
pub const CS4208_VENDOR_NID: c_uint = 0x24;

#[inline]
unsafe fn cs_vendor_coef_get(codec: *mut hda_codec, idx: c_uint) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;

    snd_hda_codec_write(
        codec,
        (*spec).vendor_nid,
        0,
        AC_VERB_SET_COEF_INDEX,
        idx,
    );
    snd_hda_codec_read(codec, (*spec).vendor_nid, 0, AC_VERB_GET_PROC_COEF, 0)
}

#[inline]
unsafe fn cs_vendor_coef_set(codec: *mut hda_codec, idx: c_uint, coef: c_uint) {
    let spec = (*codec).spec as *mut cs_spec;

    snd_hda_codec_write(
        codec,
        (*spec).vendor_nid,
        0,
        AC_VERB_SET_COEF_INDEX,
        idx,
    );
    snd_hda_codec_write(
        codec,
        (*spec).vendor_nid,
        0,
        AC_VERB_SET_PROC_COEF,
        coef,
    );
}

/*
 * auto-mute and auto-mic switching
 * CS421x auto-output redirecting
 * HP/SPK/SPDIF
 */

unsafe extern "C" fn cs_automute(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut cs_spec;

    snd_hda_gen_update_outputs(codec);

    if (*spec).gpio_eapd_hp != 0 || (*spec).gpio_eapd_speaker != 0 {
        if (*spec).gen.automute_speaker != 0 {
            (*spec).gpio_data = if (*spec).gen.hp_jack_present != 0 {
                (*spec).gpio_eapd_hp
            } else {
                (*spec).gpio_eapd_speaker
            };
        } else {
            (*spec).gpio_data = (*spec).gpio_eapd_hp | (*spec).gpio_eapd_speaker;
        }
        snd_hda_codec_write(codec, 0x01, 0, AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
    }
}

unsafe fn is_active_pin(codec: *mut hda_codec, nid: hda_nid_t) -> bool {
    let val: c_uint;

    val = snd_hda_codec_get_pincfg(codec, nid);
    get_defcfg_connect(val) != AC_JACK_PORT_NONE
}

unsafe fn init_input_coef(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut cs_spec;
    let mut coef: c_uint;

    /* CS420x has multiple ADC, CS421x has single ADC */
    if (*spec).vendor_nid == CS420X_VENDOR_NID {
        coef = cs_vendor_coef_get(codec, IDX_BEEP_CFG) as c_uint;
        if is_active_pin(codec, CS_DMIC2_PIN_NID as hda_nid_t) {
            coef |= 1 << 4; /* DMIC2 2 chan on, GPIO1 off */
        }
        if is_active_pin(codec, CS_DMIC1_PIN_NID as hda_nid_t) {
            coef |= 1 << 3; /* DMIC1 2 chan on, GPIO0 off
                             * No effect if SPDIF_OUT2 is
                             * selected in IDX_SPDIF_CTL.
                             */
        }

        cs_vendor_coef_set(codec, IDX_BEEP_CFG, coef);
    }
}

static cs_coef_init_verbs: &[hda_verb] = &[
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_STATE, param: 1 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: IDX_DAC_CFG },
    hda_verb {
        nid: 0x11,
        verb: AC_VERB_SET_PROC_COEF,
        param: 0x002a /* DAC1/2/3 SZCMode Soft Ramp */
            | 0x0040 /* Mute DACs on FIFO error */
            | 0x1000 /* Enable DACs High Pass Filter */
            | 0x0400, /* Disable Coefficient Auto increment */
    },
    /* ADC1/2 - Digital and Analog Soft Ramp */
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: IDX_ADC_CFG },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x000a },
    /* Beep */
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: IDX_BEEP_CFG },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x0007 }, /* Enable Beep thru DAC1/2/3 */

    hda_verb::zeroed(), /* terminator */
];

static cs4208_coef_init_verbs: &[hda_verb] = &[
    hda_verb { nid: 0x01, verb: AC_VERB_SET_POWER_STATE, param: 0x00 }, /* AFG: D0 */
    hda_verb { nid: 0x24, verb: AC_VERB_SET_PROC_STATE, param: 0x01 },  /* VPW: processing on */
    hda_verb { nid: 0x24, verb: AC_VERB_SET_COEF_INDEX, param: 0x0033 },
    hda_verb { nid: 0x24, verb: AC_VERB_SET_PROC_COEF, param: 0x0001 }, /* A1 ICS */
    hda_verb { nid: 0x24, verb: AC_VERB_SET_COEF_INDEX, param: 0x0034 },
    hda_verb { nid: 0x24, verb: AC_VERB_SET_PROC_COEF, param: 0x1C01 }, /* A1 Enable, A Thresh = 300mV */
    hda_verb::zeroed(), /* terminator */
];

/* Errata: CS4207 rev C0/C1/C2 Silicon
 *
 * http://www.cirrus.com/en/pubs/errata/ER880C3.pdf
 *
 * 6. At high temperature (TA > +85°C), the digital supply current (IVD)
 * may be excessive (up to an additional 200 μA), which is most easily
 * observed while the part is being held in reset (RESET# active low).
 *
 * Root Cause: At initial powerup of the device, the logic that drives
 * the clock and write enable to the S/PDIF SRC RAMs is not properly
 * initialized.
 * Certain random patterns will cause a steady leakage current in those
 * RAM cells. The issue will resolve once the SRCs are used (turned on).
 *
 * Workaround: The following verb sequence briefly turns on the S/PDIF SRC
 * blocks, which will alleviate the issue.
 */

static cs_errata_init_verbs: &[hda_verb] = &[
    hda_verb { nid: 0x01, verb: AC_VERB_SET_POWER_STATE, param: 0x00 }, /* AFG: D0 */
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_STATE, param: 0x01 },  /* VPW: processing on */

    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0008 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x9999 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0017 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0xa412 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0001 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x0009 },

    hda_verb { nid: 0x07, verb: AC_VERB_SET_POWER_STATE, param: 0x00 }, /* S/PDIF Rx: D0 */
    hda_verb { nid: 0x08, verb: AC_VERB_SET_POWER_STATE, param: 0x00 }, /* S/PDIF Tx: D0 */

    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0017 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x2412 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0008 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x0000 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: 0x0001 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x0008 },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_STATE, param: 0x00 },
    hda_verb::zeroed(), /* terminator */
];

/* SPDIF setup */
unsafe fn init_digital_coef(codec: *mut hda_codec) {
    let mut coef: c_uint;

    coef = 0x0002; /* SRC_MUTE soft-mute on SPDIF (if no lock) */
    coef |= 0x0008; /* Replace with mute on error */
    if is_active_pin(codec, CS_DIG_OUT2_PIN_NID as hda_nid_t) {
        coef |= 0x4000; /* RX to TX1 or TX2 Loopthru / SPDIF2
                         * SPDIF_OUT2 is shared with GPIO1 and
                         * DMIC_SDA2.
                         */
    }
    cs_vendor_coef_set(codec, IDX_SPDIF_CTL, coef);
}

unsafe extern "C" fn cs_init(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;

    if (*spec).vendor_nid == CS420X_VENDOR_NID {
        /* init_verb sequence for C0/C1/C2 errata*/
        snd_hda_sequence_write(codec, cs_errata_init_verbs.as_ptr());
        snd_hda_sequence_write(codec, cs_coef_init_verbs.as_ptr());
    } else if (*spec).vendor_nid == CS4208_VENDOR_NID {
        snd_hda_sequence_write(codec, cs4208_coef_init_verbs.as_ptr());
    }

    snd_hda_gen_init(codec);

    if (*spec).gpio_mask != 0 {
        snd_hda_codec_set_gpio(codec, (*spec).gpio_mask, (*spec).gpio_dir, (*spec).gpio_data, 0);
    }

    if (*spec).vendor_nid == CS420X_VENDOR_NID {
        init_input_coef(codec);
        init_digital_coef(codec);
    }

    0
}

unsafe extern "C" fn cs_build_controls(codec: *mut hda_codec) -> c_int {
    let err: c_int;

    err = snd_hda_gen_build_controls(codec);
    if err < 0 {
        return err;
    }
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_BUILD);
    0
}

unsafe fn cs_parse_auto_config(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;
    let mut err: c_int;
    let mut i: c_int;

    err = snd_hda_parse_pin_defcfg(codec, &mut (*spec).gen.autocfg, core::ptr::null(), 0);
    if err < 0 {
        return err;
    }

    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).gen.autocfg);
    if err < 0 {
        return err;
    }

    /* keep the ADCs powered up when it's dynamically switchable */
    if (*spec).gen.dyn_adc_switch != 0 {
        let mut done: c_uint = 0;

        i = 0;
        while i < (*spec).gen.input_mux.num_items {
            let idx = (*spec).gen.dyn_adc_idx[i as usize];

            if (done & (1 << idx)) != 0 {
                i += 1;
                continue;
            }
            snd_hda_gen_fix_pin_power(codec, (*spec).gen.adc_nids[idx as usize]);
            done |= 1 << idx;
            i += 1;
        }
    }

    0
}

static cs420x_models: &[hda_model_fixup] = &[
    hda_model_fixup { id: CS420X_MBP53, name: c_str!("mbp53") },
    hda_model_fixup { id: CS420X_MBP55, name: c_str!("mbp55") },
    hda_model_fixup { id: CS420X_IMAC27, name: c_str!("imac27") },
    hda_model_fixup { id: CS420X_IMAC27_122, name: c_str!("imac27_122") },
    hda_model_fixup { id: CS420X_APPLE, name: c_str!("apple") },
    hda_model_fixup { id: CS420X_MBP101, name: c_str!("mbp101") },
    hda_model_fixup { id: CS420X_MBP81, name: c_str!("mbp81") },
    hda_model_fixup { id: CS420X_MBA42, name: c_str!("mba42") },
    hda_model_fixup::zeroed(),
];

static cs420x_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK!(0x10de, 0x0ac0, "MacBookPro 5,3", CS420X_MBP53),
    SND_PCI_QUIRK!(0x10de, 0x0d94, "MacBookAir 3,1(2)", CS420X_MBP55),
    SND_PCI_QUIRK!(0x10de, 0xcb79, "MacBookPro 5,5", CS420X_MBP55),
    SND_PCI_QUIRK!(0x10de, 0xcb89, "MacBookPro 7,1", CS420X_MBP55),
    /* this conflicts with too many other models */
    /*SND_PCI_QUIRK!(0x8086, 0x7270, "IMac 27 Inch", CS420X_IMAC27),*/

    /* codec SSID */
    SND_PCI_QUIRK!(0x106b, 0x0600, "iMac 14,1", CS420X_IMAC27_122),
    SND_PCI_QUIRK!(0x106b, 0x0900, "iMac 12,1", CS420X_IMAC27_122),
    SND_PCI_QUIRK!(0x106b, 0x1c00, "MacBookPro 8,1", CS420X_MBP81),
    SND_PCI_QUIRK!(0x106b, 0x2000, "iMac 12,2", CS420X_IMAC27_122),
    SND_PCI_QUIRK!(0x106b, 0x2800, "MacBookPro 10,1", CS420X_MBP101),
    SND_PCI_QUIRK!(0x106b, 0x5600, "MacBookAir 5,2", CS420X_MBP81),
    SND_PCI_QUIRK!(0x106b, 0x5b00, "MacBookAir 4,2", CS420X_MBA42),
    SND_PCI_QUIRK_VENDOR!(0x106b, "Apple", CS420X_APPLE),
    hda_quirk::zeroed(), /* terminator */
];

static mbp53_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x09, val: 0x012b4050 },
    hda_pintbl { nid: 0x0a, val: 0x90100141 },
    hda_pintbl { nid: 0x0b, val: 0x90100140 },
    hda_pintbl { nid: 0x0c, val: 0x018b3020 },
    hda_pintbl { nid: 0x0d, val: 0x90a00110 },
    hda_pintbl { nid: 0x0e, val: 0x400000f0 },
    hda_pintbl { nid: 0x0f, val: 0x01cbe030 },
    hda_pintbl { nid: 0x10, val: 0x014be060 },
    hda_pintbl { nid: 0x12, val: 0x400000f0 },
    hda_pintbl { nid: 0x15, val: 0x400000f0 },
    hda_pintbl::zeroed(), /* terminator */
];

static mbp55_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x09, val: 0x012b4030 },
    hda_pintbl { nid: 0x0a, val: 0x90100121 },
    hda_pintbl { nid: 0x0b, val: 0x90100120 },
    hda_pintbl { nid: 0x0c, val: 0x400000f0 },
    hda_pintbl { nid: 0x0d, val: 0x90a00110 },
    hda_pintbl { nid: 0x0e, val: 0x400000f0 },
    hda_pintbl { nid: 0x0f, val: 0x400000f0 },
    hda_pintbl { nid: 0x10, val: 0x014be040 },
    hda_pintbl { nid: 0x12, val: 0x400000f0 },
    hda_pintbl { nid: 0x15, val: 0x400000f0 },
    hda_pintbl::zeroed(), /* terminator */
];

static imac27_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x09, val: 0x012b4050 },
    hda_pintbl { nid: 0x0a, val: 0x90100140 },
    hda_pintbl { nid: 0x0b, val: 0x90100142 },
    hda_pintbl { nid: 0x0c, val: 0x018b3020 },
    hda_pintbl { nid: 0x0d, val: 0x90a00110 },
    hda_pintbl { nid: 0x0e, val: 0x400000f0 },
    hda_pintbl { nid: 0x0f, val: 0x01cbe030 },
    hda_pintbl { nid: 0x10, val: 0x014be060 },
    hda_pintbl { nid: 0x12, val: 0x01ab9070 },
    hda_pintbl { nid: 0x15, val: 0x400000f0 },
    hda_pintbl::zeroed(), /* terminator */
];

static mbp101_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x0d, val: 0x40ab90f0 },
    hda_pintbl { nid: 0x0e, val: 0x90a600f0 },
    hda_pintbl { nid: 0x12, val: 0x50a600f0 },
    hda_pintbl::zeroed(), /* terminator */
];

static mba42_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x09, val: 0x012b4030 }, /* HP */
    hda_pintbl { nid: 0x0a, val: 0x400000f0 },
    hda_pintbl { nid: 0x0b, val: 0x90100120 }, /* speaker */
    hda_pintbl { nid: 0x0c, val: 0x400000f0 },
    hda_pintbl { nid: 0x0d, val: 0x90a00110 }, /* mic */
    hda_pintbl { nid: 0x0e, val: 0x400000f0 },
    hda_pintbl { nid: 0x0f, val: 0x400000f0 },
    hda_pintbl { nid: 0x10, val: 0x400000f0 },
    hda_pintbl { nid: 0x12, val: 0x400000f0 },
    hda_pintbl { nid: 0x15, val: 0x400000f0 },
    hda_pintbl::zeroed(), /* terminator */
];

static mba6_pincfgs: &[hda_pintbl] = &[
    hda_pintbl { nid: 0x10, val: 0x032120f0 }, /* HP */
    hda_pintbl { nid: 0x11, val: 0x500000f0 },
    hda_pintbl { nid: 0x12, val: 0x90100010 }, /* Speaker */
    hda_pintbl { nid: 0x13, val: 0x500000f0 },
    hda_pintbl { nid: 0x14, val: 0x500000f0 },
    hda_pintbl { nid: 0x15, val: 0x770000f0 },
    hda_pintbl { nid: 0x16, val: 0x770000f0 },
    hda_pintbl { nid: 0x17, val: 0x430000f0 },
    hda_pintbl { nid: 0x18, val: 0x43ab9030 }, /* Mic */
    hda_pintbl { nid: 0x19, val: 0x770000f0 },
    hda_pintbl { nid: 0x1a, val: 0x770000f0 },
    hda_pintbl { nid: 0x1b, val: 0x770000f0 },
    hda_pintbl { nid: 0x1c, val: 0x90a00090 },
    hda_pintbl { nid: 0x1d, val: 0x500000f0 },
    hda_pintbl { nid: 0x1e, val: 0x500000f0 },
    hda_pintbl { nid: 0x1f, val: 0x500000f0 },
    hda_pintbl { nid: 0x20, val: 0x500000f0 },
    hda_pintbl { nid: 0x21, val: 0x430000f0 },
    hda_pintbl { nid: 0x22, val: 0x430000f0 },
    hda_pintbl::zeroed(), /* terminator */
];

unsafe extern "C" fn cs420x_fixup_gpio_13(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        let spec = (*codec).spec as *mut cs_spec;

        (*spec).gpio_eapd_hp = 2; /* GPIO1 = headphones */
        (*spec).gpio_eapd_speaker = 8; /* GPIO3 = speakers */
        (*spec).gpio_mask = (*spec).gpio_eapd_hp | (*spec).gpio_eapd_speaker;
        (*spec).gpio_dir = (*spec).gpio_mask;
    }
}

unsafe extern "C" fn cs420x_fixup_gpio_23(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        let spec = (*codec).spec as *mut cs_spec;

        (*spec).gpio_eapd_hp = 4; /* GPIO2 = headphones */
        (*spec).gpio_eapd_speaker = 8; /* GPIO3 = speakers */
        (*spec).gpio_mask = (*spec).gpio_eapd_hp | (*spec).gpio_eapd_speaker;
        (*spec).gpio_dir = (*spec).gpio_mask;
    }
}

static cs420x_mbp81_verbs: &[hda_verb] = &[
    /* internal mic ADC2: right only, single ended */
    hda_verb { nid: 0x11, verb: AC_VERB_SET_COEF_INDEX, param: IDX_ADC_CFG },
    hda_verb { nid: 0x11, verb: AC_VERB_SET_PROC_COEF, param: 0x102a },
    hda_verb::zeroed(),
];

static cs420x_fixups: &[hda_fixup] = &[
    hda_fixup_pins!(CS420X_MBP53, mbp53_pincfgs.as_ptr(), true, CS420X_APPLE),
    hda_fixup_pins!(CS420X_MBP55, mbp55_pincfgs.as_ptr(), true, CS420X_GPIO_13),
    hda_fixup_pins!(CS420X_IMAC27, imac27_pincfgs.as_ptr(), true, CS420X_GPIO_13),
    hda_fixup_func!(CS420X_GPIO_13, cs420x_fixup_gpio_13),
    hda_fixup_func!(CS420X_GPIO_23, cs420x_fixup_gpio_23),
    hda_fixup_pins!(CS420X_MBP101, mbp101_pincfgs.as_ptr(), true, CS420X_GPIO_13),
    hda_fixup_verbs!(CS420X_MBP81, cs420x_mbp81_verbs.as_ptr(), true, CS420X_GPIO_13),
    hda_fixup_pins!(CS420X_MBA42, mba42_pincfgs.as_ptr(), true, CS420X_GPIO_13),
];

unsafe fn cs_alloc_spec(codec: *mut hda_codec, vendor_nid: c_int) -> *mut cs_spec {
    let spec: *mut cs_spec;

    spec = kzalloc_obj::<cs_spec>();
    if spec.is_null() {
        return core::ptr::null_mut();
    }
    (*codec).spec = spec as *mut c_void;
    (*spec).vendor_nid = vendor_nid as hda_nid_t;
    (*codec).power_save_node = 1;
    snd_hda_gen_spec_init(&mut (*spec).gen);

    spec
}

unsafe fn cs420x_probe(codec: *mut hda_codec) -> c_int {
    let err: c_int;

    (*codec).single_adc_amp = 1;

    snd_hda_pick_fixup(
        codec,
        cs420x_models.as_ptr(),
        cs420x_fixup_tbl.as_ptr(),
        cs420x_fixups.as_ptr(),
    );
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = cs_parse_auto_config(codec);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

/*
 * CS4208 support:
 * Its layout is no longer compatible with CS4206/CS4207
 */
pub const CS4208_MAC_AUTO: c_int = 0;
pub const CS4208_MBA6: c_int = 1;
pub const CS4208_MBP11: c_int = 2;
pub const CS4208_MACMINI: c_int = 3;
pub const CS4208_GPIO0: c_int = 4;

static cs4208_models: &[hda_model_fixup] = &[
    hda_model_fixup { id: CS4208_GPIO0, name: c_str!("gpio0") },
    hda_model_fixup { id: CS4208_MBA6, name: c_str!("mba6") },
    hda_model_fixup { id: CS4208_MBP11, name: c_str!("mbp11") },
    hda_model_fixup { id: CS4208_MACMINI, name: c_str!("macmini") },
    hda_model_fixup::zeroed(),
];

static cs4208_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK_VENDOR!(0x106b, "Apple", CS4208_MAC_AUTO),
    hda_quirk::zeroed(), /* terminator */
];

/* codec SSID matching */
static cs4208_mac_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK!(0x106b, 0x5e00, "MacBookPro 11,2", CS4208_MBP11),
    SND_PCI_QUIRK!(0x106b, 0x6c00, "MacMini 7,1", CS4208_MACMINI),
    SND_PCI_QUIRK!(0x106b, 0x7100, "MacBookAir 6,1", CS4208_MBA6),
    SND_PCI_QUIRK!(0x106b, 0x7200, "MacBookAir 6,2", CS4208_MBA6),
    SND_PCI_QUIRK!(0x106b, 0x7800, "MacPro 6,1", CS4208_MACMINI),
    SND_PCI_QUIRK!(0x106b, 0x7b00, "MacBookPro 12,1", CS4208_MBP11),
    SND_PCI_QUIRK!(0x106b, 0x7f00, "iMac 16,1", CS4208_MBP11),
    hda_quirk::zeroed(), /* terminator */
];

unsafe extern "C" fn cs4208_fixup_gpio0(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        let spec = (*codec).spec as *mut cs_spec;

        (*spec).gpio_eapd_hp = 0;
        (*spec).gpio_eapd_speaker = 1;
        (*spec).gpio_mask = (*spec).gpio_eapd_hp | (*spec).gpio_eapd_speaker;
        (*spec).gpio_dir = (*spec).gpio_mask;
    }
}

/* remap the fixup from codec SSID and apply it */
unsafe extern "C" fn cs4208_fixup_mac(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action != HDA_FIXUP_ACT_PRE_PROBE {
        return;
    }

    (*codec).fixup_id = HDA_FIXUP_ID_NOT_SET;
    snd_hda_pick_fixup(
        codec,
        core::ptr::null(),
        cs4208_mac_fixup_tbl.as_ptr(),
        cs4208_fixups.as_ptr(),
    );
    if (*codec).fixup_id == HDA_FIXUP_ID_NOT_SET {
        (*codec).fixup_id = CS4208_GPIO0; /* default fixup */
    }
    snd_hda_apply_fixup(codec, action);
}

/* MacMini 7,1 has the inverted jack detection */
unsafe extern "C" fn cs4208_fixup_macmini(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    static pincfgs: &[hda_pintbl] = &[
        hda_pintbl { nid: 0x18, val: 0x00ab9150 }, /* mic (audio-in) jack: disable detect */
        hda_pintbl { nid: 0x21, val: 0x004be140 }, /* SPDIF: disable detect */
        hda_pintbl::zeroed(),
    ];

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        /* HP pin (0x10) has an inverted detection */
        (*codec).inv_jack_detect = 1;
        /* disable the bogus Mic and SPDIF jack detections */
        snd_hda_apply_pincfgs(codec, pincfgs.as_ptr());
    }
}

unsafe extern "C" fn cs4208_spdif_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut cs_spec;
    let pin: hda_nid_t = (*spec).gen.autocfg.dig_out_pins[0];
    let pinctl: c_int = if (*ucontrol).value.integer.value[0] != 0 {
        PIN_OUT
    } else {
        0
    };

    snd_hda_set_pin_ctl_cache(codec, pin, pinctl);
    ((*spec).spdif_sw_put.unwrap())(kcontrol, ucontrol)
}

/* hook the SPDIF switch */
unsafe extern "C" fn cs4208_fixup_spdif_switch(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action == HDA_FIXUP_ACT_BUILD {
        let spec = (*codec).spec as *mut cs_spec;
        let kctl: *mut snd_kcontrol;

        if (*spec).gen.autocfg.dig_out_pins[0] == 0 {
            return;
        }
        kctl = snd_hda_find_mixer_ctl(codec, c_str!("IEC958 Playback Switch"));
        if kctl.is_null() {
            return;
        }
        (*spec).spdif_sw_put = (*kctl).put;
        (*kctl).put = Some(cs4208_spdif_sw_put);
    }
}

static cs4208_fixups: &[hda_fixup] = &[
    hda_fixup_pins!(CS4208_MBA6, mba6_pincfgs.as_ptr(), true, CS4208_GPIO0),
    hda_fixup_func!(CS4208_MBP11, cs4208_fixup_spdif_switch, true, CS4208_GPIO0),
    hda_fixup_func!(CS4208_MACMINI, cs4208_fixup_macmini, true, CS4208_GPIO0),
    hda_fixup_func!(CS4208_GPIO0, cs4208_fixup_gpio0),
    hda_fixup_func!(CS4208_MAC_AUTO, cs4208_fixup_mac),
];

/* correct the 0dB offset of input pins */
unsafe fn cs4208_fix_amp_caps(codec: *mut hda_codec, adc: hda_nid_t) {
    let mut caps: c_uint;

    caps = query_amp_caps(codec, adc, HDA_INPUT);
    caps &= !AC_AMPCAP_OFFSET;
    caps |= 0x02;
    snd_hda_override_amp_caps(codec, adc, HDA_INPUT, caps);
}

unsafe fn cs4208_probe(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;
    let err: c_int;

    /* exclude NID 0x10 (HP) from output volumes due to different steps */
    (*spec).gen.out_vol_mask = 1u64 << 0x10;

    snd_hda_pick_fixup(
        codec,
        cs4208_models.as_ptr(),
        cs4208_fixup_tbl.as_ptr(),
        cs4208_fixups.as_ptr(),
    );
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    snd_hda_override_wcaps(codec, 0x18, get_wcaps(codec, 0x18) | AC_WCAP_STEREO);
    cs4208_fix_amp_caps(codec, 0x18);
    cs4208_fix_amp_caps(codec, 0x1b);
    cs4208_fix_amp_caps(codec, 0x1c);

    err = cs_parse_auto_config(codec);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

unsafe extern "C" fn cs_codec_probe(
    codec: *mut hda_codec,
    id: *const hda_device_id,
) -> c_int {
    let spec: *mut cs_spec;
    let err: c_int;

    spec = cs_alloc_spec(codec, (*id).driver_data as c_int);
    if spec.is_null() {
        return -ENOMEM;
    }
    (*spec).gen.automute_hook = Some(cs_automute);

    if (*spec).vendor_nid == CS4208_VENDOR_NID {
        err = cs4208_probe(codec);
    } else {
        err = cs420x_probe(codec);
    }
    if err < 0 {
        snd_hda_gen_remove(codec);
    }
    err
}

static cs_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(cs_codec_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(cs_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(cs_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_cs420x: &[hda_device_id] = &[
    HDA_CODEC_ID_MODEL!(0x10134206, "CS4206", CS420X_VENDOR_NID),
    HDA_CODEC_ID_MODEL!(0x10134207, "CS4207", CS420X_VENDOR_NID),
    HDA_CODEC_ID_MODEL!(0x10134208, "CS4208", CS4208_VENDOR_NID),
    hda_device_id::zeroed(), /* terminator */
];
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_cs420x);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Cirrus Logic CS420x HD-audio codec");

static mut cs420x_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_cs420x.as_ptr(),
    ops: &cs_codec_ops,
};

module_hda_codec_driver!(cs420x_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
