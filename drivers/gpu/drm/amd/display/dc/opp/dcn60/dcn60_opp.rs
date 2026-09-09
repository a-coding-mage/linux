// SPDX-License-Identifier: MIT
/* Copyright 2025 Advanced Micro Devices, Inc. */

// Dependency declarations are supplied by the corresponding headers.

macro_rules! REG {
	($oppn20:expr, $reg:ident) => {
		unsafe {
			(*(($oppn20).regs as *const dcn60_opp_registers)).$reg
		}
	};
}

macro_rules! FN {
	($oppn20:expr, $field_name:ident) => {
		unsafe {
			(
				(*(($oppn20).opp_shift as *const dcn60_opp_shift)).$field_name,
				(*(($oppn20).opp_mask as *const dcn60_opp_mask)).$field_name,
			)
		}
	};
}

macro_rules! CTX {
	($oppn20:expr) => {
		unsafe { (*(($oppn20).base.ctx)) }
	};
}

extern "C" {
	fn dcn20_opp_construct(
		oppn20: *mut dcn20_opp,
		ctx: *mut dc_context,
		inst: u32,
		regs: *const dcn20_opp_registers,
		opp_shift: *const dcn20_opp_shift,
		opp_mask: *const dcn20_opp_mask,
	);
}

pub unsafe extern "C" fn dcn60_opp_construct(
	oppn20: *mut dcn20_opp,
	ctx: *mut dc_context,
	inst: u32,
	regs: *const dcn60_opp_registers,
	opp_shift: *const dcn60_opp_shift,
	opp_mask: *const dcn60_opp_mask,
) {
	dcn20_opp_construct(
		oppn20,
		ctx,
		inst,
		regs as *const dcn20_opp_registers,
		opp_shift as *const dcn20_opp_shift,
		opp_mask as *const dcn20_opp_mask,
	);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
