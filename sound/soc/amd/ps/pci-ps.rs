// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD common ACP PCI driver for ACP6.3, ACP7.0 & ACP7.1 platforms.
 *
 * Copyright 2022, 2025 Advanced Micro Devices, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type acpi_handle = *mut c_void;
type acpi_integer = u64;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
	pub dev: device,
	pub revision: u8,
	pub subsystem_vendor: u16,
	pub subsystem_device: u16,
	pub irq: c_uint,
	pub msi_cap: u8,
}

#[repr(C)]
pub struct pci_device_id {
	pub vendor: u32,
	pub device: u32,
	pub subvendor: u32,
	pub subdevice: u32,
	pub class: u32,
	pub class_mask: u32,
	pub driver_data: usize,
}

#[repr(C)]
pub struct pci_driver_driver {
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
	pub name: *const c_char,
	pub id_table: *const pci_device_id,
	pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
	pub driver: pci_driver_driver,
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	_private: [u8; 0],
}

#[repr(C)]
pub struct resource {
	pub start: u32,
	pub end: u32,
	pub flags: c_uint,
}

#[repr(C)]
pub struct fwnode_handle {
	_private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
}

#[repr(C)]
pub struct platform_device_info {
	pub parent: *mut device,
	pub fwnode: *mut fwnode_handle,
	pub name: *mut c_char,
	pub id: c_uint,
	pub res: *const resource,
	pub num_res: c_uint,
	pub data: *const c_void,
	pub size_data: size_t,
}

#[repr(C)]
pub struct acpi_device {
	pub handle: acpi_handle,
}

#[repr(C)]
pub struct acpi_object_integer {
	pub value: acpi_integer,
}

#[repr(C)]
pub struct acpi_object {
	pub integer: acpi_object_integer,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
	pub num_adr: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
	pub links: *const snd_soc_acpi_link_adr,
	pub link_mask: c_uint,
	pub subsystem_rev: u32,
	pub subsystem_vendor: u16,
	pub subsystem_device: u16,
	pub subsystem_id_set: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub drv_name: *const c_char,
	pub links: *const snd_soc_acpi_link_adr,
	pub link_mask: c_uint,
	pub machine_check: Option<unsafe extern "C" fn(*mut sdw_amd_ctx) -> bool>,
	pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pdm_dev_data {
	pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct work_struct {
	_private: [u8; 0],
}

#[repr(C)]
pub struct amd_sdw_manager {
	pub dev: *mut device,
	pub amd_sdw_irq_thread: work_struct,
}

#[repr(C)]
pub struct sdw_pdev_array {
	pub pdev: [*mut platform_device; 2],
}

#[repr(C)]
pub struct acp_sdw_info {
	pub handle: acpi_handle,
	pub count: c_int,
	pub link_mask: c_uint,
}

#[repr(C)]
pub struct sdw_amd_ctx {
	pub pdev: [*mut platform_device; 2],
	pub peripherals: *mut c_void,
}

#[repr(C)]
pub struct sdw_amd_res {
	pub addr: u32,
	pub reg_range: u32,
	pub handle: acpi_handle,
	pub parent: *mut device,
	pub dev: *mut device,
	pub acp_lock: *mut mutex,
	pub count: c_int,
	pub mmio_base: *mut c_void,
	pub acp_rev: u32,
	pub link_mask: c_uint,
}

#[repr(C)]
pub struct acp_hw_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct acp63_dev_data {
	pub acp70_sdw0_wake_event: bool,
	pub acp70_sdw1_wake_event: bool,
	pub sdw: *mut sdw_amd_ctx,
	pub acp63_base: *mut c_void,
	pub acp_rev: u32,
	pub acp63_sdw0_dma_intr_stat: [u16; 16],
	pub acp70_sdw0_dma_intr_stat: [u16; 16],
	pub acp63_sdw1_dma_intr_stat: [u16; 16],
	pub acp70_sdw1_dma_intr_stat: [u16; 16],
	pub pdm_dev: *mut platform_device,
	pub is_sdw_dev: bool,
	pub is_sdw_config: bool,
	pub is_pdm_dev: bool,
	pub is_pdm_config: bool,
	pub mach_dev: *mut platform_device,
	pub machines: *mut snd_soc_acpi_mach,
	pub info: acp_sdw_info,
	pub addr: u32,
	pub reg_range: u32,
	pub acp_lock: mutex,
	pub subsystem_vendor: u16,
	pub subsystem_device: u16,
	pub res: *mut resource,
	pub dmic_codec_dev: *mut platform_device,
	pub sdw_dma_dev: *mut platform_device,
	pub hw_ops: *mut acp_hw_ops,
}

extern "C" {
	static snd_soc_acpi_amd_acp70_sdw_machines: *mut snd_soc_acpi_mach;
	static snd_soc_acpi_amd_acp63_sdw_machines: *mut snd_soc_acpi_mach;
	static KBUILD_MODNAME: c_char;
	static acp63_pm_ops_generated: dev_pm_ops;

	fn readl(addr: *mut c_void) -> u32;
	fn writel(value: u32, addr: *mut c_void);
	fn dev_get_drvdata(dev: *const device) -> *mut c_void;
	fn pci_get_drvdata(pci: *mut pci_dev) -> *mut c_void;
	fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
	fn pm_request_resume(dev: *mut device) -> c_int;
	fn schedule_work(work: *mut work_struct) -> bool;
	fn acp_hw_sdw_dma_irq_thread(adata: *mut acp63_dev_data);
	fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
	fn acpi_find_child_device(parent: acpi_handle, addr: u64, check_children: c_uint)
		-> *mut acpi_device;
	fn ACPI_COMPANION(dev: *const device) -> acpi_handle;
	fn ACPI_HANDLE(dev: *const device) -> acpi_handle;
	fn acpi_dev_get_property(
		adev: *mut acpi_device,
		name: *const c_char,
		type_: c_uint,
		obj: *mut *const acpi_object,
	) -> c_int;
	fn acpi_evaluate_integer(
		handle: acpi_handle,
		pathname: *const c_char,
		arguments: *mut c_void,
		data: *mut acpi_integer,
	) -> c_int;
	fn amd_sdw_scan_controller(info: *mut acp_sdw_info) -> c_int;
	fn sdw_amd_probe(res: *mut sdw_amd_res, ctx: *mut *mut sdw_amd_ctx) -> c_int;
	fn sdw_amd_exit(ctx: *mut sdw_amd_ctx);
	fn sdw_amd_get_slave_info(ctx: *mut sdw_amd_ctx) -> c_int;
	fn snd_soc_acpi_sdw_link_slaves_found(
		dev: *mut device,
		link: *const snd_soc_acpi_link_adr,
		peripherals: *mut c_void,
	) -> bool;
	fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
	fn platform_device_register_data(
		dev: *mut device,
		name: *const c_char,
		id: c_int,
		data: *const c_void,
		size: size_t,
	) -> *mut platform_device;
	fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
	fn platform_device_unregister(pdev: *mut platform_device);
	fn acp_hw_get_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data);
	fn acp_hw_deinit(adata: *mut acp63_dev_data, dev: *mut device) -> c_int;
	fn acp63_hw_init_ops(ops: *mut acp_hw_ops);
	fn acp70_hw_init_ops(ops: *mut acp_hw_ops);
	fn snd_amd_acp_find_config(pci: *mut pci_dev) -> u32;
	fn pci_enable_device(pci: *mut pci_dev) -> c_int;
	fn pci_request_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
	fn pci_release_regions(pci: *mut pci_dev);
	fn pci_disable_device(pci: *mut pci_dev);
	fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> u32;
	fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> size_t;
	fn devm_ioremap(dev: *mut device, offset: u32, size: size_t) -> *mut c_void;
	fn pci_set_master(pci: *mut pci_dev);
	fn mutex_init(lock: *mut mutex);
	fn acp_hw_init(adata: *mut acp63_dev_data, dev: *mut device) -> c_int;
	fn devm_request_threaded_irq(
		dev: *mut device,
		irq: c_uint,
		handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
		thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
		irqflags: c_uint,
		devname: *const c_char,
		dev_id: *mut c_void,
	) -> c_int;
	fn device_set_wakeup_enable(dev: *mut device, enable: bool) -> c_int;
	fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
	fn pm_runtime_use_autosuspend(dev: *mut device);
	fn pm_runtime_put_noidle(dev: *mut device);
	fn pm_runtime_allow(dev: *mut device);
	fn acp_hw_suspend(dev: *mut device) -> c_int;
	fn acp_hw_runtime_resume(dev: *mut device) -> c_int;
	fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
	fn pci_read_config_word(pdev: *mut pci_dev, pos: c_int, val: *mut u16) -> c_int;
	fn pci_write_config_word(pdev: *mut pci_dev, pos: c_int, val: u16) -> c_int;
	fn acp_hw_resume(dev: *mut device) -> c_int;
	fn pm_runtime_forbid(dev: *mut device);
	fn pm_runtime_get_noresume(dev: *mut device);
	fn __pci_register_driver(driver: *mut pci_driver) -> c_int;
	fn pci_unregister_driver(driver: *mut pci_driver);
}

macro_rules! dev_err {
	($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {};
}

macro_rules! dev_dbg {
	($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {};
}

macro_rules! dev_warn {
	($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {};
}

const fn BIT(nr: u16) -> u32 {
	1u32 << nr
}

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_WAKE_THREAD: irqreturn_t = 2;
const IRQF_SHARED: u32 = 0x80;
const GFP_KERNEL: u32 = 0;
const IORESOURCE_MEM: u32 = 0x00000200;
const PLATFORM_DEVID_NONE: c_int = -1;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ACPI_TYPE_INTEGER: u32 = 1;
const PCI_VENDOR_ID_AMD: u32 = 0x1022;
const PCI_CLASS_MULTIMEDIA_OTHER: u32 = 0x0480;
const PCI_MSI_FLAGS: c_int = 2;
const PCI_MSI_FLAGS_ENABLE: u16 = 0x0001;

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
	(ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
	ptr as isize as c_int
}

fn ACPI_FAILURE(status: c_int) -> bool {
	status != 0
}

static ACP_AUDIO_DEVICE_TYPE: &[u8] = b"acp-audio-device-type\0";
static WOV_METHOD: &[u8] = b"_WOV\0";
static AMD_ACP63_AUDIO: &[u8] = b"AMD ACP6.3 audio\0";
static ACP_PCI_IRQ: &[u8] = b"ACP_PCI_IRQ\0";
static ACP_PS_MACH: &[u8] = b"acp_ps_mach\0";
static ACP_PS_PDM_DMA: &[u8] = b"acp_ps_pdm_dma\0";
static DMIC_CODEC: &[u8] = b"dmic-codec\0";
static AMD_PS_SDW_DMA: &[u8] = b"amd_ps_sdw_dma\0";

unsafe fn handle_acp70_sdw_wake_event(adata: *mut acp63_dev_data) {
	let mut amd_manager: *mut amd_sdw_manager;

	if (*adata).acp70_sdw0_wake_event {
		amd_manager = dev_get_drvdata(&(*(*(*adata).sdw).pdev[0]).dev) as *mut amd_sdw_manager;
		if !amd_manager.is_null() {
			pm_request_resume((*amd_manager).dev);
		}
		(*adata).acp70_sdw0_wake_event = false;
	}

	if (*adata).acp70_sdw1_wake_event {
		amd_manager = dev_get_drvdata(&(*(*(*adata).sdw).pdev[1]).dev) as *mut amd_sdw_manager;
		if !amd_manager.is_null() {
			pm_request_resume((*amd_manager).dev);
		}
		(*adata).acp70_sdw1_wake_event = false;
	}
}

unsafe fn check_and_handle_acp70_sdw_wake_irq(adata: *mut acp63_dev_data) -> c_short {
	let ext_intr_stat1: u32;
	let mut irq_flag: c_int = 0;
	let mut sdw_wake_irq = false;

	ext_intr_stat1 = readl((*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
	if (ext_intr_stat1 & ACP70_SDW0_HOST_WAKE_STAT) != 0 {
		writel(ACP70_SDW0_HOST_WAKE_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
		(*adata).acp70_sdw0_wake_event = true;
		sdw_wake_irq = true;
	}

	if (ext_intr_stat1 & ACP70_SDW1_HOST_WAKE_STAT) != 0 {
		writel(ACP70_SDW1_HOST_WAKE_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
		(*adata).acp70_sdw1_wake_event = true;
		sdw_wake_irq = true;
	}

	if (ext_intr_stat1 & ACP70_SDW0_PME_STAT) != 0 {
		writel(0, (*adata).acp63_base.add(ACP_SW0_WAKE_EN as usize));
		writel(ACP70_SDW0_PME_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
		(*adata).acp70_sdw0_wake_event = true;
		sdw_wake_irq = true;
	}

	if (ext_intr_stat1 & ACP70_SDW1_PME_STAT) != 0 {
		writel(0, (*adata).acp63_base.add(ACP_SW1_WAKE_EN as usize));
		writel(ACP70_SDW1_PME_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
		(*adata).acp70_sdw1_wake_event = true;
		sdw_wake_irq = true;
	}

	if sdw_wake_irq {
		handle_acp70_sdw_wake_event(adata);
		irq_flag = 1;
	}
	irq_flag as c_short
}

unsafe fn check_and_handle_sdw_dma_irq(
	adata: *mut acp63_dev_data,
	ext_intr_stat: u32,
	ext_intr_stat1: u32,
) -> c_short {
	let mut stream_id: u32 = 0;
	let mut sdw_dma_irq_flag: u16 = 0;
	let mut index: u16;

	if (ext_intr_stat & ACP63_SDW_DMA_IRQ_MASK) != 0 {
		index = ACP_AUDIO2_RX_THRESHOLD;
		while index <= ACP_AUDIO0_TX_THRESHOLD {
			if (ext_intr_stat & BIT(index)) != 0 {
				writel(BIT(index), (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT as usize));
				match index {
					ACP_AUDIO0_TX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO0_TX,
					ACP_AUDIO1_TX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO1_TX,
					ACP_AUDIO2_TX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO2_TX,
					ACP_AUDIO0_RX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO0_RX,
					ACP_AUDIO1_RX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO1_RX,
					ACP_AUDIO2_RX_THRESHOLD => stream_id = ACP63_SDW0_AUDIO2_RX,
					_ => {}
				}
				match (*adata).acp_rev {
					ACP63_PCI_REV => (*adata).acp63_sdw0_dma_intr_stat[stream_id as usize] = 1,
					ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
						(*adata).acp70_sdw0_dma_intr_stat[stream_id as usize] = 1
					}
					_ => {}
				}
				sdw_dma_irq_flag = 1;
			}
			index = index.wrapping_add(1);
		}
	}
	match (*adata).acp_rev {
		ACP63_PCI_REV => {
			if (ext_intr_stat1 & ACP63_P1_AUDIO1_RX_THRESHOLD) != 0 {
				writel(ACP63_P1_AUDIO1_RX_THRESHOLD, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
				(*adata).acp63_sdw1_dma_intr_stat[ACP63_SDW1_AUDIO1_RX as usize] = 1;
				sdw_dma_irq_flag = 1;
			}
			if (ext_intr_stat1 & ACP63_P1_AUDIO1_TX_THRESHOLD) != 0 {
				writel(ACP63_P1_AUDIO1_TX_THRESHOLD, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
				(*adata).acp63_sdw1_dma_intr_stat[ACP63_SDW1_AUDIO1_TX as usize] = 1;
				sdw_dma_irq_flag = 1;
			}
		}
		ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
			if (ext_intr_stat1 & ACP70_P1_SDW_DMA_IRQ_MASK) != 0 {
				index = ACP70_P1_AUDIO2_RX_THRESHOLD;
				while index <= ACP70_P1_AUDIO0_TX_THRESHOLD {
					if (ext_intr_stat1 & BIT(index)) != 0 {
						writel(BIT(index), (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
						match index {
							ACP70_P1_AUDIO0_TX_THRESHOLD => stream_id = ACP70_SDW_AUDIO0_TX,
							ACP70_P1_AUDIO1_TX_THRESHOLD => stream_id = ACP70_SDW_AUDIO1_TX,
							ACP70_P1_AUDIO2_TX_THRESHOLD => stream_id = ACP70_SDW_AUDIO2_TX,
							ACP70_P1_AUDIO0_RX_THRESHOLD => stream_id = ACP70_SDW_AUDIO0_RX,
							ACP70_P1_AUDIO1_RX_THRESHOLD => stream_id = ACP70_SDW_AUDIO1_RX,
							ACP70_P1_AUDIO2_RX_THRESHOLD => stream_id = ACP70_SDW_AUDIO2_RX,
							_ => {}
						}

						(*adata).acp70_sdw1_dma_intr_stat[stream_id as usize] = 1;
						sdw_dma_irq_flag = 1;
					}
					index = index.wrapping_add(1);
				}
			}
		}
		_ => {}
	}
	sdw_dma_irq_flag as c_short
}

unsafe extern "C" fn acp63_irq_thread(_irq: c_int, context: *mut c_void) -> irqreturn_t {
	let adata = context as *mut acp63_dev_data;

	acp_hw_sdw_dma_irq_thread(adata);
	IRQ_HANDLED
}

unsafe extern "C" fn acp63_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
	let mut adata: *mut acp63_dev_data;
	let mut ps_pdm_data: *mut pdm_dev_data;
	let mut amd_manager: *mut amd_sdw_manager;
	let ext_intr_stat: u32;
	let ext_intr_stat1: u32;
	let mut irq_flag: u16 = 0;
	let mut wake_irq_flag: u16 = 0;
	let mut sdw_dma_irq_flag: u16;

	adata = dev_id as *mut acp63_dev_data;
	if adata.is_null() {
		return IRQ_NONE;
	}
	/* ACP interrupts will be cleared by reading particular bit and writing
	 * same value to the status register. writing zero's doesn't have any
	 * effect.
	 * Bit by bit checking of IRQ field is implemented.
	 */
	ext_intr_stat = readl((*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT as usize));
	if (ext_intr_stat & ACP_SDW0_STAT) != 0 {
		writel(ACP_SDW0_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT as usize));
		amd_manager = dev_get_drvdata(&(*(*(*adata).sdw).pdev[0]).dev) as *mut amd_sdw_manager;
		if !amd_manager.is_null() {
			schedule_work(&mut (*amd_manager).amd_sdw_irq_thread);
		}
		irq_flag = 1;
	}

	ext_intr_stat1 = readl((*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
	if (ext_intr_stat1 & ACP_SDW1_STAT) != 0 {
		writel(ACP_SDW1_STAT, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT1 as usize));
		amd_manager = dev_get_drvdata(&(*(*(*adata).sdw).pdev[1]).dev) as *mut amd_sdw_manager;
		if !amd_manager.is_null() {
			schedule_work(&mut (*amd_manager).amd_sdw_irq_thread);
		}
		irq_flag = 1;
	}

	if (ext_intr_stat & ACP_ERROR_IRQ) != 0 {
		writel(ACP_ERROR_IRQ, (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT as usize));
		/* TODO: Report SoundWire Manager instance errors */
		writel(0, (*adata).acp63_base.add(ACP_SW0_I2S_ERROR_REASON as usize));
		writel(0, (*adata).acp63_base.add(ACP_SW1_I2S_ERROR_REASON as usize));
		writel(0, (*adata).acp63_base.add(ACP_ERROR_STATUS as usize));
		irq_flag = 1;
	}

	if (*adata).acp_rev >= ACP70_PCI_REV {
		wake_irq_flag = check_and_handle_acp70_sdw_wake_irq(adata) as u16;
	}

	if (ext_intr_stat & BIT(PDM_DMA_STAT)) != 0 {
		ps_pdm_data = dev_get_drvdata(&(*(*adata).pdm_dev).dev) as *mut pdm_dev_data;
		writel(BIT(PDM_DMA_STAT), (*adata).acp63_base.add(ACP_EXTERNAL_INTR_STAT as usize));
		if !(*ps_pdm_data).capture_stream.is_null() {
			snd_pcm_period_elapsed((*ps_pdm_data).capture_stream);
		}
		irq_flag = 1;
	}

	sdw_dma_irq_flag = check_and_handle_sdw_dma_irq(adata, ext_intr_stat, ext_intr_stat1) as u16;
	if sdw_dma_irq_flag != 0 {
		return IRQ_WAKE_THREAD;
	}

	if irq_flag != 0 || wake_irq_flag != 0 {
		IRQ_HANDLED
	} else {
		IRQ_NONE
	}
}

/* Translates #if IS_ENABLED(CONFIG_SND_SOC_AMD_SOUNDWIRE): select the enabled
 * implementations when the Rust build enables SoundWire support; otherwise use
 * the fallback functions below.
 */
unsafe fn acp_scan_sdw_devices(dev: *mut device, addr: u64) -> c_int {
	let mut sdw_dev: *mut acpi_device;
	let acp_data: *mut acp63_dev_data;

	acp_data = dev_get_drvdata(dev) as *mut acp63_dev_data;
	if addr == 0 {
		return -ENODEV;
	}

	sdw_dev = acpi_find_child_device(ACPI_COMPANION(dev), addr, 0);
	if sdw_dev.is_null() {
		return -ENODEV;
	}

	(*acp_data).info.handle = (*sdw_dev).handle;
	(*acp_data).info.count = AMD_SDW_MAX_MANAGERS;
	amd_sdw_scan_controller(&mut (*acp_data).info)
}

unsafe fn amd_sdw_probe(dev: *mut device) -> c_int {
	let acp_data: *mut acp63_dev_data;
	let mut sdw_res: sdw_amd_res = core::mem::zeroed();
	let ret: c_int;

	acp_data = dev_get_drvdata(dev) as *mut acp63_dev_data;
	sdw_res.addr = (*acp_data).addr;
	sdw_res.reg_range = (*acp_data).reg_range;
	sdw_res.handle = (*acp_data).info.handle;
	sdw_res.parent = dev;
	sdw_res.dev = dev;
	sdw_res.acp_lock = &mut (*acp_data).acp_lock;
	sdw_res.count = (*acp_data).info.count;
	sdw_res.mmio_base = (*acp_data).acp63_base;
	sdw_res.acp_rev = (*acp_data).acp_rev;
	sdw_res.link_mask = (*acp_data).info.link_mask;
	ret = sdw_amd_probe(&mut sdw_res, &mut (*acp_data).sdw);
	if ret != 0 {
		dev_err!(dev, "error: SoundWire probe failed\n");
	}
	ret
}

unsafe fn amd_sdw_exit(acp_data: *mut acp63_dev_data) -> c_int {
	if !(*acp_data).sdw.is_null() {
		sdw_amd_exit((*acp_data).sdw);
	}
	(*acp_data).sdw = ptr::null_mut();

	0
}

unsafe fn acp63_sdw_machine_select(dev: *mut device) -> *mut snd_soc_acpi_mach {
	let mut mach: *mut snd_soc_acpi_mach;
	let mut link: *const snd_soc_acpi_link_adr;
	let acp_data = dev_get_drvdata(dev) as *mut acp63_dev_data;
	let mut ret: c_int;
	let mut i: c_int;

	if (*acp_data).info.count != 0 {
		ret = sdw_amd_get_slave_info((*acp_data).sdw);
		if ret != 0 {
			dev_dbg!(dev, "failed to read slave information\n");
			return ptr::null_mut();
		}
		mach = (*acp_data).machines;
		while !mach.is_null() {
			if (*mach).links.is_null() {
				break;
			}
			link = (*mach).links;
			i = 0;
			while i < (*acp_data).info.count && (*link).num_adr != 0 {
				if !snd_soc_acpi_sdw_link_slaves_found(dev, link, (*(*acp_data).sdw).peripherals) {
					break;
				}
				link = link.add(1);
				i += 1;
			}
			if i == (*acp_data).info.count || (*link).num_adr == 0 {
				if (*mach).machine_check.is_none()
					|| (*mach).machine_check.unwrap()((*acp_data).sdw)
				{
					break;
				}
			}
			mach = mach.add(1);
		}
		if !mach.is_null() && (*mach).link_mask != 0 {
			(*mach).mach_params.links = (*mach).links;
			(*mach).mach_params.link_mask = (*mach).link_mask;
			(*mach).mach_params.subsystem_rev = (*acp_data).acp_rev;
			(*mach).mach_params.subsystem_vendor = (*acp_data).subsystem_vendor;
			(*mach).mach_params.subsystem_device = (*acp_data).subsystem_device;
			(*mach).mach_params.subsystem_id_set = true;

			dev_dbg!(
				dev,
				"SSID %x%04x\n",
				(*mach).mach_params.subsystem_vendor,
				(*mach).mach_params.subsystem_device
			);
			return mach;
		}
	}
	dev_dbg!(dev, "No SoundWire machine driver found\n");
	ptr::null_mut()
}

#[cfg(not(CONFIG_SND_SOC_AMD_SOUNDWIRE))]
unsafe fn acp_scan_sdw_devices_disabled(_dev: *mut device, _addr: u64) -> c_int {
	0
}

#[cfg(not(CONFIG_SND_SOC_AMD_SOUNDWIRE))]
unsafe fn amd_sdw_probe_disabled(_dev: *mut device) -> c_int {
	0
}

#[cfg(not(CONFIG_SND_SOC_AMD_SOUNDWIRE))]
unsafe fn amd_sdw_exit_disabled(_acp_data: *mut acp63_dev_data) -> c_int {
	0
}

#[cfg(not(CONFIG_SND_SOC_AMD_SOUNDWIRE))]
unsafe fn acp63_sdw_machine_select_disabled(_dev: *mut device) -> *mut snd_soc_acpi_mach {
	ptr::null_mut()
}

unsafe fn acp63_machine_register(dev: *mut device) -> c_int {
	let mut mach: *mut snd_soc_acpi_mach;
	let adata = dev_get_drvdata(dev) as *mut acp63_dev_data;
	let size: c_int;

	if (*adata).is_sdw_dev && (*adata).is_sdw_config {
		size = size_of::<snd_soc_acpi_mach>() as c_int;
		mach = acp63_sdw_machine_select(dev);
		if !mach.is_null() {
			(*adata).mach_dev = platform_device_register_data(
				dev,
				(*mach).drv_name,
				PLATFORM_DEVID_NONE,
				mach as *const c_void,
				size as size_t,
			);
			if IS_ERR((*adata).mach_dev) {
				dev_err!(dev, "cannot register Machine device for SoundWire Interface\n");
				return PTR_ERR((*adata).mach_dev);
			}
		}
	} else if (*adata).is_pdm_dev && !(*adata).is_sdw_dev && (*adata).is_pdm_config {
		(*adata).mach_dev = platform_device_register_data(
			dev,
			ACP_PS_MACH.as_ptr() as *const c_char,
			PLATFORM_DEVID_NONE,
			ptr::null(),
			0,
		);
		if IS_ERR((*adata).mach_dev) {
			dev_err!(dev, "cannot register amd_ps_mach device\n");
			return PTR_ERR((*adata).mach_dev);
		}
	}
	0
}

unsafe fn get_acp63_device_config(pci: *mut pci_dev, acp_data: *mut acp63_dev_data) -> c_int {
	let mut pdm_dev: *mut acpi_device;
	let mut obj: *const acpi_object = ptr::null();
	let handle: acpi_handle;
	let mut dmic_status: acpi_integer = 0;
	let mut is_dmic_dev = false;
	let mut is_sdw_dev = false;
	let mut wov_en: bool;
	let mut dmic_en: bool;
	let mut ret: c_int;

	/* IF WOV entry not found, enable dmic based on acp-audio-device-type entry*/
	wov_en = true;
	dmic_en = false;

	acp_hw_get_config(pci, acp_data);

	if (*acp_data).is_pdm_config {
		pdm_dev = acpi_find_child_device(ACPI_COMPANION(&(*pci).dev), ACP63_DMIC_ADDR, 0);
		if !pdm_dev.is_null() {
			/* is_dmic_dev flag will be set when ACP PDM controller device exists */
			if acpi_dev_get_property(
				pdm_dev,
				ACP_AUDIO_DEVICE_TYPE.as_ptr() as *const c_char,
				ACPI_TYPE_INTEGER,
				&mut obj,
			) == 0
				&& (*obj).integer.value == ACP_DMIC_DEV as acpi_integer
			{
				dmic_en = true;
			}
		}

		handle = ACPI_HANDLE(&(*pci).dev);
		ret = acpi_evaluate_integer(handle, WOV_METHOD.as_ptr() as *const c_char, ptr::null_mut(), &mut dmic_status);
		if !ACPI_FAILURE(ret) {
			wov_en = dmic_status != 0;
		}
	}

	if dmic_en && wov_en {
		is_dmic_dev = true;
	}

	if (*acp_data).is_sdw_config {
		ret = acp_scan_sdw_devices(&mut (*pci).dev, ACP63_SDW_ADDR);
		if ret == 0 && (*acp_data).info.link_mask != 0 {
			is_sdw_dev = true;
		}
	}

	(*acp_data).is_pdm_dev = is_dmic_dev;
	(*acp_data).is_sdw_dev = is_sdw_dev;
	if !is_dmic_dev && !is_sdw_dev {
		dev_dbg!(&mut (*pci).dev, "No PDM or SoundWire manager devices found\n");
		return -ENODEV;
	}
	0
}

unsafe fn acp63_fill_platform_dev_info(
	pdevinfo: *mut platform_device_info,
	parent: *mut device,
	fw_node: *mut fwnode_handle,
	name: *mut c_char,
	id: c_uint,
	res: *const resource,
	num_res: c_uint,
	data: *const c_void,
	size_data: size_t,
) {
	(*pdevinfo).name = name;
	(*pdevinfo).id = id;
	(*pdevinfo).parent = parent;
	(*pdevinfo).num_res = num_res;
	(*pdevinfo).res = res;
	(*pdevinfo).data = data;
	(*pdevinfo).size_data = size_data;
	(*pdevinfo).fwnode = fw_node;
}

unsafe fn create_acp63_platform_devs(
	pci: *mut pci_dev,
	adata: *mut acp63_dev_data,
	addr: u32,
) -> c_int {
	let mut pdevinfo: platform_device_info = core::mem::zeroed();
	let parent: *mut device;
	let mut ret: c_int = 0;

	parent = &mut (*pci).dev;

	if (*adata).is_sdw_dev || (*adata).is_pdm_dev {
		(*adata).res = devm_kzalloc(&mut (*pci).dev, size_of::<resource>(), GFP_KERNEL) as *mut resource;
		if (*adata).res.is_null() {
			ret = -ENOMEM;
			goto_de_init(pci, adata, ret);
			return ret;
		}
		(*(*adata).res).flags = IORESOURCE_MEM;
		(*(*adata).res).start = addr;
		(*(*adata).res).end = addr + (ACP63_REG_END - ACP63_REG_START);
		pdevinfo = core::mem::zeroed();
	}

	if (*adata).is_pdm_dev && (*adata).is_pdm_config {
		acp63_fill_platform_dev_info(
			&mut pdevinfo,
			parent,
			ptr::null_mut(),
			ACP_PS_PDM_DMA.as_ptr() as *mut c_char,
			0,
			(*adata).res,
			1,
			ptr::null(),
			0,
		);

		(*adata).pdm_dev = platform_device_register_full(&pdevinfo);
		if IS_ERR((*adata).pdm_dev) {
			dev_err!(&mut (*pci).dev, "cannot register %s device\n", pdevinfo.name);
			ret = PTR_ERR((*adata).pdm_dev);
			goto_de_init(pci, adata, ret);
			return ret;
		}
		pdevinfo = core::mem::zeroed();
		acp63_fill_platform_dev_info(
			&mut pdevinfo,
			parent,
			ptr::null_mut(),
			DMIC_CODEC.as_ptr() as *mut c_char,
			0,
			ptr::null(),
			0,
			ptr::null(),
			0,
		);
		(*adata).dmic_codec_dev = platform_device_register_full(&pdevinfo);
		if IS_ERR((*adata).dmic_codec_dev) {
			dev_err!(&mut (*pci).dev, "cannot register %s device\n", pdevinfo.name);
			ret = PTR_ERR((*adata).dmic_codec_dev);
			platform_device_unregister((*adata).pdm_dev);
			goto_de_init(pci, adata, ret);
			return ret;
		}
	}
	if (*adata).is_sdw_dev && (*adata).is_sdw_config {
		ret = amd_sdw_probe(&mut (*pci).dev);
		if ret != 0 {
			if (*adata).is_pdm_dev {
				platform_device_unregister((*adata).dmic_codec_dev);
				platform_device_unregister((*adata).pdm_dev);
			}
			goto_de_init(pci, adata, ret);
			return ret;
		}
		pdevinfo = core::mem::zeroed();
		acp63_fill_platform_dev_info(
			&mut pdevinfo,
			parent,
			ptr::null_mut(),
			AMD_PS_SDW_DMA.as_ptr() as *mut c_char,
			0,
			(*adata).res,
			1,
			ptr::null(),
			0,
		);

		(*adata).sdw_dma_dev = platform_device_register_full(&pdevinfo);
		if IS_ERR((*adata).sdw_dma_dev) {
			dev_err!(&mut (*pci).dev, "cannot register %s device\n", pdevinfo.name);
			ret = PTR_ERR((*adata).sdw_dma_dev);
			if (*adata).is_pdm_dev {
				platform_device_unregister((*adata).dmic_codec_dev);
				platform_device_unregister((*adata).pdm_dev);
			}
			goto_de_init(pci, adata, ret);
			return ret;
		}
	}

	0
}

unsafe fn goto_de_init(pci: *mut pci_dev, adata: *mut acp63_dev_data, _ret: c_int) {
	if acp_hw_deinit(adata, &mut (*pci).dev) != 0 {
		dev_err!(&mut (*pci).dev, "ACP de-init failed\n");
	}
}

unsafe fn acp_hw_init_ops(adata: *mut acp63_dev_data, pci: *mut pci_dev) -> c_int {
	(*adata).hw_ops = devm_kzalloc(&mut (*pci).dev, size_of::<acp_hw_ops>(), GFP_KERNEL) as *mut acp_hw_ops;
	if (*adata).hw_ops.is_null() {
		return -ENOMEM;
	}

	match (*adata).acp_rev {
		ACP63_PCI_REV => acp63_hw_init_ops((*adata).hw_ops),
		ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => acp70_hw_init_ops((*adata).hw_ops),
		_ => {
			dev_err!(&mut (*pci).dev, "ACP device not found\n");
			return -ENODEV;
		}
	}
	0
}

unsafe extern "C" fn snd_acp63_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
	let mut adata: *mut acp63_dev_data;
	let addr: u32;
	let irqflags: u32;
	let flag: u32;
	let mut ret: c_int;

	irqflags = IRQF_SHARED;

	/* Return if acp config flag is defined */
	flag = snd_amd_acp_find_config(pci);
	if flag != 0 {
		return -ENODEV;
	}

	/* ACP PCI revision id check for ACP6.3, ACP7.0 & ACP7.1 platforms */
	match (*pci).revision as u32 {
		ACP63_PCI_REV | ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {}
		_ => {
			dev_dbg!(&mut (*pci).dev, "acp63/acp70/acp71 pci device not found\n");
			return -ENODEV;
		}
	}
	if pci_enable_device(pci) != 0 {
		dev_err!(&mut (*pci).dev, "pci_enable_device failed\n");
		return -ENODEV;
	}

	ret = pci_request_regions(pci, AMD_ACP63_AUDIO.as_ptr() as *const c_char);
	if ret < 0 {
		dev_err!(&mut (*pci).dev, "pci_request_regions failed\n");
		pci_disable_device(pci);
		return ret;
	}
	adata = devm_kzalloc(&mut (*pci).dev, size_of::<acp63_dev_data>(), GFP_KERNEL) as *mut acp63_dev_data;
	if adata.is_null() {
		ret = -ENOMEM;
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}

	addr = pci_resource_start(pci, 0);
	(*adata).acp63_base = devm_ioremap(&mut (*pci).dev, addr, pci_resource_len(pci, 0));
	if (*adata).acp63_base.is_null() {
		ret = -ENOMEM;
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	(*adata).addr = addr;
	(*adata).reg_range = ACP63_REG_END - ACP63_REG_START;
	(*adata).acp_rev = (*pci).revision as u32;
	(*adata).subsystem_vendor = (*pci).subsystem_vendor;
	(*adata).subsystem_device = (*pci).subsystem_device;

	pci_set_master(pci);
	pci_set_drvdata(pci, adata as *mut c_void);
	mutex_init(&mut (*adata).acp_lock);
	ret = acp_hw_init_ops(adata, pci);
	if ret != 0 {
		dev_err!(&mut (*pci).dev, "ACP hw ops init failed\n");
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	ret = acp_hw_init(adata, &mut (*pci).dev);
	if ret != 0 {
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	ret = devm_request_threaded_irq(
		&mut (*pci).dev,
		(*pci).irq,
		Some(acp63_irq_handler),
		Some(acp63_irq_thread),
		irqflags,
		ACP_PCI_IRQ.as_ptr() as *const c_char,
		adata as *mut c_void,
	);
	if ret != 0 {
		dev_err!(&mut (*pci).dev, "ACP PCI IRQ request failed\n");
		if acp_hw_deinit(adata, &mut (*pci).dev) != 0 {
			dev_err!(&mut (*pci).dev, "ACP de-init failed\n");
		}
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	ret = get_acp63_device_config(pci, adata);
	/* ACP PCI driver probe should be continued even PDM or SoundWire Devices are not found */
	if ret != 0 {
		dev_dbg!(&mut (*pci).dev, "get acp device config failed:%d\n", ret);
		device_set_wakeup_enable(&mut (*pci).dev, true);
		pm_runtime_set_autosuspend_delay(&mut (*pci).dev, ACP_SUSPEND_DELAY_MS);
		pm_runtime_use_autosuspend(&mut (*pci).dev);
		pm_runtime_put_noidle(&mut (*pci).dev);
		pm_runtime_allow(&mut (*pci).dev);
		return 0;
	}
	ret = create_acp63_platform_devs(pci, adata, addr);
	if ret < 0 {
		dev_err!(&mut (*pci).dev, "ACP platform devices creation failed\n");
		if acp_hw_deinit(adata, &mut (*pci).dev) != 0 {
			dev_err!(&mut (*pci).dev, "ACP de-init failed\n");
		}
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	if (*adata).acp_rev >= ACP70_PCI_REV {
		(*adata).machines = snd_soc_acpi_amd_acp70_sdw_machines;
	} else {
		(*adata).machines = snd_soc_acpi_amd_acp63_sdw_machines;
	}

	ret = acp63_machine_register(&mut (*pci).dev);
	if ret != 0 {
		dev_err!(&mut (*pci).dev, "ACP machine register failed\n");
		if acp_hw_deinit(adata, &mut (*pci).dev) != 0 {
			dev_err!(&mut (*pci).dev, "ACP de-init failed\n");
		}
		pci_release_regions(pci);
		pci_disable_device(pci);
		return ret;
	}
	device_set_wakeup_enable(&mut (*pci).dev, true);
	pm_runtime_set_autosuspend_delay(&mut (*pci).dev, ACP_SUSPEND_DELAY_MS);
	pm_runtime_use_autosuspend(&mut (*pci).dev);
	pm_runtime_put_noidle(&mut (*pci).dev);
	pm_runtime_allow(&mut (*pci).dev);
	0
}

unsafe fn snd_acp_suspend(dev: *mut device) -> c_int {
	acp_hw_suspend(dev)
}

unsafe fn snd_acp_runtime_resume(dev: *mut device) -> c_int {
	acp_hw_runtime_resume(dev)
}

unsafe fn acp_disable_msi_on_resume(pdev: *mut pci_dev) {
	let mut control: u16 = 0;

	if (*pdev).msi_cap == 0 {
		return;
	}

	pci_read_config_word(pdev, (*pdev).msi_cap as c_int + PCI_MSI_FLAGS, &mut control);
	if (control & PCI_MSI_FLAGS_ENABLE) != 0 {
		dev_warn!(
			&mut (*pdev).dev,
			"ACP: MSI unexpectedly enabled after resume (flags=0x%04x), disabling\n",
			control
		);
		control &= !PCI_MSI_FLAGS_ENABLE;
		pci_write_config_word(pdev, (*pdev).msi_cap as c_int + PCI_MSI_FLAGS, control);
	}
}

unsafe fn snd_acp_resume(dev: *mut device) -> c_int {
	let pdev: *mut pci_dev = to_pci_dev(dev);

	/*
	 * BIOS/firmware may re-enable MSI in PCI config space during
	 * system resume even though this driver only uses legacy INTx
	 * interrupts. If MSI is left enabled with stale address/data
	 * registers, the device will write interrupts to a bogus address
	 * causing IOMMU IO_PAGE_FAULT and interrupt delivery failure.
	 * Explicitly clear the MSI Enable bit before reinitializing
	 * the ACP hardware.
	 */
	acp_disable_msi_on_resume(pdev);
	acp_hw_resume(dev)
}

/* C initializer:
 * static const struct dev_pm_ops acp63_pm_ops = {
 *	RUNTIME_PM_OPS(snd_acp_suspend, snd_acp_runtime_resume, NULL)
 *	SYSTEM_SLEEP_PM_OPS(snd_acp_suspend, snd_acp_resume)
 * };
 */
static acp63_pm_ops: *const dev_pm_ops = unsafe { &acp63_pm_ops_generated };

unsafe extern "C" fn snd_acp63_remove(pci: *mut pci_dev) {
	let adata: *mut acp63_dev_data;
	let ret: c_int;

	adata = pci_get_drvdata(pci) as *mut acp63_dev_data;
	if !(*adata).sdw.is_null() {
		amd_sdw_exit(adata);
		platform_device_unregister((*adata).sdw_dma_dev);
	}
	if (*adata).is_pdm_dev {
		platform_device_unregister((*adata).pdm_dev);
		platform_device_unregister((*adata).dmic_codec_dev);
	}
	if !(*adata).mach_dev.is_null() {
		platform_device_unregister((*adata).mach_dev);
	}
	ret = acp_hw_deinit(adata, &mut (*pci).dev);
	if ret != 0 {
		dev_err!(&mut (*pci).dev, "ACP de-init failed\n");
	}
	pm_runtime_forbid(&mut (*pci).dev);
	pm_runtime_get_noresume(&mut (*pci).dev);
	pci_release_regions(pci);
	pci_disable_device(pci);
}

const fn PCI_DEVICE(vend: u32, dev: u32) -> pci_device_id {
	pci_device_id {
		vendor: vend,
		device: dev,
		subvendor: !0,
		subdevice: !0,
		class: 0,
		class_mask: 0,
		driver_data: 0,
	}
}

static snd_acp63_ids: [pci_device_id; 2] = [
	pci_device_id {
		class: PCI_CLASS_MULTIMEDIA_OTHER << 8,
		class_mask: 0xffffff,
		..PCI_DEVICE(PCI_VENDOR_ID_AMD, ACP_DEVICE_ID)
	},
	pci_device_id {
		vendor: 0,
		device: 0,
		subvendor: 0,
		subdevice: 0,
		class: 0,
		class_mask: 0,
		driver_data: 0,
	},
];
/* MODULE_DEVICE_TABLE(pci, snd_acp63_ids); */

static mut ps_acp63_driver: pci_driver = pci_driver {
	name: unsafe { &KBUILD_MODNAME as *const c_char },
	id_table: snd_acp63_ids.as_ptr(),
	probe: Some(snd_acp63_probe),
	remove: Some(snd_acp63_remove),
	driver: pci_driver_driver {
		pm: unsafe { acp63_pm_ops },
	},
};

unsafe fn module_pci_driver_init() -> c_int {
	__pci_register_driver(&mut ps_acp63_driver)
}

unsafe fn module_pci_driver_exit() {
	pci_unregister_driver(&mut ps_acp63_driver);
}

/* module_pci_driver(ps_acp63_driver); */

/* MODULE_AUTHOR("Vijendar.Mukunda@amd.com"); */
/* MODULE_AUTHOR("Syed.SabaKareem@amd.com"); */
/* MODULE_DESCRIPTION("AMD common ACP PCI driver for ACP6.3, ACP7.0 & ACP7.1 platforms"); */
/* MODULE_IMPORT_NS("SOUNDWIRE_AMD_INIT"); */
/* MODULE_IMPORT_NS("SND_AMD_SOUNDWIRE_ACPI"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
