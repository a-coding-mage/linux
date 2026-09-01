/* SPDX-License-Identifier: GPL-2.0-only */

/*
 *  Driver for HAL2 sound processors
 *  Copyright (c) 1999 Ulf Carlsson <ulfc@bun.falkenberg.se>
 *  Copyright (c) 2001, 2002, 2003 Ladislav Michl <ladis@linux-mips.org>
 */

/* Dependency intent from C header: #include <linux/types.h> */

/* Indirect status register */

pub const H2_ISR_TSTATUS: u32 = 0x01; /* RO: transaction status 1=busy */
pub const H2_ISR_USTATUS: u32 = 0x02; /* RO: utime status bit 1=armed */
pub const H2_ISR_QUAD_MODE: u32 = 0x04; /* codec mode 0=indigo 1=quad */
pub const H2_ISR_GLOBAL_RESET_N: u32 = 0x08; /* chip global reset 0=reset */
pub const H2_ISR_CODEC_RESET_N: u32 = 0x10; /* codec/synth reset 0=reset  */

/* Revision register */

pub const H2_REV_AUDIO_PRESENT: u32 = 0x8000; /* RO: audio present 0=present */
pub const H2_REV_BOARD_M: u32 = 0x7000; /* RO: bits 14:12, board revision */
pub const H2_REV_MAJOR_CHIP_M: u32 = 0x00F0; /* RO: bits 7:4, major chip revision */
pub const H2_REV_MINOR_CHIP_M: u32 = 0x000F; /* RO: bits 3:0, minor chip revision */

/* Indirect address register */

/*
 * Address of indirect internal register to be accessed. A write to this
 * register initiates read or write access to the indirect registers in the
 * HAL2. Note that there af four indirect data registers for write access to
 * registers larger than 16 byte.
 */

pub const H2_IAR_TYPE_M: u32 = 0xF000; /* bits 15:12, type of functional */
                                      /* block the register resides in */
                                      /* 1=DMA Port */
                                      /* 9=Global DMA Control */
                                      /* 2=Bresenham */
                                      /* 3=Unix Timer */
pub const H2_IAR_NUM_M: u32 = 0x0F00; /* bits 11:8 instance of the */
                                     /* blockin which the indirect */
                                     /* register resides */
                                     /* If IAR_TYPE_M=DMA Port: */
                                     /* 1=Synth In */
                                     /* 2=AES In */
                                     /* 3=AES Out */
                                     /* 4=DAC Out */
                                     /* 5=ADC Out */
                                     /* 6=Synth Control */
                                     /* If IAR_TYPE_M=Global DMA Control: */
                                     /* 1=Control */
                                     /* If IAR_TYPE_M=Bresenham: */
                                     /* 1=Bresenham Clock Gen 1 */
                                     /* 2=Bresenham Clock Gen 2 */
                                     /* 3=Bresenham Clock Gen 3 */
                                     /* If IAR_TYPE_M=Unix Timer: */
                                     /* 1=Unix Timer */
pub const H2_IAR_ACCESS_SELECT: u32 = 0x0080; /* 1=read 0=write */
pub const H2_IAR_PARAM: u32 = 0x000C; /* Parameter Select */
pub const H2_IAR_RB_INDEX_M: u32 = 0x0003; /* Read Back Index */
                                         /* 00:word0 */
                                         /* 01:word1 */
                                         /* 10:word2 */
                                         /* 11:word3 */
/*
 * HAL2 internal addressing
 *
 * The HAL2 has "indirect registers" (idr) which are accessed by writing to the
 * Indirect Data registers. Write the address to the Indirect Address register
 * to transfer the data.
 *
 * We define the H2IR_* to the read address and H2IW_* to the write address and
 * H2I_* to be fields in whatever register is referred to.
 *
 * When we write to indirect registers which are larger than one word (16 bit)
 * we have to fill more than one indirect register before writing. When we read
 * back however we have to read several times, each time with different Read
 * Back Indexes (there are defs for doing this easily).
 */

/*
 * Relay Control
 */
pub const H2I_RELAY_C: u32 = 0x9100;
pub const H2I_RELAY_C_STATE: u32 = 0x01; /* state of RELAY pin signal */

/* DMA port enable */

pub const H2I_DMA_PORT_EN: u32 = 0x9104;
pub const H2I_DMA_PORT_EN_SY_IN: u32 = 0x01; /* Synth_in DMA port */
pub const H2I_DMA_PORT_EN_AESRX: u32 = 0x02; /* AES receiver DMA port */
pub const H2I_DMA_PORT_EN_AESTX: u32 = 0x04; /* AES transmitter DMA port */
pub const H2I_DMA_PORT_EN_CODECTX: u32 = 0x08; /* CODEC transmit DMA port */
pub const H2I_DMA_PORT_EN_CODECR: u32 = 0x10; /* CODEC receive DMA port */

pub const H2I_DMA_END: u32 = 0x9108; /* global dma endian select */
pub const H2I_DMA_END_SY_IN: u32 = 0x01; /* Synth_in DMA port */
pub const H2I_DMA_END_AESRX: u32 = 0x02; /* AES receiver DMA port */
pub const H2I_DMA_END_AESTX: u32 = 0x04; /* AES transmitter DMA port */
pub const H2I_DMA_END_CODECTX: u32 = 0x08; /* CODEC transmit DMA port */
pub const H2I_DMA_END_CODECR: u32 = 0x10; /* CODEC receive DMA port */
                                      /* 0=b_end 1=l_end */

pub const H2I_DMA_DRV: u32 = 0x910C; /* global PBUS DMA enable */

pub const H2I_SYNTH_C: u32 = 0x1104; /* Synth DMA control */

pub const H2I_AESRX_C: u32 = 0x1204; /* AES RX dma control */

pub const H2I_C_TS_EN: u32 = 0x20; /* Timestamp enable */
pub const H2I_C_TS_FRMT: u32 = 0x40; /* Timestamp format */
pub const H2I_C_NAUDIO: u32 = 0x80; /* Sign extend */

/* AESRX CTL, 16 bit */

pub const H2I_AESTX_C: u32 = 0x1304; /* AES TX DMA control */
pub const H2I_AESTX_C_CLKID_SHIFT: u32 = 3; /* Bresenham Clock Gen 1-3 */
pub const H2I_AESTX_C_CLKID_M: u32 = 0x18;
pub const H2I_AESTX_C_DATAT_SHIFT: u32 = 8; /* 1=mono 2=stereo (3=quad) */
pub const H2I_AESTX_C_DATAT_M: u32 = 0x300;

/* CODEC registers */

pub const H2I_DAC_C1: u32 = 0x1404; /* DAC DMA control, 16 bit */
pub const H2I_DAC_C2: u32 = 0x1408; /* DAC DMA control, 32 bit */
pub const H2I_ADC_C1: u32 = 0x1504; /* ADC DMA control, 16 bit */
pub const H2I_ADC_C2: u32 = 0x1508; /* ADC DMA control, 32 bit */

/* Bits in CTL1 register */

pub const H2I_C1_DMA_SHIFT: u32 = 0; /* DMA channel */
pub const H2I_C1_DMA_M: u32 = 0x7;
pub const H2I_C1_CLKID_SHIFT: u32 = 3; /* Bresenham Clock Gen 1-3 */
pub const H2I_C1_CLKID_M: u32 = 0x18;
pub const H2I_C1_DATAT_SHIFT: u32 = 8; /* 1=mono 2=stereo (3=quad) */
pub const H2I_C1_DATAT_M: u32 = 0x300;

/* Bits in CTL2 register */

pub const H2I_C2_R_GAIN_SHIFT: u32 = 0; /* right a/d input gain */
pub const H2I_C2_R_GAIN_M: u32 = 0xf;
pub const H2I_C2_L_GAIN_SHIFT: u32 = 4; /* left a/d input gain */
pub const H2I_C2_L_GAIN_M: u32 = 0xf0;
pub const H2I_C2_R_SEL: u32 = 0x100; /* right input select */
pub const H2I_C2_L_SEL: u32 = 0x200; /* left input select */
pub const H2I_C2_MUTE: u32 = 0x400; /* mute */
pub const H2I_C2_DO1: u32 = 0x00010000; /* digital output port bit 0 */
pub const H2I_C2_DO2: u32 = 0x00020000; /* digital output port bit 1 */
pub const H2I_C2_R_ATT_SHIFT: u32 = 18; /* right d/a output - */
pub const H2I_C2_R_ATT_M: u32 = 0x007c0000; /* attenuation */
pub const H2I_C2_L_ATT_SHIFT: u32 = 23; /* left d/a output - */
pub const H2I_C2_L_ATT_M: u32 = 0x0f800000; /* attenuation */

pub const H2I_SYNTH_MAP_C: u32 = 0x1104; /* synth dma handshake ctrl */

/* Clock generator CTL 1, 16 bit */

pub const H2I_BRES1_C1: u32 = 0x2104;
pub const H2I_BRES2_C1: u32 = 0x2204;
pub const H2I_BRES3_C1: u32 = 0x2304;

pub const H2I_BRES_C1_SHIFT: u32 = 0; /* 0=48.0 1=44.1 2=aes_rx */
pub const H2I_BRES_C1_M: u32 = 0x03;

/* Clock generator CTL 2, 32 bit */

pub const H2I_BRES1_C2: u32 = 0x2108;
pub const H2I_BRES2_C2: u32 = 0x2208;
pub const H2I_BRES3_C2: u32 = 0x2308;

pub const H2I_BRES_C2_INC_SHIFT: u32 = 0; /* increment value */
pub const H2I_BRES_C2_INC_M: u32 = 0xffff;
pub const H2I_BRES_C2_MOD_SHIFT: u32 = 16; /* modcontrol value */
pub const H2I_BRES_C2_MOD_M: u32 = 0xffff0000; /* modctrl=0xffff&(modinc-1) */

/* Unix timer, 64 bit */

pub const H2I_UTIME: u32 = 0x3104;
pub const H2I_UTIME_0_LD: u32 = 0xffff; /* microseconds, LSB's */
pub const H2I_UTIME_1_LD0: u32 = 0x0f; /* microseconds, MSB's */
pub const H2I_UTIME_1_LD1: u32 = 0xf0; /* tenths of microseconds */
pub const H2I_UTIME_2_LD: u32 = 0xffff; /* seconds, LSB's */
pub const H2I_UTIME_3_LD: u32 = 0xffff; /* seconds, MSB's */

#[repr(C)]
pub struct hal2_ctl_regs {
    pub _unused0: [u32; 4],
    pub isr: u32, /* 0x10 Status Register */
    pub _unused1: [u32; 3],
    pub rev: u32, /* 0x20 Revision Register */
    pub _unused2: [u32; 3],
    pub iar: u32, /* 0x30 Indirect Address Register */
    pub _unused3: [u32; 3],
    pub idr0: u32, /* 0x40 Indirect Data Register 0 */
    pub _unused4: [u32; 3],
    pub idr1: u32, /* 0x50 Indirect Data Register 1 */
    pub _unused5: [u32; 3],
    pub idr2: u32, /* 0x60 Indirect Data Register 2 */
    pub _unused6: [u32; 3],
    pub idr3: u32, /* 0x70 Indirect Data Register 3 */
}

#[repr(C)]
pub struct hal2_aes_regs {
    pub rx_stat: [u32; 2], /* Status registers */
    pub rx_cr: [u32; 2],   /* Control registers */
    pub rx_ud: [u32; 4],   /* User data window */
    pub rx_st: [u32; 24],  /* Channel status data */

    pub tx_stat: [u32; 1], /* Status register */
    pub tx_cr: [u32; 3],   /* Control registers */
    pub tx_ud: [u32; 4],   /* User data window */
    pub tx_st: [u32; 24],  /* Channel status data */
}

#[repr(C)]
pub struct hal2_vol_regs {
    pub right: u32, /* Right volume */
    pub left: u32,  /* Left volume */
}

#[repr(C)]
pub struct hal2_syn_regs {
    pub _unused0: [u32; 2],
    pub page: u32,   /* DOC Page register */
    pub regsel: u32, /* DOC Register selection */
    pub dlow: u32,   /* DOC Data low */
    pub dhigh: u32,  /* DOC Data high */
    pub irq: u32,    /* IRQ Status */
    pub dram: u32,   /* DRAM Access */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
