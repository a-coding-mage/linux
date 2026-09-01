// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2023 Intel Corporation

/*
 * Hardware interface for audio DSP on LunarLake.
 */

// C dependencies:
// linux/debugfs.h, linux/firmware.h, sound/hda_register.h,
// sound/sof/ipc4/header.h, trace/events/sof_intel.h, ../ipc4-priv.h,
// ../ops.h, hda.h, hda-ipc.h, ../sof-audio.h, mtl.h, lnl.h,
// sound/hda-mlink.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct hdac_bus {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
	pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dev {
	pub first_boot: bool,
	pub pdata: *mut snd_sof_pdata,
	pub debugfs_root: *mut dentry,
	pub dspless_mode_selected: bool,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
	pub imrboot_supported: bool,
	pub skip_imr_boot: bool,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
	pub probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
	pub post_fw_run: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub runtime_resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_intel_dsp_desc {
	pub cores_num: c_int,
	pub init_core_mask: u32,
	pub host_managed_cores_mask: u32,
	pub ipc_req: u32,
	pub ipc_req_mask: u32,
	pub ipc_ack: u32,
	pub ipc_ack_mask: u32,
	pub ipc_ctl: u32,
	pub rom_status_reg: u32,
	pub rom_init_timeout: c_int,
	pub ssp_count: c_int,
	pub d0i3_offset: u32,
	pub read_sdw_lcount: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub check_sdw_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
	pub check_sdw_wakeen_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
	pub sdw_process_wakeen: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
	pub check_ipc_irq: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> bool>,
	pub cl_init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub power_down_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub disable_interrupts: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub hw_ip_version: c_int,
	pub platform: *const c_char,
}

unsafe extern "C" {
	fn hdac_bus_eml_enable_offload(
		bus: *mut hdac_bus,
		alt: bool,
		leptr_id: c_int,
		enable: bool,
	);
	fn hda_dsp_probe(sdev: *mut snd_sof_dev) -> c_int;
	fn hda_dsp_remove(sdev: *mut snd_sof_dev);
	fn hda_dsp_resume(sdev: *mut snd_sof_dev) -> c_int;
	fn hda_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int;
	fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
	fn sof_debug_check_flag(flag: c_int) -> bool;
	fn debugfs_create_bool(
		name: *const c_char,
		mode: u32,
		parent: *mut dentry,
		value: *mut bool,
	) -> *mut dentry;
	fn sof_mtl_set_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops) -> c_int;
	fn hdac_bus_eml_check_interrupt(bus: *mut hdac_bus, alt: bool, leptr_id: c_int) -> bool;
	fn mtl_disable_ipc_interrupts(sdev: *mut snd_sof_dev);
	fn mtl_enable_interrupts(sdev: *mut snd_sof_dev, enable: bool) -> c_int;
	fn snd_hdac_chip_readw(bus: *mut hdac_bus, reg: c_int) -> u16;

	fn hda_sdw_check_lcount_ext(sdev: *mut snd_sof_dev) -> c_int;
	fn hda_sdw_process_wakeen_common(sdev: *mut snd_sof_dev);
	fn mtl_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
	fn mtl_dsp_cl_init(sdev: *mut snd_sof_dev) -> c_int;
	fn mtl_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
}

const AZX_REG_ML_LEPTR_ID_INTEL_SSP: c_int = 0;
const AZX_REG_ML_LEPTR_ID_INTEL_DMIC: c_int = 0;
const AZX_REG_ML_LEPTR_ID_INTEL_UAOL: c_int = 0;
const AZX_REG_ML_LEPTR_ID_SDW: c_int = 0;
const SOF_DBG_IGNORE_D3_PERSISTENT: c_int = 0;
const STATESTS: c_int = 0;
const SDW_MAX_DEVICES: u32 = 0;
const SDW_INTEL_DEV_NUM_IDA_MIN: u32 = 0;
const MTL_DSP_REG_HFIPCXIDR: u32 = 0;
const MTL_DSP_REG_HFIPCXIDR_BUSY: u32 = 0;
const MTL_DSP_REG_HFIPCXIDA: u32 = 0;
const MTL_DSP_REG_HFIPCXIDA_DONE: u32 = 0;
const MTL_DSP_REG_HFIPCXCTL: u32 = 0;
const LNL_DSP_REG_HFDSC: u32 = 0;
const MTL_SSP_COUNT: c_int = 0;
const MTL_HDA_VS_D0I3C: u32 = 0;
const SOF_INTEL_ACE_2_0: c_int = 0;

const fn bit(nr: u32) -> u32 {
	1u32 << nr
}

const fn genmask(high: u32, low: u32) -> u16 {
	((((!0u32) << low) & ((!0u32) >> (31 - high))) & 0xffff) as u16
}

/* Configure DSP offload for DMIC/SSP/UAOL */
unsafe extern "C" fn hdac_bus_set_dsp_offload(bus: *mut hdac_bus, enable: bool) {
	unsafe {
		hdac_bus_eml_enable_offload(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_SSP, enable);
		hdac_bus_eml_enable_offload(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_DMIC, enable);
		hdac_bus_eml_enable_offload(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_UAOL, enable);
	}
}

unsafe extern "C" fn lnl_hda_dsp_probe(sdev: *mut snd_sof_dev) -> c_int {
	let ret: c_int;

	unsafe {
		ret = hda_dsp_probe(sdev);
		if ret < 0 {
			return ret;
		}

		hdac_bus_set_dsp_offload(sof_to_bus(sdev), true);
	}

	0
}

unsafe extern "C" fn lnl_hda_dsp_remove(sdev: *mut snd_sof_dev) {
	unsafe {
		hdac_bus_set_dsp_offload(sof_to_bus(sdev), false);
		hda_dsp_remove(sdev);
	}
}

unsafe extern "C" fn lnl_hda_dsp_resume(sdev: *mut snd_sof_dev) -> c_int {
	let ret: c_int;

	unsafe {
		ret = hda_dsp_resume(sdev);
		if ret < 0 {
			return ret;
		}

		hdac_bus_set_dsp_offload(sof_to_bus(sdev), true);
	}

	0
}

unsafe extern "C" fn lnl_hda_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int {
	let ret: c_int;

	unsafe {
		ret = hda_dsp_runtime_resume(sdev);
		if ret < 0 {
			return ret;
		}

		hdac_bus_set_dsp_offload(sof_to_bus(sdev), true);
	}

	0
}

unsafe extern "C" fn lnl_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
	unsafe {
		if (*sdev).first_boot {
			let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

			/* Check if IMR boot is usable */
			if !sof_debug_check_flag(SOF_DBG_IGNORE_D3_PERSISTENT) {
				(*hda).imrboot_supported = true;
				debugfs_create_bool(
					c"skip_imr_boot".as_ptr(),
					0o644,
					(*sdev).debugfs_root,
					&mut (*hda).skip_imr_boot,
				);
			}
		}
	}

	0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_lnl_set_ops(
	sdev: *mut snd_sof_dev,
	dsp_ops: *mut snd_sof_dsp_ops,
) -> c_int {
	let ret: c_int;

	unsafe {
		ret = sof_mtl_set_ops(sdev, dsp_ops);
		if ret != 0 {
			return ret;
		}

		/* probe/remove */
		if !(*sdev).dspless_mode_selected {
			(*dsp_ops).probe = Some(lnl_hda_dsp_probe);
			(*dsp_ops).remove = Some(lnl_hda_dsp_remove);
		}

		/* post fw run */
		(*dsp_ops).post_fw_run = Some(lnl_dsp_post_fw_run);

		/* PM */
		if !(*sdev).dspless_mode_selected {
			(*dsp_ops).resume = Some(lnl_hda_dsp_resume);
			(*dsp_ops).runtime_resume = Some(lnl_hda_dsp_runtime_resume);
		}
	}

	0
}
// EXPORT_SYMBOL_NS(sof_lnl_set_ops, "SND_SOC_SOF_INTEL_LNL");

/* Check if an SDW IRQ occurred */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lnl_dsp_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool {
	let bus: *mut hdac_bus;

	unsafe {
		bus = sof_to_bus(sdev);

		hdac_bus_eml_check_interrupt(bus, true, AZX_REG_ML_LEPTR_ID_SDW)
	}
}
// EXPORT_SYMBOL_NS(lnl_dsp_check_sdw_irq, "SND_SOC_SOF_INTEL_LNL");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lnl_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int {
	unsafe {
		mtl_disable_ipc_interrupts(sdev);
		mtl_enable_interrupts(sdev, false)
	}
}
// EXPORT_SYMBOL_NS(lnl_dsp_disable_interrupts, "SND_SOC_SOF_INTEL_LNL");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lnl_sdw_check_wakeen_irq(sdev: *mut snd_sof_dev) -> bool {
	let bus: *mut hdac_bus;
	let wake_sts: u16;

	unsafe {
		bus = sof_to_bus(sdev);

		/*
		 * we need to use the global HDaudio WAKEEN/STS to be able to
		 * detect wakes in low-power modes. The link-specific information
		 * is handled in the process_wakeen() helper, this helper only
		 * detects a SoundWire wake without identifying the link.
		 */
		wake_sts = snd_hdac_chip_readw(bus, STATESTS);
	}

	/* filter out the range of SDIs that can be set for SoundWire */
	(wake_sts & genmask(SDW_MAX_DEVICES, SDW_INTEL_DEV_NUM_IDA_MIN)) != 0
}
// EXPORT_SYMBOL_NS(lnl_sdw_check_wakeen_irq, "SND_SOC_SOF_INTEL_LNL");

#[unsafe(no_mangle)]
pub static lnl_chip_info: sof_intel_dsp_desc = sof_intel_dsp_desc {
	cores_num: 5,
	init_core_mask: bit(0),
	host_managed_cores_mask: bit(0),
	ipc_req: MTL_DSP_REG_HFIPCXIDR,
	ipc_req_mask: MTL_DSP_REG_HFIPCXIDR_BUSY,
	ipc_ack: MTL_DSP_REG_HFIPCXIDA,
	ipc_ack_mask: MTL_DSP_REG_HFIPCXIDA_DONE,
	ipc_ctl: MTL_DSP_REG_HFIPCXCTL,
	rom_status_reg: LNL_DSP_REG_HFDSC,
	rom_init_timeout: 300,
	ssp_count: MTL_SSP_COUNT,
	d0i3_offset: MTL_HDA_VS_D0I3C,
	read_sdw_lcount: Some(hda_sdw_check_lcount_ext),
	check_sdw_irq: Some(lnl_dsp_check_sdw_irq),
	check_sdw_wakeen_irq: Some(lnl_sdw_check_wakeen_irq),
	sdw_process_wakeen: Some(hda_sdw_process_wakeen_common),
	check_ipc_irq: Some(mtl_dsp_check_ipc_irq),
	cl_init: Some(mtl_dsp_cl_init),
	power_down_dsp: Some(mtl_power_down_dsp),
	disable_interrupts: Some(lnl_dsp_disable_interrupts),
	hw_ip_version: SOF_INTEL_ACE_2_0,
	platform: c"lnl".as_ptr(),
};

// MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_MTL");
// MODULE_IMPORT_NS("SND_SOC_SOF_HDA_MLINK");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
