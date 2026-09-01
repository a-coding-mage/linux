// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

const ECHOGALS_FAMILY: bool = true;
const ECHOCARD_GINA20: bool = true;
const ECHOCARD_NAME: &str = "Gina20";
const ECHOCARD_HAS_MONITOR: bool = true;
const ECHOCARD_HAS_INPUT_GAIN: bool = true;
const ECHOCARD_HAS_DIGITAL_IO: bool = true;
const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
const ECHOCARD_HAS_ADAT: bool = false;

/* Pipe indexes */
const PX_ANALOG_OUT: u32 = 0;	/* 8 */
const PX_DIGITAL_OUT: u32 = 8;	/* 2 */
const PX_ANALOG_IN: u32 = 10;	/* 2 */
const PX_DIGITAL_IN: u32 = 12;	/* 2 */
const PX_NUM: u32 = 14;

/* Bus indexes */
const BX_ANALOG_OUT: u32 = 0;	/* 8 */
const BX_DIGITAL_OUT: u32 = 8;	/* 2 */
const BX_ANALOG_IN: u32 = 10;	/* 2 */
const BX_DIGITAL_IN: u32 = 12;	/* 2 */
const BX_NUM: u32 = 14;

/*
 * C dependencies removed from executable Rust:
 * <linux/delay.h>, <linux/init.h>, <linux/interrupt.h>, <linux/pci.h>,
 * <linux/module.h>, <linux/firmware.h>, <linux/slab.h>, <linux/io.h>,
 * <sound/core.h>, <sound/info.h>, <sound/control.h>, <sound/tlv.h>,
 * <sound/pcm.h>, <sound/pcm_params.h>, <sound/asoundef.h>,
 * <sound/initval.h>, <linux/atomic.h>, and "echoaudio.h".
 */

module_firmware!("ea/gina20_dsp.fw");

const FW_GINA20_DSP: u32 = 0;

static CARD_FW: [firmware; 1] = [
	firmware {
		size: 0,
		data: c"gina20_dsp.fw".as_ptr(),
	},
];

static SND_ECHO_IDS: [pci_device_id; 2] = [
	PCI_DEVICE_SUB!(0x1057, 0x1801, 0xECC0, 0x0020),	/* DSP 56301 Gina20 rev.0 */
	pci_device_id {},
];

static PCM_HARDWARE_SKEL: snd_pcm_hardware = snd_pcm_hardware {
	info: SNDRV_PCM_INFO_MMAP |
		SNDRV_PCM_INFO_INTERLEAVED |
		SNDRV_PCM_INFO_BLOCK_TRANSFER |
		SNDRV_PCM_INFO_MMAP_VALID |
		SNDRV_PCM_INFO_PAUSE |
		SNDRV_PCM_INFO_SYNC_START,
	formats: SNDRV_PCM_FMTBIT_U8 |
		SNDRV_PCM_FMTBIT_S16_LE |
		SNDRV_PCM_FMTBIT_S24_3LE |
		SNDRV_PCM_FMTBIT_S32_LE |
		SNDRV_PCM_FMTBIT_S32_BE,
	rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
	rate_min: 44100,
	rate_max: 48000,
	channels_min: 1,
	channels_max: 2,
	buffer_bytes_max: 262144,
	period_bytes_min: 32,
	period_bytes_max: 131072,
	periods_min: 2,
	periods_max: 220,
	/* One page (4k) contains 512 instructions. I don't know if the hw
	supports lists longer than this. In this case periods_max=220 is a
	safe limit to make sure the list never exceeds 512 instructions. */
};

/*
 * C source inclusions preserved as dependency intent:
 * "gina20_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
