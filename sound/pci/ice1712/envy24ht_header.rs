/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24)
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

/* C header dependencies:
 * <sound/control.h>
 * <sound/ac97_codec.h>
 * <sound/rawmidi.h>
 * <sound/i2c.h>
 * <sound/pcm.h>
 * "ice1712.h"
 */

pub const ICE_EEP2_SYSCONF: core::ffi::c_int = 0; /* 06 */
pub const ICE_EEP2_ACLINK: core::ffi::c_int = 1; /* 07 */
pub const ICE_EEP2_I2S: core::ffi::c_int = 2; /* 08 */
pub const ICE_EEP2_SPDIF: core::ffi::c_int = 3; /* 09 */
pub const ICE_EEP2_GPIO_DIR: core::ffi::c_int = 4; /* 0a */
pub const ICE_EEP2_GPIO_DIR1: core::ffi::c_int = 5; /* 0b */
pub const ICE_EEP2_GPIO_DIR2: core::ffi::c_int = 6; /* 0c */
pub const ICE_EEP2_GPIO_MASK: core::ffi::c_int = 7; /* 0d */
pub const ICE_EEP2_GPIO_MASK1: core::ffi::c_int = 8; /* 0e */
pub const ICE_EEP2_GPIO_MASK2: core::ffi::c_int = 9; /* 0f */
pub const ICE_EEP2_GPIO_STATE: core::ffi::c_int = 10; /* 10 */
pub const ICE_EEP2_GPIO_STATE1: core::ffi::c_int = 11; /* 11 */
pub const ICE_EEP2_GPIO_STATE2: core::ffi::c_int = 12; /* 12 */

/*
 *  Direct registers
 */

macro_rules! ICEREG1724 {
    ($ice:expr, CONTROL) => { (*$ice).port + VT1724_REG_CONTROL };
    ($ice:expr, IRQMASK) => { (*$ice).port + VT1724_REG_IRQMASK };
    ($ice:expr, IRQSTAT) => { (*$ice).port + VT1724_REG_IRQSTAT };
    ($ice:expr, SYS_CFG) => { (*$ice).port + VT1724_REG_SYS_CFG };
    ($ice:expr, AC97_CFG) => { (*$ice).port + VT1724_REG_AC97_CFG };
    ($ice:expr, I2S_FEATURES) => { (*$ice).port + VT1724_REG_I2S_FEATURES };
    ($ice:expr, SPDIF_CFG) => { (*$ice).port + VT1724_REG_SPDIF_CFG };
    ($ice:expr, MPU_TXFIFO) => { (*$ice).port + VT1724_REG_MPU_TXFIFO };
    ($ice:expr, MPU_RXFIFO) => { (*$ice).port + VT1724_REG_MPU_RXFIFO };
    ($ice:expr, MPU_DATA) => { (*$ice).port + VT1724_REG_MPU_DATA };
    ($ice:expr, MPU_CTRL) => { (*$ice).port + VT1724_REG_MPU_CTRL };
    ($ice:expr, MPU_FIFO_WM) => { (*$ice).port + VT1724_REG_MPU_FIFO_WM };
    ($ice:expr, I2C_DEV_ADDR) => { (*$ice).port + VT1724_REG_I2C_DEV_ADDR };
    ($ice:expr, I2C_BYTE_ADDR) => { (*$ice).port + VT1724_REG_I2C_BYTE_ADDR };
    ($ice:expr, I2C_DATA) => { (*$ice).port + VT1724_REG_I2C_DATA };
    ($ice:expr, I2C_CTRL) => { (*$ice).port + VT1724_REG_I2C_CTRL };
    ($ice:expr, GPIO_DATA) => { (*$ice).port + VT1724_REG_GPIO_DATA };
    ($ice:expr, GPIO_WRITE_MASK) => { (*$ice).port + VT1724_REG_GPIO_WRITE_MASK };
    ($ice:expr, GPIO_DIRECTION) => { (*$ice).port + VT1724_REG_GPIO_DIRECTION };
    ($ice:expr, POWERDOWN) => { (*$ice).port + VT1724_REG_POWERDOWN };
    ($ice:expr, GPIO_DATA_22) => { (*$ice).port + VT1724_REG_GPIO_DATA_22 };
    ($ice:expr, GPIO_WRITE_MASK_22) => { (*$ice).port + VT1724_REG_GPIO_WRITE_MASK_22 };
}

pub(crate) use ICEREG1724;

pub const VT1724_REG_CONTROL: core::ffi::c_int = 0x00; /* byte */
pub const VT1724_RESET: core::ffi::c_int = 0x80; /* reset whole chip */
pub const VT1724_REG_IRQMASK: core::ffi::c_int = 0x01; /* byte */
pub const VT1724_IRQ_MPU_RX: core::ffi::c_int = 0x80;
pub const VT1724_IRQ_MPU_TX: core::ffi::c_int = 0x20;
pub const VT1724_IRQ_MTPCM: core::ffi::c_int = 0x10;
pub const VT1724_REG_IRQSTAT: core::ffi::c_int = 0x02; /* byte */
/* look to VT1724_IRQ_* */
pub const VT1724_REG_SYS_CFG: core::ffi::c_int = 0x04; /* byte - system configuration PCI60 on Envy24*/
pub const VT1724_CFG_CLOCK: core::ffi::c_int = 0xc0;
pub const VT1724_CFG_CLOCK512: core::ffi::c_int = 0x00; /* 22.5692Mhz, 44.1kHz*512 */
pub const VT1724_CFG_CLOCK384: core::ffi::c_int = 0x40; /* 16.9344Mhz, 44.1kHz*384 */
pub const VT1724_CFG_MPU401: core::ffi::c_int = 0x20; /* MPU401 UARTs */
pub const VT1724_CFG_ADC_MASK: core::ffi::c_int = 0x0c; /* one, two or one and S/PDIF, stereo ADCs */
pub const VT1724_CFG_ADC_NONE: core::ffi::c_int = 0x0c; /* no ADCs */
pub const VT1724_CFG_DAC_MASK: core::ffi::c_int = 0x03; /* one, two, three, four stereo DACs */

pub const VT1724_REG_AC97_CFG: core::ffi::c_int = 0x05; /* byte */
pub const VT1724_CFG_PRO_I2S: core::ffi::c_int = 0x80; /* multitrack converter: I2S or AC'97 */
pub const VT1724_CFG_AC97_PACKED: core::ffi::c_int = 0x01; /* split or packed mode - AC'97 */

pub const VT1724_REG_I2S_FEATURES: core::ffi::c_int = 0x06; /* byte */
pub const VT1724_CFG_I2S_VOLUME: core::ffi::c_int = 0x80; /* volume/mute capability */
pub const VT1724_CFG_I2S_96KHZ: core::ffi::c_int = 0x40; /* supports 96kHz sampling */
pub const VT1724_CFG_I2S_RESMASK: core::ffi::c_int = 0x30; /* resolution mask, 16,18,20,24-bit */
pub const VT1724_CFG_I2S_192KHZ: core::ffi::c_int = 0x08; /* supports 192kHz sampling */
pub const VT1724_CFG_I2S_OTHER: core::ffi::c_int = 0x07; /* other I2S IDs */

pub const VT1724_REG_SPDIF_CFG: core::ffi::c_int = 0x07; /* byte */
pub const VT1724_CFG_SPDIF_OUT_EN: core::ffi::c_int = 0x80; /*Internal S/PDIF output is enabled*/
pub const VT1724_CFG_SPDIF_OUT_INT: core::ffi::c_int = 0x40; /*Internal S/PDIF output is implemented*/
pub const VT1724_CFG_I2S_CHIPID: core::ffi::c_int = 0x3c; /* I2S chip ID */
pub const VT1724_CFG_SPDIF_IN: core::ffi::c_int = 0x02; /* S/PDIF input is present */
pub const VT1724_CFG_SPDIF_OUT: core::ffi::c_int = 0x01; /* External S/PDIF output is present */

/*there is no consumer AC97 codec with the VT1724*/
/* #define VT1724_REG_AC97_INDEX		0x08 */ /* byte */
/* #define VT1724_REG_AC97_CMD		0x09 */ /* byte */

pub const VT1724_REG_MPU_TXFIFO: core::ffi::c_int = 0x0a; /*byte ro. number of bytes in TX fifo*/
pub const VT1724_REG_MPU_RXFIFO: core::ffi::c_int = 0x0b; /*byte ro. number of bytes in RX fifo*/

pub const VT1724_REG_MPU_DATA: core::ffi::c_int = 0x0c; /* byte */
pub const VT1724_REG_MPU_CTRL: core::ffi::c_int = 0x0d; /* byte */
pub const VT1724_MPU_UART: core::ffi::c_int = 0x01;
pub const VT1724_MPU_TX_EMPTY: core::ffi::c_int = 0x02;
pub const VT1724_MPU_TX_FULL: core::ffi::c_int = 0x04;
pub const VT1724_MPU_RX_EMPTY: core::ffi::c_int = 0x08;
pub const VT1724_MPU_RX_FULL: core::ffi::c_int = 0x10;

pub const VT1724_REG_MPU_FIFO_WM: core::ffi::c_int = 0x0e; /*byte set the high/low watermarks for RX/TX fifos*/
pub const VT1724_MPU_RX_FIFO: core::ffi::c_int = 0x20; //1=rx fifo watermark 0=tx fifo watermark
pub const VT1724_MPU_FIFO_MASK: core::ffi::c_int = 0x1f;

pub const VT1724_REG_I2C_DEV_ADDR: core::ffi::c_int = 0x10; /* byte */
pub const VT1724_I2C_WRITE: core::ffi::c_int = 0x01; /* write direction */
pub const VT1724_REG_I2C_BYTE_ADDR: core::ffi::c_int = 0x11; /* byte */
pub const VT1724_REG_I2C_DATA: core::ffi::c_int = 0x12; /* byte */
pub const VT1724_REG_I2C_CTRL: core::ffi::c_int = 0x13; /* byte */
pub const VT1724_I2C_EEPROM: core::ffi::c_int = 0x80; /* 1 = EEPROM exists */
pub const VT1724_I2C_BUSY: core::ffi::c_int = 0x01; /* busy bit */

pub const VT1724_REG_GPIO_DATA: core::ffi::c_int = 0x14; /* word */
pub const VT1724_REG_GPIO_WRITE_MASK: core::ffi::c_int = 0x16; /* word */
pub const VT1724_REG_GPIO_DIRECTION: core::ffi::c_int = 0x18; /* dword? (3 bytes) 0=input 1=output.
                                                              * bit3 - during reset used for Eeprom power-on strapping
                                                              * if TESTEN# pin active, bit 2 always input
                                                              */
pub const VT1724_REG_POWERDOWN: core::ffi::c_int = 0x1c;
pub const VT1724_REG_GPIO_DATA_22: core::ffi::c_int = 0x1e; /* byte direction for GPIO 16:22 */
pub const VT1724_REG_GPIO_WRITE_MASK_22: core::ffi::c_int = 0x1f; /* byte write mask for GPIO 16:22 */

/*
 *  Professional multi-track direct control registers
 */

macro_rules! ICEMT1724 {
    ($ice:expr, IRQ) => { (*$ice).profi_port + VT1724_MT_IRQ };
    ($ice:expr, RATE) => { (*$ice).profi_port + VT1724_MT_RATE };
    ($ice:expr, I2S_FORMAT) => { (*$ice).profi_port + VT1724_MT_I2S_FORMAT };
    ($ice:expr, DMA_INT_MASK) => { (*$ice).profi_port + VT1724_MT_DMA_INT_MASK };
    ($ice:expr, AC97_INDEX) => { (*$ice).profi_port + VT1724_MT_AC97_INDEX };
    ($ice:expr, AC97_CMD) => { (*$ice).profi_port + VT1724_MT_AC97_CMD };
    ($ice:expr, AC97_DATA) => { (*$ice).profi_port + VT1724_MT_AC97_DATA };
    ($ice:expr, PLAYBACK_ADDR) => { (*$ice).profi_port + VT1724_MT_PLAYBACK_ADDR };
    ($ice:expr, PLAYBACK_SIZE) => { (*$ice).profi_port + VT1724_MT_PLAYBACK_SIZE };
    ($ice:expr, DMA_CONTROL) => { (*$ice).profi_port + VT1724_MT_DMA_CONTROL };
    ($ice:expr, BURST) => { (*$ice).profi_port + VT1724_MT_BURST };
    ($ice:expr, DMA_FIFO_ERR) => { (*$ice).profi_port + VT1724_MT_DMA_FIFO_ERR };
    ($ice:expr, DMA_PAUSE) => { (*$ice).profi_port + VT1724_MT_DMA_PAUSE };
    ($ice:expr, PLAYBACK_COUNT) => { (*$ice).profi_port + VT1724_MT_PLAYBACK_COUNT };
    ($ice:expr, CAPTURE_ADDR) => { (*$ice).profi_port + VT1724_MT_CAPTURE_ADDR };
    ($ice:expr, CAPTURE_SIZE) => { (*$ice).profi_port + VT1724_MT_CAPTURE_SIZE };
    ($ice:expr, CAPTURE_COUNT) => { (*$ice).profi_port + VT1724_MT_CAPTURE_COUNT };
    ($ice:expr, ROUTE_PLAYBACK) => { (*$ice).profi_port + VT1724_MT_ROUTE_PLAYBACK };
    ($ice:expr, RDMA1_ADDR) => { (*$ice).profi_port + VT1724_MT_RDMA1_ADDR };
    ($ice:expr, RDMA1_SIZE) => { (*$ice).profi_port + VT1724_MT_RDMA1_SIZE };
    ($ice:expr, RDMA1_COUNT) => { (*$ice).profi_port + VT1724_MT_RDMA1_COUNT };
    ($ice:expr, SPDIF_CTRL) => { (*$ice).profi_port + VT1724_MT_SPDIF_CTRL };
    ($ice:expr, MONITOR_PEAKINDEX) => { (*$ice).profi_port + VT1724_MT_MONITOR_PEAKINDEX };
    ($ice:expr, MONITOR_PEAKDATA) => { (*$ice).profi_port + VT1724_MT_MONITOR_PEAKDATA };
    ($ice:expr, PDMA4_ADDR) => { (*$ice).profi_port + VT1724_MT_PDMA4_ADDR };
    ($ice:expr, PDMA4_SIZE) => { (*$ice).profi_port + VT1724_MT_PDMA4_SIZE };
    ($ice:expr, PDMA4_COUNT) => { (*$ice).profi_port + VT1724_MT_PDMA4_COUNT };
    ($ice:expr, PDMA3_ADDR) => { (*$ice).profi_port + VT1724_MT_PDMA3_ADDR };
    ($ice:expr, PDMA3_SIZE) => { (*$ice).profi_port + VT1724_MT_PDMA3_SIZE };
    ($ice:expr, PDMA3_COUNT) => { (*$ice).profi_port + VT1724_MT_PDMA3_COUNT };
    ($ice:expr, PDMA2_ADDR) => { (*$ice).profi_port + VT1724_MT_PDMA2_ADDR };
    ($ice:expr, PDMA2_SIZE) => { (*$ice).profi_port + VT1724_MT_PDMA2_SIZE };
    ($ice:expr, PDMA2_COUNT) => { (*$ice).profi_port + VT1724_MT_PDMA2_COUNT };
    ($ice:expr, PDMA1_ADDR) => { (*$ice).profi_port + VT1724_MT_PDMA1_ADDR };
    ($ice:expr, PDMA1_SIZE) => { (*$ice).profi_port + VT1724_MT_PDMA1_SIZE };
    ($ice:expr, PDMA1_COUNT) => { (*$ice).profi_port + VT1724_MT_PDMA1_COUNT };
}

pub(crate) use ICEMT1724;

pub const VT1724_MT_IRQ: core::ffi::c_int = 0x00; /* byte - interrupt mask */
pub const VT1724_MULTI_PDMA4: core::ffi::c_int = 0x80; /* SPDIF Out / PDMA4 */
pub const VT1724_MULTI_PDMA3: core::ffi::c_int = 0x40; /* PDMA3 */
pub const VT1724_MULTI_PDMA2: core::ffi::c_int = 0x20; /* PDMA2 */
pub const VT1724_MULTI_PDMA1: core::ffi::c_int = 0x10; /* PDMA1 */
pub const VT1724_MULTI_FIFO_ERR: core::ffi::c_int = 0x08; /* DMA FIFO underrun/overrun. */
pub const VT1724_MULTI_RDMA1: core::ffi::c_int = 0x04; /* RDMA1 (S/PDIF input) */
pub const VT1724_MULTI_RDMA0: core::ffi::c_int = 0x02; /* RMDA0 */
pub const VT1724_MULTI_PDMA0: core::ffi::c_int = 0x01; /* MC Interleave/PDMA0 */

pub const VT1724_MT_RATE: core::ffi::c_int = 0x01; /* byte - sampling rate select */
pub const VT1724_SPDIF_MASTER: core::ffi::c_int = 0x10; /* S/PDIF input is master clock */
pub const VT1724_MT_I2S_FORMAT: core::ffi::c_int = 0x02; /* byte - I2S data format */
pub const VT1724_MT_I2S_MCLK_128X: core::ffi::c_int = 0x08;
pub const VT1724_MT_I2S_FORMAT_MASK: core::ffi::c_int = 0x03;
pub const VT1724_MT_I2S_FORMAT_I2S: core::ffi::c_int = 0x00;
pub const VT1724_MT_DMA_INT_MASK: core::ffi::c_int = 0x03; /* byte -DMA Interrupt Mask */
/* lool to VT1724_MULTI_* */
pub const VT1724_MT_AC97_INDEX: core::ffi::c_int = 0x04; /* byte - AC'97 index */
pub const VT1724_MT_AC97_CMD: core::ffi::c_int = 0x05; /* byte - AC'97 command & status */
pub const VT1724_AC97_COLD: core::ffi::c_int = 0x80; /* cold reset */
pub const VT1724_AC97_WARM: core::ffi::c_int = 0x40; /* warm reset */
pub const VT1724_AC97_WRITE: core::ffi::c_int = 0x20; /* W: write, R: write in progress */
pub const VT1724_AC97_READ: core::ffi::c_int = 0x10; /* W: read, R: read in progress */
pub const VT1724_AC97_READY: core::ffi::c_int = 0x08; /* codec ready status bit */
pub const VT1724_AC97_ID_MASK: core::ffi::c_int = 0x03; /* codec id mask */
pub const VT1724_MT_AC97_DATA: core::ffi::c_int = 0x06; /* word - AC'97 data */
pub const VT1724_MT_PLAYBACK_ADDR: core::ffi::c_int = 0x10; /* dword - playback address */
pub const VT1724_MT_PLAYBACK_SIZE: core::ffi::c_int = 0x14; /* dword - playback size */
pub const VT1724_MT_DMA_CONTROL: core::ffi::c_int = 0x18; /* byte - control */
pub const VT1724_PDMA4_START: core::ffi::c_int = 0x80; /* SPDIF out / PDMA4 start */
pub const VT1724_PDMA3_START: core::ffi::c_int = 0x40; /* PDMA3 start */
pub const VT1724_PDMA2_START: core::ffi::c_int = 0x20; /* PDMA2 start */
pub const VT1724_PDMA1_START: core::ffi::c_int = 0x10; /* PDMA1 start */
pub const VT1724_RDMA1_START: core::ffi::c_int = 0x04; /* RDMA1 start */
pub const VT1724_RDMA0_START: core::ffi::c_int = 0x02; /* RMDA0 start */
pub const VT1724_PDMA0_START: core::ffi::c_int = 0x01; /* MC Interleave / PDMA0 start */
pub const VT1724_MT_BURST: core::ffi::c_int = 0x19; /* Interleaved playback DMA Active streams / PCI burst size */
pub const VT1724_MT_DMA_FIFO_ERR: core::ffi::c_int = 0x1a; /*Global playback and record DMA FIFO Underrun/Overrun */
pub const VT1724_PDMA4_UNDERRUN: core::ffi::c_int = 0x80;
pub const VT1724_PDMA2_UNDERRUN: core::ffi::c_int = 0x40;
pub const VT1724_PDMA3_UNDERRUN: core::ffi::c_int = 0x20;
pub const VT1724_PDMA1_UNDERRUN: core::ffi::c_int = 0x10;
pub const VT1724_RDMA1_UNDERRUN: core::ffi::c_int = 0x04;
pub const VT1724_RDMA0_UNDERRUN: core::ffi::c_int = 0x02;
pub const VT1724_PDMA0_UNDERRUN: core::ffi::c_int = 0x01;
pub const VT1724_MT_DMA_PAUSE: core::ffi::c_int = 0x1b; /*Global playback and record DMA FIFO pause/resume */
pub const VT1724_PDMA4_PAUSE: core::ffi::c_int = 0x80;
pub const VT1724_PDMA3_PAUSE: core::ffi::c_int = 0x40;
pub const VT1724_PDMA2_PAUSE: core::ffi::c_int = 0x20;
pub const VT1724_PDMA1_PAUSE: core::ffi::c_int = 0x10;
pub const VT1724_RDMA1_PAUSE: core::ffi::c_int = 0x04;
pub const VT1724_RDMA0_PAUSE: core::ffi::c_int = 0x02;
pub const VT1724_PDMA0_PAUSE: core::ffi::c_int = 0x01;
pub const VT1724_MT_PLAYBACK_COUNT: core::ffi::c_int = 0x1c; /* word - playback count */
pub const VT1724_MT_CAPTURE_ADDR: core::ffi::c_int = 0x20; /* dword - capture address */
pub const VT1724_MT_CAPTURE_SIZE: core::ffi::c_int = 0x24; /* word - capture size */
pub const VT1724_MT_CAPTURE_COUNT: core::ffi::c_int = 0x26; /* word - capture count */

pub const VT1724_MT_ROUTE_PLAYBACK: core::ffi::c_int = 0x2c; /* word */

pub const VT1724_MT_RDMA1_ADDR: core::ffi::c_int = 0x30; /* dword - RDMA1 capture address */
pub const VT1724_MT_RDMA1_SIZE: core::ffi::c_int = 0x34; /* word - RDMA1 capture size */
pub const VT1724_MT_RDMA1_COUNT: core::ffi::c_int = 0x36; /* word - RDMA1 capture count */

pub const VT1724_MT_SPDIF_CTRL: core::ffi::c_int = 0x3c; /* word */
pub const VT1724_MT_MONITOR_PEAKINDEX: core::ffi::c_int = 0x3e; /* byte */
pub const VT1724_MT_MONITOR_PEAKDATA: core::ffi::c_int = 0x3f; /* byte */

/* concurrent stereo channels */
pub const VT1724_MT_PDMA4_ADDR: core::ffi::c_int = 0x40; /* dword */
pub const VT1724_MT_PDMA4_SIZE: core::ffi::c_int = 0x44; /* word */
pub const VT1724_MT_PDMA4_COUNT: core::ffi::c_int = 0x46; /* word */
pub const VT1724_MT_PDMA3_ADDR: core::ffi::c_int = 0x50; /* dword */
pub const VT1724_MT_PDMA3_SIZE: core::ffi::c_int = 0x54; /* word */
pub const VT1724_MT_PDMA3_COUNT: core::ffi::c_int = 0x56; /* word */
pub const VT1724_MT_PDMA2_ADDR: core::ffi::c_int = 0x60; /* dword */
pub const VT1724_MT_PDMA2_SIZE: core::ffi::c_int = 0x64; /* word */
pub const VT1724_MT_PDMA2_COUNT: core::ffi::c_int = 0x66; /* word */
pub const VT1724_MT_PDMA1_ADDR: core::ffi::c_int = 0x70; /* dword */
pub const VT1724_MT_PDMA1_SIZE: core::ffi::c_int = 0x74; /* word */
pub const VT1724_MT_PDMA1_COUNT: core::ffi::c_int = 0x76; /* word */

unsafe extern "C" {
    pub fn snd_vt1724_read_i2c(
        ice: *mut snd_ice1712,
        dev: core::ffi::c_uchar,
        addr: core::ffi::c_uchar,
    ) -> core::ffi::c_uchar;
    pub fn snd_vt1724_write_i2c(
        ice: *mut snd_ice1712,
        dev: core::ffi::c_uchar,
        addr: core::ffi::c_uchar,
        data: core::ffi::c_uchar,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
