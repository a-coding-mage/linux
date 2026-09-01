// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ACP PCI driver callback routines for ACP6.3, ACP7.0 & ACP7.1
 * platforms.
 *
 * Copyright 2025 Advanced Micro Devices, Inc.
 * Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>
 */

use core::ffi::{c_char, c_int, c_void};

type u32 = u32;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
	pub dev: device,
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sdw_dev_data {
	pub pdev: [*mut platform_device; 2],
}

#[repr(C)]
pub struct sdw_dma_dev_data {
	pub acp63_sdw0_dma_stream: [*mut snd_pcm_substream; ACP63_SDW0_DMA_MAX_STREAMS as usize],
	pub acp63_sdw1_dma_stream: [*mut snd_pcm_substream; ACP63_SDW1_DMA_MAX_STREAMS as usize],
	pub acp70_sdw0_dma_stream: [*mut snd_pcm_substream; ACP70_SDW0_DMA_MAX_STREAMS as usize],
	pub acp70_sdw1_dma_stream: [*mut snd_pcm_substream; ACP70_SDW1_DMA_MAX_STREAMS as usize],
}

#[repr(C)]
pub struct acp63_dev_data {
	pub acp63_base: *mut c_void,
	pub is_pdm_config: bool,
	pub is_sdw_config: bool,
	pub is_sdw_dev: bool,
	pub acp_sw_pad_keeper_en: u32,
	pub acp_pad_pulldown_ctrl: u32,
	pub sdw_en_stat: bool,
	pub sdw: *mut sdw_dev_data,
	pub sdw_dma_dev: *mut platform_device,
	pub acp63_sdw0_dma_intr_stat: [u32; ACP63_SDW0_DMA_MAX_STREAMS as usize],
	pub acp63_sdw1_dma_intr_stat: [u32; ACP63_SDW1_DMA_MAX_STREAMS as usize],
	pub acp70_sdw0_dma_intr_stat: [u32; ACP70_SDW0_DMA_MAX_STREAMS as usize],
	pub acp70_sdw1_dma_intr_stat: [u32; ACP70_SDW1_DMA_MAX_STREAMS as usize],
}

#[repr(C)]
pub struct acp_hw_ops {
	pub acp_init: Option<unsafe extern "C" fn(*mut c_void, *mut device) -> c_int>,
	pub acp_deinit: Option<unsafe extern "C" fn(*mut c_void, *mut device) -> c_int>,
	pub acp_get_config: Option<unsafe extern "C" fn(*mut pci_dev, *mut acp63_dev_data)>,
	pub acp_sdw_dma_irq_thread: Option<unsafe extern "C" fn(*mut acp63_dev_data)>,
	pub acp_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub acp_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub acp_suspend_runtime: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	pub acp_resume_runtime: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

unsafe extern "C" {
	fn readl(addr: *mut c_void) -> u32;
	fn writel(val: u32, addr: *mut c_void);
	fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
	fn pm_request_resume(dev: *mut device) -> c_int;
	fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
	fn acp_hw_init(adata: *mut acp63_dev_data, dev: *mut device) -> c_int;
	fn acp_hw_deinit(adata: *mut acp63_dev_data, dev: *mut device) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn acp_reg(base: *mut c_void, offset: u32) -> *mut c_void {
	(base as *mut u8).add(offset as usize) as *mut c_void
}

unsafe fn readl_poll_timeout_bool(
	addr: *mut c_void,
	val: *mut u32,
	condition: unsafe fn(u32) -> bool,
	delay_us: u32,
	timeout_us: u32,
) -> c_int {
	let mut elapsed: u32 = 0;

	loop {
		*val = readl(addr);
		if condition(*val) {
			return 0;
		}
		if elapsed >= timeout_us {
			return -110;
		}
		elapsed = elapsed.wrapping_add(delay_us);
	}
}

unsafe fn poll_zero(val: u32) -> bool {
	!val != 0
}

unsafe fn poll_softreset_auddone(val: u32) -> bool {
	(val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK) != 0
}

unsafe extern "C" fn acp63_power_on(acp_base: *mut c_void) -> c_int {
	let mut val: u32;

	val = readl(acp_reg(acp_base, ACP_PGFSM_STATUS));

	if val == 0 {
		return val as c_int;
	}

	if (val & ACP63_PGFSM_STATUS_MASK) != ACP63_POWER_ON_IN_PROGRESS {
		writel(ACP63_PGFSM_CNTL_POWER_ON_MASK, acp_reg(acp_base, ACP_PGFSM_CONTROL));
	}

	readl_poll_timeout_bool(acp_reg(acp_base, ACP_PGFSM_STATUS), &mut val, poll_zero, DELAY_US, ACP63_TIMEOUT)
}

unsafe extern "C" fn acp63_reset(acp_base: *mut c_void) -> c_int {
	let mut val: u32;
	let mut ret: c_int;

	writel(1, acp_reg(acp_base, ACP_SOFT_RESET));

	ret = readl_poll_timeout_bool(
		acp_reg(acp_base, ACP_SOFT_RESET),
		&mut val,
		poll_softreset_auddone,
		DELAY_US,
		ACP63_TIMEOUT,
	);
	if ret != 0 {
		return ret;
	}

	writel(0, acp_reg(acp_base, ACP_SOFT_RESET));

	readl_poll_timeout_bool(acp_reg(acp_base, ACP_SOFT_RESET), &mut val, poll_zero, DELAY_US, ACP63_TIMEOUT)
}

unsafe extern "C" fn acp63_enable_interrupts(acp_base: *mut c_void) {
	writel(1, acp_reg(acp_base, ACP_EXTERNAL_INTR_ENB));
	writel(ACP_ERROR_IRQ, acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL));
}

unsafe extern "C" fn acp63_disable_interrupts(acp_base: *mut c_void) {
	writel(ACP_EXT_INTR_STAT_CLEAR_MASK, acp_reg(acp_base, ACP_EXTERNAL_INTR_STAT));
	writel(0, acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL));
	writel(0, acp_reg(acp_base, ACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp63_init(acp_base: *mut c_void, dev: *mut device) -> c_int {
	let mut ret: c_int;

	ret = acp63_power_on(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP power on failed\n".as_ptr());
		return ret;
	}
	writel(0x01, acp_reg(acp_base, ACP_CONTROL));
	ret = acp63_reset(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP reset failed\n".as_ptr());
		return ret;
	}
	acp63_enable_interrupts(acp_base);
	writel(0, acp_reg(acp_base, ACP_ZSC_DSP_CTRL));
	0
}

unsafe extern "C" fn acp63_deinit(acp_base: *mut c_void, dev: *mut device) -> c_int {
	let mut ret: c_int;

	acp63_disable_interrupts(acp_base);
	ret = acp63_reset(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP reset failed\n".as_ptr());
		return ret;
	}
	writel(0, acp_reg(acp_base, ACP_CONTROL));
	writel(1, acp_reg(acp_base, ACP_ZSC_DSP_CTRL));
	0
}

unsafe extern "C" fn acp63_get_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data) {
	let config: u32;

	config = readl(acp_reg((*acp_data).acp63_base, ACP_PIN_CONFIG));
	dev_dbg(&mut (*pci).dev, c"ACP config value: %d\n".as_ptr(), config);
	match config {
		ACP_CONFIG_4 | ACP_CONFIG_5 | ACP_CONFIG_10 | ACP_CONFIG_11 => {
			(*acp_data).is_pdm_config = true;
		}
		ACP_CONFIG_2 | ACP_CONFIG_3 => {
			(*acp_data).is_sdw_config = true;
		}
		ACP_CONFIG_6 | ACP_CONFIG_7 | ACP_CONFIG_12 | ACP_CONFIG_8 | ACP_CONFIG_13 | ACP_CONFIG_14 => {
			(*acp_data).is_pdm_config = true;
			(*acp_data).is_sdw_config = true;
		}
		_ => {}
	}
}

unsafe extern "C" fn check_acp_sdw_enable_status(adata: *mut acp63_dev_data) -> bool {
	let sdw0_en: u32;
	let sdw1_en: u32;

	sdw0_en = readl(acp_reg((*adata).acp63_base, ACP_SW0_EN));
	sdw1_en = readl(acp_reg((*adata).acp63_base, ACP_SW1_EN));
	(sdw0_en != 0) || (sdw1_en != 0)
}

unsafe extern "C" fn handle_acp63_sdw_pme_event(adata: *mut acp63_dev_data) {
	let mut val: u32;

	val = readl(acp_reg((*adata).acp63_base, ACP_SW0_WAKE_EN));
	if val != 0 && !(*(*adata).sdw).pdev[0].is_null() {
		pm_request_resume(&mut (*(*(*adata).sdw).pdev[0]).dev);
	}

	val = readl(acp_reg((*adata).acp63_base, ACP_SW1_WAKE_EN));
	if val != 0 && !(*(*adata).sdw).pdev[1].is_null() {
		pm_request_resume(&mut (*(*(*adata).sdw).pdev[1]).dev);
	}
}

unsafe extern "C" fn snd_acp63_suspend(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;
	if (*adata).is_sdw_dev {
		(*adata).acp_sw_pad_keeper_en = readl(acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
		(*adata).acp_pad_pulldown_ctrl = readl(acp_reg((*adata).acp63_base, ACP_PAD_PULLDOWN_CTRL));
		(*adata).sdw_en_stat = check_acp_sdw_enable_status(adata);
		if (*adata).sdw_en_stat {
			writel(1, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
			return 0;
		}
	}
	ret = acp_hw_deinit(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP de-init failed\n".as_ptr());
	}

	ret
}

unsafe extern "C" fn snd_acp63_runtime_resume(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;
	if (*adata).sdw_en_stat {
		writel(0, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
		return 0;
	}
	ret = acp_hw_init(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP init failed\n".as_ptr());
		return ret;
	}

	if !(*adata).sdw_en_stat {
		handle_acp63_sdw_pme_event(adata);
	}
	0
}

unsafe extern "C" fn snd_acp63_resume(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let acp_sw_pad_keeper_en: u32;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;
	if (*adata).sdw_en_stat {
		writel(0, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
		return 0;
	}

	ret = acp_hw_init(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP init failed\n".as_ptr());
	}

	acp_sw_pad_keeper_en = readl(acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
	dev_dbg(dev, c"ACP_SW0_PAD_KEEPER_EN:0x%x\n".as_ptr(), acp_sw_pad_keeper_en);
	if acp_sw_pad_keeper_en == 0 {
		writel((*adata).acp_sw_pad_keeper_en, acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
		writel((*adata).acp_pad_pulldown_ctrl, acp_reg((*adata).acp63_base, ACP_PAD_PULLDOWN_CTRL));
	}
	ret
}

unsafe extern "C" fn acp63_sdw_dma_irq_thread(adata: *mut acp63_dev_data) {
	let sdw_data: *mut sdw_dma_dev_data;
	let mut stream_id: u32;

	sdw_data = dev_get_drvdata(&mut (*(*adata).sdw_dma_dev).dev) as *mut sdw_dma_dev_data;

	stream_id = 0;
	while stream_id < ACP63_SDW0_DMA_MAX_STREAMS {
		if (*adata).acp63_sdw0_dma_intr_stat[stream_id as usize] != 0 {
			if !(*sdw_data).acp63_sdw0_dma_stream[stream_id as usize].is_null() {
				snd_pcm_period_elapsed((*sdw_data).acp63_sdw0_dma_stream[stream_id as usize]);
			}
			(*adata).acp63_sdw0_dma_intr_stat[stream_id as usize] = 0;
		}
		stream_id = stream_id.wrapping_add(1);
	}
	stream_id = 0;
	while stream_id < ACP63_SDW1_DMA_MAX_STREAMS {
		if (*adata).acp63_sdw1_dma_intr_stat[stream_id as usize] != 0 {
			if !(*sdw_data).acp63_sdw1_dma_stream[stream_id as usize].is_null() {
				snd_pcm_period_elapsed((*sdw_data).acp63_sdw1_dma_stream[stream_id as usize]);
			}
			(*adata).acp63_sdw1_dma_intr_stat[stream_id as usize] = 0;
		}
		stream_id = stream_id.wrapping_add(1);
	}
}

#[no_mangle]
pub unsafe extern "C" fn acp63_hw_init_ops(hw_ops: *mut acp_hw_ops) {
	(*hw_ops).acp_init = Some(acp63_init);
	(*hw_ops).acp_deinit = Some(acp63_deinit);
	(*hw_ops).acp_get_config = Some(acp63_get_config);
	(*hw_ops).acp_sdw_dma_irq_thread = Some(acp63_sdw_dma_irq_thread);
	(*hw_ops).acp_suspend = Some(snd_acp63_suspend);
	(*hw_ops).acp_resume = Some(snd_acp63_resume);
	(*hw_ops).acp_suspend_runtime = Some(snd_acp63_suspend);
	(*hw_ops).acp_resume_runtime = Some(snd_acp63_runtime_resume);
}

unsafe extern "C" fn acp70_power_on(acp_base: *mut c_void) -> c_int {
	let mut val: u32 = 0;

	val = readl(acp_reg(acp_base, ACP_PGFSM_STATUS));

	if val == 0 {
		return 0;
	}
	if (val & ACP70_PGFSM_STATUS_MASK) != 0 {
		writel(ACP70_PGFSM_CNTL_POWER_ON_MASK, acp_reg(acp_base, ACP_PGFSM_CONTROL));
	}

	readl_poll_timeout_bool(acp_reg(acp_base, ACP_PGFSM_STATUS), &mut val, poll_zero, DELAY_US, ACP70_TIMEOUT)
}

unsafe extern "C" fn acp70_reset(acp_base: *mut c_void) -> c_int {
	let mut val: u32;
	let ret: c_int;

	writel(1, acp_reg(acp_base, ACP_SOFT_RESET));

	ret = readl_poll_timeout_bool(
		acp_reg(acp_base, ACP_SOFT_RESET),
		&mut val,
		poll_softreset_auddone,
		DELAY_US,
		ACP70_TIMEOUT,
	);
	if ret != 0 {
		return ret;
	}

	writel(0, acp_reg(acp_base, ACP_SOFT_RESET));

	readl_poll_timeout_bool(acp_reg(acp_base, ACP_SOFT_RESET), &mut val, poll_zero, DELAY_US, ACP70_TIMEOUT)
}

unsafe extern "C" fn acp70_enable_sdw_host_wake_interrupts(acp_base: *mut c_void) {
	let mut ext_intr_cntl1: u32;

	ext_intr_cntl1 = readl(acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL1));
	ext_intr_cntl1 |= ACP70_SDW_HOST_WAKE_MASK;
	writel(ext_intr_cntl1, acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL1));
}

unsafe extern "C" fn acp70_enable_interrupts(acp_base: *mut c_void) {
	let sdw0_wake_en: u32;
	let sdw1_wake_en: u32;

	writel(1, acp_reg(acp_base, ACP_EXTERNAL_INTR_ENB));
	writel(ACP_ERROR_IRQ, acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL));
	sdw0_wake_en = readl(acp_reg(acp_base, ACP_SW0_WAKE_EN));
	sdw1_wake_en = readl(acp_reg(acp_base, ACP_SW1_WAKE_EN));
	if sdw0_wake_en != 0 || sdw1_wake_en != 0 {
		acp70_enable_sdw_host_wake_interrupts(acp_base);
	}
}

unsafe extern "C" fn acp70_disable_interrupts(acp_base: *mut c_void) {
	writel(ACP_EXT_INTR_STAT_CLEAR_MASK, acp_reg(acp_base, ACP_EXTERNAL_INTR_STAT));
	writel(0, acp_reg(acp_base, ACP_EXTERNAL_INTR_CNTL));
	writel(0, acp_reg(acp_base, ACP_EXTERNAL_INTR_ENB));
}

unsafe extern "C" fn acp70_init(acp_base: *mut c_void, dev: *mut device) -> c_int {
	let mut ret: c_int;

	ret = acp70_power_on(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP power on failed\n".as_ptr());
		return ret;
	}
	writel(0x01, acp_reg(acp_base, ACP_CONTROL));
	ret = acp70_reset(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP reset failed\n".as_ptr());
		return ret;
	}
	writel(0, acp_reg(acp_base, ACP_ZSC_DSP_CTRL));
	acp70_enable_interrupts(acp_base);
	writel(0x1, acp_reg(acp_base, ACP_PME_EN));
	0
}

unsafe extern "C" fn acp70_deinit(acp_base: *mut c_void, dev: *mut device) -> c_int {
	let ret: c_int;

	acp70_disable_interrupts(acp_base);
	ret = acp70_reset(acp_base);
	if ret != 0 {
		dev_err(dev, c"ACP reset failed\n".as_ptr());
		return ret;
	}
	writel(0x01, acp_reg(acp_base, ACP_ZSC_DSP_CTRL));
	0
}

unsafe extern "C" fn acp70_get_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data) {
	let config: u32;

	config = readl(acp_reg((*acp_data).acp63_base, ACP_PIN_CONFIG));
	dev_dbg(&mut (*pci).dev, c"ACP config value: %d\n".as_ptr(), config);
	match config {
		ACP_CONFIG_4 | ACP_CONFIG_5 | ACP_CONFIG_10 | ACP_CONFIG_11 | ACP_CONFIG_20 => {
			(*acp_data).is_pdm_config = true;
		}
		ACP_CONFIG_2 | ACP_CONFIG_3 | ACP_CONFIG_16 => {
			(*acp_data).is_sdw_config = true;
		}
		ACP_CONFIG_6
		| ACP_CONFIG_7
		| ACP_CONFIG_12
		| ACP_CONFIG_8
		| ACP_CONFIG_13
		| ACP_CONFIG_14
		| ACP_CONFIG_17
		| ACP_CONFIG_18
		| ACP_CONFIG_19 => {
			(*acp_data).is_pdm_config = true;
			(*acp_data).is_sdw_config = true;
		}
		_ => {}
	}
}

unsafe extern "C" fn acp70_sdw_dma_irq_thread(adata: *mut acp63_dev_data) {
	let sdw_data: *mut sdw_dma_dev_data;
	let mut stream_id: u32;

	sdw_data = dev_get_drvdata(&mut (*(*adata).sdw_dma_dev).dev) as *mut sdw_dma_dev_data;

	stream_id = 0;
	while stream_id < ACP70_SDW0_DMA_MAX_STREAMS {
		if (*adata).acp70_sdw0_dma_intr_stat[stream_id as usize] != 0 {
			if !(*sdw_data).acp70_sdw0_dma_stream[stream_id as usize].is_null() {
				snd_pcm_period_elapsed((*sdw_data).acp70_sdw0_dma_stream[stream_id as usize]);
			}
			(*adata).acp70_sdw0_dma_intr_stat[stream_id as usize] = 0;
		}
		stream_id = stream_id.wrapping_add(1);
	}
	stream_id = 0;
	while stream_id < ACP70_SDW1_DMA_MAX_STREAMS {
		if (*adata).acp70_sdw1_dma_intr_stat[stream_id as usize] != 0 {
			if !(*sdw_data).acp70_sdw1_dma_stream[stream_id as usize].is_null() {
				snd_pcm_period_elapsed((*sdw_data).acp70_sdw1_dma_stream[stream_id as usize]);
			}
			(*adata).acp70_sdw1_dma_intr_stat[stream_id as usize] = 0;
		}
		stream_id = stream_id.wrapping_add(1);
	}
}

unsafe extern "C" fn snd_acp70_suspend(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;
	if (*adata).is_sdw_dev {
		(*adata).acp_sw_pad_keeper_en = readl(acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
		(*adata).acp_pad_pulldown_ctrl = readl(acp_reg((*adata).acp63_base, ACP_PAD_PULLDOWN_CTRL));
		(*adata).sdw_en_stat = check_acp_sdw_enable_status(adata);
		if (*adata).sdw_en_stat {
			writel(1, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
			return 0;
		}
	}
	ret = acp_hw_deinit(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP de-init failed\n".as_ptr());
	}

	ret
}

unsafe extern "C" fn snd_acp70_runtime_resume(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

	if (*adata).sdw_en_stat {
		writel(0, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
		writel(1, acp_reg((*adata).acp63_base, ACP_PME_EN));
		return 0;
	}

	ret = acp_hw_init(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP init failed\n".as_ptr());
		return ret;
	}
	0
}

unsafe extern "C" fn snd_acp70_resume(dev: *mut device) -> c_int {
	let adata: *mut acp63_dev_data;
	let acp_sw_pad_keeper_en: u32;
	let ret: c_int;

	adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

	if (*adata).sdw_en_stat {
		writel(0, acp_reg((*adata).acp63_base, ACP_ZSC_DSP_CTRL));
		writel(1, acp_reg((*adata).acp63_base, ACP_PME_EN));
		return 0;
	}

	ret = acp_hw_init(adata, dev);
	if ret != 0 {
		dev_err(dev, c"ACP init failed\n".as_ptr());
	}

	acp_sw_pad_keeper_en = readl(acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
	dev_dbg(dev, c"ACP_SW0_PAD_KEEPER_EN:0x%x\n".as_ptr(), acp_sw_pad_keeper_en);
	if acp_sw_pad_keeper_en == 0 {
		writel((*adata).acp_sw_pad_keeper_en, acp_reg((*adata).acp63_base, ACP_SW0_PAD_KEEPER_EN));
		writel((*adata).acp_pad_pulldown_ctrl, acp_reg((*adata).acp63_base, ACP_PAD_PULLDOWN_CTRL));
	}
	ret
}

#[no_mangle]
pub unsafe extern "C" fn acp70_hw_init_ops(hw_ops: *mut acp_hw_ops) {
	(*hw_ops).acp_init = Some(acp70_init);
	(*hw_ops).acp_deinit = Some(acp70_deinit);
	(*hw_ops).acp_get_config = Some(acp70_get_config);
	(*hw_ops).acp_sdw_dma_irq_thread = Some(acp70_sdw_dma_irq_thread);
	(*hw_ops).acp_suspend = Some(snd_acp70_suspend);
	(*hw_ops).acp_resume = Some(snd_acp70_resume);
	(*hw_ops).acp_suspend_runtime = Some(snd_acp70_suspend);
	(*hw_ops).acp_resume_runtime = Some(snd_acp70_runtime_resume);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
