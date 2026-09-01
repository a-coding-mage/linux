/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of ca0106.h.
 * Header guards and C include directives are omitted; ca_midi.h and ALSA/Linux types
 * are expected to be provided by surrounding translated dependencies.
 */

pub const CA0106_PTR: u32 = 0x00;
pub const CA0106_DATA: u32 = 0x04;
pub const CA0106_IPR: u32 = 0x08;
pub const IPR_MIDI_RX_B: u32 = 0x00020000;
pub const IPR_MIDI_TX_B: u32 = 0x00010000;
pub const IPR_SPDIF_IN_USER: u32 = 0x00004000;
pub const IPR_SPDIF_OUT_USER: u32 = 0x00002000;
pub const IPR_SPDIF_OUT_FRAME: u32 = 0x00001000;
pub const IPR_SPI: u32 = 0x00000800;
pub const IPR_I2C_EEPROM: u32 = 0x00000400;
pub const IPR_I2C_DAC: u32 = 0x00000200;
pub const IPR_AI: u32 = 0x00000100;
pub const IPR_GPI: u32 = 0x00000080;
pub const IPR_SRC_LOCKED: u32 = 0x00000040;
pub const IPR_SPDIF_STATUS: u32 = 0x00000020;
pub const IPR_TIMER2: u32 = 0x00000010;
pub const IPR_TIMER1: u32 = 0x00000008;
pub const IPR_MIDI_RX_A: u32 = 0x00000004;
pub const IPR_MIDI_TX_A: u32 = 0x00000002;
pub const IPR_PCI: u32 = 0x00000001;
pub const CA0106_INTE: u32 = 0x0c;
pub const INTE_MIDI_RX_B: u32 = 0x00020000;
pub const INTE_MIDI_TX_B: u32 = 0x00010000;
pub const INTE_SPDIF_IN_USER: u32 = 0x00004000;
pub const INTE_SPDIF_OUT_USER: u32 = 0x00002000;
pub const INTE_SPDIF_OUT_FRAME: u32 = 0x00001000;
pub const INTE_SPI: u32 = 0x00000800;
pub const INTE_I2C_EEPROM: u32 = 0x00000400;
pub const INTE_I2C_DAC: u32 = 0x00000200;
pub const INTE_AI: u32 = 0x00000100;
pub const INTE_GPI: u32 = 0x00000080;
pub const INTE_SRC_LOCKED: u32 = 0x00000040;
pub const INTE_SPDIF_STATUS: u32 = 0x00000020;
pub const INTE_TIMER2: u32 = 0x00000010;
pub const INTE_TIMER1: u32 = 0x00000008;
pub const INTE_MIDI_RX_A: u32 = 0x00000004;
pub const INTE_MIDI_TX_A: u32 = 0x00000002;
pub const INTE_PCI: u32 = 0x00000001;
pub const CA0106_UNKNOWN10: u32 = 0x10;
pub const CA0106_HCFG: u32 = 0x14;
pub const HCFG_STAC: u32 = 0x10000000;
pub const HCFG_CAPTURE_I2S_BYPASS: u32 = 0x08000000;
pub const HCFG_CAPTURE_SPDIF_BYPASS: u32 = 0x04000000;
pub const HCFG_PLAYBACK_I2S_BYPASS: u32 = 0x02000000;
pub const HCFG_FORCE_LOCK: u32 = 0x01000000;
pub const HCFG_PLAYBACK_ATTENUATION: u32 = 0x00006000;
pub const HCFG_PLAYBACK_DITHER: u32 = 0x00001000;
pub const HCFG_PLAYBACK_S32_LE: u32 = 0x00000800;
pub const HCFG_CAPTURE_S32_LE: u32 = 0x00000400;
pub const HCFG_8_CHANNEL_PLAY: u32 = 0x00000200;
pub const HCFG_8_CHANNEL_CAPTURE: u32 = 0x00000100;
pub const HCFG_MONO: u32 = 0x00000080;
pub const HCFG_I2S_OUTPUT: u32 = 0x00000010;
pub const HCFG_AC97: u32 = 0x00000008;
pub const HCFG_LOCK_PLAYBACK_CACHE: u32 = 0x00000004;
pub const HCFG_LOCK_CAPTURE_CACHE: u32 = 0x00000002;
pub const HCFG_AUDIOENABLE: u32 = 0x00000001;
pub const CA0106_GPIO: u32 = 0x18;
pub const CA0106_AC97DATA: u32 = 0x1c;
pub const CA0106_AC97ADDRESS: u32 = 0x1e;
pub const PLAYBACK_LIST_ADDR: u32 = 0x00;
pub const PLAYBACK_LIST_SIZE: u32 = 0x01;
pub const PLAYBACK_LIST_PTR: u32 = 0x02;
pub const PLAYBACK_UNKNOWN3: u32 = 0x03;
pub const PLAYBACK_DMA_ADDR: u32 = 0x04;
pub const PLAYBACK_PERIOD_SIZE: u32 = 0x05;
pub const PLAYBACK_POINTER: u32 = 0x06;
pub const PLAYBACK_PERIOD_END_ADDR: u32 = 0x07;
pub const PLAYBACK_FIFO_OFFSET_ADDRESS: u32 = 0x08;
pub const PLAYBACK_UNKNOWN9: u32 = 0x09;
pub const CAPTURE_DMA_ADDR: u32 = 0x10;
pub const CAPTURE_BUFFER_SIZE: u32 = 0x11;
pub const CAPTURE_POINTER: u32 = 0x12;
pub const CAPTURE_FIFO_OFFSET_ADDRESS: u32 = 0x13;
pub const PLAYBACK_LAST_SAMPLE: u32 = 0x20;
pub const BASIC_INTERRUPT: u32 = 0x40;
pub const SPCS0: u32 = 0x41;
pub const SPCS1: u32 = 0x42;
pub const SPCS2: u32 = 0x43;
pub const SPCS3: u32 = 0x44;
pub const SPCS_CLKACCYMASK: u32 = 0x30000000;
pub const SPCS_CLKACCY_1000PPM: u32 = 0x00000000;
pub const SPCS_CLKACCY_50PPM: u32 = 0x10000000;
pub const SPCS_CLKACCY_VARIABLE: u32 = 0x20000000;
pub const SPCS_SAMPLERATEMASK: u32 = 0x0f000000;
pub const SPCS_SAMPLERATE_44: u32 = 0x00000000;
pub const SPCS_SAMPLERATE_48: u32 = 0x02000000;
pub const SPCS_SAMPLERATE_32: u32 = 0x03000000;
pub const SPCS_CHANNELNUMMASK: u32 = 0x00f00000;
pub const SPCS_CHANNELNUM_UNSPEC: u32 = 0x00000000;
pub const SPCS_CHANNELNUM_LEFT: u32 = 0x00100000;
pub const SPCS_CHANNELNUM_RIGHT: u32 = 0x00200000;
pub const SPCS_SOURCENUMMASK: u32 = 0x000f0000;
pub const SPCS_SOURCENUM_UNSPEC: u32 = 0x00000000;
pub const SPCS_GENERATIONSTATUS: u32 = 0x00008000;
pub const SPCS_CATEGORYCODEMASK: u32 = 0x00007f00;
pub const SPCS_MODEMASK: u32 = 0x000000c0;
pub const SPCS_EMPHASISMASK: u32 = 0x00000038;
pub const SPCS_EMPHASIS_NONE: u32 = 0x00000000;
pub const SPCS_EMPHASIS_50_15: u32 = 0x00000008;
pub const SPCS_COPYRIGHT: u32 = 0x00000004;
pub const SPCS_NOTAUDIODATA: u32 = 0x00000002;
pub const SPCS_PROFESSIONAL: u32 = 0x00000001;
pub const SPCS_WORD_LENGTH_MASK: u32 = 0x0000000f;
pub const SPCS_WORD_LENGTH_16: u32 = 0x00000008;
pub const SPCS_WORD_LENGTH_17: u32 = 0x00000006;
pub const SPCS_WORD_LENGTH_18: u32 = 0x00000004;
pub const SPCS_WORD_LENGTH_19: u32 = 0x00000002;
pub const SPCS_WORD_LENGTH_20A: u32 = 0x0000000a;
pub const SPCS_WORD_LENGTH_20: u32 = 0x00000009;
pub const SPCS_WORD_LENGTH_21: u32 = 0x00000007;
pub const SPCS_WORD_LENGTH_22: u32 = 0x00000005;
pub const SPCS_WORD_LENGTH_23: u32 = 0x00000003;
pub const SPCS_WORD_LENGTH_24: u32 = 0x0000000b;
pub const SPCS_ORIGINAL_SAMPLE_RATE_MASK: u32 = 0x000000f0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_NONE: u32 = 0x00000000;
pub const SPCS_ORIGINAL_SAMPLE_RATE_16000: u32 = 0x00000010;
pub const SPCS_ORIGINAL_SAMPLE_RATE_RES1: u32 = 0x00000020;
pub const SPCS_ORIGINAL_SAMPLE_RATE_32000: u32 = 0x00000030;
pub const SPCS_ORIGINAL_SAMPLE_RATE_12000: u32 = 0x00000040;
pub const SPCS_ORIGINAL_SAMPLE_RATE_11025: u32 = 0x00000050;
pub const SPCS_ORIGINAL_SAMPLE_RATE_8000: u32 = 0x00000060;
pub const SPCS_ORIGINAL_SAMPLE_RATE_RES2: u32 = 0x00000070;
pub const SPCS_ORIGINAL_SAMPLE_RATE_192000: u32 = 0x00000080;
pub const SPCS_ORIGINAL_SAMPLE_RATE_24000: u32 = 0x00000090;
pub const SPCS_ORIGINAL_SAMPLE_RATE_96000: u32 = 0x000000a0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_48000: u32 = 0x000000b0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_176400: u32 = 0x000000c0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_22050: u32 = 0x000000d0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_88200: u32 = 0x000000e0;
pub const SPCS_ORIGINAL_SAMPLE_RATE_44100: u32 = 0x000000f0;
pub const SPDIF_SELECT1: u32 = 0x45;
pub const WATERMARK: u32 = 0x46;
pub const SPDIF_INPUT_STATUS: u32 = 0x49;
pub const CAPTURE_CACHE_DATA: u32 = 0x50;
pub const CAPTURE_SOURCE: u32 = 0x60;
pub const CAPTURE_SOURCE_CHANNEL0: u32 = 0xf0000000;
pub const CAPTURE_SOURCE_CHANNEL1: u32 = 0x0f000000;
pub const CAPTURE_SOURCE_CHANNEL2: u32 = 0x00f00000;
pub const CAPTURE_SOURCE_CHANNEL3: u32 = 0x000f0000;
pub const CAPTURE_SOURCE_RECORD_MAP: u32 = 0x0000ffff;
pub const CAPTURE_VOLUME1: u32 = 0x61;
pub const CAPTURE_VOLUME2: u32 = 0x62;
pub const PLAYBACK_ROUTING1: u32 = 0x63;
pub const ROUTING1_REAR: u32 = 0x77000000;
pub const ROUTING1_NULL: u32 = 0x00770000;
pub const ROUTING1_CENTER_LFE: u32 = 0x00007700;
pub const ROUTING1_FRONT: u32 = 0x00000077;
pub const PLAYBACK_ROUTING2: u32 = 0x64;
pub const PLAYBACK_MUTE: u32 = 0x65;
pub const PLAYBACK_VOLUME1: u32 = 0x66;
pub const CAPTURE_ROUTING1: u32 = 0x67;
pub const CAPTURE_ROUTING2: u32 = 0x68;
pub const CAPTURE_MUTE: u32 = 0x69;
pub const PLAYBACK_VOLUME2: u32 = 0x6a;
pub const UNKNOWN6b: u32 = 0x6b;
pub const MIDI_UART_A_DATA: u32 = 0x6c;
pub const MIDI_UART_A_CMD: u32 = 0x6d;
pub const MIDI_UART_B_DATA: u32 = 0x6e;
pub const MIDI_UART_B_CMD: u32 = 0x6f;
pub const CA0106_MIDI_CHAN_A: u32 = 0x1;
pub const CA0106_MIDI_CHAN_B: u32 = 0x2;
pub const CA0106_MIDI_INPUT_AVAIL: u32 = 0x80;
pub const CA0106_MIDI_OUTPUT_READY: u32 = 0x40;
pub const CA0106_MPU401_RESET: u32 = 0xff;
pub const CA0106_MPU401_ENTER_UART: u32 = 0x3f;
pub const CA0106_MPU401_ACK: u32 = 0xfe;
pub const SAMPLE_RATE_TRACKER_STATUS: u32 = 0x70;
pub const CAPTURE_CONTROL: u32 = 0x71;
pub const SPDIF_SELECT2: u32 = 0x72;
pub const ROUTING2_FRONT_MASK: u32 = 0x00010000;
pub const ROUTING2_CENTER_LFE_MASK: u32 = 0x00020000;
pub const ROUTING2_REAR_MASK: u32 = 0x00080000;
pub const UNKNOWN73: u32 = 0x73;
pub const CHIP_VERSION: u32 = 0x74;
pub const EXTENDED_INT_MASK: u32 = 0x75;
pub const EXTENDED_INT: u32 = 0x76;
pub const COUNTER77: u32 = 0x77;
pub const COUNTER78: u32 = 0x78;
pub const EXTENDED_INT_TIMER: u32 = 0x79;
pub const SPI: u32 = 0x7a;
pub const I2C_A: u32 = 0x7b;
pub const I2C_D0: u32 = 0x7c;
pub const I2C_D1: u32 = 0x7d;
pub const I2C_A_ADC_ADD_MASK: u32 = 0x000000fe;
pub const I2C_A_ADC_RW_MASK: u32 = 0x00000001;
pub const I2C_A_ADC_TRANS_MASK: u32 = 0x00000010;
pub const I2C_A_ADC_ABORT_MASK: u32 = 0x00000020;
pub const I2C_A_ADC_LAST_MASK: u32 = 0x00000040;
pub const I2C_A_ADC_BYTE_MASK: u32 = 0x00000080;
pub const I2C_A_ADC_ADD: u32 = 0x00000034;
pub const I2C_A_ADC_READ: u32 = 0x00000001;
pub const I2C_A_ADC_START: u32 = 0x00000100;
pub const I2C_A_ADC_ABORT: u32 = 0x00000200;
pub const I2C_A_ADC_LAST: u32 = 0x00000400;
pub const I2C_A_ADC_BYTE: u32 = 0x00000800;
pub const I2C_D_ADC_REG_MASK: u32 = 0xfe000000;
pub const I2C_D_ADC_DAT_MASK: u32 = 0x01ff0000;
pub const ADC_TIMEOUT: u32 = 0x00000007;
pub const ADC_IFC_CTRL: u32 = 0x0000000b;
pub const ADC_MASTER: u32 = 0x0000000c;
pub const ADC_POWER: u32 = 0x0000000d;
pub const ADC_ATTEN_ADCL: u32 = 0x0000000e;
pub const ADC_ATTEN_ADCR: u32 = 0x0000000f;
pub const ADC_ALC_CTRL1: u32 = 0x00000010;
pub const ADC_ALC_CTRL2: u32 = 0x00000011;
pub const ADC_ALC_CTRL3: u32 = 0x00000012;
pub const ADC_NOISE_CTRL: u32 = 0x00000013;
pub const ADC_LIMIT_CTRL: u32 = 0x00000014;
pub const ADC_MUX: u32 = 0x00000015;
/* #if 0 block omitted from active Rust items: FIXME: Not tested yet ADC constants. */
pub const ADC_MUX_MASK: u32 = 0x0000000f;
pub const ADC_MUX_PHONE: u32 = 0x00000001;
pub const ADC_MUX_MIC: u32 = 0x00000002;
pub const ADC_MUX_LINEIN: u32 = 0x00000004;
pub const ADC_MUX_AUX: u32 = 0x00000008;
pub const SET_CHANNEL: u32 = 0;
pub const PCM_FRONT_CHANNEL: u32 = 0;
pub const PCM_REAR_CHANNEL: u32 = 1;
pub const PCM_CENTER_LFE_CHANNEL: u32 = 2;
pub const PCM_UNKNOWN_CHANNEL: u32 = 3;
pub const CONTROL_FRONT_CHANNEL: u32 = 0;
pub const CONTROL_REAR_CHANNEL: u32 = 3;
pub const CONTROL_CENTER_LFE_CHANNEL: u32 = 1;
pub const CONTROL_UNKNOWN_CHANNEL: u32 = 2;
pub const SPI_REG_MASK: u32 = 0x1ff;
pub const SPI_REG_SHIFT: u32 = 9;
pub const SPI_LDA1_REG: u32 = 0;
pub const SPI_RDA1_REG: u32 = 1;
pub const SPI_LDA2_REG: u32 = 4;
pub const SPI_RDA2_REG: u32 = 5;
pub const SPI_LDA3_REG: u32 = 6;
pub const SPI_RDA3_REG: u32 = 7;
pub const SPI_LDA4_REG: u32 = 13;
pub const SPI_RDA4_REG: u32 = 14;
pub const SPI_MASTDA_REG: u32 = 8;
pub const SPI_DA_BIT_UPDATE: u32 = (1 << 8);
pub const SPI_DA_BIT_0dB: u32 = 0xff;
pub const SPI_DA_BIT_infdB: u32 = 0x00;
pub const SPI_PL_REG: u32 = 2;
pub const SPI_PL_BIT_L_M: u32 = (0 << 5);
pub const SPI_PL_BIT_L_L: u32 = (1 << 5);
pub const SPI_PL_BIT_L_R: u32 = (2 << 5);
pub const SPI_PL_BIT_L_C: u32 = (3 << 5);
pub const SPI_PL_BIT_R_M: u32 = (0 << 7);
pub const SPI_PL_BIT_R_L: u32 = (1 << 7);
pub const SPI_PL_BIT_R_R: u32 = (2 << 7);
pub const SPI_PL_BIT_R_C: u32 = (3 << 7);
pub const SPI_IZD_REG: u32 = 2;
pub const SPI_IZD_BIT: u32 = (0 << 4);
pub const SPI_FMT_REG: u32 = 3;
pub const SPI_FMT_BIT_RJ: u32 = (0 << 0);
pub const SPI_FMT_BIT_LJ: u32 = (1 << 0);
pub const SPI_FMT_BIT_I2S: u32 = (2 << 0);
pub const SPI_FMT_BIT_DSP: u32 = (3 << 0);
pub const SPI_LRP_REG: u32 = 3;
pub const SPI_LRP_BIT: u32 = (1 << 2);
pub const SPI_BCP_REG: u32 = 3;
pub const SPI_BCP_BIT: u32 = (1 << 3);
pub const SPI_IWL_REG: u32 = 3;
pub const SPI_IWL_BIT_16: u32 = (0 << 4);
pub const SPI_IWL_BIT_20: u32 = (1 << 4);
pub const SPI_IWL_BIT_24: u32 = (2 << 4);
pub const SPI_IWL_BIT_32: u32 = (3 << 4);
pub const SPI_MS_REG: u32 = 10;
pub const SPI_MS_BIT: u32 = (1 << 5);
pub const SPI_RATE_REG: u32 = 10;
pub const SPI_RATE_BIT_128: u32 = (0 << 6);
pub const SPI_RATE_BIT_192: u32 = (1 << 6);
pub const SPI_RATE_BIT_256: u32 = (2 << 6);
pub const SPI_RATE_BIT_384: u32 = (3 << 6);
pub const SPI_RATE_BIT_512: u32 = (4 << 6);
pub const SPI_RATE_BIT_768: u32 = (5 << 6);
pub const SPI_DMUTE0_REG: u32 = 9;
pub const SPI_DMUTE1_REG: u32 = 9;
pub const SPI_DMUTE2_REG: u32 = 9;
pub const SPI_DMUTE4_REG: u32 = 15;
pub const SPI_DMUTE0_BIT: u32 = (1 << 3);
pub const SPI_DMUTE1_BIT: u32 = (1 << 4);
pub const SPI_DMUTE2_BIT: u32 = (1 << 5);
pub const SPI_DMUTE4_BIT: u32 = (1 << 2);
pub const SPI_PHASE0_REG: u32 = 3;
pub const SPI_PHASE1_REG: u32 = 3;
pub const SPI_PHASE2_REG: u32 = 3;
pub const SPI_PHASE4_REG: u32 = 15;
pub const SPI_PHASE0_BIT: u32 = (1 << 6);
pub const SPI_PHASE1_BIT: u32 = (1 << 7);
pub const SPI_PHASE2_BIT: u32 = (1 << 8);
pub const SPI_PHASE4_BIT: u32 = (1 << 3);
pub const SPI_PDWN_REG: u32 = 2;
pub const SPI_PDWN_BIT: u32 = (1 << 2);
pub const SPI_DACD0_REG: u32 = 10;
pub const SPI_DACD1_REG: u32 = 10;
pub const SPI_DACD2_REG: u32 = 10;
pub const SPI_DACD4_REG: u32 = 15;
pub const SPI_DACD0_BIT: u32 = (1 << 1);
pub const SPI_DACD1_BIT: u32 = (1 << 2);
pub const SPI_DACD2_BIT: u32 = (1 << 3);
pub const SPI_DACD4_BIT: u32 = (1 << 0);
pub const SPI_PWRDNALL_REG: u32 = 10;
pub const SPI_PWRDNALL_BIT: u32 = (1 << 4);
pub const NUM_SAVED_VOLUMES: u32 = 9;

pub const NUM_SAVED_VOLUMES: usize = 9;

/* Opaque external C types supplied by surrounding translated headers. */
pub enum snd_card {}
pub enum pci_dev {}
pub enum snd_ac97 {}
pub enum snd_pcm {}
pub enum snd_pcm_substream {}
pub enum snd_dma_buffer {}
pub enum snd_ca_midi {}
pub enum spinlock_t {}

#[repr(C)]
pub struct snd_ca0106_channel {
    pub emu: *mut snd_ca0106,
    pub number: ::core::ffi::c_int,
    pub use_: ::core::ffi::c_int,
    pub interrupt: Option<unsafe extern "C" fn(emu: *mut snd_ca0106, channel: *mut snd_ca0106_channel)>,
    pub epcm: *mut snd_ca0106_pcm,
}

#[repr(C)]
pub struct snd_ca0106_pcm {
    pub emu: *mut snd_ca0106,
    pub substream: *mut snd_pcm_substream,
    pub channel_id: ::core::ffi::c_int,
    pub running: u16,
}

#[repr(C)]
pub struct snd_ca0106_details {
    pub serial: u32,
    pub name: *mut ::core::ffi::c_char,
    pub ac97: ::core::ffi::c_int,
    pub gpio_type: ::core::ffi::c_int,
    pub i2c_adc: ::core::ffi::c_int,
    pub spi_dac: u16,
}

#[repr(C)]
pub struct snd_ca0106 {
    pub card: *mut snd_card,
    pub details: *const snd_ca0106_details,
    pub pci: *mut pci_dev,
    pub port: ::core::ffi::c_ulong,
    pub irq: ::core::ffi::c_int,
    pub serial: ::core::ffi::c_uint,
    pub model: ::core::ffi::c_ushort,
    pub emu_lock: spinlock_t,
    pub ac97: *mut snd_ac97,
    pub pcm: [*mut snd_pcm; 4],
    pub playback_channels: [snd_ca0106_channel; 4],
    pub capture_channels: [snd_ca0106_channel; 4],
    pub spdif_bits: [u32; 4],
    pub spdif_str_bits: [u32; 4],
    pub spdif_enable: ::core::ffi::c_int,
    pub capture_source: ::core::ffi::c_int,
    pub i2c_capture_source: ::core::ffi::c_int,
    pub i2c_capture_volume: [[u8; 2]; 4],
    pub capture_mic_line_in: ::core::ffi::c_int,
    pub buffer: *mut snd_dma_buffer,
    pub midi: snd_ca_midi,
    pub midi2: snd_ca_midi,
    pub spi_dac_reg: [u16; 16],
    /* CONFIG_PM_SLEEP: unsigned int saved_vol[NUM_SAVED_VOLUMES]; */
    pub saved_vol: [::core::ffi::c_uint; NUM_SAVED_VOLUMES],
}

unsafe extern "C" {
    pub fn snd_ca0106_mixer(emu: *mut snd_ca0106) -> ::core::ffi::c_int;
    pub fn snd_ca0106_proc_init(emu: *mut snd_ca0106) -> ::core::ffi::c_int;
    pub fn snd_ca0106_ptr_read(
        emu: *mut snd_ca0106,
        reg: ::core::ffi::c_uint,
        chn: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn snd_ca0106_ptr_write(
        emu: *mut snd_ca0106,
        reg: ::core::ffi::c_uint,
        chn: ::core::ffi::c_uint,
        data: ::core::ffi::c_uint,
    );
    pub fn snd_ca0106_i2c_write(emu: *mut snd_ca0106, reg: u32, value: u32) -> ::core::ffi::c_int;
    pub fn snd_ca0106_spi_write(emu: *mut snd_ca0106, data: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    /* CONFIG_PM_SLEEP declarations. */
    pub fn snd_ca0106_mixer_suspend(chip: *mut snd_ca0106);
    pub fn snd_ca0106_mixer_resume(chip: *mut snd_ca0106);
}
/* Without CONFIG_PM_SLEEP, snd_ca0106_mixer_suspend/resume are empty do-while macros in C. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
