// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on clk-super.c
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 *
 * Based on older tegra20-cpufreq driver by Colin Cross <ccross@google.com>
 * Copyright (C) 2010 Google, Inc.
 *
 * Author: Dmitry Osipenko <digetx@gmail.com>
 * Copyright (C) 2019 GRATE-DRIVER project
 */

// Linux header dependencies are supplied by the surrounding translation unit.

const PLLP_INDEX: u8 = 4;
const PLLX_INDEX: u8 = 8;

const SUPER_CDIV_ENB: u32 = 1u32 << 31;
const TSENSOR_SLOWDOWN: u32 = 1u32 << 23;

static mut cclk_super: *mut tegra_clk_super_mux = core::ptr::null_mut();
static mut cclk_on_pllx: bool = false;

unsafe fn cclk_super_get_parent(hw: *mut clk_hw) -> u8 {
	return tegra_clk_super_ops.get_parent.unwrap()(hw);
}

unsafe fn cclk_super_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
	return tegra_clk_super_ops.set_parent.unwrap()(hw, index);
}

unsafe fn cclk_super_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
	return tegra_clk_super_ops.set_rate.unwrap()(hw, rate, parent_rate);
}

unsafe fn cclk_super_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
	let super_: *mut tegra_clk_super_mux = to_clk_super_mux(hw);
	let val: u32 = readl_relaxed((*super_).reg);
	let div2: u32;

	/* check whether thermal throttling is active */
	if val & TSENSOR_SLOWDOWN != 0 {
		div2 = 1;
	} else {
		div2 = 0;
	}

	if cclk_super_get_parent(hw) == PLLX_INDEX {
		return parent_rate >> div2;
	}

	return tegra_clk_super_ops.recalc_rate.unwrap()(hw, parent_rate) >> div2;
}

unsafe fn cclk_super_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
	let pllp_hw: *mut clk_hw = clk_hw_get_parent_by_index(hw, PLLP_INDEX);
	let pllx_hw: *mut clk_hw = clk_hw_get_parent_by_index(hw, PLLX_INDEX);
	let super_: *mut tegra_clk_super_mux = to_clk_super_mux(hw);
	let pllp_rate: c_ulong;
	let mut rate: c_long = (*req).rate as c_long;

	if WARN_ON_ONCE(pllp_hw.is_null() || pllx_hw.is_null()) {
		return -EINVAL;
	}

	/*
	 * Switch parent to PLLP for all CCLK rates that are suitable for PLLP.
	 * PLLX will be disabled in this case, saving some power.
	 */
	pllp_rate = clk_hw_get_rate(pllp_hw);

	if rate <= pllp_rate as c_long {
		if (*super_).flags & TEGRA20_SUPER_CLK != 0 {
			rate = pllp_rate as c_long;
		} else {
			let mut parent = clk_rate_request {
				rate: (*req).rate,
				best_parent_rate: pllp_rate,
				..core::mem::zeroed()
			};
			clk_hw_get_rate_range(hw, &mut parent.min_rate, &mut parent.max_rate);
			tegra_clk_super_ops.determine_rate.unwrap()(hw, &mut parent);
			pllp_rate = parent.best_parent_rate;
			rate = parent.rate as c_long;
		}

		(*req).best_parent_rate = pllp_rate;
		(*req).best_parent_hw = pllp_hw;
		(*req).rate = rate as c_ulong;
	} else {
		rate = clk_hw_round_rate(pllx_hw, rate as c_ulong) as c_long;
		(*req).best_parent_rate = rate as c_ulong;
		(*req).best_parent_hw = pllx_hw;
		(*req).rate = rate as c_ulong;
	}

	if WARN_ON_ONCE(rate <= 0) {
		return -EINVAL;
	}

	return 0;
}

static tegra_cclk_super_ops: clk_ops = clk_ops {
	get_parent: Some(cclk_super_get_parent),
	set_parent: Some(cclk_super_set_parent),
	set_rate: Some(cclk_super_set_rate),
	recalc_rate: Some(cclk_super_recalc_rate),
	determine_rate: Some(cclk_super_determine_rate),
	..clk_ops::zeroed()
};

static tegra_cclk_super_mux_ops: clk_ops = clk_ops {
	get_parent: Some(cclk_super_get_parent),
	set_parent: Some(cclk_super_set_parent),
	determine_rate: Some(cclk_super_determine_rate),
	..clk_ops::zeroed()
};

unsafe fn tegra_clk_register_super_cclk(
	name: *const c_char,
	parent_names: *const *const c_char,
	num_parents: u8,
	flags: c_ulong,
	reg: *mut core::ffi::c_void,
	clk_super_flags: u8,
	lock: *mut spinlock_t,
) -> *mut clk {
	let super_: *mut tegra_clk_super_mux;
	let clk: *mut clk;
	let mut init: clk_init_data = core::mem::zeroed();
	let mut val: u32;

	if WARN_ON(!cclk_super.is_null()) {
		return ERR_PTR(-EBUSY);
	}

	super_ = kzalloc_obj();
	if super_.is_null() {
		return ERR_PTR(-ENOMEM);
	}

	init.name = name;
	init.flags = flags;
	init.parent_names = parent_names;
	init.num_parents = num_parents;

	(*super_).reg = reg;
	(*super_).lock = lock;
	(*super_).width = 4;
	(*super_).flags = clk_super_flags;
	(*super_).hw.init = &mut init;

	if (*super_).flags & TEGRA20_SUPER_CLK != 0 {
		init.ops = &tegra_cclk_super_mux_ops;
	} else {
		init.ops = &tegra_cclk_super_ops;
		(*super_).frac_div.reg = reg.add(4);
		(*super_).frac_div.shift = 16;
		(*super_).frac_div.width = 8;
		(*super_).frac_div.frac_width = 1;
		(*super_).frac_div.lock = lock;
		(*super_).div_ops = &tegra_clk_frac_div_ops;
	}

	/* Disable the clock-skipper; its topology is documented in the C source. */
	val = readl_relaxed(reg.add(4));
	val &= !SUPER_CDIV_ENB;
	writel_relaxed(val, reg.add(4));

	clk = clk_register(core::ptr::null_mut(), &mut (*super_).hw);
	if IS_ERR(clk) {
		kfree(super_ as *mut core::ffi::c_void);
	} else {
		cclk_super = super_;
	}

	return clk;
}

unsafe fn tegra_cclk_pre_pllx_rate_change() -> i32 {
	if IS_ERR_OR_NULL(cclk_super as *mut core::ffi::c_void) {
		return -EINVAL;
	}

	cclk_on_pllx = cclk_super_get_parent(&mut (*cclk_super).hw) == PLLX_INDEX;
	if cclk_on_pllx {
		cclk_super_set_parent(&mut (*cclk_super).hw, PLLP_INDEX);
	}

	return 0;
}

unsafe fn tegra_cclk_post_pllx_rate_change() {
	if cclk_on_pllx {
		cclk_super_set_parent(&mut (*cclk_super).hw, PLLX_INDEX);
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
