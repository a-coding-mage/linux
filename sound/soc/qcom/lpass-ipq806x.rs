// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * lpass-ipq806x.c -- ALSA SoC CPU DAI driver for QTi LPASS
 * Splited out the IPQ8064 soc specific from lpass-cpu.c
 */

// Rust translation of dependencies originally included from:
// <linux/clk.h>, <linux/device.h>, <linux/err.h>, <linux/kernel.h>,
// <linux/module.h>, <linux/of.h>, <linux/platform_device.h>,
// <sound/pcm.h>, <sound/soc.h>, <sound/soc-dai.h>,
// "lpass-lpaif-reg.h", and "lpass.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct clk {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
}

#[repr(C)]
pub struct lpass_data {
	pub ahbix_clk: *mut clk,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub stream_name: *const c_char,
	pub formats: c_ulong,
	pub rates: c_uint,
	pub rate_min: c_uint,
	pub rate_max: c_uint,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub id: c_int,
	pub playback: snd_soc_pcm_stream,
	pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_field {
	pub reg: c_uint,
	pub lsb: c_uint,
	pub msb: c_uint,
	pub id_size: c_uint,
	pub id_offset: c_uint,
}

#[repr(C)]
pub struct lpass_variant {
	pub i2sctrl_reg_base: c_uint,
	pub i2sctrl_reg_stride: c_uint,
	pub i2s_ports: c_uint,
	pub irq_reg_base: c_uint,
	pub irq_reg_stride: c_uint,
	pub irq_ports: c_uint,
	pub rdma_reg_base: c_uint,
	pub rdma_reg_stride: c_uint,
	pub rdma_channels: c_uint,
	pub wrdma_reg_base: c_uint,
	pub wrdma_reg_stride: c_uint,
	pub wrdma_channel_start: c_uint,
	pub wrdma_channels: c_uint,
	pub loopback: reg_field,
	pub spken: reg_field,
	pub spkmode: reg_field,
	pub spkmono: reg_field,
	pub micen: reg_field,
	pub micmode: reg_field,
	pub micmono: reg_field,
	pub wssrc: reg_field,
	pub bitwidth: reg_field,
	pub rdma_dyncclk: reg_field,
	pub rdma_bursten: reg_field,
	pub rdma_wpscnt: reg_field,
	pub rdma_intf: reg_field,
	pub rdma_fifowm: reg_field,
	pub rdma_enable: reg_field,
	pub wrdma_dyncclk: reg_field,
	pub wrdma_bursten: reg_field,
	pub wrdma_wpscnt: reg_field,
	pub wrdma_intf: reg_field,
	pub wrdma_fifowm: reg_field,
	pub wrdma_enable: reg_field,
	pub dai_driver: *mut snd_soc_dai_driver,
	pub num_dai: c_uint,
	pub dai_osr_clk_names: *const *const c_char,
	pub dai_bit_clk_names: *const *const c_char,
	pub init: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub exit: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub alloc_dma_channel: Option<unsafe extern "C" fn(*mut lpass_data, c_int, c_uint) -> c_int>,
	pub free_dma_channel: Option<unsafe extern "C" fn(*mut lpass_data, c_int, c_uint) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
	pub compatible: *const c_char,
	pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
	pub name: *const c_char,
	pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
	pub driver: device_driver,
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const SNDRV_PCM_FMTBIT_S16: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S24: c_ulong = 1 << 6;
const SNDRV_PCM_FMTBIT_S32: c_ulong = 1 << 10;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 10;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 13;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const EINVAL: c_int = 22;
const LPASS_AHBIX_CLOCK_FREQUENCY: c_ulong = 131072000;

const fn REG_FIELD_ID(reg: c_uint, lsb: c_uint, msb: c_uint, id_size: c_uint, id_offset: c_uint) -> reg_field {
	reg_field {
		reg,
		lsb,
		msb,
		id_size,
		id_offset,
	}
}

unsafe extern "C" {
	static asoc_qcom_lpass_cpu_dai_ops: snd_soc_dai_ops;

	fn platform_get_drvdata(pdev: *mut platform_device) -> *mut lpass_data;
	fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_long;
	fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
	fn clk_get_rate(clk: *mut clk) -> c_ulong;
	fn clk_prepare_enable(clk: *mut clk) -> c_int;
	fn clk_disable_unprepare(clk: *mut clk);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn asoc_qcom_lpass_cpu_platform_probe(pdev: *mut platform_device) -> c_int;
	fn asoc_qcom_lpass_cpu_platform_remove(pdev: *mut platform_device) -> c_int;
}

#[repr(C)]
enum lpaif_i2s_ports {
	IPQ806X_LPAIF_I2S_PORT_CODEC_SPK,
	IPQ806X_LPAIF_I2S_PORT_CODEC_MIC,
	IPQ806X_LPAIF_I2S_PORT_SEC_SPK,
	IPQ806X_LPAIF_I2S_PORT_SEC_MIC,
	IPQ806X_LPAIF_I2S_PORT_MI2S,
}

#[repr(C)]
enum lpaif_dma_channels {
	IPQ806X_LPAIF_RDMA_CHAN_MI2S,
	IPQ806X_LPAIF_RDMA_CHAN_PCM0,
	IPQ806X_LPAIF_RDMA_CHAN_PCM1,
}

static mut ipq806x_lpass_cpu_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
	id: lpaif_i2s_ports::IPQ806X_LPAIF_I2S_PORT_MI2S as c_int,
	playback: snd_soc_pcm_stream {
		stream_name: c"lpass-cpu-playback".as_ptr(),
		formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
		rates: SNDRV_PCM_RATE_8000
			| SNDRV_PCM_RATE_16000
			| SNDRV_PCM_RATE_32000
			| SNDRV_PCM_RATE_48000
			| SNDRV_PCM_RATE_96000,
		rate_min: 8000,
		rate_max: 96000,
		channels_min: 1,
		channels_max: 8,
	},
	ops: unsafe { &asoc_qcom_lpass_cpu_dai_ops },
};

unsafe extern "C" fn ipq806x_lpass_init(pdev: *mut platform_device) -> c_int {
	let drvdata: *mut lpass_data = platform_get_drvdata(pdev);
	let dev: *mut device = &mut (*pdev).dev;
	let mut ret: c_int;

	(*drvdata).ahbix_clk = devm_clk_get(dev, c"ahbix-clk".as_ptr());
	if IS_ERR((*drvdata).ahbix_clk as *const c_void) {
		dev_err(
			dev,
			c"error getting ahbix-clk: %ld\n".as_ptr(),
			PTR_ERR((*drvdata).ahbix_clk as *const c_void),
		);
		ret = PTR_ERR((*drvdata).ahbix_clk as *const c_void) as c_int;
		return ret;
	}

	ret = clk_set_rate((*drvdata).ahbix_clk, LPASS_AHBIX_CLOCK_FREQUENCY);
	if ret != 0 {
		dev_err(dev, c"error setting rate on ahbix_clk: %d\n".as_ptr(), ret);
		return ret;
	}
	dev_dbg(
		dev,
		c"set ahbix_clk rate to %lu\n".as_ptr(),
		clk_get_rate((*drvdata).ahbix_clk),
	);

	ret = clk_prepare_enable((*drvdata).ahbix_clk);
	if ret != 0 {
		dev_err(dev, c"error enabling ahbix_clk: %d\n".as_ptr(), ret);
		return ret;
	}

	ret
}

unsafe extern "C" fn ipq806x_lpass_exit(pdev: *mut platform_device) -> c_int {
	let drvdata: *mut lpass_data = platform_get_drvdata(pdev);

	clk_disable_unprepare((*drvdata).ahbix_clk);

	0
}

unsafe extern "C" fn ipq806x_lpass_alloc_dma_channel(
	_drvdata: *mut lpass_data,
	dir: c_int,
	_dai_id: c_uint,
) -> c_int {
	if dir == SNDRV_PCM_STREAM_PLAYBACK {
		lpaif_dma_channels::IPQ806X_LPAIF_RDMA_CHAN_MI2S as c_int
	} else {
		/* Capture currently not implemented */
		-EINVAL
	}
}

unsafe extern "C" fn ipq806x_lpass_free_dma_channel(
	_drvdata: *mut lpass_data,
	_chan: c_int,
	_dai_id: c_uint,
) -> c_int {
	0
}

static DAI_OSR_CLK_NAMES: [*const c_char; 1] = [c"mi2s-osr-clk".as_ptr()];
static DAI_BIT_CLK_NAMES: [*const c_char; 1] = [c"mi2s-bit-clk".as_ptr()];

static ipq806x_data: lpass_variant = lpass_variant {
	i2sctrl_reg_base: 0x0010,
	i2sctrl_reg_stride: 0x04,
	i2s_ports: 5,
	irq_reg_base: 0x3000,
	irq_reg_stride: 0x1000,
	irq_ports: 3,
	rdma_reg_base: 0x6000,
	rdma_reg_stride: 0x1000,
	rdma_channels: 4,
	wrdma_reg_base: 0xB000,
	wrdma_reg_stride: 0x1000,
	wrdma_channel_start: 5,
	wrdma_channels: 4,
	loopback: REG_FIELD_ID(0x0010, 15, 15, 5, 0x4),
	spken: REG_FIELD_ID(0x0010, 14, 14, 5, 0x4),
	spkmode: REG_FIELD_ID(0x0010, 10, 13, 5, 0x4),
	spkmono: REG_FIELD_ID(0x0010, 9, 9, 5, 0x4),
	micen: REG_FIELD_ID(0x0010, 8, 8, 5, 0x4),
	micmode: REG_FIELD_ID(0x0010, 4, 7, 5, 0x4),
	micmono: REG_FIELD_ID(0x0010, 3, 3, 5, 0x4),
	wssrc: REG_FIELD_ID(0x0010, 2, 2, 5, 0x4),
	bitwidth: REG_FIELD_ID(0x0010, 0, 1, 5, 0x4),

	rdma_dyncclk: REG_FIELD_ID(0x6000, 12, 12, 4, 0x1000),
	rdma_bursten: REG_FIELD_ID(0x6000, 11, 11, 4, 0x1000),
	rdma_wpscnt: REG_FIELD_ID(0x6000, 8, 10, 4, 0x1000),
	rdma_intf: REG_FIELD_ID(0x6000, 4, 7, 4, 0x1000),
	rdma_fifowm: REG_FIELD_ID(0x6000, 1, 3, 4, 0x1000),
	rdma_enable: REG_FIELD_ID(0x6000, 0, 0, 4, 0x1000),

	wrdma_dyncclk: REG_FIELD_ID(0xB000, 12, 12, 4, 0x1000),
	wrdma_bursten: REG_FIELD_ID(0xB000, 11, 11, 4, 0x1000),
	wrdma_wpscnt: REG_FIELD_ID(0xB000, 8, 10, 4, 0x1000),
	wrdma_intf: REG_FIELD_ID(0xB000, 4, 7, 4, 0x1000),
	wrdma_fifowm: REG_FIELD_ID(0xB000, 1, 3, 4, 0x1000),
	wrdma_enable: REG_FIELD_ID(0xB000, 0, 0, 4, 0x1000),

	dai_driver: unsafe { &raw mut ipq806x_lpass_cpu_dai_driver },
	num_dai: 1,
	dai_osr_clk_names: DAI_OSR_CLK_NAMES.as_ptr(),
	dai_bit_clk_names: DAI_BIT_CLK_NAMES.as_ptr(),
	init: Some(ipq806x_lpass_init),
	exit: Some(ipq806x_lpass_exit),
	alloc_dma_channel: Some(ipq806x_lpass_alloc_dma_channel),
	free_dma_channel: Some(ipq806x_lpass_free_dma_channel),
};

// Original C declaration used __maybe_unused.
static ipq806x_lpass_cpu_device_id: [of_device_id; 2] = [
	of_device_id {
		compatible: c"qcom,lpass-cpu".as_ptr(),
		data: &ipq806x_data as *const lpass_variant as *const c_void,
	},
	of_device_id {
		compatible: core::ptr::null(),
		data: core::ptr::null(),
	},
];

// MODULE_DEVICE_TABLE(of, ipq806x_lpass_cpu_device_id);

static mut ipq806x_lpass_cpu_platform_driver: platform_driver = platform_driver {
	driver: device_driver {
		name: c"lpass-cpu".as_ptr(),
		of_match_table: ipq806x_lpass_cpu_device_id.as_ptr(),
	},
	probe: Some(asoc_qcom_lpass_cpu_platform_probe),
	remove: Some(asoc_qcom_lpass_cpu_platform_remove),
};

// module_platform_driver(ipq806x_lpass_cpu_platform_driver);
// MODULE_DESCRIPTION("QTi LPASS CPU Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
