/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Includes from the C header are external dependencies:
 * linux/io.h, sound/control.h, sound/ac97_codec.h, sound/rawmidi.h,
 * sound/i2c.h, sound/ak4xxx-adda.h, sound/ak4114.h, sound/pt2258.h,
 * sound/pcm.h, sound/mpu401.h.
 */

pub type u8 = u8;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_rawmidi {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_akm4xxx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_i2c_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_i2c_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut c_void) -> c_int;
}

/*
 *  Direct registers
 */

pub const ICE1712_REG_CONTROL: c_ulong = 0x00; /* byte */
pub const ICE1712_RESET: c_uint = 0x80; /* soft reset whole chip */
pub const ICE1712_SERR_ASSERT_DS_DMA: c_uint = 0x40; /* disabled SERR# assertion for the DS DMA Ch-C irq otherwise enabled */
pub const ICE1712_DOS_VOL: c_uint = 0x10; /* DOS WT/FM volume control */
pub const ICE1712_SERR_LEVEL: c_uint = 0x08; /* SERR# level otherwise edge */
pub const ICE1712_SERR_ASSERT_SB: c_uint = 0x02; /* disabled SERR# assertion for SB irq otherwise enabled */
pub const ICE1712_NATIVE: c_uint = 0x01; /* native mode otherwise SB */
pub const ICE1712_REG_IRQMASK: c_ulong = 0x01; /* byte */
pub const ICE1712_IRQ_MPU1: c_uint = 0x80; /* MIDI irq mask */
pub const ICE1712_IRQ_TIMER: c_uint = 0x40; /* Timer mask */
pub const ICE1712_IRQ_MPU2: c_uint = 0x20; /* Secondary MIDI irq mask */
pub const ICE1712_IRQ_PROPCM: c_uint = 0x10; /* professional multi-track */
pub const ICE1712_IRQ_FM: c_uint = 0x08; /* FM/MIDI - legacy */
pub const ICE1712_IRQ_PBKDS: c_uint = 0x04; /* playback DS channels */
pub const ICE1712_IRQ_CONCAP: c_uint = 0x02; /* consumer capture */
pub const ICE1712_IRQ_CONPBK: c_uint = 0x01; /* consumer playback */
pub const ICE1712_REG_IRQSTAT: c_ulong = 0x02; /* byte */
/* look to ICE1712_IRQ_* */
pub const ICE1712_REG_INDEX: c_ulong = 0x03; /* byte - indirect CCIxx regs */
pub const ICE1712_REG_DATA: c_ulong = 0x04; /* byte - indirect CCIxx regs */
pub const ICE1712_REG_NMI_STAT1: c_ulong = 0x05; /* byte */
pub const ICE1712_REG_NMI_DATA: c_ulong = 0x06; /* byte */
pub const ICE1712_REG_NMI_INDEX: c_ulong = 0x07; /* byte */
pub const ICE1712_REG_AC97_INDEX: c_ulong = 0x08; /* byte */
pub const ICE1712_REG_AC97_CMD: c_ulong = 0x09; /* byte */
pub const ICE1712_AC97_COLD: c_uint = 0x80; /* cold reset */
pub const ICE1712_AC97_WARM: c_uint = 0x40; /* warm reset */
pub const ICE1712_AC97_WRITE: c_uint = 0x20; /* W: write, R: write in progress */
pub const ICE1712_AC97_READ: c_uint = 0x10; /* W: read, R: read in progress */
pub const ICE1712_AC97_READY: c_uint = 0x08; /* codec ready status bit */
pub const ICE1712_AC97_PBK_VSR: c_uint = 0x02; /* playback VSR */
pub const ICE1712_AC97_CAP_VSR: c_uint = 0x01; /* capture VSR */
pub const ICE1712_REG_AC97_DATA: c_ulong = 0x0a; /* word (little endian) */
pub const ICE1712_REG_MPU1_CTRL: c_ulong = 0x0c; /* byte */
pub const ICE1712_REG_MPU1_DATA: c_ulong = 0x0d; /* byte */
pub const ICE1712_REG_I2C_DEV_ADDR: c_ulong = 0x10; /* byte */
pub const ICE1712_I2C_WRITE: c_uint = 0x01; /* write direction */
pub const ICE1712_REG_I2C_BYTE_ADDR: c_ulong = 0x11; /* byte */
pub const ICE1712_REG_I2C_DATA: c_ulong = 0x12; /* byte */
pub const ICE1712_REG_I2C_CTRL: c_ulong = 0x13; /* byte */
pub const ICE1712_I2C_EEPROM: c_uint = 0x80; /* EEPROM exists */
pub const ICE1712_I2C_BUSY: c_uint = 0x01; /* busy bit */
pub const ICE1712_REG_CONCAP_ADDR: c_ulong = 0x14; /* dword - consumer capture */
pub const ICE1712_REG_CONCAP_COUNT: c_ulong = 0x18; /* word - current/base count */
pub const ICE1712_REG_SERR_SHADOW: c_ulong = 0x1b; /* byte */
pub const ICE1712_REG_MPU2_CTRL: c_ulong = 0x1c; /* byte */
pub const ICE1712_REG_MPU2_DATA: c_ulong = 0x1d; /* byte */
pub const ICE1712_REG_TIMER: c_ulong = 0x1e; /* word */

#[inline]
pub unsafe fn ICEREG(ice: *const snd_ice1712, x: c_ulong) -> c_ulong {
    unsafe { (*ice).port.wrapping_add(x) }
}

/*
 *  Indirect registers
 */

pub const ICE1712_IREG_PBK_COUNT_LO: c_uint = 0x00;
pub const ICE1712_IREG_PBK_COUNT_HI: c_uint = 0x01;
pub const ICE1712_IREG_PBK_CTRL: c_uint = 0x02;
pub const ICE1712_IREG_PBK_LEFT: c_uint = 0x03; /* left volume */
pub const ICE1712_IREG_PBK_RIGHT: c_uint = 0x04; /* right volume */
pub const ICE1712_IREG_PBK_SOFT: c_uint = 0x05; /* soft volume */
pub const ICE1712_IREG_PBK_RATE_LO: c_uint = 0x06;
pub const ICE1712_IREG_PBK_RATE_MID: c_uint = 0x07;
pub const ICE1712_IREG_PBK_RATE_HI: c_uint = 0x08;
pub const ICE1712_IREG_CAP_COUNT_LO: c_uint = 0x10;
pub const ICE1712_IREG_CAP_COUNT_HI: c_uint = 0x11;
pub const ICE1712_IREG_CAP_CTRL: c_uint = 0x12;
pub const ICE1712_IREG_GPIO_DATA: c_uint = 0x20;
pub const ICE1712_IREG_GPIO_WRITE_MASK: c_uint = 0x21;
pub const ICE1712_IREG_GPIO_DIRECTION: c_uint = 0x22;
pub const ICE1712_IREG_CONSUMER_POWERDOWN: c_uint = 0x30;
pub const ICE1712_IREG_PRO_POWERDOWN: c_uint = 0x31;

/*
 *  Consumer section direct DMA registers
 */

pub const ICE1712_DS_INTMASK: c_ulong = 0x00; /* word - interrupt mask */
pub const ICE1712_DS_INTSTAT: c_ulong = 0x02; /* word - interrupt status */
pub const ICE1712_DS_DATA: c_ulong = 0x04; /* dword - channel data */
pub const ICE1712_DS_INDEX: c_ulong = 0x08; /* dword - channel index */

#[inline]
pub unsafe fn ICEDS(ice: *const snd_ice1712, x: c_ulong) -> c_ulong {
    unsafe { (*ice).dmapath_port.wrapping_add(x) }
}

/*
 *  Consumer section channel registers
 */

pub const ICE1712_DSC_ADDR0: c_uint = 0x00; /* dword - base address 0 */
pub const ICE1712_DSC_COUNT0: c_uint = 0x01; /* word - count 0 */
pub const ICE1712_DSC_ADDR1: c_uint = 0x02; /* dword - base address 1 */
pub const ICE1712_DSC_COUNT1: c_uint = 0x03; /* word - count 1 */
pub const ICE1712_DSC_CONTROL: c_uint = 0x04; /* byte - control & status */
pub const ICE1712_BUFFER1: c_uint = 0x80; /* buffer1 is active */
pub const ICE1712_BUFFER1_AUTO: c_uint = 0x40; /* buffer1 auto init */
pub const ICE1712_BUFFER0_AUTO: c_uint = 0x20; /* buffer0 auto init */
pub const ICE1712_FLUSH: c_uint = 0x10; /* flush FIFO */
pub const ICE1712_STEREO: c_uint = 0x08; /* stereo */
pub const ICE1712_16BIT: c_uint = 0x04; /* 16-bit data */
pub const ICE1712_PAUSE: c_uint = 0x02; /* pause */
pub const ICE1712_START: c_uint = 0x01; /* start */
pub const ICE1712_DSC_RATE: c_uint = 0x05; /* dword - rate */
pub const ICE1712_DSC_VOLUME: c_uint = 0x06; /* word - volume control */

/*
 *  Professional multi-track direct control registers
 */

pub const ICE1712_MT_IRQ: c_ulong = 0x00; /* byte - interrupt mask */
pub const ICE1712_MULTI_CAPTURE: c_uint = 0x80; /* capture IRQ */
pub const ICE1712_MULTI_PLAYBACK: c_uint = 0x40; /* playback IRQ */
pub const ICE1712_MULTI_CAPSTATUS: c_uint = 0x02; /* capture IRQ status */
pub const ICE1712_MULTI_PBKSTATUS: c_uint = 0x01; /* playback IRQ status */
pub const ICE1712_MT_RATE: c_ulong = 0x01; /* byte - sampling rate select */
pub const ICE1712_SPDIF_MASTER: c_uint = 0x10; /* S/PDIF input is master clock */
pub const ICE1712_MT_I2S_FORMAT: c_ulong = 0x02; /* byte - I2S data format */
pub const ICE1712_MT_AC97_INDEX: c_ulong = 0x04; /* byte - AC'97 index */
pub const ICE1712_MT_AC97_CMD: c_ulong = 0x05; /* byte - AC'97 command & status */
/* look to ICE1712_AC97_* */
pub const ICE1712_MT_AC97_DATA: c_ulong = 0x06; /* word - AC'97 data */
pub const ICE1712_MT_PLAYBACK_ADDR: c_ulong = 0x10; /* dword - playback address */
pub const ICE1712_MT_PLAYBACK_SIZE: c_ulong = 0x14; /* word - playback size */
pub const ICE1712_MT_PLAYBACK_COUNT: c_ulong = 0x16; /* word - playback count */
pub const ICE1712_MT_PLAYBACK_CONTROL: c_ulong = 0x18; /* byte - control */
pub const ICE1712_CAPTURE_START_SHADOW: c_uint = 0x04; /* capture start */
pub const ICE1712_PLAYBACK_PAUSE: c_uint = 0x02; /* playback pause */
pub const ICE1712_PLAYBACK_START: c_uint = 0x01; /* playback start */
pub const ICE1712_MT_CAPTURE_ADDR: c_ulong = 0x20; /* dword - capture address */
pub const ICE1712_MT_CAPTURE_SIZE: c_ulong = 0x24; /* word - capture size */
pub const ICE1712_MT_CAPTURE_COUNT: c_ulong = 0x26; /* word - capture count */
pub const ICE1712_MT_CAPTURE_CONTROL: c_ulong = 0x28; /* byte - control */
pub const ICE1712_CAPTURE_START: c_uint = 0x01; /* capture start */
pub const ICE1712_MT_ROUTE_PSDOUT03: c_ulong = 0x30; /* word */
pub const ICE1712_MT_ROUTE_SPDOUT: c_ulong = 0x32; /* word */
pub const ICE1712_MT_ROUTE_CAPTURE: c_ulong = 0x34; /* dword */
pub const ICE1712_MT_MONITOR_VOLUME: c_ulong = 0x38; /* word */
pub const ICE1712_MT_MONITOR_INDEX: c_ulong = 0x3a; /* byte */
pub const ICE1712_MT_MONITOR_RATE: c_ulong = 0x3b; /* byte */
pub const ICE1712_MT_MONITOR_ROUTECTRL: c_ulong = 0x3c; /* byte */
pub const ICE1712_ROUTE_AC97: c_uint = 0x01; /* route digital mixer output to AC'97 */
pub const ICE1712_MT_MONITOR_PEAKINDEX: c_ulong = 0x3e; /* byte */
pub const ICE1712_MT_MONITOR_PEAKDATA: c_ulong = 0x3f; /* byte */

#[inline]
pub unsafe fn ICEMT(ice: *const snd_ice1712, x: c_ulong) -> c_ulong {
    unsafe { (*ice).profi_port.wrapping_add(x) }
}

/*
 *  Codec configuration bits
 */

/* PCI[60] System Configuration */
pub const ICE1712_CFG_CLOCK: c_uint = 0xc0;
pub const ICE1712_CFG_CLOCK512: c_uint = 0x00; /* 22.5692Mhz, 44.1kHz*512 */
pub const ICE1712_CFG_CLOCK384: c_uint = 0x40; /* 16.9344Mhz, 44.1kHz*384 */
pub const ICE1712_CFG_EXT: c_uint = 0x80; /* external clock */
pub const ICE1712_CFG_2X_MPU401: c_uint = 0x20; /* two MPU401 UARTs */
pub const ICE1712_CFG_NO_CON_AC97: c_uint = 0x10; /* consumer AC'97 codec is not present */
pub const ICE1712_CFG_ADC_MASK: c_uint = 0x0c; /* one, two, three, four stereo ADCs */
pub const ICE1712_CFG_DAC_MASK: c_uint = 0x03; /* one, two, three, four stereo DACs */
/* PCI[61] AC-Link Configuration */
pub const ICE1712_CFG_PRO_I2S: c_uint = 0x80; /* multitrack converter: I2S or AC'97 */
pub const ICE1712_CFG_AC97_PACKED: c_uint = 0x01; /* split or packed mode - AC'97 */
/* PCI[62] I2S Features */
pub const ICE1712_CFG_I2S_VOLUME: c_uint = 0x80; /* volume/mute capability */
pub const ICE1712_CFG_I2S_96KHZ: c_uint = 0x40; /* supports 96kHz sampling */
pub const ICE1712_CFG_I2S_RESMASK: c_uint = 0x30; /* resolution mask, 16,18,20,24-bit */
pub const ICE1712_CFG_I2S_OTHER: c_uint = 0x0f; /* other I2S IDs */
/* PCI[63] S/PDIF Configuration */
pub const ICE1712_CFG_I2S_CHIPID: c_uint = 0xfc; /* I2S chip ID */
pub const ICE1712_CFG_SPDIF_IN: c_uint = 0x02; /* S/PDIF input is present */
pub const ICE1712_CFG_SPDIF_OUT: c_uint = 0x01; /* S/PDIF output is present */

/*
 * DMA mode values
 * identical with DMA_XXX on i386 architecture.
 */
pub const ICE1712_DMA_MODE_WRITE: c_uint = 0x48;
pub const ICE1712_DMA_AUTOINIT: c_uint = 0x10;

/*
 * I2C EEPROM Address
 */
pub const ICE_I2C_EEPROM_ADDR: c_uint = 0xA0;

#[repr(C)]
pub struct snd_ice1712_eeprom {
    pub subvendor: c_uint, /* PCI[2c-2f] */
    pub size: u8,          /* size of EEPROM image in bytes */
    pub version: u8,       /* must be 1 (or 2 for vt1724) */
    pub data: [u8; 32],
    pub gpiomask: c_uint,
    pub gpiostate: c_uint,
    pub gpiodir: c_uint,
}

pub const ICE_EEP1_CODEC: usize = 0; /* 06 */
pub const ICE_EEP1_ACLINK: usize = 1; /* 07 */
pub const ICE_EEP1_I2SID: usize = 2; /* 08 */
pub const ICE_EEP1_SPDIF: usize = 3; /* 09 */
pub const ICE_EEP1_GPIO_MASK: usize = 4; /* 0a */
pub const ICE_EEP1_GPIO_STATE: usize = 5; /* 0b */
pub const ICE_EEP1_GPIO_DIR: usize = 6; /* 0c */
pub const ICE_EEP1_AC97_MAIN_LO: usize = 7; /* 0d */
pub const ICE_EEP1_AC97_MAIN_HI: usize = 8; /* 0e */
pub const ICE_EEP1_AC97_PCM_LO: usize = 9; /* 0f */
pub const ICE_EEP1_AC97_PCM_HI: usize = 10; /* 10 */
pub const ICE_EEP1_AC97_REC_LO: usize = 11; /* 11 */
pub const ICE_EEP1_AC97_REC_HI: usize = 12; /* 12 */
pub const ICE_EEP1_AC97_RECSRC: usize = 13; /* 13 */
pub const ICE_EEP1_DAC_ID: usize = 14; /* 14 */
pub const ICE_EEP1_DAC_ID1: usize = 15;
pub const ICE_EEP1_DAC_ID2: usize = 16;
pub const ICE_EEP1_DAC_ID3: usize = 17;
pub const ICE_EEP1_ADC_ID: usize = 18; /* 18 */
pub const ICE_EEP1_ADC_ID1: usize = 19;
pub const ICE_EEP1_ADC_ID2: usize = 20;
pub const ICE_EEP1_ADC_ID3: usize = 21;

#[inline]
pub unsafe fn ice_has_con_ac97(ice: *const snd_ice1712) -> bool {
    unsafe { ((*ice).eeprom.data[ICE_EEP1_CODEC] & ICE1712_CFG_NO_CON_AC97 as u8) == 0 }
}

#[repr(C)]
pub struct snd_ak4xxx_private {
    pub cif: c_uint, /* C bitfield: unsigned int cif:1; CIF mode */
    pub caddr: u8,  /* C0 and C1 bits */
    pub data_mask: c_uint, /* DATA gpio bit */
    pub clk_mask: c_uint,  /* CLK gpio bit */
    pub cs_mask: c_uint,   /* bit mask for select/deselect address */
    pub cs_addr: c_uint,   /* bits to select address */
    pub cs_none: c_uint,   /* bits to deselect address */
    pub add_flags: c_uint, /* additional bits at init */
    pub mask_flags: c_uint, /* total mask bits */
    pub ops: snd_akm4xxx_ops,
}

#[repr(C)]
pub struct snd_akm4xxx_ops {
    pub set_rate_val: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, rate: c_uint)>,
}

#[repr(C)]
pub struct snd_ice1712_spdif {
    pub cs8403_bits: u8,
    pub cs8403_stream_bits: u8,
    pub stream_ctl: *mut snd_kcontrol,
    pub ops: snd_ice1712_spdif_ops,
}

#[repr(C)]
pub struct snd_ice1712_spdif_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
    pub setup_rate: Option<unsafe extern "C" fn(*mut snd_ice1712, rate: c_int)>,
    pub close: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
    pub default_get:
        Option<unsafe extern "C" fn(*mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value)>,
    pub default_put:
        Option<unsafe extern "C" fn(*mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> c_int>,
    pub stream_get:
        Option<unsafe extern "C" fn(*mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value)>,
    pub stream_put:
        Option<unsafe extern "C" fn(*mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub conp_dma_size: c_ulong,
    pub conc_dma_size: c_ulong,
    pub prop_dma_size: c_ulong,
    pub proc_dma_size: c_ulong,
    pub irq: c_int,
    pub port: c_ulong,
    pub ddma_port: c_ulong,
    pub dmapath_port: c_ulong,
    pub profi_port: c_ulong,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub pcm_ds: *mut snd_pcm,
    pub pcm_pro: *mut snd_pcm,
    pub playback_con_substream: *mut snd_pcm_substream,
    pub playback_con_substream_ds: [*mut snd_pcm_substream; 6],
    pub capture_con_substream: *mut snd_pcm_substream,
    pub playback_pro_substream: *mut snd_pcm_substream,
    pub capture_pro_substream: *mut snd_pcm_substream,
    pub playback_pro_size: c_uint,
    pub capture_pro_size: c_uint,
    pub playback_con_virt_addr: [c_uint; 6],
    pub playback_con_active_buf: [c_uint; 6],
    pub capture_con_virt_addr: c_uint,
    pub ac97_ext_id: c_uint,
    pub ac97: *mut snd_ac97,
    pub rmidi: [*mut snd_rawmidi; 2],
    pub reg_lock: spinlock_t,
    pub proc_entry: *mut snd_info_entry,
    pub eeprom: snd_ice1712_eeprom,
    pub card_info: *const snd_ice1712_card_info,
    pub pro_volumes: [c_uint; 20],
    pub omni: c_uint,       /* C bitfield: unsigned int omni:1; Delta Omni I/O */
    pub dxr_enable: c_uint, /* C bitfield: unsigned int dxr_enable:1; Terratec DXR enable for DMX6FIRE */
    pub vt1724: c_uint,     /* C bitfield: unsigned int vt1724:1; */
    pub vt1720: c_uint,     /* C bitfield: unsigned int vt1720:1; */
    pub has_spdif: c_uint,  /* C bitfield: unsigned int has_spdif:1; VT1720/4 - has SPDIF I/O */
    pub force_pdma4: c_uint, /* C bitfield: unsigned int force_pdma4:1; VT1720/4 - PDMA4 as non-spdif */
    pub force_rdma1: c_uint, /* C bitfield: unsigned int force_rdma1:1; VT1720/4 - RDMA1 as non-spdif */
    pub midi_output: c_uint, /* C bitfield: unsigned int midi_output:1; VT1720/4: MIDI output triggered */
    pub midi_input: c_uint, /* C bitfield: unsigned int midi_input:1; VT1720/4: MIDI input triggered */
    pub own_routing: c_uint, /* C bitfield: unsigned int own_routing:1; VT1720/4: use own routing ctls */
    pub num_total_dacs: c_uint, /* total DACs */
    pub num_total_adcs: c_uint, /* total ADCs */
    pub cur_rate: c_uint,       /* current rate */
    pub open_mutex: mutex,
    pub pcm_reserved: [*mut snd_pcm_substream; 4],
    pub hw_rates: *const snd_pcm_hw_constraint_list, /* card-specific rate constraints */
    pub akm_codecs: c_uint,
    pub akm: *mut snd_akm4xxx,
    pub spdif: snd_ice1712_spdif,
    pub i2c_mutex: mutex,        /* I2C mutex for ICE1724 registers */
    pub i2c: *mut snd_i2c_bus,   /* I2C bus */
    pub cs8427: *mut snd_i2c_device, /* CS8427 I2C device */
    pub cs8427_timeout: c_uint,  /* CS8427 reset timeout in HZ/100 */
    pub gpio: ice1712_gpio,
    pub gpio_mutex: mutex,
    pub spec: *mut c_void, /* other board-specific data */
    /* VT172x specific */
    pub pro_rate_default: c_int,
    pub is_spdif_master: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_int>,
    pub get_rate: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_uint>,
    pub set_rate: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, rate: c_uint)>,
    pub set_mclk: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, rate: c_uint) -> u8>,
    pub set_spdif_clock: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, type_: c_int) -> c_int>,
    pub get_spdif_master_type: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_int>,
    pub ext_clock_names: *const *const c_char,
    pub ext_clock_count: c_int,
    pub pro_open: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
    /* CONFIG_PM_SLEEP fields from the C header:
     * int (*pm_suspend)(struct snd_ice1712 *);
     * int (*pm_resume)(struct snd_ice1712 *);
     * unsigned int pm_suspend_enabled:1;
     * unsigned int pm_saved_is_spdif_master:1;
     * unsigned int pm_saved_spdif_ctrl;
     * unsigned char pm_saved_spdif_cfg;
     * unsigned int pm_saved_route;
     */
}

#[repr(C)]
pub struct ice1712_gpio {
    pub direction: c_uint,  /* current direction bits */
    pub write_mask: c_uint, /* current mask bits */
    pub saved: [c_uint; 2], /* for ewx_i2c */
    /* operators */
    pub set_mask: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, data: c_uint)>,
    pub get_mask: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_uint>,
    pub set_dir: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, data: c_uint)>,
    pub get_dir: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_uint>,
    pub set_data: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, data: c_uint)>,
    pub get_data: Option<unsafe extern "C" fn(ice: *mut snd_ice1712) -> c_uint>,
    /* misc operators - move to another place? */
    pub set_pro_rate: Option<unsafe extern "C" fn(ice: *mut snd_ice1712, rate: c_uint)>,
    pub i2s_mclk_changed: Option<unsafe extern "C" fn(ice: *mut snd_ice1712)>,
}

/*
 * gpio access functions
 */
#[inline]
pub unsafe fn snd_ice1712_gpio_set_dir(ice: *mut snd_ice1712, bits: c_uint) {
    unsafe {
        if let Some(set_dir) = (*ice).gpio.set_dir {
            set_dir(ice, bits);
        }
    }
}

#[inline]
pub unsafe fn snd_ice1712_gpio_get_dir(ice: *mut snd_ice1712) -> c_uint {
    unsafe { (*ice).gpio.get_dir.map_or(0, |get_dir| get_dir(ice)) }
}

#[inline]
pub unsafe fn snd_ice1712_gpio_set_mask(ice: *mut snd_ice1712, bits: c_uint) {
    unsafe {
        if let Some(set_mask) = (*ice).gpio.set_mask {
            set_mask(ice, bits);
        }
    }
}

#[inline]
pub unsafe fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, val: c_uint) {
    unsafe {
        if let Some(set_data) = (*ice).gpio.set_data {
            set_data(ice, val);
        }
    }
}

#[inline]
pub unsafe fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint {
    unsafe { (*ice).gpio.get_data.map_or(0, |get_data| get_data(ice)) }
}

/*
 * save and restore gpio status
 * The access to gpio will be protected by mutex, so don't forget to
 * restore!
 */
#[inline]
pub unsafe fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712) {
    unsafe {
        mutex_lock(&mut (*ice).gpio_mutex);
        (*ice).gpio.saved[0] = (*ice).gpio.direction;
        (*ice).gpio.saved[1] = (*ice).gpio.write_mask;
    }
}

#[inline]
pub unsafe fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712) {
    unsafe {
        if let Some(set_dir) = (*ice).gpio.set_dir {
            set_dir(ice, (*ice).gpio.saved[0]);
        }
        if let Some(set_mask) = (*ice).gpio.set_mask {
            set_mask(ice, (*ice).gpio.saved[1]);
        }
        (*ice).gpio.direction = (*ice).gpio.saved[0];
        (*ice).gpio.write_mask = (*ice).gpio.saved[1];
        mutex_unlock(&mut (*ice).gpio_mutex);
    }
}

/* for bit controls */
/* C macro translated by intent:
 * ICE1712_GPIO(xiface, xname, xindex, mask, invert, xaccess)
 * initializes a snd_kcontrol_new with:
 * .iface = xiface, .name = xname, .access = xaccess,
 * .info = snd_ctl_boolean_mono_info,
 * .get = snd_ice1712_gpio_get, .put = snd_ice1712_gpio_put,
 * .private_value = mask | (invert << 24)
 */

unsafe extern "C" {
    pub fn snd_ice1712_gpio_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn snd_ice1712_gpio_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
}

/*
 * set gpio direction, write mask and data
 */
#[inline]
pub unsafe fn snd_ice1712_gpio_write_bits(
    ice: *mut snd_ice1712,
    mask: c_uint,
    bits: c_uint,
) {
    unsafe {
        let mut val: c_uint;

        (*ice).gpio.direction |= mask;
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
        val = snd_ice1712_gpio_read(ice);
        val &= !mask;
        val |= mask & bits;
        snd_ice1712_gpio_write(ice, val);
    }
}

#[inline]
pub unsafe fn snd_ice1712_gpio_read_bits(ice: *mut snd_ice1712, mask: c_uint) -> c_int {
    unsafe {
        (*ice).gpio.direction &= !mask;
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
        (snd_ice1712_gpio_read(ice) & mask) as c_int
    }
}

/* route access functions */
unsafe extern "C" {
    pub fn snd_ice1724_get_route_val(ice: *mut snd_ice1712, shift: c_int) -> c_int;
    pub fn snd_ice1724_put_route_val(
        ice: *mut snd_ice1712,
        val: c_uint,
        shift: c_int,
    ) -> c_int;

    pub fn snd_ice1712_spdif_build_controls(ice: *mut snd_ice1712) -> c_int;

    pub fn snd_ice1712_akm4xxx_init(
        ak: *mut snd_akm4xxx,
        template: *const snd_akm4xxx,
        priv_: *const snd_ak4xxx_private,
        ice: *mut snd_ice1712,
    ) -> c_int;
    pub fn snd_ice1712_akm4xxx_free(ice: *mut snd_ice1712);
    pub fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int;

    pub fn snd_ice1712_init_cs8427(ice: *mut snd_ice1712, addr: c_int) -> c_int;
}

#[inline]
pub unsafe fn snd_ice1712_write(ice: *mut snd_ice1712, addr: u8, data: u8) {
    unsafe {
        outb(addr, ICEREG(ice, ICE1712_REG_INDEX));
        outb(data, ICEREG(ice, ICE1712_REG_DATA));
    }
}

#[inline]
pub unsafe fn snd_ice1712_read(ice: *mut snd_ice1712, addr: u8) -> u8 {
    unsafe {
        outb(addr, ICEREG(ice, ICE1712_REG_INDEX));
        inb(ICEREG(ice, ICE1712_REG_DATA))
    }
}

/*
 * entry pointer
 */

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub driver: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub chip_exit: Option<unsafe extern "C" fn(*mut snd_ice1712)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub no_mpu401: c_uint, /* C bitfield: unsigned int no_mpu401:1; */
    pub mpu401_1_info_flags: c_uint,
    pub mpu401_2_info_flags: c_uint,
    pub mpu401_1_name: *const c_char,
    pub mpu401_2_name: *const c_char,
    pub eeprom_size: c_uint,
    pub eeprom_data: *const u8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
