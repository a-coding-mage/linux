// SPDX-License-Identifier: GPL-2.0+
//
// Machine driver for AMD Renoir platform using DMIC
//
// Copyright 2020 Advanced Micro Devices, Inc.

// C dependencies translated from:
// <sound/soc.h>, <sound/soc-dapm.h>, <linux/module.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, <linux/io.h>, and "rn_acp3x.h".

pub const DRV_NAME: &[u8; 13] = b"acp_pdm_mach\0";

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
	pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
	pub name: *const i8,
	pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
	pub driver: device_driver,
	pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[repr(C)]
pub struct module {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const i8,
	pub owner: *mut module,
	pub dai_link: *mut snd_soc_dai_link,
	pub num_links: i32,
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
	pub name: *const i8,
	pub dai_name: *const i8,
}

#[repr(C)]
pub struct snd_soc_dai_link {
	pub name: *const i8,
	pub stream_name: *const i8,
	pub capture_only: i32,
	pub cpus: *mut snd_soc_dai_link_component,
	pub num_cpus: u32,
	pub codecs: *mut snd_soc_dai_link_component,
	pub num_codecs: u32,
	pub platforms: *mut snd_soc_dai_link_component,
	pub num_platforms: u32,
}

unsafe extern "C" {
	pub static mut THIS_MODULE: *mut module;
	pub static snd_soc_pm_ops: dev_pm_ops;

	pub fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
	pub fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> i32;
	pub fn dev_err_probe(dev: *mut device, err: i32, fmt: *const i8, ...) -> i32;
	pub fn platform_driver_register(driver: *mut platform_driver) -> i32;
	pub fn platform_driver_unregister(driver: *mut platform_driver);
}

// SND_SOC_DAILINK_DEF(acp_pdm,
//		    DAILINK_COMP_ARRAY(COMP_CPU("acp_rn_pdm_dma.0")));
static mut acp_pdm: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: b"acp_rn_pdm_dma.0\0".as_ptr() as *const i8,
	dai_name: core::ptr::null(),
}];

// SND_SOC_DAILINK_DEF(dmic_codec,
//		    DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec.0",
//						  "dmic-hifi")));
static mut dmic_codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: b"dmic-codec.0\0".as_ptr() as *const i8,
	dai_name: b"dmic-hifi\0".as_ptr() as *const i8,
}];

// SND_SOC_DAILINK_DEF(platform,
//		    DAILINK_COMP_ARRAY(COMP_PLATFORM("acp_rn_pdm_dma.0")));
static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
	name: b"acp_rn_pdm_dma.0\0".as_ptr() as *const i8,
	dai_name: core::ptr::null(),
}];

static mut acp_dai_pdm: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
	name: b"acp3x-dmic-capture\0".as_ptr() as *const i8,
	stream_name: b"DMIC capture\0".as_ptr() as *const i8,
	capture_only: 1,
	// SND_SOC_DAILINK_REG(acp_pdm, dmic_codec, platform)
	cpus: core::ptr::addr_of_mut!(acp_pdm) as *mut snd_soc_dai_link_component,
	num_cpus: 1,
	codecs: core::ptr::addr_of_mut!(dmic_codec) as *mut snd_soc_dai_link_component,
	num_codecs: 1,
	platforms: core::ptr::addr_of_mut!(platform) as *mut snd_soc_dai_link_component,
	num_platforms: 1,
}];

static mut acp_card: snd_soc_card = snd_soc_card {
	name: b"acp\0".as_ptr() as *const i8,
	owner: unsafe { THIS_MODULE },
	dai_link: core::ptr::addr_of_mut!(acp_dai_pdm) as *mut snd_soc_dai_link,
	num_links: 1,
	dev: core::ptr::null_mut(),
};

unsafe extern "C" fn acp_probe(pdev: *mut platform_device) -> i32 {
	let ret: i32;
	let card: *mut snd_soc_card;

	card = core::ptr::addr_of_mut!(acp_card);
	acp_card.dev = core::ptr::addr_of_mut!((*pdev).dev);

	platform_set_drvdata(pdev, card as *mut core::ffi::c_void);
	ret = devm_snd_soc_register_card(core::ptr::addr_of_mut!((*pdev).dev), card);
	if ret != 0 {
		return dev_err_probe(
			core::ptr::addr_of_mut!((*pdev).dev),
			ret,
			b"snd_soc_register_card(%s) failed\n\0".as_ptr() as *const i8,
			(*card).name,
		);
	}
	return 0;
}

static mut acp_mach_driver: platform_driver = platform_driver {
	driver: device_driver {
		name: b"acp_pdm_mach\0".as_ptr() as *const i8,
		pm: unsafe { core::ptr::addr_of!(snd_soc_pm_ops) },
	},
	probe: Some(acp_probe),
};

// module_platform_driver(acp_mach_driver);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_module() -> i32 {
	platform_driver_register(core::ptr::addr_of_mut!(acp_mach_driver))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_module() {
	platform_driver_unregister(core::ptr::addr_of_mut!(acp_mach_driver));
}

// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD Renoir support for DMIC");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);
#[used]
#[unsafe(link_section = ".modinfo")]
static MODULE_AUTHOR: [u8; 35] = *b"author=Vijendar.Mukunda@amd.com\0";

#[used]
#[unsafe(link_section = ".modinfo")]
static MODULE_DESCRIPTION: [u8; 39] = *b"description=AMD Renoir support for DMIC\0";

#[used]
#[unsafe(link_section = ".modinfo")]
static MODULE_LICENSE: [u8; 15] = *b"license=GPL v2\0";

#[used]
#[unsafe(link_section = ".modinfo")]
static MODULE_ALIAS: [u8; 28] = *b"alias=platform:acp_pdm_mach\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
