/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NXP AUDMIX ALSA SoC Digital Audio Interface (DAI) driver
 *
 * Copyright 2017 NXP
 */

// Dependencies originally supplied by C includes:
// SNDRV_PCM_FMTBIT_S16_LE, SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S32_LE,
// platform_device, regmap, clk, and spinlock_t.

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const FSL_AUDMIX_FORMATS: u32 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* AUDMIX Registers */
pub const FSL_AUDMIX_CTR: u32 = 0x200; /* Control */
pub const FSL_AUDMIX_STR: u32 = 0x204; /* Status */

pub const FSL_AUDMIX_ATCR0: u32 = 0x208; /* Attenuation Control */
pub const FSL_AUDMIX_ATIVAL0: u32 = 0x20c; /* Attenuation Initial Value */
pub const FSL_AUDMIX_ATSTPUP0: u32 = 0x210; /* Attenuation step up factor */
pub const FSL_AUDMIX_ATSTPDN0: u32 = 0x214; /* Attenuation step down factor */
pub const FSL_AUDMIX_ATSTPTGT0: u32 = 0x218; /* Attenuation step target */
pub const FSL_AUDMIX_ATTNVAL0: u32 = 0x21c; /* Attenuation Value */
pub const FSL_AUDMIX_ATSTP0: u32 = 0x220; /* Attenuation step number */

pub const FSL_AUDMIX_ATCR1: u32 = 0x228; /* Attenuation Control */
pub const FSL_AUDMIX_ATIVAL1: u32 = 0x22c; /* Attenuation Initial Value */
pub const FSL_AUDMIX_ATSTPUP1: u32 = 0x230; /* Attenuation step up factor */
pub const FSL_AUDMIX_ATSTPDN1: u32 = 0x234; /* Attenuation step down factor */
pub const FSL_AUDMIX_ATSTPTGT1: u32 = 0x238; /* Attenuation step target */
pub const FSL_AUDMIX_ATTNVAL1: u32 = 0x23c; /* Attenuation Value */
pub const FSL_AUDMIX_ATSTP1: u32 = 0x240; /* Attenuation step number */

/* AUDMIX Control Register */
pub const FSL_AUDMIX_CTR_MIXCLK_SHIFT: u32 = 0;
pub const FSL_AUDMIX_CTR_MIXCLK_MASK: u32 = BIT(FSL_AUDMIX_CTR_MIXCLK_SHIFT);
pub const fn FSL_AUDMIX_CTR_MIXCLK(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_MIXCLK_SHIFT
}

pub const FSL_AUDMIX_CTR_OUTSRC_SHIFT: u32 = 1;
pub const FSL_AUDMIX_CTR_OUTSRC_MASK: u32 = 0x3 << FSL_AUDMIX_CTR_OUTSRC_SHIFT;
pub const fn FSL_AUDMIX_CTR_OUTSRC(i: u32) -> u32 {
    (i << FSL_AUDMIX_CTR_OUTSRC_SHIFT) & FSL_AUDMIX_CTR_OUTSRC_MASK
}

pub const FSL_AUDMIX_CTR_OUTWIDTH_SHIFT: u32 = 3;
pub const FSL_AUDMIX_CTR_OUTWIDTH_MASK: u32 = 0x7 << FSL_AUDMIX_CTR_OUTWIDTH_SHIFT;
pub const fn FSL_AUDMIX_CTR_OUTWIDTH(i: u32) -> u32 {
    (i << FSL_AUDMIX_CTR_OUTWIDTH_SHIFT) & FSL_AUDMIX_CTR_OUTWIDTH_MASK
}

pub const FSL_AUDMIX_CTR_OUTCKPOL_SHIFT: u32 = 6;
pub const FSL_AUDMIX_CTR_OUTCKPOL_MASK: u32 = BIT(FSL_AUDMIX_CTR_OUTCKPOL_SHIFT);
pub const fn FSL_AUDMIX_CTR_OUTCKPOL(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_OUTCKPOL_SHIFT
}

pub const FSL_AUDMIX_CTR_MASKRTDF_SHIFT: u32 = 7;
pub const FSL_AUDMIX_CTR_MASKRTDF_MASK: u32 = BIT(FSL_AUDMIX_CTR_MASKRTDF_SHIFT);
pub const fn FSL_AUDMIX_CTR_MASKRTDF(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_MASKRTDF_SHIFT
}

pub const FSL_AUDMIX_CTR_MASKCKDF_SHIFT: u32 = 8;
pub const FSL_AUDMIX_CTR_MASKCKDF_MASK: u32 = BIT(FSL_AUDMIX_CTR_MASKCKDF_SHIFT);
pub const fn FSL_AUDMIX_CTR_MASKCKDF(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_MASKCKDF_SHIFT
}

pub const FSL_AUDMIX_CTR_SYNCMODE_SHIFT: u32 = 9;
pub const FSL_AUDMIX_CTR_SYNCMODE_MASK: u32 = BIT(FSL_AUDMIX_CTR_SYNCMODE_SHIFT);
pub const fn FSL_AUDMIX_CTR_SYNCMODE(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_SYNCMODE_SHIFT
}

pub const FSL_AUDMIX_CTR_SYNCSRC_SHIFT: u32 = 10;
pub const FSL_AUDMIX_CTR_SYNCSRC_MASK: u32 = BIT(FSL_AUDMIX_CTR_SYNCSRC_SHIFT);
pub const fn FSL_AUDMIX_CTR_SYNCSRC(i: u32) -> u32 {
    i << FSL_AUDMIX_CTR_SYNCSRC_SHIFT
}

/* AUDMIX Status Register */
pub const FSL_AUDMIX_STR_RATEDIFF: u32 = BIT(0);
pub const FSL_AUDMIX_STR_CLKDIFF: u32 = BIT(1);
pub const FSL_AUDMIX_STR_MIXSTAT_SHIFT: u32 = 2;
pub const FSL_AUDMIX_STR_MIXSTAT_MASK: u32 = 0x3 << FSL_AUDMIX_STR_MIXSTAT_SHIFT;
pub const fn FSL_AUDMIX_STR_MIXSTAT(i: u32) -> u32 {
    (i & FSL_AUDMIX_STR_MIXSTAT_MASK) >> FSL_AUDMIX_STR_MIXSTAT_SHIFT
}

/* AUDMIX Attenuation Control Register */
pub const FSL_AUDMIX_ATCR_AT_EN: u32 = BIT(0);
pub const FSL_AUDMIX_ATCR_AT_UPDN: u32 = BIT(1);
pub const FSL_AUDMIX_ATCR_ATSTPDIF_SHIFT: u32 = 2;
pub const FSL_AUDMIX_ATCR_ATSTPDFI_MASK: u32 = 0xfff << FSL_AUDMIX_ATCR_ATSTPDIF_SHIFT;

/* AUDMIX Attenuation Initial Value Register */
pub const FSL_AUDMIX_ATIVAL_ATINVAL_MASK: u32 = 0x3FFFF;

/* AUDMIX Attenuation Step Up Factor Register */
pub const FSL_AUDMIX_ATSTPUP_ATSTEPUP_MASK: u32 = 0x3FFFF;

/* AUDMIX Attenuation Step Down Factor Register */
pub const FSL_AUDMIX_ATSTPDN_ATSTEPDN_MASK: u32 = 0x3FFFF;

/* AUDMIX Attenuation Step Target Register */
pub const FSL_AUDMIX_ATSTPTGT_ATSTPTG_MASK: u32 = 0x3FFFF;

/* AUDMIX Attenuation Value Register */
pub const FSL_AUDMIX_ATTNVAL_ATCURVAL_MASK: u32 = 0x3FFFF;

/* AUDMIX Attenuation Step Number Register */
pub const FSL_AUDMIX_ATSTP_STPCTR_MASK: u32 = 0x3FFFF;

pub const FSL_AUDMIX_MAX_DAIS: u32 = 2;

#[repr(C)]
pub struct fsl_audmix {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub ipg_clk: *mut clk,
    pub lock: spinlock_t, /* Protect tdms */
    pub tdms: u8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
