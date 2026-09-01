/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC CS42L73 codec driver
 *
 * Copyright 2011 Cirrus Logic, Inc.
 *
 * Author: Georgi Vlaev <joe@nucleusys.com>
 *	   Brian Austin <brian.austin@cirrus.com>
 */

/* I2C Registers */
/* I2C Address: 1001010[R/W] - 10010100 = 0x94(Write); 10010101 = 0x95(Read) */
pub const CS42L73_CHIP_ID: u32 = 0x4a;
pub const CS42L73_DEVID_AB: u32 = 0x01; /* Device ID A & B [RO]. */
pub const CS42L73_DEVID_CD: u32 = 0x02; /* Device ID C & D [RO]. */
pub const CS42L73_DEVID_E: u32 = 0x03; /* Device ID E [RO]. */
pub const CS42L73_REVID: u32 = 0x05; /* Revision ID [RO]. */
pub const CS42L73_PWRCTL1: u32 = 0x06; /* Power Control 1. */
pub const CS42L73_PWRCTL2: u32 = 0x07; /* Power Control 2. */
pub const CS42L73_PWRCTL3: u32 = 0x08; /* Power Control 3. */
pub const CS42L73_CPFCHC: u32 = 0x09; /* Charge Pump Freq. Class H Ctl. */
pub const CS42L73_OLMBMSDC: u32 = 0x0A; /* Output Load, MIC Bias, MIC2 SDT */
pub const CS42L73_DMMCC: u32 = 0x0B; /* Digital MIC & Master Clock Ctl. */
pub const CS42L73_XSPC: u32 = 0x0C; /* Auxiliary Serial Port (XSP) Ctl. */
pub const CS42L73_XSPMMCC: u32 = 0x0D; /* XSP Master Mode Clocking Control. */
pub const CS42L73_ASPC: u32 = 0x0E; /* Audio Serial Port (ASP) Control. */
pub const CS42L73_ASPMMCC: u32 = 0x0F; /* ASP Master Mode Clocking Control. */
pub const CS42L73_VSPC: u32 = 0x10; /* Voice Serial Port (VSP) Control. */
pub const CS42L73_VSPMMCC: u32 = 0x11; /* VSP Master Mode Clocking Control. */
pub const CS42L73_VXSPFS: u32 = 0x12; /* VSP & XSP Sample Rate. */
pub const CS42L73_MIOPC: u32 = 0x13; /* Misc. Input & Output Path Control. */
pub const CS42L73_ADCIPC: u32 = 0x14; /* ADC/IP Control. */
pub const CS42L73_MICAPREPGAAVOL: u32 = 0x15; /* MIC 1 [A] PreAmp, PGAA Vol. */
pub const CS42L73_MICBPREPGABVOL: u32 = 0x16; /* MIC 2 [B] PreAmp, PGAB Vol. */
pub const CS42L73_IPADVOL: u32 = 0x17; /* Input Pat7h A Digital Volume. */
pub const CS42L73_IPBDVOL: u32 = 0x18; /* Input Path B Digital Volume. */
pub const CS42L73_PBDC: u32 = 0x19; /* Playback Digital Control. */
pub const CS42L73_HLADVOL: u32 = 0x1A; /* HP/Line A Out Digital Vol. */
pub const CS42L73_HLBDVOL: u32 = 0x1B; /* HP/Line B Out Digital Vol. */
pub const CS42L73_SPKDVOL: u32 = 0x1C; /* Spkphone Out [A] Digital Vol. */
pub const CS42L73_ESLDVOL: u32 = 0x1D; /* Ear/Spkphone LO [B] Digital */
pub const CS42L73_HPAAVOL: u32 = 0x1E; /* HP A Analog Volume. */
pub const CS42L73_HPBAVOL: u32 = 0x1F; /* HP B Analog Volume. */
pub const CS42L73_LOAAVOL: u32 = 0x20; /* Line Out A Analog Volume. */
pub const CS42L73_LOBAVOL: u32 = 0x21; /* Line Out B Analog Volume. */
pub const CS42L73_STRINV: u32 = 0x22; /* Stereo Input Path Adv. Vol. */
pub const CS42L73_XSPINV: u32 = 0x23; /* Auxiliary Port Input Advisory Vol. */
pub const CS42L73_ASPINV: u32 = 0x24; /* Audio Port Input Advisory Vol. */
pub const CS42L73_VSPINV: u32 = 0x25; /* Voice Port Input Advisory Vol. */
pub const CS42L73_LIMARATEHL: u32 = 0x26; /* Lmtr Attack Rate HP/Line. */
pub const CS42L73_LIMRRATEHL: u32 = 0x27; /* Lmtr Ctl, Rel.Rate HP/Line. */
pub const CS42L73_LMAXHL: u32 = 0x28; /* Lmtr Thresholds HP/Line. */
pub const CS42L73_LIMARATESPK: u32 = 0x29; /* Lmtr Attack Rate Spkphone [A]. */
pub const CS42L73_LIMRRATESPK: u32 = 0x2A; /* Lmtr Ctl,Release Rate Spk. [A]. */
pub const CS42L73_LMAXSPK: u32 = 0x2B; /* Lmtr Thresholds Spkphone [A]. */
pub const CS42L73_LIMARATEESL: u32 = 0x2C; /* Lmtr Attack Rate  */
pub const CS42L73_LIMRRATEESL: u32 = 0x2D; /* Lmtr Ctl,Release Rate */
pub const CS42L73_LMAXESL: u32 = 0x2E; /* Lmtr Thresholds */
pub const CS42L73_ALCARATE: u32 = 0x2F; /* ALC Enable, Attack Rate AB. */
pub const CS42L73_ALCRRATE: u32 = 0x30; /* ALC Release Rate AB.  */
pub const CS42L73_ALCMINMAX: u32 = 0x31; /* ALC Thresholds AB. */
pub const CS42L73_NGCAB: u32 = 0x32; /* Noise Gate Ctl AB. */
pub const CS42L73_ALCNGMC: u32 = 0x33; /* ALC & Noise Gate Misc Ctl. */
pub const CS42L73_MIXERCTL: u32 = 0x34; /* Mixer Control. */
pub const CS42L73_HLAIPAA: u32 = 0x35; /* HP/LO Left Mixer: L. */
pub const CS42L73_HLBIPBA: u32 = 0x36; /* HP/LO Right Mixer: R.  */
pub const CS42L73_HLAXSPAA: u32 = 0x37; /* HP/LO Left Mixer: XSP L */
pub const CS42L73_HLBXSPBA: u32 = 0x38; /* HP/LO Right Mixer: XSP R */
pub const CS42L73_HLAASPAA: u32 = 0x39; /* HP/LO Left Mixer: ASP L */
pub const CS42L73_HLBASPBA: u32 = 0x3A; /* HP/LO Right Mixer: ASP R */
pub const CS42L73_HLAVSPMA: u32 = 0x3B; /* HP/LO Left Mixer: VSP. */
pub const CS42L73_HLBVSPMA: u32 = 0x3C; /* HP/LO Right Mixer: VSP */
pub const CS42L73_XSPAIPAA: u32 = 0x3D; /* XSP Left Mixer: Left */
pub const CS42L73_XSPBIPBA: u32 = 0x3E; /* XSP Rt. Mixer: Right */
pub const CS42L73_XSPAXSPAA: u32 = 0x3F; /* XSP Left Mixer: XSP L */
pub const CS42L73_XSPBXSPBA: u32 = 0x40; /* XSP Rt. Mixer: XSP R */
pub const CS42L73_XSPAASPAA: u32 = 0x41; /* XSP Left Mixer: ASP L */
pub const CS42L73_XSPAASPBA: u32 = 0x42; /* XSP Rt. Mixer: ASP R */
pub const CS42L73_XSPAVSPMA: u32 = 0x43; /* XSP Left Mixer: VSP */
pub const CS42L73_XSPBVSPMA: u32 = 0x44; /* XSP Rt. Mixer: VSP */
pub const CS42L73_ASPAIPAA: u32 = 0x45; /* ASP Left Mixer: Left */
pub const CS42L73_ASPBIPBA: u32 = 0x46; /* ASP Rt. Mixer: Right */
pub const CS42L73_ASPAXSPAA: u32 = 0x47; /* ASP Left Mixer: XSP L */
pub const CS42L73_ASPBXSPBA: u32 = 0x48; /* ASP Rt. Mixer: XSP R */
pub const CS42L73_ASPAASPAA: u32 = 0x49; /* ASP Left Mixer: ASP L */
pub const CS42L73_ASPBASPBA: u32 = 0x4A; /* ASP Rt. Mixer: ASP R */
pub const CS42L73_ASPAVSPMA: u32 = 0x4B; /* ASP Left Mixer: VSP */
pub const CS42L73_ASPBVSPMA: u32 = 0x4C; /* ASP Rt. Mixer: VSP */
pub const CS42L73_VSPAIPAA: u32 = 0x4D; /* VSP Left Mixer: Left */
pub const CS42L73_VSPBIPBA: u32 = 0x4E; /* VSP Rt. Mixer: Right */
pub const CS42L73_VSPAXSPAA: u32 = 0x4F; /* VSP Left Mixer: XSP L */
pub const CS42L73_VSPBXSPBA: u32 = 0x50; /* VSP Rt. Mixer: XSP R */
pub const CS42L73_VSPAASPAA: u32 = 0x51; /* VSP Left Mixer: ASP Left */
pub const CS42L73_VSPBASPBA: u32 = 0x52; /* VSP Rt. Mixer: ASP Right */
pub const CS42L73_VSPAVSPMA: u32 = 0x53; /* VSP Left Mixer: VSP */
pub const CS42L73_VSPBVSPMA: u32 = 0x54; /* VSP Rt. Mixer: VSP */
pub const CS42L73_MMIXCTL: u32 = 0x55; /* Mono Mixer Controls. */
pub const CS42L73_SPKMIPMA: u32 = 0x56; /* SPK Mono Mixer: In. Path */
pub const CS42L73_SPKMXSPA: u32 = 0x57; /* SPK Mono Mixer: XSP Mono/L/R Att. */
pub const CS42L73_SPKMASPA: u32 = 0x58; /* SPK Mono Mixer: ASP Mono/L/R Att. */
pub const CS42L73_SPKMVSPMA: u32 = 0x59; /* SPK Mono Mixer: VSP Mono Atten. */
pub const CS42L73_ESLMIPMA: u32 = 0x5A; /* Ear/SpLO Mono Mixer: */
pub const CS42L73_ESLMXSPA: u32 = 0x5B; /* Ear/SpLO Mono Mixer: XSP */
pub const CS42L73_ESLMASPA: u32 = 0x5C; /* Ear/SpLO Mono Mixer: ASP */
pub const CS42L73_ESLMVSPMA: u32 = 0x5D; /* Ear/SpLO Mono Mixer: VSP */
pub const CS42L73_IM1: u32 = 0x5E; /* Interrupt Mask 1.  */
pub const CS42L73_IM2: u32 = 0x5F; /* Interrupt Mask 2. */
pub const CS42L73_IS1: u32 = 0x60; /* Interrupt Status 1 [RO]. */
pub const CS42L73_IS2: u32 = 0x61; /* Interrupt Status 2 [RO]. */
pub const CS42L73_MAX_REGISTER: u32 = 0x61; /* Total Registers */
/* Bitfield Definitions */

/* CS42L73_PWRCTL1 */
pub const CS42L73_PDN_ADCB: u32 = 1 << 7;
pub const CS42L73_PDN_DMICB: u32 = 1 << 6;
pub const CS42L73_PDN_ADCA: u32 = 1 << 5;
pub const CS42L73_PDN_DMICA: u32 = 1 << 4;
pub const CS42L73_PDN_LDO: u32 = 1 << 2;
pub const CS42L73_DISCHG_FILT: u32 = 1 << 1;
pub const CS42L73_PDN: u32 = 1 << 0;

/* CS42L73_PWRCTL2 */
pub const CS42L73_PDN_MIC2_BIAS: u32 = 1 << 7;
pub const CS42L73_PDN_MIC1_BIAS: u32 = 1 << 6;
pub const CS42L73_PDN_VSP: u32 = 1 << 4;
pub const CS42L73_PDN_ASP_SDOUT: u32 = 1 << 3;
pub const CS42L73_PDN_ASP_SDIN: u32 = 1 << 2;
pub const CS42L73_PDN_XSP_SDOUT: u32 = 1 << 1;
pub const CS42L73_PDN_XSP_SDIN: u32 = 1 << 0;

/* CS42L73_PWRCTL3 */
pub const CS42L73_PDN_THMS: u32 = 1 << 5;
pub const CS42L73_PDN_SPKLO: u32 = 1 << 4;
pub const CS42L73_PDN_EAR: u32 = 1 << 3;
pub const CS42L73_PDN_SPK: u32 = 1 << 2;
pub const CS42L73_PDN_LO: u32 = 1 << 1;
pub const CS42L73_PDN_HP: u32 = 1 << 0;

/* Thermal Overload Detect. Requires interrupt ... */
pub const CS42L73_THMOVLD_150C: u32 = 0;
pub const CS42L73_THMOVLD_132C: u32 = 1;
pub const CS42L73_THMOVLD_115C: u32 = 2;
pub const CS42L73_THMOVLD_098C: u32 = 3;

pub const CS42L73_CHARGEPUMP_MASK: u32 = 0xF0;

/* CS42L73_ASPC, CS42L73_XSPC, CS42L73_VSPC */
pub const CS42L73_SP_3ST: u32 = 1 << 7;
pub const CS42L73_SPDIF_I2S: u32 = 0 << 6;
pub const CS42L73_SPDIF_PCM: u32 = 1 << 6;
pub const CS42L73_PCM_MODE0: u32 = 0 << 4;
pub const CS42L73_PCM_MODE1: u32 = 1 << 4;
pub const CS42L73_PCM_MODE2: u32 = 2 << 4;
pub const CS42L73_PCM_MODE_MASK: u32 = 3 << 4;
pub const CS42L73_PCM_BIT_ORDER: u32 = 1 << 3;
pub const CS42L73_MCK_SCLK_64FS: u32 = 0 << 0;
pub const CS42L73_MCK_SCLK_MCLK: u32 = 2 << 0;
pub const CS42L73_MCK_SCLK_PREMCLK: u32 = 3 << 0;

/* CS42L73_xSPMMCC */
pub const CS42L73_MS_MASTER: u32 = 1 << 7;

/* CS42L73_DMMCC */
pub const CS42L73_MCLKDIS: u32 = 1 << 0;
pub const CS42L73_MCLKSEL_MCLK2: u32 = 1 << 4;
pub const CS42L73_MCLKSEL_MCLK1: u32 = 0 << 4;

/* CS42L73 MCLK derived from MCLK1 or MCLK2 */
pub const CS42L73_CLKID_MCLK1: u32 = 0;
pub const CS42L73_CLKID_MCLK2: u32 = 1;

pub const CS42L73_MCLKXDIV: u32 = 0;
pub const CS42L73_MMCCDIV: u32 = 1;

pub const CS42L73_XSP: u32 = 0;
pub const CS42L73_ASP: u32 = 1;
pub const CS42L73_VSP: u32 = 2;

/* IS1, IM1 */
pub const CS42L73_MIC2_SDET: u32 = 1 << 6;
pub const CS42L73_THMOVLD: u32 = 1 << 4;
pub const CS42L73_DIGMIXOVFL: u32 = 1 << 3;
pub const CS42L73_IPBOVFL: u32 = 1 << 1;
pub const CS42L73_IPAOVFL: u32 = 1 << 0;

/* Analog Softramp */
pub const CS42L73_ANLGOSFT: u32 = 1 << 0;

/* HP A/B Analog Mute */
pub const CS42L73_HPA_MUTE: u32 = 1 << 7;
/* LO A/B Analog Mute	*/
pub const CS42L73_LOA_MUTE: u32 = 1 << 7;
/* Digital Mute */
pub const CS42L73_HLAD_MUTE: u32 = 1 << 0;
pub const CS42L73_HLBD_MUTE: u32 = 1 << 1;
pub const CS42L73_SPKD_MUTE: u32 = 1 << 2;
pub const CS42L73_ESLD_MUTE: u32 = 1 << 3;

/* Misc defines for codec */
pub const CS42L73_DEVID: u32 = 0x00042A73;
pub const CS42L73_MCLKX_MIN: u32 = 5644800;
pub const CS42L73_MCLKX_MAX: u32 = 38400000;

pub const fn CS42L73_SPC(id: u32) -> u32 {
    CS42L73_XSPC + (id << 1)
}

pub const fn CS42L73_MMCC(id: u32) -> u32 {
    CS42L73_XSPMMCC + (id << 1)
}

pub const fn CS42L73_SPFS(id: u32) -> u32 {
    if id == CS42L73_ASP {
        CS42L73_ASPC
    } else {
        CS42L73_VXSPFS
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
