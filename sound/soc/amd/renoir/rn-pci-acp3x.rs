// SPDX-License-Identifier: GPL-2.0+
//
// AMD Renoir ACP PCI Driver
//
// Copyright 2020 Advanced Micro Devices, Inc.

// Translated from C. External Linux kernel and rn_acp3x.h symbols are declared
// here as dependencies supplied by other translation units.

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type size_t = usize;
type u32_t = u32;
type acpi_handle = *mut core::ffi::c_void;
type acpi_integer = u64;

const ACP_DMIC_AUTO: c_int = -1;
const ACP_DEVS: usize = 3;
const ACP_PGFSM_STATUS: usize = 0;
const ACP_PGFSM_STATUS_MASK: u32_t = 0;
const ACP_POWER_ON_IN_PROGRESS: u32_t = 0;
const ACP_PGFSM_CNTL_POWER_ON_MASK: u32_t = 0;
const ACP_PGFSM_CONTROL: usize = 0;
const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32_t = 0;
const ACP_POWERED_OFF: u32_t = 0;
const ACP_SOFT_RESET: usize = 0;
const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32_t = 0;
const ACP_EXTERNAL_INTR_ENB: usize = 0;
const ACP_EXTERNAL_INTR_CNTL: usize = 0;
const ACP_ERROR_MASK: u32_t = 0;
const ACP_EXT_INTR_STAT_CLEAR_MASK: u32_t = 0;
const ACP_EXTERNAL_INTR_STAT: usize = 0;
const ACP_CONTROL: usize = 0;
const ACP_CLKMUX_SEL: usize = 0;
const ACP_REG_END: u32_t = 0;
const ACP_REG_START: u32_t = 0;
const ACP_SUSPEND_DELAY_MS: c_int = 0;
const ACP_DEVICE_ID: c_uint = 0;
const PCI_VENDOR_ID_AMD: c_uint = 0;
const PCI_CLASS_MULTIMEDIA_OTHER: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const IRQF_SHARED: c_uint = 0;
const IORESOURCE_MEM: c_ulong = 0;
const IORESOURCE_IRQ: c_ulong = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const KBUILD_MODNAME: *const i8 = b"rn_acp_driver\0".as_ptr() as *const i8;

static mut acp_power_gating: c_int = 0;

/*
 * dmic_acpi_check = -1 - Use ACPI/DMI method to detect the DMIC hardware presence at runtime
 *                 =  0 - Skip the DMIC device creation and return probe failure
 *                 =  1 - Force DMIC support
 */
static mut dmic_acpi_check: c_int = ACP_DMIC_AUTO;

#[repr(C)]
struct device {
	_private: [u8; 0],
}

#[repr(C)]
struct pci_dev {
	dev: device,
	revision: u8,
	irq: c_uint,
}

#[repr(C)]
struct pci_device_id {
	vendor: c_uint,
	device: c_uint,
	subvendor: c_uint,
	subdevice: c_uint,
	class: c_uint,
	class_mask: c_uint,
	driver_data: c_ulong,
}

#[repr(C)]
struct resource {
	name: *const i8,
	start: c_ulong,
	end: c_ulong,
	flags: c_ulong,
}

#[repr(C)]
struct platform_device {
	_private: [u8; 0],
}

#[repr(C)]
struct platform_device_info {
	parent: *mut device,
	fwnode: *mut core::ffi::c_void,
	name: *const i8,
	id: c_int,
	res: *mut resource,
	num_res: c_uint,
	data: *const core::ffi::c_void,
	size_data: size_t,
	dma_mask: u64,
}

#[repr(C)]
struct dmi_system_id {
	matches: [dmi_strmatch; 4],
	driver_data: *const core::ffi::c_void,
}

#[repr(C)]
struct dmi_strmatch {
	slot: c_int,
	substr: *const i8,
}

#[repr(C)]
struct dev_pm_ops {
	runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
	poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct device_driver {
	pm: *const dev_pm_ops,
}

#[repr(C)]
struct pci_driver {
	name: *const i8,
	id_table: *const pci_device_id,
	probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
	remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
	driver: device_driver,
}

#[repr(C)]
struct acp_dev_data {
	acp_base: *mut core::ffi::c_void,
	res: *mut resource,
	pdev: [*mut platform_device; ACP_DEVS],
}

extern "C" {
	fn rn_readl(addr: *mut core::ffi::c_void) -> u32_t;
	fn rn_writel(val: u32_t, addr: *mut core::ffi::c_void);
	fn udelay(usecs: c_ulong);
	fn cpu_relax();
	fn pr_err(fmt: *const i8, ...);
	fn dev_err(dev: *mut device, fmt: *const i8, ...);
	fn dev_info(dev: *mut device, fmt: *const i8, ...);
	fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
	fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_uint;
	fn pci_enable_device(pci: *mut pci_dev) -> c_int;
	fn pci_request_regions(pci: *mut pci_dev, name: *const i8) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut core::ffi::c_void;
	fn pci_enable_msi(pci: *mut pci_dev) -> c_int;
	fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
	fn pci_resource_len(pci: *mut pci_dev, bar: c_int) -> c_ulong;
	fn devm_ioremap(dev: *mut device, offset: c_ulong, size: c_ulong) -> *mut core::ffi::c_void;
	fn pci_set_master(pci: *mut pci_dev);
	fn pci_set_drvdata(pci: *mut pci_dev, data: *mut core::ffi::c_void);
	fn ACPI_HANDLE(dev: *mut device) -> acpi_handle;
	fn acpi_evaluate_integer(
		handle: acpi_handle,
		pathname: *const i8,
		arguments: *mut core::ffi::c_void,
		data: *mut acpi_integer,
	) -> c_int;
	fn ACPI_FAILURE(status: c_int) -> bool;
	fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
	fn platform_device_register_full(pdevinfo: *const platform_device_info) -> *mut platform_device;
	fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
	fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
	fn platform_device_unregister(pdev: *mut platform_device);
	fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
	fn pm_runtime_use_autosuspend(dev: *mut device);
	fn pm_runtime_put_noidle(dev: *mut device);
	fn pm_runtime_allow(dev: *mut device);
	fn pci_disable_msi(pci: *mut pci_dev);
	fn pci_release_regions(pci: *mut pci_dev);
	fn pci_disable_device(pci: *mut pci_dev);
	fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
	fn pci_get_drvdata(pci: *mut pci_dev) -> *mut core::ffi::c_void;
	fn pm_runtime_forbid(dev: *mut device);
	fn pm_runtime_get_noresume(dev: *mut device);
	fn memset(s: *mut core::ffi::c_void, c: c_int, n: size_t) -> *mut core::ffi::c_void;
}

unsafe extern "C" fn rn_acp_power_on(acp_base: *mut core::ffi::c_void) -> c_int {
	let mut val: u32_t;
	let mut timeout: c_int;

	val = rn_readl((acp_base as *mut u8).add(ACP_PGFSM_STATUS) as *mut core::ffi::c_void);

	if val == 0 {
		return val as c_int;
	}

	if (val & ACP_PGFSM_STATUS_MASK) != ACP_POWER_ON_IN_PROGRESS {
		rn_writel(
			ACP_PGFSM_CNTL_POWER_ON_MASK,
			(acp_base as *mut u8).add(ACP_PGFSM_CONTROL) as *mut core::ffi::c_void,
		);
	}
	timeout = 0;
	while {
		timeout += 1;
		timeout < 500
	} {
		val = rn_readl((acp_base as *mut u8).add(ACP_PGFSM_STATUS) as *mut core::ffi::c_void);
		if val == 0 {
			return 0;
		}
		udelay(1);
	}
	-ETIMEDOUT
}

unsafe extern "C" fn rn_acp_power_off(acp_base: *mut core::ffi::c_void) -> c_int {
	let mut val: u32_t;
	let mut timeout: c_int;

	rn_writel(
		ACP_PGFSM_CNTL_POWER_OFF_MASK,
		(acp_base as *mut u8).add(ACP_PGFSM_CONTROL) as *mut core::ffi::c_void,
	);
	timeout = 0;
	while {
		timeout += 1;
		timeout < 500
	} {
		val = rn_readl((acp_base as *mut u8).add(ACP_PGFSM_STATUS) as *mut core::ffi::c_void);
		if (val & ACP_PGFSM_STATUS_MASK) == ACP_POWERED_OFF {
			return 0;
		}
		udelay(1);
	}
	-ETIMEDOUT
}

unsafe extern "C" fn rn_acp_reset(acp_base: *mut core::ffi::c_void) -> c_int {
	let mut val: u32_t;
	let mut timeout: c_int;

	rn_writel(1, (acp_base as *mut u8).add(ACP_SOFT_RESET) as *mut core::ffi::c_void);
	timeout = 0;
	while {
		timeout += 1;
		timeout < 500
	} {
		val = rn_readl((acp_base as *mut u8).add(ACP_SOFT_RESET) as *mut core::ffi::c_void);
		if (val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK) != 0 {
			break;
		}
		cpu_relax();
	}
	rn_writel(0, (acp_base as *mut u8).add(ACP_SOFT_RESET) as *mut core::ffi::c_void);
	timeout = 0;
	while {
		timeout += 1;
		timeout < 500
	} {
		val = rn_readl((acp_base as *mut u8).add(ACP_SOFT_RESET) as *mut core::ffi::c_void);
		if val == 0 {
			return 0;
		}
		cpu_relax();
	}
	-ETIMEDOUT
}

unsafe extern "C" fn rn_acp_enable_interrupts(acp_base: *mut core::ffi::c_void) {
	let mut ext_intr_ctrl: u32_t;

	rn_writel(0x01, (acp_base as *mut u8).add(ACP_EXTERNAL_INTR_ENB) as *mut core::ffi::c_void);
	ext_intr_ctrl = rn_readl((acp_base as *mut u8).add(ACP_EXTERNAL_INTR_CNTL) as *mut core::ffi::c_void);
	ext_intr_ctrl |= ACP_ERROR_MASK;
	rn_writel(
		ext_intr_ctrl,
		(acp_base as *mut u8).add(ACP_EXTERNAL_INTR_CNTL) as *mut core::ffi::c_void,
	);
}

unsafe extern "C" fn rn_acp_disable_interrupts(acp_base: *mut core::ffi::c_void) {
	rn_writel(
		ACP_EXT_INTR_STAT_CLEAR_MASK,
		(acp_base as *mut u8).add(ACP_EXTERNAL_INTR_STAT) as *mut core::ffi::c_void,
	);
	rn_writel(0x00, (acp_base as *mut u8).add(ACP_EXTERNAL_INTR_ENB) as *mut core::ffi::c_void);
}

unsafe extern "C" fn rn_acp_init(acp_base: *mut core::ffi::c_void) -> c_int {
	let mut ret: c_int;

	/* power on */
	ret = rn_acp_power_on(acp_base);
	if ret != 0 {
		pr_err(b"ACP power on failed\n\0".as_ptr() as *const i8);
		return ret;
	}
	rn_writel(0x01, (acp_base as *mut u8).add(ACP_CONTROL) as *mut core::ffi::c_void);
	/* Reset */
	ret = rn_acp_reset(acp_base);
	if ret != 0 {
		pr_err(b"ACP reset failed\n\0".as_ptr() as *const i8);
		return ret;
	}
	rn_writel(0x03, (acp_base as *mut u8).add(ACP_CLKMUX_SEL) as *mut core::ffi::c_void);
	rn_acp_enable_interrupts(acp_base);
	0
}

unsafe extern "C" fn rn_acp_deinit(acp_base: *mut core::ffi::c_void) -> c_int {
	let mut ret: c_int;

	rn_acp_disable_interrupts(acp_base);
	/* Reset */
	ret = rn_acp_reset(acp_base);
	if ret != 0 {
		pr_err(b"ACP reset failed\n\0".as_ptr() as *const i8);
		return ret;
	}
	rn_writel(0x00, (acp_base as *mut u8).add(ACP_CLKMUX_SEL) as *mut core::ffi::c_void);
	rn_writel(0x00, (acp_base as *mut u8).add(ACP_CONTROL) as *mut core::ffi::c_void);
	/* power off */
	if acp_power_gating != 0 {
		ret = rn_acp_power_off(acp_base);
		if ret != 0 {
			pr_err(b"ACP power off failed\n\0".as_ptr() as *const i8);
			return ret;
		}
	}
	0
}

static rn_acp_quirk_table: [dmi_system_id; 6] = [
	dmi_system_id {
		/* Lenovo IdeaPad S340-14API */
		matches: [
			dmi_strmatch { slot: 0, substr: b"LENOVO\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: b"81NB\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
	dmi_system_id {
		/* Lenovo IdeaPad Flex 5 14ARE05 */
		matches: [
			dmi_strmatch { slot: 0, substr: b"LENOVO\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: b"81X2\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
	dmi_system_id {
		/* Lenovo IdeaPad 5 15ARE05 */
		matches: [
			dmi_strmatch { slot: 0, substr: b"LENOVO\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: b"81YQ\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
	dmi_system_id {
		/* Lenovo ThinkPad E14 Gen 2 */
		matches: [
			dmi_strmatch { slot: 0, substr: b"LENOVO\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: b"20T6CTO1WW\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
	dmi_system_id {
		/* Lenovo ThinkPad X395 */
		matches: [
			dmi_strmatch { slot: 0, substr: b"LENOVO\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: b"20NLCTO1WW\0".as_ptr() as *const i8 },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
	dmi_system_id {
		matches: [
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
			dmi_strmatch { slot: 0, substr: core::ptr::null() },
		],
		driver_data: core::ptr::null(),
	},
];

unsafe extern "C" fn snd_rn_acp_probe(
	pci: *mut pci_dev,
	_pci_id: *const pci_device_id,
) -> c_int {
	let mut adata: *mut acp_dev_data;
	let mut pdevinfo: [platform_device_info; ACP_DEVS] = core::mem::zeroed();
	/* #if defined(CONFIG_ACPI) */
	let mut handle: acpi_handle;
	let mut dmic_status: acpi_integer = 0;
	/* #endif */
	let mut dmi_id: *const dmi_system_id;
	let mut irqflags: c_uint;
	let mut flag: c_uint;
	let mut ret: c_int;
	let mut index: c_int;
	let mut addr: u32_t;

	/* Return if acp config flag is defined */
	flag = snd_amd_acp_find_config(pci);
	if flag != 0 {
		return -ENODEV;
	}

	/* Renoir device check */
	if (*pci).revision as c_int != 0x01 {
		return -ENODEV;
	}

	if pci_enable_device(pci) != 0 {
		dev_err(&mut (*pci).dev, b"pci_enable_device failed\n\0".as_ptr() as *const i8);
		return -ENODEV;
	}

	ret = pci_request_regions(pci, b"AMD ACP3x audio\0".as_ptr() as *const i8);
	if ret < 0 {
		dev_err(&mut (*pci).dev, b"pci_request_regions failed\n\0".as_ptr() as *const i8);
		goto_disable_pci(pci);
		return ret;
	}

	adata = devm_kzalloc(
		&mut (*pci).dev,
		core::mem::size_of::<acp_dev_data>(),
		GFP_KERNEL,
	) as *mut acp_dev_data;
	if adata.is_null() {
		ret = -ENOMEM;
		goto_release_regions(pci);
		goto_disable_pci(pci);
		return ret;
	}

	/* check for msi interrupt support */
	ret = pci_enable_msi(pci);
	if ret != 0 {
		/* msi is not enabled */
		irqflags = IRQF_SHARED;
	} else {
		/* msi is enabled */
		irqflags = 0;
	}

	addr = pci_resource_start(pci, 0) as u32_t;
	(*adata).acp_base = devm_ioremap(&mut (*pci).dev, addr as c_ulong, pci_resource_len(pci, 0));
	if (*adata).acp_base.is_null() {
		ret = -ENOMEM;
		goto_disable_msi(pci);
		goto_release_regions(pci);
		goto_disable_pci(pci);
		return ret;
	}
	pci_set_master(pci);
	pci_set_drvdata(pci, adata as *mut core::ffi::c_void);
	ret = rn_acp_init((*adata).acp_base);
	if ret != 0 {
		goto_disable_msi(pci);
		goto_release_regions(pci);
		goto_disable_pci(pci);
		return ret;
	}

	if dmic_acpi_check == 0 {
		ret = -ENODEV;
		goto_de_init(pci, adata);
		goto_disable_msi(pci);
		goto_release_regions(pci);
		goto_disable_pci(pci);
		return ret;
	} else if dmic_acpi_check == ACP_DMIC_AUTO {
		/* #if defined(CONFIG_ACPI) */
		handle = ACPI_HANDLE(&mut (*pci).dev);
		ret = acpi_evaluate_integer(handle, b"_WOV\0".as_ptr() as *const i8, core::ptr::null_mut(), &mut dmic_status);
		if ACPI_FAILURE(ret) {
			ret = -ENODEV;
			goto_de_init(pci, adata);
			goto_disable_msi(pci);
			goto_release_regions(pci);
			goto_disable_pci(pci);
			return ret;
		}
		if dmic_status == 0 {
			ret = -ENODEV;
			goto_de_init(pci, adata);
			goto_disable_msi(pci);
			goto_release_regions(pci);
			goto_disable_pci(pci);
			return ret;
		}
		/* #endif */
		dmi_id = dmi_first_match(rn_acp_quirk_table.as_ptr());
		if !dmi_id.is_null() && (*dmi_id).driver_data.is_null() {
			dev_info(
				&mut (*pci).dev,
				b"ACPI settings override using DMI (ACP mic is not present)\0".as_ptr() as *const i8,
			);
			ret = -ENODEV;
			goto_de_init(pci, adata);
			goto_disable_msi(pci);
			goto_release_regions(pci);
			goto_disable_pci(pci);
			return ret;
		}
	}

	(*adata).res = devm_kzalloc(
		&mut (*pci).dev,
		core::mem::size_of::<resource>() * 2,
		GFP_KERNEL,
	) as *mut resource;
	if (*adata).res.is_null() {
		ret = -ENOMEM;
		goto_de_init(pci, adata);
		goto_disable_msi(pci);
		goto_release_regions(pci);
		goto_disable_pci(pci);
		return ret;
	}

	(*adata).res.add(0).write(resource {
		name: b"acp_pdm_iomem\0".as_ptr() as *const i8,
		flags: IORESOURCE_MEM,
		start: addr as c_ulong,
		end: addr.wrapping_add(ACP_REG_END.wrapping_sub(ACP_REG_START)) as c_ulong,
	});
	(*adata).res.add(1).write(resource {
		name: b"acp_pdm_irq\0".as_ptr() as *const i8,
		flags: IORESOURCE_IRQ,
		start: (*pci).irq as c_ulong,
		end: (*pci).irq as c_ulong,
	});

	memset(
		pdevinfo.as_mut_ptr() as *mut core::ffi::c_void,
		0,
		core::mem::size_of_val(&pdevinfo),
	);
	pdevinfo[0].name = b"acp_rn_pdm_dma\0".as_ptr() as *const i8;
	pdevinfo[0].id = 0;
	pdevinfo[0].parent = &mut (*pci).dev;
	pdevinfo[0].num_res = 2;
	pdevinfo[0].res = (*adata).res;
	pdevinfo[0].data = &irqflags as *const c_uint as *const core::ffi::c_void;
	pdevinfo[0].size_data = core::mem::size_of_val(&irqflags);

	pdevinfo[1].name = b"dmic-codec\0".as_ptr() as *const i8;
	pdevinfo[1].id = 0;
	pdevinfo[1].parent = &mut (*pci).dev;
	pdevinfo[2].name = b"acp_pdm_mach\0".as_ptr() as *const i8;
	pdevinfo[2].id = 0;
	pdevinfo[2].parent = &mut (*pci).dev;
	index = 0;
	while index < ACP_DEVS as c_int {
		(*adata).pdev[index as usize] = platform_device_register_full(&pdevinfo[index as usize]);
		if IS_ERR((*adata).pdev[index as usize] as *const core::ffi::c_void) {
			dev_err(
				&mut (*pci).dev,
				b"cannot register %s device\n\0".as_ptr() as *const i8,
				pdevinfo[index as usize].name,
			);
			ret = PTR_ERR((*adata).pdev[index as usize] as *const core::ffi::c_void);
			goto_unregister_devs(pci, adata);
			goto_de_init(pci, adata);
			goto_disable_msi(pci);
			goto_release_regions(pci);
			goto_disable_pci(pci);
			return ret;
		}
		index += 1;
	}
	pm_runtime_set_autosuspend_delay(&mut (*pci).dev, ACP_SUSPEND_DELAY_MS);
	pm_runtime_use_autosuspend(&mut (*pci).dev);
	pm_runtime_put_noidle(&mut (*pci).dev);
	pm_runtime_allow(&mut (*pci).dev);
	0
}

unsafe fn goto_unregister_devs(_pci: *mut pci_dev, adata: *mut acp_dev_data) {
	let mut index: c_int = 0;
	while index < ACP_DEVS as c_int {
		platform_device_unregister((*adata).pdev[index as usize]);
		index += 1;
	}
}

unsafe fn goto_de_init(pci: *mut pci_dev, adata: *mut acp_dev_data) {
	if rn_acp_deinit((*adata).acp_base) != 0 {
		dev_err(&mut (*pci).dev, b"ACP de-init failed\n\0".as_ptr() as *const i8);
	}
}

unsafe fn goto_disable_msi(pci: *mut pci_dev) {
	pci_disable_msi(pci);
}

unsafe fn goto_release_regions(pci: *mut pci_dev) {
	pci_release_regions(pci);
}

unsafe fn goto_disable_pci(pci: *mut pci_dev) {
	pci_disable_device(pci);
}

unsafe extern "C" fn snd_rn_acp_suspend(dev: *mut device) -> c_int {
	let mut ret: c_int;
	let mut adata: *mut acp_dev_data;

	adata = dev_get_drvdata(dev) as *mut acp_dev_data;
	ret = rn_acp_deinit((*adata).acp_base);
	if ret != 0 {
		dev_err(dev, b"ACP de-init failed\n\0".as_ptr() as *const i8);
	} else {
		dev_dbg(dev, b"ACP de-initialized\n\0".as_ptr() as *const i8);
	}

	ret
}

unsafe extern "C" fn snd_rn_acp_resume(dev: *mut device) -> c_int {
	let mut ret: c_int;
	let mut adata: *mut acp_dev_data;

	adata = dev_get_drvdata(dev) as *mut acp_dev_data;
	ret = rn_acp_init((*adata).acp_base);
	if ret != 0 {
		dev_err(dev, b"ACP init failed\n\0".as_ptr() as *const i8);
		return ret;
	}
	0
}

static rn_acp_pm: dev_pm_ops = dev_pm_ops {
	runtime_suspend: Some(snd_rn_acp_suspend),
	runtime_resume: Some(snd_rn_acp_resume),
	suspend: Some(snd_rn_acp_suspend),
	resume: Some(snd_rn_acp_resume),
	restore: Some(snd_rn_acp_resume),
	poweroff: Some(snd_rn_acp_suspend),
};

unsafe extern "C" fn snd_rn_acp_remove(pci: *mut pci_dev) {
	let mut adata: *mut acp_dev_data;
	let mut ret: c_int;
	let mut index: c_int;

	adata = pci_get_drvdata(pci) as *mut acp_dev_data;
	index = 0;
	while index < ACP_DEVS as c_int {
		platform_device_unregister((*adata).pdev[index as usize]);
		index += 1;
	}
	ret = rn_acp_deinit((*adata).acp_base);
	if ret != 0 {
		dev_err(&mut (*pci).dev, b"ACP de-init failed\n\0".as_ptr() as *const i8);
	}
	pm_runtime_forbid(&mut (*pci).dev);
	pm_runtime_get_noresume(&mut (*pci).dev);
	pci_disable_msi(pci);
	pci_release_regions(pci);
	pci_disable_device(pci);
}

static snd_rn_acp_ids: [pci_device_id; 2] = [
	pci_device_id {
		vendor: PCI_VENDOR_ID_AMD,
		device: ACP_DEVICE_ID,
		subvendor: 0,
		subdevice: 0,
		class: PCI_CLASS_MULTIMEDIA_OTHER << 8,
		class_mask: 0xffffff,
		driver_data: 0,
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

static mut rn_acp_driver: pci_driver = pci_driver {
	name: KBUILD_MODNAME,
	id_table: snd_rn_acp_ids.as_ptr(),
	probe: Some(snd_rn_acp_probe),
	remove: Some(snd_rn_acp_remove),
	driver: device_driver {
		pm: &rn_acp_pm,
	},
};

// module_param(acp_power_gating, int, 0644);
// MODULE_PARM_DESC(acp_power_gating, "Enable acp power gating");
// module_param(dmic_acpi_check, bint, 0644);
// MODULE_PARM_DESC(dmic_acpi_check, "Digital microphone presence (-1=auto, 0=none, 1=force)");
// MODULE_DEVICE_TABLE(pci, snd_rn_acp_ids);
// module_pci_driver(rn_acp_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP Renoir PCI driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
