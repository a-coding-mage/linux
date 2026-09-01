// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Intel Corporation

/* telemetry data queried from debug window */

// C dependencies:
// <sound/sof/ipc4/header.h>
// <sound/sof/xtensa.h>
// "../ipc4-priv.h"
// "../sof-priv.h"
// "hda.h"
// "telemetry.h"

use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
	pub dev: *mut device,
}

#[repr(C)]
pub struct coredump_hdr {
	pub id: [c_char; 2],
}

#[repr(C)]
pub struct coredump_arch_hdr {
	pub id: c_char,
}

#[repr(C)]
pub struct sof_ipc4_telemetry_slot_data {
	pub separator: u32,
	pub hdr: coredump_hdr,
	pub arch_hdr: coredump_arch_hdr,
}

#[repr(C)]
pub struct xtensa_arch_block {
	pub soc: u32,
	pub toolchain: u32,
	pub exccause: u32,
	pub excvaddr: u32,
	pub pc: u32,
	pub ps: u32,
	pub sar: u32,
	pub ar: [u32; XTENSA_CORE_AR_REGS_COUNT as usize],
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_plat_hdr {
	pub numaregs: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
	pub exccause: u32,
	pub excvaddr: u32,
	pub epc1: u32,
	pub ps: u32,
	pub sar: u32,
	pub plat_hdr: sof_ipc_dsp_oops_plat_hdr,
	pub ar: [u32; XTENSA_CORE_AR_REGS_COUNT as usize],
}

extern "C" {
	static KERN_DEBUG: *const c_char;
	static KERN_ERR: *const c_char;

	static SOF_DBG_DUMP_OPTIONAL: u32;
	static SOF_IPC4_DEBUG_SLOT_TELEMETRY: u32;
	static XTENSA_CORE_DUMP_SEPARATOR: u32;
	static XTENSA_SOC_INTEL_ADSP: u32;
	static COREDUMP_HDR_ID0: c_char;
	static COREDUMP_HDR_ID1: c_char;
	static COREDUMP_ARCH_HDR_ID: c_char;
	static XTENSA_TOOL_CHAIN_ZEPHYR: u32;
	static XTENSA_TOOL_CHAIN_XCC: u32;
}

pub const XTENSA_CORE_AR_REGS_COUNT: u32 = 64;

extern "C" {
	fn sof_ipc4_find_debug_slot_offset_by_type(sdev: *mut snd_sof_dev, slot_type: u32) -> u32;
	fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: usize);
	fn sof_oops(sdev: *mut snd_sof_dev, level: *const c_char, xoops: *mut sof_ipc_dsp_oops_xtensa);
	fn sof_stack(
		sdev: *mut snd_sof_dev,
		level: *const c_char,
		xoops: *mut sof_ipc_dsp_oops_xtensa,
		stack: *mut c_void,
		stack_words: u32,
	);
	fn kmalloc(size: usize, flags: u32) -> *mut c_void;
	fn kzalloc(size: usize, flags: u32) -> *mut c_void;
	fn kfree(ptr: *mut c_void);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_printk(level: *const c_char, dev: *mut device, fmt: *const c_char, ...);
}

const GFP_KERNEL: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_intel_dump_telemetry_state(
	sdev: *mut snd_sof_dev,
	flags: u32,
) {
	static INVALID_SLOT_MSG: &[u8] = b"Core dump is not available due to\0";
	let mut telemetry_data: *mut sof_ipc4_telemetry_slot_data;
	let mut xoops: *mut sof_ipc_dsp_oops_xtensa;
	let mut block: *mut xtensa_arch_block;
	let slot_offset: u32;
	let level: *const c_char;

	level = if (flags & SOF_DBG_DUMP_OPTIONAL) != 0 {
		KERN_DEBUG
	} else {
		KERN_ERR
	};

	slot_offset = sof_ipc4_find_debug_slot_offset_by_type(sdev, SOF_IPC4_DEBUG_SLOT_TELEMETRY);
	if slot_offset == 0 {
		return;
	}

	telemetry_data = kmalloc(size_of::<sof_ipc4_telemetry_slot_data>(), GFP_KERNEL)
		as *mut sof_ipc4_telemetry_slot_data;
	if telemetry_data.is_null() {
		return;
	}
	sof_mailbox_read(
		sdev,
		slot_offset,
		telemetry_data as *mut c_void,
		size_of::<sof_ipc4_telemetry_slot_data>(),
	);
	if (*telemetry_data).separator != XTENSA_CORE_DUMP_SEPARATOR {
		dev_err(
			(*sdev).dev,
			b"%s invalid separator %#x\n\0".as_ptr() as *const c_char,
			INVALID_SLOT_MSG.as_ptr() as *const c_char,
			(*telemetry_data).separator,
		);
		goto_free_telemetry_data(telemetry_data);
		return;
	}

	block = kmalloc(size_of::<xtensa_arch_block>(), GFP_KERNEL) as *mut xtensa_arch_block;
	if block.is_null() {
		goto_free_telemetry_data(telemetry_data);
		return;
	}

	sof_mailbox_read(
		sdev,
		slot_offset.wrapping_add(size_of::<sof_ipc4_telemetry_slot_data>() as u32),
		block as *mut c_void,
		size_of::<xtensa_arch_block>(),
	);
	if (*block).soc != XTENSA_SOC_INTEL_ADSP {
		dev_err(
			(*sdev).dev,
			b"%s invalid SOC %d\n\0".as_ptr() as *const c_char,
			INVALID_SLOT_MSG.as_ptr() as *const c_char,
			(*block).soc,
		);
		goto_free_block(block, telemetry_data);
		return;
	}

	if (*telemetry_data).hdr.id[0] != COREDUMP_HDR_ID0
		|| (*telemetry_data).hdr.id[1] != COREDUMP_HDR_ID1
		|| (*telemetry_data).arch_hdr.id != COREDUMP_ARCH_HDR_ID
	{
		dev_err(
			(*sdev).dev,
			b"%s invalid coredump header %c%c, arch hdr %c\n\0".as_ptr() as *const c_char,
			INVALID_SLOT_MSG.as_ptr() as *const c_char,
			(*telemetry_data).hdr.id[0],
			(*telemetry_data).hdr.id[1],
			(*telemetry_data).arch_hdr.id,
		);
		goto_free_block(block, telemetry_data);
		return;
	}

	if (*block).toolchain == XTENSA_TOOL_CHAIN_ZEPHYR {
		dev_printk(
			level,
			(*sdev).dev,
			b"FW is built with Zephyr toolchain\n\0".as_ptr() as *const c_char,
		);
	} else if (*block).toolchain == XTENSA_TOOL_CHAIN_XCC {
		dev_printk(
			level,
			(*sdev).dev,
			b"FW is built with XCC toolchain\n\0".as_ptr() as *const c_char,
		);
	} else {
		dev_printk(
			level,
			(*sdev).dev,
			b"Unknown toolchain is used\n\0".as_ptr() as *const c_char,
		);
	}

	xoops = kzalloc(size_of::<sof_ipc_dsp_oops_xtensa>(), GFP_KERNEL)
		as *mut sof_ipc_dsp_oops_xtensa;
	if xoops.is_null() {
		goto_free_block(block, telemetry_data);
		return;
	}

	(*xoops).exccause = (*block).exccause;
	(*xoops).excvaddr = (*block).excvaddr;
	(*xoops).epc1 = (*block).pc;
	(*xoops).ps = (*block).ps;
	(*xoops).sar = (*block).sar;

	(*xoops).plat_hdr.numaregs = XTENSA_CORE_AR_REGS_COUNT;
	ptr::copy_nonoverlapping(
		(*block).ar.as_ptr(),
		(*xoops).ar.as_mut_ptr(),
		XTENSA_CORE_AR_REGS_COUNT as usize,
	);

	sof_oops(sdev, level, xoops);
	sof_stack(sdev, level, xoops, ptr::null_mut(), 0);

	kfree(xoops as *mut c_void);
	goto_free_block(block, telemetry_data);
}

unsafe fn goto_free_block(
	block: *mut xtensa_arch_block,
	telemetry_data: *mut sof_ipc4_telemetry_slot_data,
) {
	kfree(block as *mut c_void);
	goto_free_telemetry_data(telemetry_data);
}

unsafe fn goto_free_telemetry_data(telemetry_data: *mut sof_ipc4_telemetry_slot_data) {
	kfree(telemetry_data as *mut c_void);
}

// EXPORT_SYMBOL_NS(sof_ipc4_intel_dump_telemetry_state, "SND_SOC_SOF_INTEL_HDA_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
