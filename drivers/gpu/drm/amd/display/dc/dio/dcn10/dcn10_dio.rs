// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dc_hw_types.h, dm_services.h, reg_helper.h, and dcn10_dio.h.

/*
 * C macro equivalents retained as Rust macros.  The referenced types,
 * register helpers, and fields are provided by the surrounding codebase.
 */
macro_rules! CTX {
	($dio10:expr) => {
		$dio10.base.ctx
	};
}

macro_rules! REG {
	($dio10:expr, $reg:ident) => {
		$dio10.regs.$reg
	};
}

macro_rules! FN {
	($dio10:expr, $reg_name:ident, $field_name:ident) => {
		($dio10.shifts.$field_name, $dio10.masks.$field_name)
	};
}

unsafe fn dcn10_dio_mem_pwr_ctrl(
	dio: *mut dio,
	enable_i2c_light_sleep: bool,
) {
	let dio10: *mut dcn10_dio = TO_DCN10_DIO(dio);

	/* power AFMT HDMI memory */
	REG_WRITE!((*dio10), DIO_MEM_PWR_CTRL, 0);

	if enable_i2c_light_sleep {
		REG_UPDATE!((*dio10), DIO_MEM_PWR_CTRL, I2C_LIGHT_SLEEP_FORCE, 1);
	}
}

static dcn10_dio_funcs: dio_funcs = dio_funcs {
	mem_pwr_ctrl: Some(dcn10_dio_mem_pwr_ctrl),
};

unsafe fn dcn10_dio_construct(
	dio10: *mut dcn10_dio,
	ctx: *mut dc_context,
	regs: *const dcn_dio_registers,
	shifts: *const dcn_dio_shift,
	masks: *const dcn_dio_mask,
) {
	(*dio10).base.ctx = ctx;
	(*dio10).base.funcs = &dcn10_dio_funcs;

	(*dio10).regs = regs;
	(*dio10).shifts = shifts;
	(*dio10).masks = masks;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
