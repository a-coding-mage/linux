/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * i2sbus driver -- interface register definitions
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

/* i2s bus control registers, at least what we know about them */

#[repr(C, packed)]
pub struct i2s_interface_regs {
    pub intr_ctl: __le32,         /* 0x00 */
    pub __pad0: [u8; 12],
    pub serial_format: __le32,    /* 0x10 */
    pub __pad1: [u8; 12],
    pub codec_msg_out: __le32,    /* 0x20 */
    pub __pad2: [u8; 12],
    pub codec_msg_in: __le32,     /* 0x30 */
    pub __pad3: [u8; 12],
    pub frame_count: __le32,      /* 0x40 */
    pub __pad4: [u8; 12],
    pub frame_match: __le32,      /* 0x50 */
    pub __pad5: [u8; 12],
    pub data_word_sizes: __le32,  /* 0x60 */
    pub __pad6: [u8; 12],
    pub peak_level_sel: __le32,   /* 0x70 */
    pub __pad7: [u8; 12],
    pub peak_level_in0: __le32,   /* 0x80 */
    pub __pad8: [u8; 12],
    pub peak_level_in1: __le32,   /* 0x90 */
    pub __pad9: [u8; 108],
    /* total size: 0x100 bytes */
}

/* interrupt register is just a bitfield with
 * interrupt enable and pending bits */
pub const I2S_REG_INTR_CTL: u32 = 0x00;
pub const I2S_INT_FRAME_COUNT: u32 = 1 << 31;
pub const I2S_PENDING_FRAME_COUNT: u32 = 1 << 30;
pub const I2S_INT_MESSAGE_FLAG: u32 = 1 << 29;
pub const I2S_PENDING_MESSAGE_FLAG: u32 = 1 << 28;
pub const I2S_INT_NEW_PEAK: u32 = 1 << 27;
pub const I2S_PENDING_NEW_PEAK: u32 = 1 << 26;
pub const I2S_INT_CLOCKS_STOPPED: u32 = 1 << 25;
pub const I2S_PENDING_CLOCKS_STOPPED: u32 = 1 << 24;
pub const I2S_INT_EXTERNAL_SYNC_ERROR: u32 = 1 << 23;
pub const I2S_PENDING_EXTERNAL_SYNC_ERROR: u32 = 1 << 22;
pub const I2S_INT_EXTERNAL_SYNC_OK: u32 = 1 << 21;
pub const I2S_PENDING_EXTERNAL_SYNC_OK: u32 = 1 << 20;
pub const I2S_INT_NEW_SAMPLE_RATE: u32 = 1 << 19;
pub const I2S_PENDING_NEW_SAMPLE_RATE: u32 = 1 << 18;
pub const I2S_INT_STATUS_FLAG: u32 = 1 << 17;
pub const I2S_PENDING_STATUS_FLAG: u32 = 1 << 16;

/* serial format register is more interesting :)
 * It contains:
 *  - clock source
 *  - MClk divisor
 *  - SClk divisor
 *  - SClk master flag
 *  - serial format (sony, i2s 64x, i2s 32x, dav, silabs)
 *  - external sample frequency interrupt (don't understand)
 *  - external sample frequency
 */
pub const I2S_REG_SERIAL_FORMAT: u32 = 0x10;
/* clock source. You get either 18.432, 45.1584 or 49.1520 MHz */
pub const I2S_SF_CLOCK_SOURCE_SHIFT: u32 = 30;
pub const I2S_SF_CLOCK_SOURCE_MASK: u32 = 3 << I2S_SF_CLOCK_SOURCE_SHIFT;
pub const I2S_SF_CLOCK_SOURCE_18MHz: u32 = 0 << I2S_SF_CLOCK_SOURCE_SHIFT;
pub const I2S_SF_CLOCK_SOURCE_45MHz: u32 = 1 << I2S_SF_CLOCK_SOURCE_SHIFT;
pub const I2S_SF_CLOCK_SOURCE_49MHz: u32 = 2 << I2S_SF_CLOCK_SOURCE_SHIFT;
/* also, let's define the exact clock speeds here, in Hz */
pub const I2S_CLOCK_SPEED_18MHz: u32 = 18432000;
pub const I2S_CLOCK_SPEED_45MHz: u32 = 45158400;
pub const I2S_CLOCK_SPEED_49MHz: u32 = 49152000;
/* MClk is the clock that drives the codec, usually called its 'system clock'.
 * It is derived by taking only every 'divisor' tick of the clock.
 */
pub const I2S_SF_MCLKDIV_SHIFT: u32 = 24;
pub const I2S_SF_MCLKDIV_MASK: u32 = 0x1F << I2S_SF_MCLKDIV_SHIFT;
pub const I2S_SF_MCLKDIV_1: u32 = 0x14 << I2S_SF_MCLKDIV_SHIFT;
pub const I2S_SF_MCLKDIV_3: u32 = 0x13 << I2S_SF_MCLKDIV_SHIFT;
pub const I2S_SF_MCLKDIV_5: u32 = 0x12 << I2S_SF_MCLKDIV_SHIFT;
pub const I2S_SF_MCLKDIV_14: u32 = 0x0E << I2S_SF_MCLKDIV_SHIFT;

pub const fn I2S_SF_MCLKDIV_OTHER(div: i32) -> u32 {
    ((((div / 2 - 1) as u32) << I2S_SF_MCLKDIV_SHIFT) & I2S_SF_MCLKDIV_MASK)
}

pub unsafe fn i2s_sf_mclkdiv(div: i32, out: *mut i32) -> i32 {
    let d: i32;

    match div {
        1 => {
            *out |= I2S_SF_MCLKDIV_1 as i32;
            return 0;
        }
        3 => {
            *out |= I2S_SF_MCLKDIV_3 as i32;
            return 0;
        }
        5 => {
            *out |= I2S_SF_MCLKDIV_5 as i32;
            return 0;
        }
        14 => {
            *out |= I2S_SF_MCLKDIV_14 as i32;
            return 0;
        }
        _ => {
            if div % 2 != 0 {
                return -1;
            }
            d = div / 2 - 1;
            if d == 0x14 || d == 0x13 || d == 0x12 || d == 0x0E {
                return -1;
            }
            *out |= I2S_SF_MCLKDIV_OTHER(div) as i32;
            return 0;
        }
    }
}

/* SClk is the clock that drives the i2s wire bus. Note that it is
 * derived from the MClk above by taking only every 'divisor' tick
 * of MClk.
 */
pub const I2S_SF_SCLKDIV_SHIFT: u32 = 20;
pub const I2S_SF_SCLKDIV_MASK: u32 = 0xF << I2S_SF_SCLKDIV_SHIFT;
pub const I2S_SF_SCLKDIV_1: u32 = 8 << I2S_SF_SCLKDIV_SHIFT;
pub const I2S_SF_SCLKDIV_3: u32 = 9 << I2S_SF_SCLKDIV_SHIFT;

pub const fn I2S_SF_SCLKDIV_OTHER(div: i32) -> u32 {
    ((((div / 2 - 1) as u32) << I2S_SF_SCLKDIV_SHIFT) & I2S_SF_SCLKDIV_MASK)
}

pub unsafe fn i2s_sf_sclkdiv(div: i32, out: *mut i32) -> i32 {
    let d: i32;

    match div {
        1 => {
            *out |= I2S_SF_SCLKDIV_1 as i32;
            return 0;
        }
        3 => {
            *out |= I2S_SF_SCLKDIV_3 as i32;
            return 0;
        }
        _ => {
            if div % 2 != 0 {
                return -1;
            }
            d = div / 2 - 1;
            if d == 8 || d == 9 {
                return -1;
            }
            *out |= I2S_SF_SCLKDIV_OTHER(div) as i32;
            return 0;
        }
    }
}

pub const I2S_SF_SCLK_MASTER: u32 = 1 << 19;
/* serial format is the way the data is put to the i2s wire bus */
pub const I2S_SF_SERIAL_FORMAT_SHIFT: u32 = 16;
pub const I2S_SF_SERIAL_FORMAT_MASK: u32 = 7 << I2S_SF_SERIAL_FORMAT_SHIFT;
pub const I2S_SF_SERIAL_FORMAT_SONY: u32 = 0 << I2S_SF_SERIAL_FORMAT_SHIFT;
pub const I2S_SF_SERIAL_FORMAT_I2S_64X: u32 = 1 << I2S_SF_SERIAL_FORMAT_SHIFT;
pub const I2S_SF_SERIAL_FORMAT_I2S_32X: u32 = 2 << I2S_SF_SERIAL_FORMAT_SHIFT;
pub const I2S_SF_SERIAL_FORMAT_I2S_DAV: u32 = 4 << I2S_SF_SERIAL_FORMAT_SHIFT;
pub const I2S_SF_SERIAL_FORMAT_I2S_SILABS: u32 = 5 << I2S_SF_SERIAL_FORMAT_SHIFT;
/* unknown */
pub const I2S_SF_EXT_SAMPLE_FREQ_INT_SHIFT: u32 = 12;
/* The C macro refers to I2S_SF_SAMPLE_FREQ_INT_SHIFT here. */
pub const I2S_SF_EXT_SAMPLE_FREQ_INT_MASK: u32 = 0xF << I2S_SF_SAMPLE_FREQ_INT_SHIFT;
/* probably gives external frequency? */
pub const I2S_SF_EXT_SAMPLE_FREQ_MASK: u32 = 0xFFF;

/* used to send codec messages, but how isn't clear */
pub const I2S_REG_CODEC_MSG_OUT: u32 = 0x20;

/* used to receive codec messages, but how isn't clear */
pub const I2S_REG_CODEC_MSG_IN: u32 = 0x30;

/* frame count reg isn't clear to me yet, but probably useful */
pub const I2S_REG_FRAME_COUNT: u32 = 0x40;

/* program to some value, and get interrupt if frame count reaches it */
pub const I2S_REG_FRAME_MATCH: u32 = 0x50;

/* this register describes how the bus transfers data */
pub const I2S_REG_DATA_WORD_SIZES: u32 = 0x60;
/* number of interleaved input channels */
pub const I2S_DWS_NUM_CHANNELS_IN_SHIFT: u32 = 24;
pub const I2S_DWS_NUM_CHANNELS_IN_MASK: u32 = 0x1F << I2S_DWS_NUM_CHANNELS_IN_SHIFT;
/* word size of input data */
pub const I2S_DWS_DATA_IN_SIZE_SHIFT: u32 = 16;
pub const I2S_DWS_DATA_IN_16BIT: u32 = 0 << I2S_DWS_DATA_IN_SIZE_SHIFT;
pub const I2S_DWS_DATA_IN_24BIT: u32 = 3 << I2S_DWS_DATA_IN_SIZE_SHIFT;
/* number of interleaved output channels */
pub const I2S_DWS_NUM_CHANNELS_OUT_SHIFT: u32 = 8;
pub const I2S_DWS_NUM_CHANNELS_OUT_MASK: u32 = 0x1F << I2S_DWS_NUM_CHANNELS_OUT_SHIFT;
/* word size of output data */
pub const I2S_DWS_DATA_OUT_SIZE_SHIFT: u32 = 0;
pub const I2S_DWS_DATA_OUT_16BIT: u32 = 0 << I2S_DWS_DATA_OUT_SIZE_SHIFT;
pub const I2S_DWS_DATA_OUT_24BIT: u32 = 3 << I2S_DWS_DATA_OUT_SIZE_SHIFT;


/* unknown */
pub const I2S_REG_PEAK_LEVEL_SEL: u32 = 0x70;

/* unknown */
pub const I2S_REG_PEAK_LEVEL_IN0: u32 = 0x80;

/* unknown */
pub const I2S_REG_PEAK_LEVEL_IN1: u32 = 0x90;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
