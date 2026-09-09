/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    saa7115.h - definition for saa7111/3/4/5 inputs and frequency flags

    Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)

*/

/* s_routing inputs, outputs, and config */

/* SAA7111/3/4/5 HW inputs */
pub const SAA7115_COMPOSITE0: i32 = 0;
pub const SAA7115_COMPOSITE1: i32 = 1;
pub const SAA7115_COMPOSITE2: i32 = 2;
pub const SAA7115_COMPOSITE3: i32 = 3;
pub const SAA7115_COMPOSITE4: i32 = 4; /* not available for the saa7111/3 */
pub const SAA7115_COMPOSITE5: i32 = 5; /* not available for the saa7111/3 */
pub const SAA7115_SVIDEO0: i32 = 6;
pub const SAA7115_SVIDEO1: i32 = 7;
pub const SAA7115_SVIDEO2: i32 = 8;
pub const SAA7115_SVIDEO3: i32 = 9;

/* outputs */
pub const SAA7115_IPORT_ON: i32 = 1;
pub const SAA7115_IPORT_OFF: i32 = 0;

/* SAA7111 specific outputs. */
pub const SAA7111_VBI_BYPASS: i32 = 2;
pub const SAA7111_FMT_YUV422: i32 = 0x00;
pub const SAA7111_FMT_RGB: i32 = 0x40;
pub const SAA7111_FMT_CCIR: i32 = 0x80;
pub const SAA7111_FMT_YUV411: i32 = 0xc0;

/* config flags */
/*
 * Register 0x85 should set bit 0 to 0 (it's 1 by default). This bit
 * controls the IDQ signal polarity which is set to 'inverted' if the bit
 * it 1 and to 'default' if it is 0.
 */
pub const SAA7115_IDQ_IS_DEFAULT: i32 = 1 << 0;

/* s_crystal_freq values and flags */

/* SAA7115 v4l2_crystal_freq frequency values */
pub const SAA7115_FREQ_32_11_MHZ: i32 = 32110000; /* 32.11 MHz crystal, SAA7114/5 only */
pub const SAA7115_FREQ_24_576_MHZ: i32 = 24576000; /* 24.576 MHz crystal */

/* SAA7115 v4l2_crystal_freq audio clock control flags */
pub const SAA7115_FREQ_FL_UCGC: i32 = 1 << 0; /* SA 3A[7], UCGC, SAA7115 only */
pub const SAA7115_FREQ_FL_CGCDIV: i32 = 1 << 1; /* SA 3A[6], CGCDIV, SAA7115 only */
pub const SAA7115_FREQ_FL_APLL: i32 = 1 << 2; /* SA 3A[3], APLL, SAA7114/5 only */
pub const SAA7115_FREQ_FL_DOUBLE_ASCLK: i32 = 1 << 3; /* SA 39, LRDIV, SAA7114/5 only */

/* ===== SAA7113 Config enums ===== */

/* Register 0x08 "Horizontal time constant" [Bit 3..4]:
 * Should be set to "Fast Locking Mode" according to the datasheet,
 * and that is the default setting in the gm7113c_init table.
 * saa7113_init sets this value to "VTR Mode".
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum saa7113_r08_htc {
    SAA7113_HTC_TV_MODE = 0x00,
    SAA7113_HTC_VTR_MODE,
    SAA7113_HTC_FAST_LOCKING_MODE = 0x03,
}

/* Register 0x10 "Output format selection" [Bit 6..7]:
 * Defaults to ITU_656 as specified in datasheet. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum saa7113_r10_ofts {
    SAA7113_OFTS_ITU_656 = 0x0,
    SAA7113_OFTS_VFLAG_BY_VREF,
    SAA7113_OFTS_VFLAG_BY_DATA_TYPE,
}

/*
 * Register 0x12 "Output control" [Bit 0..3 Or Bit 4..7]:
 * This is used to select what data is output on the RTS0 and RTS1 pins.
 * RTS1 [Bit 4..7] Defaults to DOT_IN. (This value can not be set for RTS0)
 * RTS0 [Bit 0..3] Defaults to VIPB in gm7113c_init as specified
 * in the datasheet, but is set to HREF_HS in the saa7113_init table.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum saa7113_r12_rts {
    SAA7113_RTS_DOT_IN = 0,
    SAA7113_RTS_VIPB,
    SAA7113_RTS_GPSW,
    SAA7115_RTS_HL,
    SAA7113_RTS_VL,
    SAA7113_RTS_DL,
    SAA7113_RTS_PLIN,
    SAA7113_RTS_HREF_HS,
    SAA7113_RTS_HS,
    SAA7113_RTS_HQ,
    SAA7113_RTS_ODD,
    SAA7113_RTS_VS,
    SAA7113_RTS_V123,
    SAA7113_RTS_VGATE,
    SAA7113_RTS_VREF,
    SAA7113_RTS_FID,
}

/**
 * struct saa7115_platform_data - Allow overriding default initialization
 *
 * @saa7113_force_gm7113c_init: Force the use of the gm7113c_init table
 *                              instead of saa7113_init table
 *                              (saa7113 only)
 * @saa7113_r08_htc:             [R_08 - Bit 3..4]
 * @saa7113_r10_vrln:             [R_10 - Bit 3]
 *                              default: Disabled for gm7113c_init
 *                                       Enabled for saa7113c_init
 * @saa7113_r10_ofts:             [R_10 - Bit 6..7]
 * @saa7113_r12_rts0:             [R_12 - Bit 0..3]
 * @saa7113_r12_rts1:             [R_12 - Bit 4..7]
 * @saa7113_r13_adlsb:            [R_13 - Bit 7] - default: disabled
 */
#[repr(C)]
pub struct saa7115_platform_data {
    pub saa7113_force_gm7113c_init: bool,
    pub saa7113_r08_htc: *mut saa7113_r08_htc,
    pub saa7113_r10_vrln: *mut bool,
    pub saa7113_r10_ofts: *mut saa7113_r10_ofts,
    pub saa7113_r12_rts0: *mut saa7113_r12_rts,
    pub saa7113_r12_rts1: *mut saa7113_r12_rts,
    pub saa7113_r13_adlsb: *mut bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
