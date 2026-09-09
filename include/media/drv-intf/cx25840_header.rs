/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   cx25840.h - definition for cx25840/1/2/3 inputs
 *
 *   Copyright (C) 2006 Hans Verkuil (hverkuil@kernel.org)
 */

/* The original header guard and preprocessor-only includes are omitted. */

/*
 * Note that the cx25840 driver requires that the bridge driver calls the
 * v4l2_subdev's load_fw operation in order to load the driver's firmware.
 * This will load the firmware on the first invocation (further ones are NOP).
 * Without this the audio standard detection will fail and you will
 * only get mono.
 * Alternatively, you can call the reset operation (this can be done
 * multiple times if needed, each invocation will fully reinitialize the device).
 *
 * Since loading the firmware is often problematic when the driver is
 * compiled into the kernel I recommend postponing calling this function
 * until the first open of the video device. Another reason for
 * postponing it is that loading this firmware takes a long time (seconds)
 * due to the slow i2c bus speed. So it will speed up the boot process if
 * you can avoid loading the fw as long as the video device isn't used.
 */

#[repr(i32)]
pub enum cx25840_video_input {
    /* Composite video inputs In1-In8 */
    CX25840_COMPOSITE1 = 1,
    CX25840_COMPOSITE2,
    CX25840_COMPOSITE3,
    CX25840_COMPOSITE4,
    CX25840_COMPOSITE5,
    CX25840_COMPOSITE6,
    CX25840_COMPOSITE7,
    CX25840_COMPOSITE8,

    /* S-Video inputs consist of one luma input (In1-In8) ORed with one
     * chroma input (In5-In8) */
    CX25840_SVIDEO_LUMA1 = 0x10,
    CX25840_SVIDEO_LUMA2 = 0x20,
    CX25840_SVIDEO_LUMA3 = 0x30,
    CX25840_SVIDEO_LUMA4 = 0x40,
    CX25840_SVIDEO_LUMA5 = 0x50,
    CX25840_SVIDEO_LUMA6 = 0x60,
    CX25840_SVIDEO_LUMA7 = 0x70,
    CX25840_SVIDEO_LUMA8 = 0x80,
    CX25840_SVIDEO_CHROMA4 = 0x400,
    CX25840_SVIDEO_CHROMA5 = 0x500,
    CX25840_SVIDEO_CHROMA6 = 0x600,
    CX25840_SVIDEO_CHROMA7 = 0x700,
    CX25840_SVIDEO_CHROMA8 = 0x800,

    /* S-Video aliases for common luma/chroma combinations */
    CX25840_SVIDEO1 = 0x510,
    CX25840_SVIDEO2 = 0x620,
    CX25840_SVIDEO3 = 0x730,
    CX25840_SVIDEO4 = 0x840,
}

pub const CX25840_VIN1_CH1: u32 = 0x80000000;
pub const CX25840_VIN2_CH1: u32 = 0x80000001;
pub const CX25840_VIN3_CH1: u32 = 0x80000002;
pub const CX25840_VIN4_CH1: u32 = 0x80000003;
pub const CX25840_VIN5_CH1: u32 = 0x80000004;
pub const CX25840_VIN6_CH1: u32 = 0x80000005;
pub const CX25840_VIN7_CH1: u32 = 0x80000006;
pub const CX25840_VIN8_CH1: u32 = 0x80000007;
pub const CX25840_VIN4_CH2: u32 = 0x80000000;
pub const CX25840_VIN5_CH2: u32 = 0x80000010;
pub const CX25840_VIN6_CH2: u32 = 0x80000020;
pub const CX25840_NONE_CH2: u32 = 0x80000030;
pub const CX25840_VIN7_CH3: u32 = 0x80000000;
pub const CX25840_VIN8_CH3: u32 = 0x80000040;
pub const CX25840_NONE0_CH3: u32 = 0x80000080;
pub const CX25840_NONE1_CH3: u32 = 0x800000c0;
pub const CX25840_SVIDEO_ON: u32 = 0x80000100;
pub const CX25840_COMPONENT_ON: u32 = 0x80000200;
pub const CX25840_DIF_ON: u32 = 0x80000400;

/* Video output configuration constants. */
pub const CX25840_VCONFIG_FMT_SHIFT: u32 = 0;
pub const CX25840_VCONFIG_FMT_MASK: u32 = 0x7;
pub const CX25840_VCONFIG_FMT_BT601: u32 = 1 << 0;
pub const CX25840_VCONFIG_FMT_BT656: u32 = 1 << 1;
pub const CX25840_VCONFIG_FMT_VIP11: u32 = 0x3;
pub const CX25840_VCONFIG_FMT_VIP2: u32 = 1 << 2;
pub const CX25840_VCONFIG_RES_SHIFT: u32 = 3;
pub const CX25840_VCONFIG_RES_MASK: u32 = 0x18;
pub const CX25840_VCONFIG_RES_8BIT: u32 = 1 << 3;
pub const CX25840_VCONFIG_RES_10BIT: u32 = 1 << 4;
pub const CX25840_VCONFIG_VBIRAW_SHIFT: u32 = 5;
pub const CX25840_VCONFIG_VBIRAW_MASK: u32 = 0x60;
pub const CX25840_VCONFIG_VBIRAW_DISABLED: u32 = 1 << 5;
pub const CX25840_VCONFIG_VBIRAW_ENABLED: u32 = 1 << 6;
pub const CX25840_VCONFIG_ANCDATA_SHIFT: u32 = 7;
pub const CX25840_VCONFIG_ANCDATA_MASK: u32 = 0x180;
pub const CX25840_VCONFIG_ANCDATA_DISABLED: u32 = 1 << 7;
pub const CX25840_VCONFIG_ANCDATA_ENABLED: u32 = 1 << 8;
pub const CX25840_VCONFIG_TASKBIT_SHIFT: u32 = 9;
pub const CX25840_VCONFIG_TASKBIT_MASK: u32 = 0x600;
pub const CX25840_VCONFIG_TASKBIT_ZERO: u32 = 1 << 9;
pub const CX25840_VCONFIG_TASKBIT_ONE: u32 = 1 << 10;
pub const CX25840_VCONFIG_ACTIVE_SHIFT: u32 = 11;
pub const CX25840_VCONFIG_ACTIVE_MASK: u32 = 0x1800;
pub const CX25840_VCONFIG_ACTIVE_COMPOSITE: u32 = 1 << 11;
pub const CX25840_VCONFIG_ACTIVE_HORIZONTAL: u32 = 1 << 12;
pub const CX25840_VCONFIG_VALID_SHIFT: u32 = 13;
pub const CX25840_VCONFIG_VALID_MASK: u32 = 0x6000;
pub const CX25840_VCONFIG_VALID_NORMAL: u32 = 1 << 13;
pub const CX25840_VCONFIG_VALID_ANDACTIVE: u32 = 1 << 14;
pub const CX25840_VCONFIG_HRESETW_SHIFT: u32 = 15;
pub const CX25840_VCONFIG_HRESETW_MASK: u32 = 0x18000;
pub const CX25840_VCONFIG_HRESETW_NORMAL: u32 = 1 << 15;
pub const CX25840_VCONFIG_HRESETW_PIXCLK: u32 = 1 << 16;
pub const CX25840_VCONFIG_CLKGATE_SHIFT: u32 = 17;
pub const CX25840_VCONFIG_CLKGATE_MASK: u32 = 0x60000;
pub const CX25840_VCONFIG_CLKGATE_NONE: u32 = 1 << 17;
pub const CX25840_VCONFIG_CLKGATE_VALID: u32 = 1 << 18;
pub const CX25840_VCONFIG_CLKGATE_VALIDACTIVE: u32 = 0x60000;
pub const CX25840_VCONFIG_DCMODE_SHIFT: u32 = 19;
pub const CX25840_VCONFIG_DCMODE_MASK: u32 = 0x180000;
pub const CX25840_VCONFIG_DCMODE_DWORDS: u32 = 1 << 19;
pub const CX25840_VCONFIG_DCMODE_BYTES: u32 = 1 << 20;
pub const CX25840_VCONFIG_IDID0S_SHIFT: u32 = 21;
pub const CX25840_VCONFIG_IDID0S_MASK: u32 = 0x600000;
pub const CX25840_VCONFIG_IDID0S_NORMAL: u32 = 1 << 21;
pub const CX25840_VCONFIG_IDID0S_LINECNT: u32 = 1 << 22;
pub const CX25840_VCONFIG_VIPCLAMP_SHIFT: u32 = 23;
pub const CX25840_VCONFIG_VIPCLAMP_MASK: u32 = 0x1800000;
pub const CX25840_VCONFIG_VIPCLAMP_ENABLED: u32 = 1 << 23;
pub const CX25840_VCONFIG_VIPCLAMP_DISABLED: u32 = 1 << 24;

#[repr(i32)]
pub enum cx25840_audio_input {
    CX25840_AUDIO_SERIAL,
    CX25840_AUDIO4 = 4,
    CX25840_AUDIO5,
    CX25840_AUDIO6,
    CX25840_AUDIO7,
    CX25840_AUDIO8,
}

#[repr(i32)]
pub enum cx25840_io_pin {
    CX25840_PIN_DVALID_PRGM0 = 0,
    CX25840_PIN_FIELD_PRGM1,
    CX25840_PIN_HRESET_PRGM2,
    CX25840_PIN_VRESET_HCTL_PRGM3,
    CX25840_PIN_IRQ_N_PRGM4,
    CX25840_PIN_IR_TX_PRGM6,
    CX25840_PIN_IR_RX_PRGM5,
    CX25840_PIN_GPIO0_PRGM8,
    CX25840_PIN_GPIO1_PRGM9,
    CX25840_PIN_SA_SDIN,
    CX25840_PIN_SA_SDOUT,
    CX25840_PIN_PLL_CLK_PRGM7,
    CX25840_PIN_CHIP_SEL_VIPCLK,
}

#[repr(i32)]
pub enum cx25840_io_pad {
    CX25840_PAD_DEFAULT = 0,
    CX25840_PAD_ACTIVE,
    CX25840_PAD_VACTIVE,
    CX25840_PAD_CBFLAG,
    CX25840_PAD_VID_DATA_EXT0,
    CX25840_PAD_VID_DATA_EXT1,
    CX25840_PAD_GPO0,
    CX25840_PAD_GPO1,
    CX25840_PAD_GPO2,
    CX25840_PAD_GPO3,
    CX25840_PAD_IRQ_N,
    CX25840_PAD_AC_SYNC,
    CX25840_PAD_AC_SDOUT,
    CX25840_PAD_PLL_CLK,
    CX25840_PAD_VRESET,
    CX25840_PAD_RESERVED,
    CX25840_PAD_XTI_X5_DLL,
    CX25840_PAD_AUX_PLL,
    CX25840_PAD_VID_PLL,
    CX25840_PAD_XTI,
    CX25840_PAD_GPI0,
    CX25840_PAD_GPI1,
    CX25840_PAD_GPI2,
    CX25840_PAD_GPI3,
}

#[repr(i32)]
pub enum cx25840_io_pin_strength {
    CX25840_PIN_DRIVE_MEDIUM = 0,
    CX25840_PIN_DRIVE_SLOW,
    CX25840_PIN_DRIVE_FAST,
}

#[repr(i32)]
pub enum cx23885_io_pin {
    CX23885_PIN_IR_RX_GPIO19,
    CX23885_PIN_IR_TX_GPIO20,
    CX23885_PIN_I2S_SDAT_GPIO21,
    CX23885_PIN_I2S_WCLK_GPIO22,
    CX23885_PIN_I2S_BCLK_GPIO23,
    CX23885_PIN_IRQ_N_GPIO16,
}

#[repr(i32)]
pub enum cx23885_io_pad {
    CX23885_PAD_IR_RX,
    CX23885_PAD_GPIO19,
    CX23885_PAD_IR_TX,
    CX23885_PAD_GPIO20,
    CX23885_PAD_I2S_SDAT,
    CX23885_PAD_GPIO21,
    CX23885_PAD_I2S_WCLK,
    CX23885_PAD_GPIO22,
    CX23885_PAD_I2S_BCLK,
    CX23885_PAD_GPIO23,
    CX23885_PAD_IRQ_N,
    CX23885_PAD_GPIO16,
}

#[repr(C)]
pub struct cx25840_platform_data {
    pub pvr150_workaround: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
