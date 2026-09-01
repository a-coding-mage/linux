// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Pan Xiuli <xiuli.pan@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_uchar, c_void};

// C dependencies:
// #include <linux/module.h>
// #include <sound/sof.h>
// #include <sound/sof/xtensa.h>
// #include "../sof-priv.h"

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
	pub dev: *mut device,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa_plat_hdr {
	pub stackptr: u32,
	pub numaregs: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
	pub plat_hdr: sof_ipc_dsp_oops_xtensa_plat_hdr,
	pub exccause: u32,
	pub excvaddr: u32,
	pub ps: u32,
	pub sar: u32,
	pub epc1: u32,
	pub epc2: u32,
	pub epc3: u32,
	pub epc4: u32,
	pub epc5: u32,
	pub epc6: u32,
	pub epc7: u32,
	pub depc: u32,
	pub eps2: u32,
	pub eps3: u32,
	pub eps4: u32,
	pub eps5: u32,
	pub eps6: u32,
	pub eps7: u32,
	pub intenable: u32,
	pub interrupt: u32,
	pub ar: [u32; 0],
}

#[repr(C)]
pub struct dsp_arch_ops {
	pub dsp_oops: Option<unsafe extern "C" fn(*mut snd_sof_dev, *const c_char, *mut c_void)>,
	pub dsp_stack:
		Option<unsafe extern "C" fn(*mut snd_sof_dev, *const c_char, *mut c_void, *mut u32, u32)>,
}

unsafe extern "C" {
	fn dev_printk(level: *const c_char, dev: *mut device, fmt: *const c_char, ...);
	fn hex_dump_to_buffer(
		buf: *const c_void,
		len: usize,
		rowsize: c_int,
		groupsize: c_int,
		linebuf: *mut c_uchar,
		linebuflen: usize,
		ascii: bool,
	);
}

#[repr(C)]
struct xtensa_exception_cause {
	id: u32,
	msg: *const c_char,
	description: *const c_char,
}

unsafe impl Sync for xtensa_exception_cause {}

/*
 * From 4.4.1.5 table 4-64 Exception Causes of Xtensa
 * Instruction Set Architecture (ISA) Reference Manual
 */
static xtensa_exception_causes: [xtensa_exception_cause; 29] = [
	xtensa_exception_cause {
		id: 0,
		msg: c"IllegalInstructionCause".as_ptr(),
		description: c"Illegal instruction".as_ptr(),
	},
	xtensa_exception_cause {
		id: 1,
		msg: c"SyscallCause".as_ptr(),
		description: c"SYSCALL instruction".as_ptr(),
	},
	xtensa_exception_cause {
		id: 2,
		msg: c"InstructionFetchErrorCause".as_ptr(),
		description: c"Processor internal physical address or data error during instruction fetch".as_ptr(),
	},
	xtensa_exception_cause {
		id: 3,
		msg: c"LoadStoreErrorCause".as_ptr(),
		description: c"Processor internal physical address or data error during load or store".as_ptr(),
	},
	xtensa_exception_cause {
		id: 4,
		msg: c"Level1InterruptCause".as_ptr(),
		description: c"Level-1 interrupt as indicated by set level-1 bits in the INTERRUPT register"
			.as_ptr(),
	},
	xtensa_exception_cause {
		id: 5,
		msg: c"AllocaCause".as_ptr(),
		description: c"MOVSP instruction, if caller's registers are not in the register file".as_ptr(),
	},
	xtensa_exception_cause {
		id: 6,
		msg: c"IntegerDivideByZeroCause".as_ptr(),
		description: c"QUOS, QUOU, REMS, or REMU divisor operand is zero".as_ptr(),
	},
	xtensa_exception_cause {
		id: 8,
		msg: c"PrivilegedCause".as_ptr(),
		description: c"Attempt to execute a privileged operation when CRING ? 0".as_ptr(),
	},
	xtensa_exception_cause {
		id: 9,
		msg: c"LoadStoreAlignmentCause".as_ptr(),
		description: c"Load or store to an unaligned address".as_ptr(),
	},
	xtensa_exception_cause {
		id: 12,
		msg: c"InstrPIFDataErrorCause".as_ptr(),
		description: c"PIF data error during instruction fetch".as_ptr(),
	},
	xtensa_exception_cause {
		id: 13,
		msg: c"LoadStorePIFDataErrorCause".as_ptr(),
		description: c"Synchronous PIF data error during LoadStore access".as_ptr(),
	},
	xtensa_exception_cause {
		id: 14,
		msg: c"InstrPIFAddrErrorCause".as_ptr(),
		description: c"PIF address error during instruction fetch".as_ptr(),
	},
	xtensa_exception_cause {
		id: 15,
		msg: c"LoadStorePIFAddrErrorCause".as_ptr(),
		description: c"Synchronous PIF address error during LoadStore access".as_ptr(),
	},
	xtensa_exception_cause {
		id: 16,
		msg: c"InstTLBMissCause".as_ptr(),
		description: c"Error during Instruction TLB refill".as_ptr(),
	},
	xtensa_exception_cause {
		id: 17,
		msg: c"InstTLBMultiHitCause".as_ptr(),
		description: c"Multiple instruction TLB entries matched".as_ptr(),
	},
	xtensa_exception_cause {
		id: 18,
		msg: c"InstFetchPrivilegeCause".as_ptr(),
		description: c"An instruction fetch referenced a virtual address at a ring level less than CRING"
			.as_ptr(),
	},
	xtensa_exception_cause {
		id: 20,
		msg: c"InstFetchProhibitedCause".as_ptr(),
		description:
			c"An instruction fetch referenced a page mapped with an attribute that does not permit instruction fetch"
				.as_ptr(),
	},
	xtensa_exception_cause {
		id: 24,
		msg: c"LoadStoreTLBMissCause".as_ptr(),
		description: c"Error during TLB refill for a load or store".as_ptr(),
	},
	xtensa_exception_cause {
		id: 25,
		msg: c"LoadStoreTLBMultiHitCause".as_ptr(),
		description: c"Multiple TLB entries matched for a load or store".as_ptr(),
	},
	xtensa_exception_cause {
		id: 26,
		msg: c"LoadStorePrivilegeCause".as_ptr(),
		description: c"A load or store referenced a virtual address at a ring level less than CRING"
			.as_ptr(),
	},
	xtensa_exception_cause {
		id: 28,
		msg: c"LoadProhibitedCause".as_ptr(),
		description: c"A load referenced a page mapped with an attribute that does not permit loads".as_ptr(),
	},
	xtensa_exception_cause {
		id: 32,
		msg: c"Coprocessor0Disabled".as_ptr(),
		description: c"Coprocessor 0 instruction when cp0 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 33,
		msg: c"Coprocessor1Disabled".as_ptr(),
		description: c"Coprocessor 1 instruction when cp1 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 34,
		msg: c"Coprocessor2Disabled".as_ptr(),
		description: c"Coprocessor 2 instruction when cp2 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 35,
		msg: c"Coprocessor3Disabled".as_ptr(),
		description: c"Coprocessor 3 instruction when cp3 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 36,
		msg: c"Coprocessor4Disabled".as_ptr(),
		description: c"Coprocessor 4 instruction when cp4 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 37,
		msg: c"Coprocessor5Disabled".as_ptr(),
		description: c"Coprocessor 5 instruction when cp5 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 38,
		msg: c"Coprocessor6Disabled".as_ptr(),
		description: c"Coprocessor 6 instruction when cp6 disabled".as_ptr(),
	},
	xtensa_exception_cause {
		id: 39,
		msg: c"Coprocessor7Disabled".as_ptr(),
		description: c"Coprocessor 7 instruction when cp7 disabled".as_ptr(),
	},
];

/* only need xtensa atm */
unsafe extern "C" fn xtensa_dsp_oops(
	sdev: *mut snd_sof_dev,
	level: *const c_char,
	oops: *mut c_void,
) {
	let xoops = oops as *mut sof_ipc_dsp_oops_xtensa;

	unsafe {
		dev_printk(level, (*sdev).dev, c"error: DSP Firmware Oops\n".as_ptr());
		for i in 0..xtensa_exception_causes.len() {
			if xtensa_exception_causes[i].id == (*xoops).exccause {
				dev_printk(
					level,
					(*sdev).dev,
					c"error: Exception Cause: %s, %s\n".as_ptr(),
					xtensa_exception_causes[i].msg,
					xtensa_exception_causes[i].description,
				);
			}
		}
		dev_printk(
			level,
			(*sdev).dev,
			c"EXCCAUSE 0x%8.8x EXCVADDR 0x%8.8x PS       0x%8.8x SAR     0x%8.8x\n"
				.as_ptr(),
			(*xoops).exccause,
			(*xoops).excvaddr,
			(*xoops).ps,
			(*xoops).sar,
		);
		dev_printk(
			level,
			(*sdev).dev,
			c"EPC1     0x%8.8x EPC2     0x%8.8x EPC3     0x%8.8x EPC4    0x%8.8x"
				.as_ptr(),
			(*xoops).epc1,
			(*xoops).epc2,
			(*xoops).epc3,
			(*xoops).epc4,
		);
		dev_printk(
			level,
			(*sdev).dev,
			c"EPC5     0x%8.8x EPC6     0x%8.8x EPC7     0x%8.8x DEPC    0x%8.8x"
				.as_ptr(),
			(*xoops).epc5,
			(*xoops).epc6,
			(*xoops).epc7,
			(*xoops).depc,
		);
		dev_printk(
			level,
			(*sdev).dev,
			c"EPS2     0x%8.8x EPS3     0x%8.8x EPS4     0x%8.8x EPS5    0x%8.8x"
				.as_ptr(),
			(*xoops).eps2,
			(*xoops).eps3,
			(*xoops).eps4,
			(*xoops).eps5,
		);
		dev_printk(
			level,
			(*sdev).dev,
			c"EPS6     0x%8.8x EPS7     0x%8.8x INTENABL 0x%8.8x INTERRU 0x%8.8x"
				.as_ptr(),
			(*xoops).eps6,
			(*xoops).eps7,
			(*xoops).intenable,
			(*xoops).interrupt,
		);
	}
}

unsafe extern "C" fn xtensa_stack(
	sdev: *mut snd_sof_dev,
	level: *const c_char,
	oops: *mut c_void,
	stack: *mut u32,
	stack_words: u32,
) {
	let xoops = oops as *mut sof_ipc_dsp_oops_xtensa;
	let stack_ptr: u32 = unsafe { (*xoops).plat_hdr.stackptr };
	/* 4 * 8chars + 3 ws + 1 terminating NUL */
	let mut buf: [c_uchar; 4 * 8 + 3 + 1] = [0; 4 * 8 + 3 + 1];
	let mut i: u32;

	unsafe {
		dev_printk(
			level,
			(*sdev).dev,
			c"stack dump from 0x%8.8x\n".as_ptr(),
			stack_ptr,
		);

		/*
		 * example output:
		 * 0x0049fbb0: 8000f2d0 0049fc00 6f6c6c61 00632e63
		 */
		i = 0;
		while i < stack_words {
			hex_dump_to_buffer(
				stack.add(i as usize) as *const c_void,
				16,
				16,
				4,
				buf.as_mut_ptr(),
				buf.len(),
				false,
			);
			dev_printk(
				level,
				(*sdev).dev,
				c"0x%08x: %s\n".as_ptr(),
				stack_ptr.wrapping_add(i.wrapping_mul(4)),
				buf.as_mut_ptr(),
			);
			i = i.wrapping_add(4);
		}

		if (*xoops).plat_hdr.numaregs == 0 {
			return;
		}

		dev_printk(level, (*sdev).dev, c"AR registers:\n".as_ptr());
		/* the number of ar registers is a multiple of 4 */
		i = 0;
		while i < (*xoops).plat_hdr.numaregs {
			hex_dump_to_buffer(
				(*xoops).ar.as_ptr().add(i as usize) as *const c_void,
				16,
				16,
				4,
				buf.as_mut_ptr(),
				buf.len(),
				false,
			);
			dev_printk(
				level,
				(*sdev).dev,
				c"%#x: %s\n".as_ptr(),
				i.wrapping_mul(4),
				buf.as_mut_ptr(),
			);
			i = i.wrapping_add(4);
		}
	}
}

#[unsafe(no_mangle)]
pub static sof_xtensa_arch_ops: dsp_arch_ops = dsp_arch_ops {
	dsp_oops: Some(xtensa_dsp_oops),
	dsp_stack: Some(xtensa_stack),
};
// EXPORT_SYMBOL_NS(sof_xtensa_arch_ops, "SND_SOC_SOF_XTENSA");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF Xtensa DSP support");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
